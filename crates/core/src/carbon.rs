//! The carbon cycle of the air and the sea.
//!
//! - `AirMix`: the air is well mixed. Each month every tile's air carbon is set to the world mean
//!   (carbon moves through a transit junction, so nothing is lost).
//! - `OceanCarbon`: a surface layer (`c_ocean`, DIC in a 100 m mixed layer) and the deep sea
//!   (`c_deep`) on every sea tile.
//!   * Gas exchange: flux = K x (pCO2 air - pCO2 sea), K = 0.72 gC/m2/yr per ppm (Takahashi: 0.06
//!     mol/m2/yr/uatm), switched off under sea ice. The sea's pCO2 rises 4.23 percent per K of warming
//!     (Takahashi et al.) and as the Revelle factor (10) times the relative rise in DIC.
//!   * Mixing down: the surface loses S/tau_s to the deep sea. Tau_s is 15 years.
//!   * Pump: the biological pump takes `pump` kgC/m2/yr from the surface to the deep, where it is
//!     only returned by ventilation D/tau_v (500 years), which is cut where sea ice shuts the
//!     surface. A tile starts balanced, so only changes in ice, warmth and the pump move CO2.
//! Where a plankton module exists, its export replaces the fixed pump.

use crate::env::*;
use crate::flows::{Endpoint, FlowOut};
use crate::grid::Grid;
use crate::module::*;
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;

pub const PPM_REF: f64 = 350.0;
pub const SURF_DIC: f64 = 2.46; // kgC/m2, 100 m at 2.05 mol/m3
pub const DEEP_DIC: f64 = 94.0; // kgC/m2, 3,500 m
pub const REVELLE: f64 = 10.0;
pub const K_GAS: f64 = 0.72e-3; // kgC/m2/yr per ppm
pub const TAU_SURF_YR: f64 = 15.0; // surface to deep exchange; fits the ocean's decades-long uptake of a pulse
pub const TAU_VENT_YR: f64 = 500.0; // with 0.024 kgC/m2/yr of pump this is a deep carbon pool turned over in 500 years
pub const PUMP0: f64 = DEEP_DIC / TAU_VENT_YR - SURF_DIC / TAU_SURF_YR; // balances the start, kgC/m2/yr
pub const SOLUBILITY_PER_K: f64 = 0.0423;

pub struct AirMix { env: Env, c_air: StockId }
impl AirMix { pub fn new(env: Env) -> Self { AirMix { env, c_air: StockId(0) } } }
impl Module for AirMix {
    fn spec(&self) -> ModuleSpec { ModuleSpec::new("airmix", Phase::Climate).reads(&["c_air"]).writes(&["c_air"]) }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> { self.c_air = reg.stock("c_air").ok_or_else(|| WorldError::Unknown("c_air".into()))?; Ok(()) }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let c = view.stock(self.c_air)?;
        let area = &ctx.grid.area_km2;
        let total: f64 = c.iter().zip(area.iter()).map(|(&v, &a)| v as f64 * a as f64).sum();
        let mean = total / ctx.grid.total_area_km2();
        let n = self.env.n();
        let mut d: Vec<f64> = (0..n).map(|i| (mean - c[i] as f64) * area[i] as f64).collect();
        // f64 rounding over thousands of tiles: settle the hair of imbalance on the biggest tile
        let r: f64 = d.iter().sum();
        let big = (0..n).max_by(|&a, &b| area[a].partial_cmp(&area[b]).unwrap()).unwrap();
        d[big] -= r;
        for i in 0..n {
            if d[i] > 0.0 { out.push(Endpoint::Transit, Endpoint::Stock(self.c_air), 0, i, d[i]); }
            else if d[i] < 0.0 { out.push(Endpoint::Stock(self.c_air), Endpoint::Transit, i, 0, -d[i]); }
        }
        Ok(())
    }
}

pub struct OceanCarbon { env: Env, ids: Option<Ids>, bio_pump: bool }
struct Ids { air: StockId, surf: StockId, deep: StockId, heat: StockId, sea: StockId }
impl OceanCarbon {
    pub fn new(env: Env) -> Self { OceanCarbon { env, ids: None, bio_pump: false } }
    /// With sea life in the world, the sinking of dead matter is the pump, so the fixed pump term is dropped.
    pub fn with_sea_life(env: Env) -> Self { OceanCarbon { env, ids: None, bio_pump: true } }
}

impl Module for OceanCarbon {
    fn spec(&self) -> ModuleSpec { ModuleSpec::new("oceancarbon", Phase::Water).reads(&["c_air", "c_ocean", "c_deep", "heat", "w_sea"]).writes(&["c_air", "c_ocean", "c_deep"]) }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(Ids { air: g("c_air")?, surf: g("c_ocean")?, deep: g("c_deep")?, heat: g("heat")?, sea: g("w_sea")? });
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("oceancarbon not bound".into()))?;
        let env = &self.env;
        let n = env.n();
        let m = ctx.month_of_year as usize;
        let dt = ctx.dt_months as f64 / 12.0;
        let (air, surf, deep, heat, sea) = (view.stock(ids.air)?, view.stock(ids.surf)?, view.stock(ids.deep)?, view.stock(ids.heat)?, view.stock(ids.sea)?);
        let area = &ctx.grid.area_km2;
        let c_mean: f64 = air.iter().zip(area.iter()).map(|(&c, &a)| c as f64 * a as f64).sum::<f64>() / ctx.grid.total_area_km2();
        let ppm_air = c_mean * crate::climate::PPM_PER_KGC_M2;
        let (mut a2s, mut s2a) = (vec![0f32; n], vec![0f32; n]);
        let (mut s2d, mut d2s) = (vec![0f32; n], vec![0f32; n]);
        for i in 0..n {
            if !env.ocean0[i] || sea[i] <= 0.0 { continue; }
            let h = heat[i];
            let t = env.t_from_h(i, h);
            let t_ann: f32 = env.tobs[m][i];
            let f_si = (env.sea_ice_mass(i, h) / crate::climate::SEA_ICE_SCALE_MM).clamp(0.0, 1.0) as f64;
            // baseline ice cover this month, so the start is balanced
            let h0 = env.h_from_t(i, t_ann);
            let f_si0 = (env.sea_ice_mass(i, h0) / crate::climate::SEA_ICE_SCALE_MM).clamp(0.0, 1.0) as f64;
            let s = surf[i] as f64; let d = deep[i] as f64;
            let p_sea = PPM_REF * (s / SURF_DIC).max(1e-6).powf(REVELLE) * (SOLUBILITY_PER_K * (t - t_ann) as f64).exp();
            let flux = K_GAS * (ppm_air - p_sea) * (1.0 - f_si) * dt; // kgC/m2 into the sea
            if flux >= 0.0 { a2s[i] = flux.min(0.5 * c_mean) as f32; } else { s2a[i] = (-flux).min(0.5 * s) as f32; }
            // mixing and pump, baseline at this tile's ice cover
            let open = (1.0 - f_si) / (1.0 - f_si0).max(0.05);
            let down = (s / TAU_SURF_YR + if self.bio_pump { 0.0 } else { PUMP0 * open.min(2.0) }) * dt;
            let up = d / TAU_VENT_YR * open.min(2.0) * dt;
            s2d[i] = down.min(0.5 * s) as f32; d2s[i] = up.min(0.5 * d) as f32;
        }
        out.dense_local(ids.air, ids.surf, n).copy_from_slice(&a2s);
        out.dense_local(ids.surf, ids.air, n).copy_from_slice(&s2a);
        out.dense_local(ids.surf, ids.deep, n).copy_from_slice(&s2d);
        out.dense_local(ids.deep, ids.surf, n).copy_from_slice(&d2s);
        Ok(())
    }
}

/// During a spin-up the air's CO2 is held at a target, because the land is still filling with carbon.
/// The carbon that takes is tallied in the `nudge` account. After `until_month` the module does nothing and
/// the air is left to the books alone. Reset the ledger when the spin-up ends.
pub struct Nudge { env: Env, c_air: StockId, target_ppm: f64, until_month: u64 }
impl Nudge {
    pub fn new(env: Env, target_ppm: f64, until_month: u64) -> Self { Nudge { env, c_air: StockId(0), target_ppm, until_month } }
}
impl Module for Nudge {
    fn spec(&self) -> ModuleSpec { ModuleSpec::new("nudge", Phase::Climate).reads(&["c_air"]).writes(&["c_air"]).boundaries(&["nudge"]) }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> { self.c_air = reg.stock("c_air").ok_or_else(|| WorldError::Unknown("c_air".into()))?; Ok(()) }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        if ctx.month_index >= self.until_month { return Ok(()); }
        let c = view.stock(self.c_air)?;
        let n = self.env.n();
        let target = (self.target_ppm / crate::climate::PPM_PER_KGC_M2) as f32;
        let (mut up, mut dn) = (vec![0f32; n], vec![0f32; n]);
        for i in 0..n { let d = 0.5 * (target - c[i]); if d >= 0.0 { up[i] = d } else { dn[i] = -d } }
        out.dense(Endpoint::Boundary(self.env.ph.b_nudge), Endpoint::Stock(self.c_air), n).copy_from_slice(&up);
        out.dense(Endpoint::Stock(self.c_air), Endpoint::Boundary(self.env.ph.b_nudge), n).copy_from_slice(&dn);
        Ok(())
    }
}
