//! Earthquakes and volcanoes as stocks that fill and release.
//!
//! Quakes. Each tile owes moment (1e18 N m) that plate motion adds at the long-term rate read
//! from 125 years of the USGS catalogue (M 5.5 and up). A quake releases some of what is owed.
//! The chance of one grows with what is owed. Its size follows Gutenberg-Richter (b = 1, so the
//! moment has a Pareto tail with exponent 2/3) and cannot exceed the stock. So a tile that just
//! broke stays quiet and then reloads. A quake lifts the ground on one side and drops it on the
//! next tile, shakes regolith loose into colluvium, and above M 7.3 on a coast raises a tsunami.
//!
//! Volcanoes. Each tile owes magma at a rate set by its significant eruptions since 8000 BC (NOAA).
//! An eruption releases a random share of what is owed, so the size follows the time since the last
//! one. The volume sets the VEI. Lava stays as new rock, ash settles as new regolith on the tile
//! and its neighbours, and above VEI 2 the sulphur that reaches the stratosphere dims the sun
//! (about 0.15 W/m2 per Mt SO2 and a one-year e-fold: Pinatubo, 20 Mt, gave about 3 W/m2).

use crate::earth::{nearest, unit};
use crate::env::*;
use crate::flows::{Endpoint, Event, EventKind, FlowOut};
use crate::grid::Grid;
use crate::module::*;
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;
use std::path::Path;

pub const MW_MIN: f64 = 5.5;
const ALPHA: f64 = 2.0 / 3.0;
const T_EQ_QUAKE: f64 = 600.0;
const T_RET_VOLCANO: f64 = 300.0;
const SHEAR_MODULUS: f64 = 3.0e10;
pub const SO2_FORCING_PER_MT: f64 = 0.15;

pub fn moment_1e18(mw: f64) -> f64 { 10f64.powf(1.5 * mw + 9.1 - 18.0) }
pub fn mw_of(moment_1e18: f64) -> f64 { ((moment_1e18 * 1e18).log10() - 9.1) / 1.5 }

/// Bulk eruption volume in km3 for a VEI (mid-range of each class).
pub fn vei_volume_km3(vei: i32) -> f64 { [1e-5, 5e-4, 4e-3, 0.04, 0.4, 4.0, 40.0, 400.0, 400.0][vei.clamp(0, 8) as usize] }
pub fn vei_of_m3(v: f64) -> i32 { if v < 1e4 { 0 } else if v < 1e6 { 1 } else { ((v.log10().floor() as i32) - 4).clamp(2, 8) } }
pub fn so2_mt(vei: i32) -> f64 { if vei < 3 { 0.0 } else { 20.0 * 10f64.powi(vei - 6) } }

#[derive(Debug, Clone)]
pub struct HazardMap {
    /// 1e18 N m per year
    pub quake_moment: Vec<f64>,
    /// m of rock per year
    pub magma_m: Vec<f64>,
}

impl HazardMap {
    pub fn load(grid: &Grid, dir: &Path) -> Result<HazardMap, String> {
        let n = grid.len();
        let mut quake_moment = vec![0f64; n];
        let mut magma_m = vec![0f64; n];
        let read = |f: &str| std::fs::read_to_string(dir.join(f)).map_err(|e| format!("{f}: {e}"));
        let mut hint = 0usize;
        for line in read("quakes_m55.csv")?.lines().skip(1) {
            let c: Vec<&str> = line.split(',').collect();
            if c.len() < 4 { continue; }
            let (lat, lon, mag): (f64, f64, f64) = (c[1].parse().map_err(|_| "bad lat")?, c[2].parse().map_err(|_| "bad lon")?, c[3].parse().map_err(|_| "bad mag")?);
            hint = nearest(grid, unit(lat, lon), hint);
            quake_moment[hint] += moment_1e18(mag) / 125.0;
        }
        for line in read("eruptions.csv")?.lines().skip(1) {
            let c: Vec<&str> = line.splitn(5, ',').collect();
            if c.len() < 4 { continue; }
            let (lat, lon, vei): (f64, f64, i32) = (c[1].parse().map_err(|_| "bad lat")?, c[2].parse().map_err(|_| "bad lon")?, c[3].parse().map_err(|_| "bad vei")?);
            hint = nearest(grid, unit(lat, lon), hint);
            magma_m[hint] += vei_volume_km3(vei) * 1000.0 / grid.area_km2[hint] as f64 / 12_000.0;
        }
        Ok(HazardMap { quake_moment, magma_m })
    }
}

impl HazardMap {
    /// Starting moment owed, 1e18 N m per km2: somewhere between nothing and a little over the long-run level.
    pub fn initial_strain(&self, grid: &Grid, seed: u64) -> Vec<f32> {
        let mut r = crate::rng::Rng::derive(seed, "strain0", 0);
        (0..grid.len()).map(|i| (self.quake_moment[i] * T_EQ_QUAKE * (0.2 + r.f64()) / grid.area_km2[i] as f64) as f32).collect()
    }
    pub fn initial_magma(&self, seed: u64) -> Vec<f32> {
        let mut r = crate::rng::Rng::derive(seed, "magma0", 0);
        self.magma_m.iter().map(|&m| (m * T_RET_VOLCANO * (0.3 + 1.2 * r.f64())) as f32).collect()
    }
}

/// Mean of a Pareto(alpha) moment truncated to [m_min, m_max].
fn mean_event(m_min: f64, m_max: f64) -> f64 {
    if m_max <= m_min * 1.0000001 { return m_min; }
    let a = ALPHA;
    a * m_min.powf(a) / (1.0 - a) * (m_max.powf(1.0 - a) - m_min.powf(1.0 - a)) / (1.0 - (m_min / m_max).powf(a))
}

pub struct Quakes { env: Env, map: std::sync::Arc<HazardMap>, ids: Option<QIds>, active: Vec<usize> }
struct QIds { strain: StockId, rock: StockId, reg: StockId, sg: StockId, ss: StockId }

impl Quakes {
    pub fn new(env: Env, map: std::sync::Arc<HazardMap>) -> Self {
        let active = (0..env.n()).filter(|&i| map.quake_moment[i] > 0.0).collect();
        Quakes { env, map, ids: None, active }
    }
}

impl Module for Quakes {
    fn spec(&self) -> ModuleSpec {
        ModuleSpec::new("quakes", Phase::Climate).reads(&["strain", "reg"]).writes(&["strain", "rock", "reg", "sed_g", "sed_s"]).boundaries(&["plates", "rupture", "uplift"])
    }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(QIds { strain: g("strain")?, rock: g("rock")?, reg: g("reg")?, sg: g("sed_g")?, ss: g("sed_s")? });
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("quakes not bound".into()))?;
        let ph = &self.env.ph;
        let grid = ctx.grid;
        let area = &grid.area_km2;
        let n = self.env.n();
        let strain = view.stock(ids.strain)?;
        let regl = view.stock(ids.reg)?;
        let dt_yr = ctx.dt_months as f64 / 12.0;
        let mut load = vec![0f32; n];
        for &i in &self.active { load[i] = (self.map.quake_moment[i] * dt_yr / area[i] as f64) as f32; }
        out.dense(Endpoint::Boundary(ph.b_plates), Endpoint::Stock(ids.strain), n).copy_from_slice(&load);
        let mut rng = ctx.rng("quakes");
        let m_min = moment_1e18(MW_MIN);
        let m_cap = moment_1e18(9.5);
        for &i in &self.active {
            let owed = strain[i] as f64 * area[i] as f64;
            if owed < m_min { continue; }
            let supply = self.map.quake_moment[i];
            let m_max = owed.min(m_cap);
            let rate_yr = supply / mean_event(m_min, m_max) * (owed / (supply * T_EQ_QUAKE)).min(10.0);
            let p = 1.0 - (-rate_yr * dt_yr).exp();
            if rng.f64() >= p { continue; }
            let u = rng.f64();
            let m = m_min * (1.0 - u * (1.0 - (m_min / m_max).powf(ALPHA))).powf(-1.0 / ALPHA);
            let m = m.min(owed);
            let mw = mw_of(m);
            out.push(Endpoint::Stock(ids.strain), Endpoint::Boundary(ph.b_rupture), i, 0, m);
            // ground motion: lift one side, drop the next tile
            let rupture_km2 = 10f64.powf((mw - 4.07) / 0.98);
            let slip = m * 1e18 / (SHEAR_MODULUS * rupture_km2 * 1e6);
            let dh = 0.3 * slip * (rupture_km2 / area[i] as f64).min(1.0);
            let nb: Vec<usize> = grid.neighbours(i).collect();
            let j = nb[(rng.next_u64() % nb.len() as u64) as usize];
            out.push(Endpoint::Boundary(ph.b_uplift), Endpoint::Stock(ids.rock), 0, i, dh * area[i] as f64);
            out.push(Endpoint::Stock(ids.rock), Endpoint::Boundary(ph.b_uplift), j, 0, dh * area[i] as f64);
            // shaken regolith slumps into colluvium on the tile
            let f = ((mw - 5.5) * 0.03).clamp(0.0, 0.25);
            let slump = f * regl[i] as f64 * area[i] as f64;
            if slump > 0.0 {
                out.push(Endpoint::Stock(ids.reg), Endpoint::Stock(ids.sg), i, i, 0.5 * slump);
                out.push(Endpoint::Stock(ids.reg), Endpoint::Stock(ids.ss), i, i, 0.5 * slump);
                out.events.push(Event { month: ctx.month_index, kind: EventKind::Landslide, tile: i as u32, size: f as f32 });
            }
            out.events.push(Event { month: ctx.month_index, kind: EventKind::Quake, tile: i as u32, size: mw as f32 });
            if mw >= 7.3 && (self.env.ocean0[i] || nb.iter().any(|&k| self.env.ocean0[k])) {
                out.events.push(Event { month: ctx.month_index, kind: EventKind::Tsunami, tile: i as u32, size: mw as f32 });
            }
        }
        Ok(())
    }
}

pub struct Volcanoes { env: Env, map: std::sync::Arc<HazardMap>, ids: Option<VIds>, active: Vec<usize> }
struct VIds { magma: StockId, rock: StockId, reg: StockId, aerosol: StockId }

impl Volcanoes {
    pub fn new(env: Env, map: std::sync::Arc<HazardMap>) -> Self {
        let active = (0..env.n()).filter(|&i| map.magma_m[i] > 0.0).collect();
        Volcanoes { env, map, ids: None, active }
    }
}

impl Module for Volcanoes {
    fn spec(&self) -> ModuleSpec {
        ModuleSpec::new("volcanoes", Phase::Climate).reads(&["magma", "aerosol"]).writes(&["magma", "rock", "reg", "aerosol"]).boundaries(&["melt", "diag"])
    }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(VIds { magma: g("magma")?, rock: g("rock")?, reg: g("reg")?, aerosol: g("aerosol")? });
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("volcanoes not bound".into()))?;
        let ph = &self.env.ph;
        let grid = ctx.grid;
        let area = &grid.area_km2;
        let n = self.env.n();
        let magma = view.stock(ids.magma)?;
        let aer = view.stock(ids.aerosol)?;
        let dt_yr = ctx.dt_months as f64 / 12.0;
        let mut supply = vec![0f32; n];
        for &i in &self.active { supply[i] = (self.map.magma_m[i] * dt_yr) as f32; }
        out.dense(Endpoint::Boundary(ph.b_melt), Endpoint::Stock(ids.magma), n).copy_from_slice(&supply);
        // the sulphur veil fades with a one-year e-fold
        if aer.iter().any(|&a| a > 0.0) {
            let k = (1.0 - (-dt_yr).exp()) as f32;
            let fade: Vec<f32> = aer.iter().map(|&a| a * k).collect();
            out.dense(Endpoint::Stock(ids.aerosol), Endpoint::Boundary(ph.b_diag), n).copy_from_slice(&fade);
        }
        let mut rng = ctx.rng("volcanoes");
        let total_area = grid.total_area_km2();
        let mut new_aerosol = 0.0f64;
        for &i in &self.active {
            let s = magma[i] as f64;
            if s <= 0.0 { continue; }
            let s_eq = self.map.magma_m[i] * T_RET_VOLCANO;
            let rate = (s / s_eq) / T_RET_VOLCANO;
            let p = 1.0 - (-rate * dt_yr).exp();
            if rng.f64() >= p { continue; }
            let f = 0.2 + 0.8 * rng.f64();
            let v_m = f * s; // m of rock over the tile
            let v_m3 = v_m * area[i] as f64 * 1e6;
            let vei = vei_of_m3(v_m3);
            let a = area[i] as f64;
            // lava becomes rock; ash becomes regolith
            out.push(Endpoint::Stock(ids.magma), Endpoint::Stock(ids.rock), i, i, 0.6 * v_m * a);
            if vei >= 4 {
                out.push(Endpoint::Stock(ids.magma), Endpoint::Stock(ids.reg), i, i, 0.2 * v_m * a);
                let nb: Vec<usize> = grid.neighbours(i).collect();
                for &j in &nb { out.push(Endpoint::Stock(ids.magma), Endpoint::Stock(ids.reg), i, j, 0.2 * v_m * a / nb.len() as f64); }
            } else {
                out.push(Endpoint::Stock(ids.magma), Endpoint::Stock(ids.reg), i, i, 0.4 * v_m * a);
            }
            new_aerosol += so2_mt(vei);
            out.events.push(Event { month: ctx.month_index, kind: EventKind::Eruption, tile: i as u32, size: vei as f32 });
        }
        if new_aerosol > 0.0 {
            let d = vec![(new_aerosol / total_area) as f32; n];
            out.dense(Endpoint::Boundary(ph.b_diag), Endpoint::Stock(ids.aerosol), n).copy_from_slice(&d);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn moment_magnitude_round_trip() {
        for mw in [5.5, 6.0, 7.0, 8.2, 9.0] { assert!((mw_of(moment_1e18(mw)) - mw).abs() < 1e-9); }
        // Mw 7 is 3.5e19 N m: 35 in units of 1e18
        assert!((moment_1e18(7.0) - 35.5).abs() < 1.0);
    }
    #[test]
    fn vei_classes() {
        assert_eq!(vei_of_m3(5e3), 0);
        assert_eq!(vei_of_m3(5e5), 1);
        assert_eq!(vei_of_m3(5e6), 2);
        assert_eq!(vei_of_m3(5e9), 5);
        assert_eq!(vei_of_m3(5e10), 6);
        assert!((so2_mt(6) - 20.0).abs() < 1e-9);
    }
    #[test]
    fn mean_event_is_bounded() {
        let a = moment_1e18(5.5);
        assert!(mean_event(a, a) == a);
        let m = mean_event(a, moment_1e18(9.0));
        assert!(m > a && m < moment_1e18(9.0));
        // heavy tail: the mean sits far above the smallest event
        assert!(m > 5.0 * a);
    }
}
