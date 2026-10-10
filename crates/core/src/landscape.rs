//! The landscape: rivers cut, slopes shed soil, rock turns to soil, sediment travels and settles.
//!
//! Follows the FastScape family (Braun and Willett 2013): implicit stream power along the flow
//! tree, so one pass over the tiles is enough and large steps stay stable.
//!
//! Each year:
//! 1. Rock rises by the uplift field (from the `uplift` account).
//! 2. Rock weathers into regolith at P0 exp(-k h) (Heimsath et al. 1997). Warmth and rain scale P0.
//!    Silicate weathering takes CO2 from the air and sends it to the sea as carbonate.
//! 3. Regolith creeps and slides into the river at a rate that rises with the slope inside the tile.
//! 4. Rivers cut by the stream power law E = K A^m S^n (m = 0.5, n = 1), using the year's runoff.
//!    Cutting takes stored sediment first, then regolith, then rock.
//! 5. What was cut travels downstream in three grain sizes. Each has a capacity set by flow and
//!    slope. Above capacity it drops: gravel soon, sand later, mud on floodplains, in lakes and at
//!    the coast. At sea the shelf keeps gravel, sand and half the mud. The rest goes to the abyss account.
//!
//! The uplift field is not looked up. It is set so the first year's erosion is exactly balanced, which
//! makes today's landscape stand still under today's climate. K is set so total erosion is the 19 Gt/yr
//! of sediment the world's rivers carry (Milliman and Farnsworth 2011). Both are stated in docs/DECISIONS.md.
//!
//! Not yet: glacial erosion and moraines, wind and loess, coastal waves, meanders.

use crate::earth::BANDS;
use crate::env::*;
use crate::flows::{Endpoint, FlowOut};
use crate::grid::Grid;
use crate::module::*;
use crate::routing::{Routing, NONE};
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;

pub const P0_BASE: f64 = 1.5e-4; // m/yr, inside the 0.05 to 2 mm/yr range of Heimsath et al.
pub const SOIL_K: f64 = 3.0; // 1/m, inside 2 to 4
pub const LAMBDA0: f64 = 3.0e-6; // 1/yr, share of regolith that creeps away on a gentle slope
pub const FRAC: [f64; 3] = [0.15, 0.25, 0.60]; // gravel, sand, mud from fresh rock and regolith
pub const CAP_COEF: [f64; 3] = [2.0e-5, 2.0e-4, 2.0e-3];
pub const SEDIMENT_TARGET_KG: f64 = 19.0e12; // kg per year, Milliman and Farnsworth 2011
pub const RHO_SED: f64 = 2700.0;
/// kg C taken from the air for each m3 of rock weathered: with about 7e9 m3 a year this is about 0.1 GtC/yr,
/// the order of the silicate weathering sink (Gaillardet et al. 1999; figure to be re-checked).
pub const CO2_PER_M3: f64 = 14.0;
const M3_PER_MKM2: f64 = 1.0e6; // one m x km2 is 1e6 m3
pub const SEC_PER_YEAR: f64 = crate::units::SECONDS_PER_YEAR;

pub fn sub_slope(env: &Env, i: usize) -> f64 {
    let relief = (env.p.z0_band(i, BANDS - 1) - env.p.z0_band(i, 0)).max(1.0) as f64;
    (relief / (0.5 * env.p.tile_km[i] as f64 * 1000.0)).min(1.0)
}

pub fn t_factor(t_ann_k: f64) -> f64 { 2f64.powf((t_ann_k - 283.15) / 10.0).clamp(0.05, 8.0) }
pub fn w_factor(runoff_mm: f64) -> f64 { (runoff_mm / 300.0).clamp(0.1, 1.8) }

/// Weathering rate P0 (m/yr) for a tile.
pub fn p0(t_ann_k: f64, runoff_mm: f64) -> f64 { P0_BASE * t_factor(t_ann_k) * w_factor(runoff_mm) }
pub fn creep_rate(env: &Env, i: usize) -> f64 { let s = sub_slope(env, i); LAMBDA0 * (1.0 + (s / 0.05).powi(2)) }

/// Regolith thickness (m) where weathering and creep balance: P0 exp(-k h) = lambda h.
pub fn steady_regolith(p0: f64, lambda: f64) -> f64 {
    let mut h = 1.0f64;
    for _ in 0..60 {
        let f = p0 * (-SOIL_K * h).exp() - lambda * h;
        let df = -SOIL_K * p0 * (-SOIL_K * h).exp() - lambda;
        let nh = (h - f / df).clamp(0.0, 30.0);
        if (nh - h).abs() < 1e-9 { h = nh; break; }
        h = nh;
    }
    h
}

pub struct Landscape {
    env: Env,
    ids: Option<Ids>,
    /// m/yr, signed: negative is subsidence
    uplift: Vec<f32>,
    k: f64,
    stage: u8, // 0: first call, clear the runoff; 1: calibrate on the next call; 2: running
    rt: Option<Routing>,
}
struct Ids {
    rock: StockId, reg: StockId, sg: StockId, ss: StockId, sm: StockId, defl: StockId,
    sea: StockId, heat: StockId, runoff: StockId, c_air: StockId, c_ocean: StockId, ice: Vec<StockId>,
}

struct Year { shift: Vec<f32>, sea: Vec<bool>, surface: Vec<f32>, runoff_mm: Vec<f32>, t_ann: Vec<f64>, icy: Vec<bool> }

/// What one year does, as numbers per tile.
struct Plan {
    weather: Vec<f64>,   // m of rock turned to regolith
    hill: Vec<f64>,      // m of regolith creeping into the river
    take: [Vec<f64>; 5], // m cut by the stream from sed_g, sed_s, sed_m, regolith, rock
    dep: [Vec<f64>; 3],  // m laid down on the tile, by grain size
    abyss: Vec<f64>,     // m x km2 lost to the deep sea at this tile
    co2: Vec<f64>,       // kg C per m2 taken from the air
    cut_m3: f64,         // total stream cutting, m3
    hill_m3: f64,
}

impl Landscape {
    pub fn new(env: Env) -> Landscape { Landscape { env, ids: None, uplift: Vec::new(), k: 1.0, stage: 0, rt: None } }
    pub fn uplift(&self) -> &[f32] { &self.uplift }

    #[allow(clippy::too_many_arguments)]
    fn year(&self, rock: &[f32], regl: &[f32], sg: &[f32], ss: &[f32], sm: &[f32], defl: &[f32], sea_w: &[f32], heat: &[f32], runoff: &[f32], ice: &[&[f32]]) -> Year {
        let env = &self.env;
        let n = env.n();
        let mut y = Year { shift: vec![0.0; n], sea: vec![false; n], surface: vec![0.0; n], runoff_mm: runoff.to_vec(), t_ann: vec![0.0; n], icy: vec![false; n] };
        for i in 0..n {
            y.shift[i] = env.bed_shift(i, rock[i], regl[i], sg[i], ss[i], sm[i], defl[i]);
            y.sea[i] = env.is_sea(i, y.shift[i], sea_w[i]);
            y.surface[i] = env.p.bed0[i] + y.shift[i];
            let mean_obs: f64 = (0..12).map(|k| env.tobs[k][i] as f64).sum::<f64>() / 12.0;
            y.t_ann[i] = mean_obs + (env.t_from_h(i, heat[i]) - env.tobs[0][i]) as f64;
            let ice_m: f32 = (0..BANDS).map(|b| Env::ice_m(ice[b][i])).sum::<f32>() / BANDS as f32;
            y.icy[i] = ice_m > 50.0;
        }
        y
    }

    #[allow(clippy::too_many_arguments)]
    fn plan(&self, grid: &Grid, y: &Year, rt: &Routing, regl: &[f32], sg: &[f32], ss: &[f32], sm: &[f32], k: f64, dt: f64) -> Plan {
        let env = &self.env;
        let n = env.n();
        let area = &grid.area_km2;
        let mut p = Plan { weather: vec![0.0; n], hill: vec![0.0; n], take: std::array::from_fn(|_| vec![0.0; n]), dep: std::array::from_fn(|_| vec![0.0; n]),
            abyss: vec![0.0; n], co2: vec![0.0; n], cut_m3: 0.0, hill_m3: 0.0 };
        // weathering and creep, inside the tile
        for i in 0..n {
            if y.sea[i] || y.icy[i] { continue; }
            let h = regl[i] as f64;
            let pr = p0(y.t_ann[i], y.runoff_mm[i] as f64);
            p.weather[i] = (pr * (-SOIL_K * h).exp() * dt).min(1.0);
            p.hill[i] = (creep_rate(env, i) * h * dt).min(0.5 * h);
            p.co2[i] = p.weather[i] * CO2_PER_M3;
            p.hill_m3 += p.hill[i] * area[i] as f64 * M3_PER_MKM2;
        }
        // discharge down the tree, m3/yr (1 mm over 1 km2 is 1e3 m3)
        let mut q = vec![0f64; n];
        for i in 0..n { q[i] = y.runoff_mm[i] as f64 * area[i] as f64 * 1.0e3; }
        for &c in rt.order.iter().rev() {
            let r = rt.recv[c as usize];
            if r != NONE { q[r as usize] += q[c as usize]; }
        }
        // stream cutting, implicit along the tree, outlets first
        let mut h_new: Vec<f64> = y.surface.iter().map(|&s| s as f64).collect();
        let mut slope = vec![0f64; n];
        let mut cut = vec![0f64; n];
        for &c in &rt.order {
            let c = c as usize;
            let r = rt.recv[c];
            if r == NONE || y.sea[c] { continue; }
            let r = r as usize;
            let dist = grid.nbr[c].iter().position(|&j| j == r as i32).map(|k| env.p.nbr_km[c][k] as f64 * 1000.0).unwrap_or(env.p.tile_km[c] as f64 * 1000.0);
            let lake = rt.fill[c] > y.surface[c] + 0.1;
            let relief = (env.p.z0_band(c, BANDS - 1) - env.p.z0_band(c, 0)).max(1.0) as f64;
            // A big tile holds whole valleys, so its channel is steeper than the drop to the next tile.
            // The sub-grid rule: the drop is at least 2 percent of the tile's own relief.
            let drop = (h_new[c] - h_new[r]).max(0.02 * relief);
            slope[c] = if lake { 0.0 } else { (drop / dist).max(0.1 * sub_slope(env, c)) };
            if y.icy[c] || lake { continue; }
            let a = (q[c] / 1.0e9).max(1e-9); // 1e9 m3/yr
            let f = dt * k * a.sqrt() / dist * 1.0e4;
            let e = (f * drop / (1.0 + f)).min(0.5 * relief);
            cut[c] = e;
            h_new[c] -= e;
            p.cut_m3 += e * area[c] as f64 * M3_PER_MKM2;
        }
        // loads: hillslope soil and stream cutting, split into grain sizes
        let mut load: [Vec<f64>; 3] = std::array::from_fn(|_| vec![0.0; n]);
        for i in 0..n {
            for g in 0..3 { load[g][i] += p.hill[i] * area[i] as f64 * FRAC[g]; }
            if cut[i] <= 0.0 { continue; }
            let mut left = cut[i];
            let avail = [sg[i] as f64, ss[i] as f64, sm[i] as f64, (regl[i] as f64 - p.hill[i]).max(0.0)];
            for (k2, a) in avail.iter().enumerate() {
                let take = left.min(a * 0.9);
                p.take[k2][i] = take; left -= take;
                if k2 < 3 { load[k2][i] += take * area[i] as f64; } else { for g in 0..3 { load[g][i] += take * area[i] as f64 * FRAC[g]; } }
            }
            p.take[4][i] = left;
            for g in 0..3 { load[g][i] += left * area[i] as f64 * FRAC[g]; }
        }
        // travel downstream, dropping what the flow cannot carry
        let mut carry = load;
        for &c in rt.order.iter().rev() {
            let c = c as usize;
            let r = rt.recv[c];
            if y.sea[c] {
                let deep = y.surface[c] < -200.0;
                for g in 0..3 {
                    let v = carry[g][c];
                    if v <= 0.0 { continue; }
                    let keep = if deep { 0.0 } else if g == 2 { 0.5 } else { 1.0 };
                    p.dep[g][c] += v * keep / area[c] as f64;
                    p.abyss[c] += v * (1.0 - keep);
                }
                continue;
            }
            if r == NONE {
                for g in 0..3 { p.dep[g][c] += carry[g][c] / area[c] as f64; }
                continue;
            }
            let lake = rt.fill[c] > y.surface[c] + 0.1;
            for g in 0..3 {
                let cap = if lake { 0.0 } else { CAP_COEF[g] * q[c] * slope[c] / M3_PER_MKM2 };
                let d = (carry[g][c] - cap).max(0.0);
                if d > 0.0 { p.dep[g][c] += d / area[c] as f64; }
                carry[g][r as usize] += carry[g][c] - d;
            }
        }
        p
    }

    fn routing(&self, grid: &Grid, y: &Year) -> Routing { Routing::build(grid, &y.surface, &y.sea, &grid.area_km2) }
}

impl Module for Landscape {
    fn spec(&self) -> ModuleSpec {
        let mut s = ModuleSpec::new("landscape", Phase::SoilPlants).every(12).boundaries(&["uplift", "abyss", "diag"]);
        s.reads = ["rock", "reg", "sed_g", "sed_s", "sed_m", "defl", "w_sea", "heat", "runoff_y", "c_air", "c_ocean"].iter().map(|x| x.to_string()).chain((0..BANDS).map(|b| format!("w_ice{b}"))).collect();
        s.writes = ["rock", "reg", "sed_g", "sed_s", "sed_m", "runoff_y", "c_air", "c_ocean"].iter().map(|x| x.to_string()).collect();
        s
    }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(Ids { rock: g("rock")?, reg: g("reg")?, sg: g("sed_g")?, ss: g("sed_s")?, sm: g("sed_m")?, defl: g("defl")?, sea: g("w_sea")?, heat: g("heat")?,
            runoff: g("runoff_y")?, c_air: g("c_air")?, c_ocean: g("c_ocean")?, ice: (0..BANDS).map(|b| g(&format!("w_ice{b}"))).collect::<Result<_, _>>()? });
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("landscape not bound".into()))?;
        let env = self.env.clone();
        let n = env.n();
        let grid = ctx.grid;
        let area = &grid.area_km2;
        let (rock, regl, sg, ss, sm, defl) = (view.stock(ids.rock)?, view.stock(ids.reg)?, view.stock(ids.sg)?, view.stock(ids.ss)?, view.stock(ids.sm)?, view.stock(ids.defl)?);
        let (sea_w, heat, runoff) = (view.stock(ids.sea)?, view.stock(ids.heat)?, view.stock(ids.runoff)?);
        let ice: Vec<&[f32]> = ids.ice.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        // the runoff handed over by hydro is a year's worth: take it all back
        out.dense(Endpoint::Stock(ids.runoff), Endpoint::Boundary(env.ph.b_diag), n).copy_from_slice(runoff);
        if self.stage == 0 { self.stage = 1; return Ok(()); }

        let y = self.year(rock, regl, sg, ss, sm, defl, sea_w, heat, runoff, &ice);
        let rt = self.routing(grid, &y);
        let dt = ctx.dt_months as f64 / 12.0;
        if self.stage == 1 {
            // Calibrate on the first full year of real runoff: one K, then the uplift that holds today's land still.
            let target_m3 = SEDIMENT_TARGET_KG / RHO_SED;
            let p1 = self.plan(grid, &y, &rt, regl, sg, ss, sm, 1.0, dt);
            let want_cut = (target_m3 * dt - p1.hill_m3).max(0.02 * target_m3 * dt);
            let (mut lo, mut hi) = (1e-12f64, 1e6f64);
            for _ in 0..60 {
                let mid = (lo * hi).sqrt();
                let c = self.plan(grid, &y, &rt, regl, sg, ss, sm, mid, dt).cut_m3;
                if c < want_cut { lo = mid } else { hi = mid }
            }
            self.k = (lo * hi).sqrt();
            let p = self.plan(grid, &y, &rt, regl, sg, ss, sm, self.k, dt);
            self.uplift = (0..n).map(|i| {
                if y.sea[i] { return -(p.dep[0][i] + p.dep[1][i] + p.dep[2][i]) as f32 / dt as f32; }
                let cut: f64 = (0..5).map(|k| p.take[k][i]).sum();
                ((p.hill[i] + cut - p.dep[0][i] - p.dep[1][i] - p.dep[2][i]) / dt) as f32
            }).collect();
            self.stage = 2;
        }
        let p = self.plan(grid, &y, &rt, regl, sg, ss, sm, self.k, dt);
        let (rock_id, reg_id) = (ids.rock, ids.reg);
        let sed = [ids.sg, ids.ss, ids.sm];
        // uplift and subsidence
        let (mut up, mut dn) = (vec![0f32; n], vec![0f32; n]);
        for i in 0..n { let u = self.uplift[i] as f64 * dt; if u >= 0.0 { up[i] = u as f32; } else { dn[i] = (-u) as f32; } }
        out.dense(Endpoint::Boundary(env.ph.b_uplift), Endpoint::Stock(rock_id), n).copy_from_slice(&up);
        out.dense(Endpoint::Stock(rock_id), Endpoint::Boundary(env.ph.b_uplift), n).copy_from_slice(&dn);
        // weathering rock -> regolith
        let w32: Vec<f32> = p.weather.iter().map(|&v| v as f32).collect();
        out.dense_local(rock_id, reg_id, n).copy_from_slice(&w32);
        // CO2: from the air where the rock weathers to the sea where the river ends
        let sources = [ids.sg, ids.ss, ids.sm, reg_id, rock_id];
        let tr = Endpoint::Transit;
        for i in 0..n {
            let a = area[i] as f64;
            // into the river
            if p.hill[i] > 0.0 { out.push(Endpoint::Stock(reg_id), tr, i, 0, p.hill[i] * a); }
            for (k, s) in sources.iter().enumerate() { if p.take[k][i] > 0.0 { out.push(Endpoint::Stock(*s), tr, i, 0, p.take[k][i] * a); } }
            // out of the river
            for g in 0..3 { if p.dep[g][i] > 0.0 { out.push(tr, Endpoint::Stock(sed[g]), 0, i, p.dep[g][i] * a); } }
            if p.abyss[i] > 0.0 { out.push(tr, Endpoint::Boundary(env.ph.b_abyss), 0, i, p.abyss[i]); }
            if p.co2[i] > 0.0 {
                let d = rt.dest_dry[i] as usize;
                out.push(Endpoint::Stock(ids.c_air), Endpoint::Stock(ids.c_ocean), i, d, p.co2[i] * a);
            }
        }
        self.rt = Some(rt);
        Ok(())
    }
    fn save_state(&self) -> Option<Vec<u8>> {
        let mut b = vec![self.stage];
        b.extend(self.k.to_le_bytes());
        for u in &self.uplift { b.extend(u.to_le_bytes()); }
        Some(b)
    }
    fn load_state(&mut self, b: &[u8]) -> Result<(), WorldError> {
        if b.len() < 9 { return Err(WorldError::Save("landscape state".into())); }
        self.stage = b[0];
        self.k = f64::from_le_bytes(b[1..9].try_into().unwrap());
        self.uplift = b[9..].chunks_exact(4).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect();
        Ok(())
    }
}

/// Starting regolith: where weathering and creep balance under today's climate.
pub fn initial_regolith(env: &Env) -> Vec<f32> {
    (0..env.n()).map(|i| {
        if env.ocean0[i] || env.p.earth.ice[i] > 100.0 { return 0.0; }
        let t_ann: f64 = (0..12).map(|m| env.tobs[m][i] as f64).sum::<f64>() / 12.0;
        let runoff = (0..12).map(|m| env.p.earth.prec[m][i] as f64).sum::<f64>() * 0.3;
        steady_regolith(p0(t_ann, runoff), creep_rate(env, i)) as f32
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn soil_thickens_where_it_is_not_carried_away() {
        let gentle = steady_regolith(1.5e-4, 3e-6);
        let steep = steady_regolith(1.5e-4, 3e-6 * (1.0 + 25.0));
        assert!(gentle > steep * 1.5, "gentle {gentle} steep {steep}");
        assert!(gentle > 0.3 && gentle < 3.0, "{gentle}");
        // balance holds
        let h = gentle;
        assert!((1.5e-4 * (-SOIL_K * h).exp() - 3e-6 * h).abs() < 1e-9);
    }
    #[test]
    fn weathering_rises_with_warmth_and_rain() {
        assert!(p0(298.0, 600.0) > p0(278.0, 600.0));
        assert!(p0(288.0, 900.0) > p0(288.0, 100.0));
    }
}
