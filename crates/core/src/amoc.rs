//! The Atlantic overturning circulation as a Stommel salt-advection box.
//!
//! The overturning is driven by the temperature contrast between the tropical and the subpolar North Atlantic
//! and braked by fresh water in the north. In scaled units (Stommel 1961; Rahmstorf 1996):
//!
//!   psi = dT - S,          dS/dt = (F - |psi| S) / tau
//!
//! where dT is the contrast against today's, S is the salt deficit of the northern box and F is the freshwater
//! input. A flow of psi = 0.75 at today's contrast sets S = 0.25 and F0 = 0.1875. The steady states vanish at
//! F = dT^2 / 4 = 0.25, a third above F0, so enough fresh water switches the overturning off, and it does not
//! come back when the water stops until F falls well below that.
//!
//! Fresh water is the melt of northern ice (the ice stocks north of 45 N between 100 W and 60 E, mass lost
//! since last month). FW_REF is the flow that raises F by its own size, 0.45 Sv, so a collapse needs about
//! 0.15 Sv held for decades, the middle of the hosing experiments (Rahmstorf et al. 2005, from memory, flagged).
//!
//! When the overturning weakens the north Atlantic sector (40 to 75 N, 70 W to 45 E) loses heat and the south
//! Atlantic gains the same total (the bipolar seesaw). The full-collapse flux is AMOC_HEAT W/m2 in the north,
//! chosen to give 2 to 4 K of cooling over Europe, a mid-range of Jackson et al. 2015 (from memory, flagged).
//! Not yet: salinity of the real basin, wind-driven return flow, the southern overturning.

use crate::env::*;
use crate::flows::{Endpoint, FlowOut};
use crate::grid::Grid;
use crate::module::*;
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;

pub const S0: f32 = 0.25;
pub const PSI0: f32 = 0.75;
pub const TAU_YR: f64 = 75.0;
pub const FW_REF_SV: f64 = 0.45;
pub const AMOC_HEAT: f32 = 8.0;
const SV_M3_PER_MONTH: f64 = 1.0e6 * SEC_PER_MONTH;

fn bump(d: f32, w: f32) -> f32 { if d.abs() >= w { 0.0 } else { 0.5 * (1.0 + (std::f32::consts::PI * d / w).cos()) } }
fn in_box(lat: f32, lon: f32, la: (f32, f32), lo: (f32, f32)) -> bool { lat >= la.0 && lat <= la.1 && lon >= lo.0 && lon <= lo.1 }

/// One step of the salt box: change of S over `months`, for a contrast `dt_n` (1 is today's) and a fresh water flow in Sv.
pub fn ds_of(dt_n: f64, s: f64, fw_sv: f64, months: f64) -> f64 {
    let f = (S0 * PSI0) as f64 * (1.0 + fw_sv / FW_REF_SV);
    let psi = dt_n - s;
    ((f - psi.abs() * s) * months / 12.0 / TAU_YR).clamp(-0.05, 0.05)
}

pub struct Amoc { env: Env, ids: Option<Ids>, sub: Vec<usize>, trop: Vec<usize>, north: Vec<f32>, south: Vec<f32>, ice_tiles: Vec<usize>, dt0: f32, ice: Vec<StockId> }
struct Ids { s: StockId, prev: StockId, heat: StockId }

impl Amoc {
    pub fn new(env: Env) -> Amoc { Amoc { env, ids: None, sub: vec![], trop: vec![], north: vec![], south: vec![], ice_tiles: vec![], dt0: 1.0, ice: vec![] } }
}

impl Module for Amoc {
    fn spec(&self) -> ModuleSpec {
        let mut s = ModuleSpec::new("amoc", Phase::Climate).boundaries(&["diag", "weather"]);
        let mut r: Vec<String> = vec!["heat".into(), "amoc_s".into(), "amoc_ice".into()];
        for b in 0..crate::earth::BANDS { r.push(format!("w_ice{b}")); }
        s.reads = r;
        s.writes = vec!["heat".into(), "amoc_s".into(), "amoc_ice".into()];
        s
    }
    fn bind(&mut self, reg: &Registry, grid: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(Ids { s: g("amoc_s")?, prev: g("amoc_ice")?, heat: g("heat")? });
        self.ice = (0..crate::earth::BANDS).map(|b| g(&format!("w_ice{b}"))).collect::<Result<_, _>>()?;
        let n = grid.len();
        let (lat, lon): (Vec<f32>, Vec<f32>) = (grid.lat.iter().map(|x| x.to_degrees()).collect(), grid.lon.iter().map(|x| { let d = x.to_degrees(); if d > 180.0 { d - 360.0 } else { d } }).collect());
        self.sub = (0..n).filter(|&i| self.env.ocean0[i] && in_box(lat[i], lon[i], (50.0, 65.0), (-50.0, -10.0))).collect();
        self.trop = (0..n).filter(|&i| self.env.ocean0[i] && in_box(lat[i], lon[i], (0.0, 20.0), (-60.0, -20.0))).collect();
        self.ice_tiles = (0..n).filter(|&i| !self.env.ocean0[i] && in_box(lat[i], lon[i], (45.0, 90.0), (-100.0, 60.0))).collect();
        // heat patterns: north cools, south warms the same total
        let nw: Vec<f32> = (0..n).map(|i| bump(lat[i] - 57.5, 17.5) * bump(lon[i] + 12.5, 57.5)).collect();
        let sw: Vec<f32> = (0..n).map(|i| bump(lat[i] + 22.0, 23.0) * bump(lon[i] + 15.0, 35.0)).collect();
        let a = &grid.area_km2;
        let tn: f64 = (0..n).map(|i| nw[i] as f64 * a[i] as f64).sum::<f64>().max(1e-9);
        let ts: f64 = (0..n).map(|i| sw[i] as f64 * a[i] as f64).sum::<f64>().max(1e-9);
        self.north = nw; self.south = sw.iter().map(|&w| (w as f64 * tn / ts) as f32).collect();
        // today's contrast from the observed climate
        let mean = |tiles: &[usize]| -> f32 { let mut s = 0f64; let mut w = 0f64; for &i in tiles { for m in 0..12 { s += self.env.tobs[m][i] as f64 * a[i] as f64; w += a[i] as f64; } } if w > 0.0 { (s / w) as f32 } else { 280.0 } };
        self.dt0 = (mean(&self.trop) - mean(&self.sub)).max(1.0);
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("amoc not bound".into()))?;
        let env = &self.env;
        let n = env.n();
        let area = &ctx.grid.area_km2;
        let heat = view.stock(ids.heat)?;
        let s_now = view.stock(ids.s)?[0];
        let months = ctx.dt_months as f64;
        // contrast now
        let mean_t = |tiles: &[usize]| -> f32 { let mut s = 0f64; let mut w = 0f64; for &i in tiles { s += env.t_from_h(i, heat[i]) as f64 * area[i] as f64; w += area[i] as f64; } if w > 0.0 { (s / w) as f32 } else { 280.0 } };
        let dt_n = ((mean_t(&self.trop) - mean_t(&self.sub)) / self.dt0).clamp(0.3, 2.5);
        // fresh water: northern ice lost since last month (mm w.e. x km2)
        let ice_now: f64 = self.ice_tiles.iter().map(|&i| self.ice.iter().map(|&s| view.stock(s).map(|v| v[i] as f64).unwrap_or(0.0)).sum::<f64>() * area[i] as f64).sum();
        let prev = view.stock(ids.prev)?[0] as f64;
        let lost = if prev > 0.0 { (prev - ice_now).max(0.0) } else { 0.0 };
        let fw_sv = lost * 1.0e3 / (SV_M3_PER_MONTH * months); // mm km2 = 1e3 m3
        let psi = dt_n as f64 - s_now as f64;
        let ds = ds_of(dt_n as f64, s_now as f64, fw_sv, months);
        let diag = Endpoint::Boundary(env.ph.b_diag);
        let a0 = area[0] as f64;
        let book = |out: &mut FlowOut, st: StockId, d: f64| { if d > 0.0 { out.push(diag, Endpoint::Stock(st), 0, 0, d * a0) } else if d < 0.0 { out.push(Endpoint::Stock(st), diag, 0, 0, -d * a0) } };
        book(out, ids.s, ds);
        book(out, ids.prev, ice_now - prev);
        // heat
        let psi_rel = (psi / PSI0 as f64).clamp(0.0, 1.5) as f32;
        let amp = AMOC_HEAT * (1.0 - psi_rel); // W/m2 of cooling in the north when psi_rel < 1; warming when it is stronger
        if amp.abs() > 1e-4 {
            let mj = (MJ_PER_WM2_MONTH * months) as f32;
            let w = Endpoint::Boundary(env.ph.b_weather);
            let (mut nu, mut nd, mut su, mut sd) = (vec![0f32; n], vec![0f32; n], vec![0f32; n], vec![0f32; n]);
            for i in 0..n {
                let fn_ = -amp * self.north[i] * mj; // north: heat leaves when weaker
                let fs_ = amp * self.south[i] * mj;
                if fn_ < 0.0 { nd[i] = (-fn_).min(0.2 * heat[i].max(0.0)) } else { nu[i] = fn_ }
                if fs_ >= 0.0 { su[i] = fs_ } else { sd[i] = -fs_ }
            }
            let up: Vec<f32> = (0..n).map(|i| nu[i] + su[i]).collect(); let dn: Vec<f32> = (0..n).map(|i| nd[i] + sd[i]).collect();
            out.dense(w, Endpoint::Stock(ids.heat), n).copy_from_slice(&up);
            out.dense(Endpoint::Stock(ids.heat), w, n).copy_from_slice(&dn);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn run(s0: f64, fw: f64, years: usize) -> f64 { let mut s = s0; for _ in 0..years * 12 { s += ds_of(1.0, s, fw, 1.0); } s }
    #[test]
    fn today_is_a_steady_state() { let s = run(S0 as f64, 0.0, 500); assert!((s - S0 as f64).abs() < 1e-3, "{s}"); }
    #[test]
    fn a_small_flow_slows_it_and_a_large_one_stops_it() {
        let weak = run(S0 as f64, 0.10, 3000);
        assert!(1.0 - weak > 0.4, "psi should stay up with 0.1 Sv, S={weak}");
        let off = run(S0 as f64, 0.25, 3000);
        assert!(1.0 - off < 0.0, "0.25 Sv should reverse it, S={off}");
    }
    #[test]
    fn it_does_not_come_back_when_the_water_stops() {
        let off = run(S0 as f64, 0.25, 3000);
        let after = run(off, 0.0, 2000);
        assert!(1.0 - after < 0.0, "still off after the water stops: S={after}");
        let more = run(off, 0.0, 6000);
        assert!(1.0 - more < 0.0);
    }
}
