//! Demo world: three small placeholder modules that prove the groundwork.
//!
//! These are NOT the climate or water model of the plan. They are the smallest
//! modules that use every part of the framework: boundary accounts (sun, space),
//! a coarse-grid module with an implicit solve and a transit split, lateral tile
//! to tile flows, module periods, and conserved water. Every constant marked
//! "placeholder" is a stand-in and is to be replaced when the real module lands.
//!
//! - `demo_radiation`: Budyko-Sellers energy balance per tile (North et al. 1981:
//!   OLR = A + B (T - 273.15), A = 203.3 W/m2, B = 2.09 W/m2/K; figures from memory).
//! - `demo_mixing`: heat diffusion on a coarse grid, solved implicitly, because an
//!   explicit step at tile size is unstable by a factor of about 100.
//! - `demo_water`: evaporation, advection, rain, snow melt and runoff. Closed: no boundary.

use crate::flows::{Endpoint, FlowOut};
use crate::grid::{Coarse, Grid, EARTH_RADIUS_KM};
use crate::module::{Module, ModuleSpec, Phase, StepCtx, View};
use crate::rng::Rng;
use crate::sim::Sim;
use crate::stocks::{BoundaryId, Group, Registry, Storage, StockId};
use crate::units::SECONDS_PER_MONTH;
use crate::world::{World, WorldError};
use std::sync::Arc;

const S0: f64 = 1361.0;
const ALBEDO: f64 = 0.30; // placeholder: fixed, later from ice and plants
const OLR_A: f64 = 203.3;
const OLR_B: f64 = 2.09;
const DIFF_D: f64 = 0.649; // W/m2/K on the unit sphere, North et al. 1981 (from memory)
const C_LAND: f64 = 10.0; // MJ/m2/K, placeholder
const C_SEA: f64 = 150.0; // MJ/m2/K, placeholder: about a 36 m mixed layer
const MJ_PER_W_MONTH: f64 = SECONDS_PER_MONTH * 1e-6;

/// Daily-mean insolation in W/m2 at latitude `lat` (radians) for a solar declination.
pub fn insolation(lat: f64, decl: f64) -> f64 {
    // an f32 latitude at the pole can sit a hair past pi/2
    let lat = lat.clamp(-std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2);
    let x = -lat.tan() * decl.tan();
    let h0 = if x >= 1.0 { 0.0 } else if x <= -1.0 { std::f64::consts::PI } else { x.acos() };
    (S0 / std::f64::consts::PI * (h0 * lat.sin() * decl.sin() + lat.cos() * decl.cos() * h0.sin())).max(0.0)
}

pub fn declination(month_of_year: u32) -> f64 {
    // zero near 21 March, which falls at month-index 2.7 counted from 1 January
    23.44f64.to_radians() * (2.0 * std::f64::consts::PI * (month_of_year as f64 + 0.5 - 2.7) / 12.0).sin()
}

pub struct Radiation { heat: StockId, sun: BoundaryId, space: BoundaryId, cap: Vec<f32> }
impl Radiation {
    pub fn new(cap: Vec<f32>) -> Self { Radiation { heat: StockId(0), sun: BoundaryId(0), space: BoundaryId(0), cap } }
}
impl Module for Radiation {
    fn spec(&self) -> ModuleSpec { ModuleSpec::new("demo_radiation", Phase::Climate).reads(&["heat"]).writes(&["heat"]).boundaries(&["sun", "space"]) }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
        self.heat = reg.stock("heat").ok_or_else(|| WorldError::Unknown("heat".into()))?;
        self.sun = reg.boundary("sun").ok_or_else(|| WorldError::Unknown("sun".into()))?;
        self.space = reg.boundary("space").ok_or_else(|| WorldError::Unknown("space".into()))?;
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let heat = view.stock(self.heat)?;
        let decl = declination(ctx.month_of_year);
        for i in 0..heat.len() {
            let area = ctx.grid.area_km2[i] as f64;
            let t = heat[i] as f64 / self.cap[i] as f64;
            let q_in = insolation(ctx.grid.lat[i] as f64, decl) * (1.0 - ALBEDO);
            let q_out = (OLR_A + OLR_B * (t - 273.15)).max(0.0);
            out.push(Endpoint::Boundary(self.sun), Endpoint::Stock(self.heat), 0, i, q_in * MJ_PER_W_MONTH * area);
            // cannot lose more than the tile holds: a cold tile radiates what it has, no more
            let lose = (q_out * MJ_PER_W_MONTH).min(heat[i] as f64 * 0.5);
            out.push(Endpoint::Stock(self.heat), Endpoint::Boundary(self.space), i, 0, lose * area);
        }
        Ok(())
    }
}

pub struct Mixing { heat: StockId, coarse: Option<Coarse>, cap: Vec<f32>, coarse_n: u32, pub iterations: usize }
impl Mixing {
    pub fn new(cap: Vec<f32>, coarse_n: u32) -> Self { Mixing { heat: StockId(0), coarse: None, cap, coarse_n, iterations: 60 } }
}
impl Module for Mixing {
    fn spec(&self) -> ModuleSpec { ModuleSpec::new("demo_mixing", Phase::Climate).reads(&["heat"]).writes(&["heat"]) }
    fn bind(&mut self, reg: &Registry, grid: &Grid) -> Result<(), WorldError> {
        self.heat = reg.stock("heat").ok_or_else(|| WorldError::Unknown("heat".into()))?;
        self.coarse = Some(Coarse::new(grid, self.coarse_n));
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let co = self.coarse.as_ref().ok_or_else(|| WorldError::Unknown("mixing not bound".into()))?;
        let heat = view.stock(self.heat)?;
        let n = heat.len();
        let area = &ctx.grid.area_km2;
        // heat capacity x area per fine cell (MJ/K x 1e-6 m2 -> kept as MJ/m2/K x km2)
        let ca_fine: Vec<f64> = (0..n).map(|i| self.cap[i] as f64 * area[i] as f64).collect();
        let h_fine: Vec<f64> = (0..n).map(|i| heat[i] as f64 * area[i] as f64).collect();
        let ca = co.sum_up(&ca_fine).map_err(|e| WorldError::Unknown(e.to_string()))?;
        let h = co.sum_up(&h_fine).map_err(|e| WorldError::Unknown(e.to_string()))?;
        let t0: Vec<f64> = (0..co.len()).map(|c| h[c] / ca[c]).collect();
        // Edge conductance in W/K, independent of grid size: D R^2 / sqrt(3).
        let r_m = EARTH_RADIUS_KM * 1000.0;
        let kappa = DIFF_D * r_m * r_m / 3f64.sqrt();
        // In capacity units: dt * kappa / (CA * 1e12 J/K per MJ/m2/K x km2), dt = one step in seconds.
        let dt = SECONDS_PER_MONTH * ctx.dt_months as f64;
        let k = kappa * dt / 1e12; // MJ/m2/K x km2 per edge, per unit of (T_i - T_j) after implicit step
        // Backward Euler: (CA_c + k * deg_c) T_c - k * sum_j T_j = CA_c * T0_c, solved by Gauss-Seidel.
        let mut t = t0.clone();
        for _ in 0..self.iterations {
            for c in 0..co.len() {
                let mut sum = 0.0;
                let mut deg = 0.0;
                for j in co.grid.neighbours(c) { sum += t[j]; deg += 1.0; }
                t[c] = (ca[c] * t0[c] + k * sum) / (ca[c] + k * deg);
            }
        }
        // Net heat gained by each coarse cell; remove the solver residual so the sum is exactly zero.
        let mut dh: Vec<f64> = (0..co.len()).map(|c| ca[c] * (t[c] - t0[c])).collect();
        let mean = dh.iter().sum::<f64>() / ca.iter().sum::<f64>();
        for c in 0..co.len() { dh[c] -= mean * ca[c]; }
        // Same temperature change on every member: split by capacity x area.
        let share = co.split_down(&dh, &ca_fine).map_err(|e| WorldError::Unknown(e.to_string()))?;
        for (i, &x) in share.iter().enumerate() {
            if x > 0.0 { out.push(Endpoint::Transit, Endpoint::Stock(self.heat), 0, i, x); }
            else if x < 0.0 { out.push(Endpoint::Stock(self.heat), Endpoint::Transit, i, 0, -x); }
        }
        Ok(())
    }
}

pub struct Water {
    air: StockId, land: StockId, snow: StockId, sea: StockId, heat: StockId,
    is_land: Vec<bool>, cap: Vec<f32>,
    /// Neighbour the air drifts to in each season of wind: [westerly, easterly].
    wind_to: [Vec<u32>; 2],
    /// Lowest neighbour, or u32::MAX for a pit.
    down: Vec<u32>,
}
impl Water {
    /// `down` is filled in by `build`: the lowest neighbour of each tile.
    pub fn new(is_land: Vec<bool>, cap: Vec<f32>) -> Self {
        let n = is_land.len();
        Water { air: StockId(0), land: StockId(0), snow: StockId(0), sea: StockId(0), heat: StockId(0), is_land, cap, wind_to: [vec![], vec![]], down: vec![u32::MAX; n] }
    }
}
impl Module for Water {
    fn spec(&self) -> ModuleSpec {
        ModuleSpec::new("demo_water", Phase::Water).reads(&["heat", "water_air", "water_land", "water_snow", "water_sea"]).writes(&["water_air", "water_land", "water_snow", "water_sea"])
    }
    fn bind(&mut self, reg: &Registry, grid: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.heat = g("heat")?; self.air = g("water_air")?; self.land = g("water_land")?; self.snow = g("water_snow")?; self.sea = g("water_sea")?;
        // wind: the neighbour most to the east (westerlies carry air east) or west (trades)
        let n = grid.len();
        let mut east = vec![0u32; n];
        let mut west = vec![0u32; n];
        for i in 0..n {
            let lon = grid.lon[i] as f64;
            let e = [-lon.sin(), 0.0, -lon.cos()];
            let p = grid.pos[i];
            let (mut be, mut bw, mut se, mut sw) = (i as u32, i as u32, f64::MIN, f64::MIN);
            for j in grid.neighbours(i) {
                let q = grid.pos[j];
                let d = [q[0] as f64 - p[0] as f64, q[1] as f64 - p[1] as f64, q[2] as f64 - p[2] as f64];
                let s = d[0] * e[0] + d[1] * e[1] + d[2] * e[2];
                if s > se { se = s; be = j as u32; }
                if -s > sw { sw = -s; bw = j as u32; }
            }
            east[i] = be; west[i] = bw;
        }
        self.wind_to = [east, west];
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let (heat, air, land, snow) = (view.stock(self.heat)?, view.stock(self.air)?, view.stock(self.land)?, view.stock(self.snow)?);
        let dt = ctx.dt_months as f64;
        let area = &ctx.grid.area_km2;
        const ADVECT: f64 = 0.5; // share of the air that drifts one tile a month, placeholder
        const TAU_RAIN: f64 = 0.3; // months, placeholder (vapour lives about 9 days)
        let rain_frac = 1.0 - (-dt / TAU_RAIN).exp();
        // One connected ocean: sea tiles level out toward the area-weighted sea mean through
        // a transit junction. Without it, a sea tile that loses more to the air than it
        // gains in rain runs dry, which no real ocean does. Placeholder rate.
        let sea = view.stock(self.sea)?;
        let (mut num, mut den) = (0.0f64, 0.0f64);
        for i in 0..heat.len() { if !self.is_land[i] { num += sea[i] as f64 * area[i] as f64; den += area[i] as f64; } }
        let sea_mean = if den > 0.0 { num / den } else { 0.0 };
        let level = 1.0 - (-dt / 5.0f64).exp();
        for i in 0..heat.len() {
            let a = area[i] as f64;
            let t = heat[i] as f64 / self.cap[i] as f64;
            let tc = t - 273.15;
            // evaporation, mm per month, placeholder curve through about 83 mm at 15 C
            let e_pot = (83.0 * (0.06 * (t - 288.15)).exp()).min(250.0) * dt;
            if !self.is_land[i] {
                let x = (sea[i] as f64 - sea_mean) * level;
                if x > 0.0 { out.push(Endpoint::Stock(self.sea), Endpoint::Transit, i, 0, x * a); }
                else if x < 0.0 { out.push(Endpoint::Transit, Endpoint::Stock(self.sea), 0, i, -x * a); }
            }
            if self.is_land[i] {
                let w = land[i] as f64;
                let e = (e_pot * w / (w + 150.0)).min(0.5 * w);
                out.local(self.land, self.air, i, e * a);
            } else {
                out.local(self.sea, self.air, i, e_pot * a);
            }
            // air: part drifts to a neighbour, part falls here
            let av = air[i] as f64;
            let dir = if (ctx.grid.lat[i].abs() as f64) > std::f64::consts::FRAC_PI_6 { 0 } else { 1 };
            let j = self.wind_to[dir][i] as usize;
            let adv = if j != i { av * ADVECT } else { 0.0 };
            if adv > 0.0 { out.lateral(self.air, i, j, adv * a); }
            let rain = (av - adv) * rain_frac;
            if rain > 0.0 {
                let target = if !self.is_land[i] { self.sea } else if tc < 0.0 { self.snow } else { self.land };
                out.local(self.air, target, i, rain * a);
            }
            // snow melts by degree-days, 4.5 mm per degree per day (plan: carries over from v0.49)
            let s = snow[i] as f64;
            if s > 0.0 && tc > 0.0 {
                let melt = (4.5 * 30.4 * tc * dt).min(s);
                out.local(self.snow, if self.is_land[i] { self.land } else { self.sea }, i, melt * a);
            }
            // runoff to the lowest neighbour, 20 percent of the stored water a month
            if self.is_land[i] && self.down[i] != u32::MAX {
                let r = (land[i] as f64 * 0.2 * dt).min(land[i] as f64 * 0.5);
                let d = self.down[i] as usize;
                let to = if self.is_land[d] { self.land } else { self.sea };
                out.push(Endpoint::Stock(self.land), Endpoint::Stock(to), i, d, r * a);
            }
        }
        Ok(())
    }
}

/// Build a small test world. `land_share` is the rough fraction of land.
/// With `ocean_only` the world is all sea (used to test resolution independence).
pub struct Demo { pub sim: Sim, pub is_land: Vec<bool>, pub cap: Vec<f32> }

pub fn build(n: u32, seed: u64, ocean_only: bool) -> Demo {
    let grid = Arc::new(Grid::new(n));
    let mut reg = Registry::new();
    let heat = reg.add_stock("heat", "MJ/m2", Group::Heat, Storage::Exact);
    let air = reg.add_stock("water_air", "mm", Group::Water, Storage::Exact);
    let land = reg.add_stock("water_land", "mm", Group::Water, Storage::Exact);
    let _snow = reg.add_stock("water_snow", "mm", Group::Water, Storage::Sparse);
    let sea = reg.add_stock("water_sea", "mm", Group::Water, Storage::Exact);
    reg.add_boundary("sun", Group::Heat);
    reg.add_boundary("space", Group::Heat);

    // Synthetic land: smooth value noise from the seed. Placeholder for baked Earth.
    let mut rng = Rng::new(seed);
    let phases: Vec<[f64; 4]> = (0..6).map(|_| [rng.f64() * std::f64::consts::TAU, rng.f64() * std::f64::consts::TAU, rng.f64() * std::f64::consts::TAU, 0.7 + rng.f64()]).collect();
    let height: Vec<f32> = (0..grid.len()).map(|i| {
        let p = grid.pos[i];
        phases.iter().map(|q| ((p[0] as f64 * 3.0 * q[3] + q[0]).sin() + (p[1] as f64 * 3.0 * q[3] + q[1]).sin() + (p[2] as f64 * 3.0 * q[3] + q[2]).sin()) / 6.0).sum::<f64>() as f32
    }).collect();
    let mut sorted = height.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let sea_level = sorted[(sorted.len() as f64 * 0.7) as usize];
    let is_land: Vec<bool> = height.iter().map(|&h| !ocean_only && h > sea_level).collect();
    let cap: Vec<f32> = is_land.iter().map(|&l| if l { C_LAND as f32 } else { C_SEA as f32 }).collect();

    let mut world = World::new(grid.clone(), reg, seed, 0);
    for i in 0..grid.len() {
        let lat = grid.lat[i] as f64;
        let t0 = 273.15 + 30.0 * lat.cos() - 12.0; // placeholder start, the balance finds its own
        world.stock_mut(heat)[i] = (t0 * cap[i] as f64) as f32;
        world.stock_mut(air)[i] = 20.0;
        if is_land[i] { world.stock_mut(land)[i] = 300.0; } else { world.stock_mut(sea)[i] = 20000.0; }
    }
    world.reset_ledger();

    let mut water = Water::new(is_land.clone(), cap.clone());
    // lowest neighbour that is lower than the tile
    for i in 0..grid.len() {
        let mut best = u32::MAX;
        let mut bh = height[i];
        for j in grid.neighbours(i) { if height[j] < bh { bh = height[j]; best = j as u32; } }
        water.down[i] = best;
    }
    let mut sim = Sim::new(world);
    sim.add_module(Box::new(Radiation::new(cap.clone()))).expect("radiation");
    sim.add_module(Box::new(Mixing::new(cap.clone(), (n / 4).max(2)))).expect("mixing");
    sim.add_module(Box::new(water)).expect("water");
    Demo { sim, is_land, cap }
}

/// Area-weighted global mean temperature in K.
pub fn mean_temperature(d: &Demo) -> f64 {
    let w = &d.sim.world;
    let heat = w.stock(w.reg.stock("heat").unwrap());
    let (mut num, mut den) = (0.0, 0.0);
    for i in 0..heat.len() {
        let a = w.grid.area_km2[i] as f64;
        num += heat[i] as f64 / d.cap[i] as f64 * a; den += a;
    }
    num / den
}

#[cfg(test)]
mod tests_insolation {
    use super::*;
    #[test]
    fn insolation_is_sane() {
        for m in 0..12 { for lat in [-90.0f64, -60.0, 0.0, 45.0, 89.9, 90.0, 90.0000001] {
            let q = insolation(lat.to_radians(), declination(m));
            assert!(q.is_finite() && q >= 0.0, "m{m} lat{lat} -> {q}");
        } }
    }
}
