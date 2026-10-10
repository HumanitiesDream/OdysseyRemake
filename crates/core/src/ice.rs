//! Ice flow. Ice is a stock the climate builds (snow that survives, in `hydro`) and this
//! module moves.
//!
//! - Between tiles: the shallow-ice approximation (Glen's law, n = 3). Flux between two tiles
//!   goes with thickness^5 and surface slope^2 times the height drop. It runs in sub-steps
//!   small enough to stay stable.
//! - Inside a tile: ice in a high band slides down to the band below, at the same law.
//! - At the sea: ice reaching a sea tile calves into it. Ice resting below sea level that
//!   is too thin to hold its bed down floats off at a fixed share a year.
//!
//! Not yet: ice streams on soft beds, ice shelves, rock debris carried by the ice.

use crate::earth::BANDS;
use crate::env::*;
use crate::flows::{Endpoint, FlowOut};
use crate::grid::Grid;
use crate::module::*;
use crate::planet::RHO_ICE;
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;

/// 2A(rho g)^n/(n+2) in 1/(m^3 yr), n = 3, with A = 1.1e-17 /(Pa^3 yr) at -10 C
/// (Cuffey and Paterson 2010 table, rounded) and an Arrhenius law with Q = 60 kJ/mol.
const GAMMA_REF: f64 = 3.2e-6;
fn gamma(t_ice_k: f64) -> f64 { GAMMA_REF * (7217.0 * (1.0 / 263.15 - 1.0 / t_ice_k.min(273.15))).exp() }
/// Calving: ice leaves a sea-side tile at U x H per metre of coast, with U the speed of the
/// ice front (about 400 m/yr for Antarctica: 2,200 km3/yr over 18,000 km of coast at 300 m)
/// and H the thickness at the front, capped at 400 m. Per metre of coast, so it does not
/// depend on the size of the tile.
/// One cubic metre of ice, in mm of water over a km2: 917 kg / (1e-3 m x 1e6 m2 x 1000 kg/m3).
const M3_ICE_TO_MM_KM2: f64 = 917.0 / 1.0e6;
/// Ice flow and melt both draw on the same stock in the same month. Each may take at most this
/// share (flow) or half (melt), so together they cannot overdraw it.
const SHARE: f64 = 0.45;
const CALVE_SPEED_M: f64 = 400.0;
const FRONT_H_M: f64 = 400.0;
/// Share of the tile width that valley ice uses when it slides down between bands.
const VALLEY: f64 = 0.15;
/// Years in which the calving speed is calibrated.
const CALIB_YEARS: u8 = 3;

pub struct IceFlow { env: Env, ids: Option<Ids>, calv: f64, prev_mass: f64, prev_calving: f64, calls: u8 }
struct Ids { ice: Vec<StockId>, sea: StockId, rock: StockId, reg: StockId, sg: StockId, ss: StockId, sm: StockId, defl: StockId }

impl IceFlow {
    pub fn new(env: Env) -> Self { IceFlow { env, ids: None, calv: 1.0, prev_mass: 0.0, prev_calving: 0.0, calls: 0 } }
}

impl Module for IceFlow {
    fn spec(&self) -> ModuleSpec {
        let mut s = ModuleSpec::new("iceflow", Phase::Water).every(12);
        s.reads = ["w_sea", "rock", "reg", "sed_g", "sed_s", "sed_m", "defl"].iter().map(|x| x.to_string()).chain((0..BANDS).map(|b| format!("w_ice{b}"))).collect();
        s.writes = ["w_sea"].iter().map(|x| x.to_string()).chain((0..BANDS).map(|b| format!("w_ice{b}"))).collect();
        s
    }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(Ids { ice: (0..BANDS).map(|b| g(&format!("w_ice{b}"))).collect::<Result<_, _>>()?, sea: g("w_sea")?,
            rock: g("rock")?, reg: g("reg")?, sg: g("sed_g")?, ss: g("sed_s")?, sm: g("sed_m")?, defl: g("defl")? });
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("iceflow not bound".into()))?;
        let env = &self.env;
        let n = env.n();
        let grid = ctx.grid;
        let area = &grid.area_km2;
        let dt = ctx.dt_months as f64 / 12.0;
        let w_sea = view.stock(ids.sea)?;
        let ice: Vec<&[f32]> = ids.ice.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let (rock, regl, sg, ss, sm, defl) = (view.stock(ids.rock)?, view.stock(ids.reg)?, view.stock(ids.sg)?, view.stock(ids.ss)?, view.stock(ids.sm)?, view.stock(ids.defl)?);

        // tile-level fields
        let mut shift = vec![0f32; n];
        let mut sea = vec![false; n];
        let mut mass = vec![0f64; n]; // mm w.e. over the tile, summed over bands
        for i in 0..n {
            shift[i] = env.bed_shift(i, rock[i], regl[i], sg[i], ss[i], sm[i], defl[i]);
            sea[i] = env.is_sea(i, shift[i], w_sea[i]);
            mass[i] = (0..BANDS).map(|b| ice[b][i] as f64).sum();
        }
        if mass.iter().all(|&m| m <= 0.0) { return Ok(()); }
        // Calibration of the calving speed. Today's ice sheets are taken to be near balance. In each of the first
        // three years the net change of the whole ice stock is turned into a change of calving, so that the ice
        // starts level. After that the calving speed is fixed and the ice follows the climate freely.
        let total: f64 = (0..n).map(|i| mass[i] * area[i] as f64).sum();
        if self.calls >= 1 && self.calls <= CALIB_YEARS && self.prev_calving > 0.0 {
            let net = total - self.prev_mass;
            self.calv = (self.calv * (1.0 + net / self.prev_calving)).clamp(0.2 * self.calv.min(1.0), 3.0);
        }
        self.prev_mass = total;
        self.calls = self.calls.saturating_add(1);
        let mut calved = 0f64;
        // ice temperature: the tile's mean annual air temperature, never above freezing
        let t_ice: Vec<f64> = (0..n).map(|i| (0..12).map(|m| env.tobs[m][i] as f64).sum::<f64>() / 12.0).collect();
        let bed_mean = |i: usize| env.p.bed0[i] + shift[i];
        // thickness as a tile mean, m: total stock over the tile, as ice
        let thick = |m: f64| m / RHO_ICE as f64;
        let mut m_cur = mass.clone();
        let mut moved: std::collections::BTreeMap<(u32, u32), f64> = std::collections::BTreeMap::new();

        // sub-steps
        let dx_min = (0..n).filter(|&i| mass[i] > 0.0).map(|i| env.p.tile_km[i] as f64 * 1000.0).fold(f64::MAX, f64::min);
        let h_max = m_cur.iter().cloned().fold(0.0, f64::max) / RHO_ICE as f64 * 1.0;
        let dmax = gamma(273.15) * h_max.powi(5) * (0.05f64).powi(2); // slope 0.05 as the working maximum
        let nsub = ((dt * dmax * 4.0 / (dx_min * dx_min)).ceil() as usize).clamp(1, 120);
        let h = dt / nsub as f64;
        for _ in 0..nsub {
            let mut dm = vec![0f64; n];
            for i in 0..n {
                if m_cur[i] <= 0.0 { continue; }
                let h_i = thick(m_cur[i]);
                let s_i = bed_mean(i) as f64 + h_i;
                let mut outflow = 0.0;
                let mut plan: Vec<(usize, f64)> = Vec::new();
                for (k, j) in grid.nbr[i].iter().enumerate() {
                    if *j < 0 { continue; }
                    let j = *j as usize;
                    let dist = env.p.nbr_km[i][k] as f64 * 1000.0;
                    let s_j = if sea[j] { env.p.bed0[j] as f64 + shift[j] as f64 } else { bed_mean(j) as f64 + thick(m_cur[j]) };
                    let ds = s_i - s_j;
                    if ds <= 0.0 { continue; }
                    let h_ij = if sea[j] { h_i } else { 0.5 * (h_i + thick(m_cur[j])) };
                    let g = gamma(t_ice[i]);
                    let d = g * h_ij.powi(5) * (ds / dist).powi(2); // m2/yr
                    // volume m3 over the step; edge length ~ dist/sqrt(3)
                    let v = h * d * ds / 3f64.sqrt();
                    let amount = v * M3_ICE_TO_MM_KM2; // mm km2
                    plan.push((j, amount));
                    outflow += amount;
                }
                // a tile cannot give away more than a fifth of what it holds in one sub-step
                let cap = 0.2 * m_cur[i] * area[i] as f64;
                let scale = if outflow > cap { cap / outflow } else { 1.0 };
                for (j, a) in plan {
                    let a = a * scale;
                    dm[i] -= a / area[i] as f64;
                    if !sea[j] { dm[j] += a / area[j] as f64; }
                    *moved.entry((i as u32, j as u32)).or_insert(0.0) += a;
                }
            }
            for i in 0..n { m_cur[i] = (m_cur[i] + dm[i]).max(0.0); }
        }
        // floating ice calves into the sea next door
        for i in 0..n {
            if m_cur[i] <= 0.0 || sea[i] { continue; }
            let bed = bed_mean(i) as f64;
            let _ = bed;
            for (k, j) in grid.nbr[i].iter().enumerate() {
                if *j < 0 || !sea[*j as usize] { continue; }
                let j = *j as usize;
                {
                    let edge_m = env.p.nbr_km[i][k] as f64 * 1000.0 / 3f64.sqrt();
                    let v = dt * CALVE_SPEED_M * self.calv * thick(m_cur[i]).min(FRONT_H_M) * edge_m; // m3
                    let a = (v * M3_ICE_TO_MM_KM2).min(0.2 * m_cur[i] * area[i] as f64);
                    *moved.entry((i as u32, j as u32)).or_insert(0.0) += a;
                    m_cur[i] -= a / area[i] as f64;
                    calved += a;
                }
            }
        }
        self.prev_calving = calved;
        // A tile cannot give away more than it holds. Ice that came in earlier in the year and went
        // out again is booked against the start-of-year bands, so keep a margin.
        {
            let mut out_i = vec![0f64; n];
            for ((i, _), a) in &moved { out_i[*i as usize] += a; }
            for ((i, _), a) in moved.iter_mut() {
                let have = mass[*i as usize] * area[*i as usize] as f64;
                let want = out_i[*i as usize];
                if want > SHARE * have { *a *= SHARE * have / want; }
            }
        }
        // book the moves: take from the donor's bands by share, put into the receiver's bands the same way
        for ((i, j), a) in &moved {
            let (i, j) = (*i as usize, *j as usize);
            if mass[i] <= 0.0 { continue; }
            for b in 0..BANDS {
                let share = ice[b][i] as f64 / mass[i];
                if share <= 0.0 { continue; }
                let amt = a * share;
                if sea[j] { out.push(Endpoint::Stock(ids.ice[b]), Endpoint::Stock(ids.sea), i, j, amt); }
                else { out.push(Endpoint::Stock(ids.ice[b]), Endpoint::Stock(ids.ice[b]), i, j, amt); }
            }
        }
        // what the lateral moves and calving already take from each tile
        let mut taken = vec![0f64; n];
        for ((i, _), a) in &moved { taken[*i as usize] += a / area[*i as usize] as f64; }
        // ice sliding down between bands inside a tile
        let spacing_div = BANDS as f64 * 0.5;
        let mut down: Vec<Vec<f32>> = (0..BANDS).map(|_| vec![0f32; n]).collect();
        let mut any = false;
        for i in 0..n {
            if mass[i] <= 0.0 || sea[i] { continue; }
            let width = env.p.tile_km[i] as f64 * 1000.0 * VALLEY;
            let l_band = env.p.tile_km[i] as f64 * 1000.0 / spacing_div;
            for b in (1..BANDS).rev() {
                let loc = Env::ice_m(ice[b][i]) as f64;
                if loc <= 1.0 { continue; }
                let s_hi = env.band_bed(i, b, shift[i]) as f64 + loc;
                let s_lo = env.band_bed(i, b - 1, shift[i]) as f64 + Env::ice_m(ice[b - 1][i]) as f64;
                let slope = (s_hi - s_lo) / l_band;
                if slope <= 0.0 { continue; }
                let q = gamma(t_ice[i]) * loc.powi(5) * slope.powi(3); // m2/yr
                let band_area = area[i] as f64 * 1e6 / BANDS as f64;
                let frac = (q * width * dt / (band_area * loc)).min(0.3);
                let left = if mass[i] > 0.0 { (1.0 - taken[i] / mass[i]).max(0.0) } else { 0.0 };
                let moved_stock = (ice[b][i] as f64 * frac * left * SHARE) as f32;
                if moved_stock > 0.0 { down[b][i] = moved_stock; any = true; }
            }
        }
        if any { for b in 1..BANDS { out.dense_local(ids.ice[b], ids.ice[b - 1], n).copy_from_slice(&down[b]); } }
        let _ = Endpoint::Transit;
        Ok(())
    }
    fn save_state(&self) -> Option<Vec<u8>> {
        let mut b = vec![self.calls];
        for v in [self.calv, self.prev_mass, self.prev_calving] { b.extend(v.to_le_bytes()); }
        Some(b)
    }
    fn load_state(&mut self, b: &[u8]) -> Result<(), WorldError> {
        if b.len() < 25 { return Err(WorldError::Save("iceflow state".into())); }
        self.calls = b[0];
        let f = |k: usize| f64::from_le_bytes(b[1 + 8 * k..9 + 8 * k].try_into().unwrap());
        self.calv = f(0); self.prev_mass = f(1); self.prev_calving = f(2);
        Ok(())
    }
}
