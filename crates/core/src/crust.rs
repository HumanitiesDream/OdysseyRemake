//! The crust: ELRA, "elastic lithosphere, relaxing asthenosphere" (Le Meur and Huybrechts 1996).
//!
//! 1. Load: ice, sea water and sediment, as weight, against what the crust carried at the start.
//! 2. Bending: a rigid plate spreads each load over a few hundred km. The equilibrium sag is the
//!    load map convolved with one fixed shape: the Kelvin function kei, scaled by the flexural
//!    length alpha = (D / (rho_mantle g))^(1/4). It sinks under the load and lifts a forebulge beyond.
//!    The shape is normalised so a very wide load gives plain local isostasy: w = load / (rho_m g).
//! 3. Delay: the mantle flows slowly, so the sag moves toward equilibrium with a relaxation time
//!    of 3,000 years, the usual ELRA value (checked in docs/VERIFIED.md).
//!
//! The sag is the `defl` stock (metres, positive down, with an offset so it stays positive). It moves
//! to and from the `mantle` account. Bed height is worked out from it, never set.

use crate::earth::BANDS;
use crate::env::*;
use crate::flows::{Endpoint, FlowOut};
use crate::grid::{Coarse, Grid, EARTH_RADIUS_KM};
use crate::module::*;
use crate::planet::*;
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;

pub const RHO_MANTLE: f64 = 3300.0;
pub const RHO_SEAWATER: f64 = 1028.0;
pub const RHO_ROCK: f64 = 2700.0;
/// Flexural rigidity, N m. The usual ELRA value (about 100 km of lithosphere).
pub const RIGIDITY: f64 = 1.0e25;
pub const TAU_YEARS: f64 = 3000.0;

/// Kelvin function kei(x), by integrating the imaginary part of K0(x e^{i pi/4}).
pub fn kei(x: f64) -> f64 {
    let u = x / 2f64.sqrt();
    if u < 1e-9 { return -std::f64::consts::FRAC_PI_4; }
    // integrate over t where the integrand has died away
    let t_max = ((60.0 / u) + 1.0).acosh().max(1.0);
    let n = 4000usize;
    let h = t_max / n as f64;
    let f = |t: f64| { let s = u * t.cosh(); (-s).exp() * s.sin() };
    let mut sum = f(0.0) + f(t_max);
    for k in 1..n { sum += f(k as f64 * h) * if k % 2 == 1 { 4.0 } else { 2.0 }; }
    -sum * h / 3.0
}

/// Shape of the sag around a point load, 1 at the load, from kei: g(x) = -kei(x) / (pi/4).
pub fn shape(x: f64) -> f64 { -kei(x) / std::f64::consts::FRAC_PI_4 }

pub fn flexural_length_m() -> f64 { (RIGIDITY / (RHO_MANTLE * 9.81)).powf(0.25) }

pub struct Crust {
    env: Env,
    ids: Option<Ids>,
    co: Option<Coarse>,
    /// per coarse cell: neighbours within reach and their weights; weights sum to 1
    kernel: Vec<Vec<(u32, f32)>>,
    base_load: Vec<f32>,
}
struct Ids { defl: StockId, sea: StockId, ice: Vec<StockId>, sg: StockId, ss: StockId, sm: StockId, reg: StockId }

impl Crust {
    /// `base_load`: kg/m2 each tile carried at the start (the sag then stands for zero).
    pub fn new(env: Env, base_load: Vec<f32>) -> Crust { Crust { env, ids: None, co: None, kernel: Vec::new(), base_load } }

    pub fn load_of(sea_mm: f32, ice_stocks: &[f32], sed_m: f32) -> f32 {
        sea_mm * RHO_SEAWATER as f32 / 1000.0 + ice_stocks.iter().sum::<f32>() + sed_m * RHO_ROCK as f32
    }
}

impl Module for Crust {
    fn spec(&self) -> ModuleSpec {
        let mut s = ModuleSpec::new("crust", Phase::Climate).every(12).boundaries(&["mantle"]);
        s.reads = ["defl", "w_sea", "reg", "sed_g", "sed_s", "sed_m"].iter().map(|x| x.to_string()).chain((0..BANDS).map(|b| format!("w_ice{b}"))).collect();
        s.writes = vec!["defl".into()];
        s
    }
    fn bind(&mut self, reg: &Registry, grid: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(Ids { defl: g("defl")?, sea: g("w_sea")?, ice: (0..BANDS).map(|b| g(&format!("w_ice{b}"))).collect::<Result<_, _>>()?, sg: g("sed_g")?, ss: g("sed_s")?, sm: g("sed_m")?, reg: g("reg")? });
        // coarse cells about 150-250 km across; the forebulge reaches several flexural lengths out
        let cn = (grid.n / 4).clamp(4, 45);
        let co = Coarse::new(grid, cn.min(grid.n - 1));
        let alpha = flexural_length_m() / 1000.0; // km
        let reach = 8.0 * alpha;
        let nc = co.len();
        let mut kernel = Vec::with_capacity(nc);
        for c in 0..nc {
            // grow outward over neighbours until past the reach
            let mut seen = vec![c];
            let mut frontier = vec![c];
            let mut set = std::collections::BTreeSet::new();
            set.insert(c);
            let pc = co.grid.pos[c];
            while let Some(t) = frontier.pop() {
                for j in co.grid.neighbours(t) {
                    if set.contains(&j) { continue; }
                    let pj = co.grid.pos[j];
                    let d = (pc[0] * pj[0] + pc[1] * pj[1] + pc[2] * pj[2]).clamp(-1.0, 1.0) as f64;
                    if d.acos() * EARTH_RADIUS_KM <= reach { set.insert(j); frontier.push(j); seen.push(j); }
                }
            }
            let mut w: Vec<(u32, f32)> = Vec::new();
            let mut total = 0.0f64;
            for &j in &set {
                let pj = co.grid.pos[j];
                let d = (pc[0] * pj[0] + pc[1] * pj[1] + pc[2] * pj[2]).clamp(-1.0, 1.0) as f64;
                let r = d.acos() * EARTH_RADIUS_KM;
                let val = shape(r / alpha) * co.grid.area_km2[j] as f64;
                total += val;
                w.push((j as u32, val as f32));
            }
            // Normalise so that a uniform load over the whole reach gives local isostasy.
            let norm = total.max(1e-9);
            for e in w.iter_mut() { e.1 = (e.1 as f64 / norm) as f32; }
            kernel.push(w);
        }
        self.kernel = kernel;
        self.co = Some(co);
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("crust not bound".into()))?;
        let co = self.co.as_ref().ok_or_else(|| WorldError::Unknown("crust not bound".into()))?;
        let n = self.env.n();
        let area = &ctx.grid.area_km2;
        let defl = view.stock(ids.defl)?;
        let sea = view.stock(ids.sea)?;
        let ice: Vec<&[f32]> = ids.ice.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let (sg, ss, sm) = (view.stock(ids.sg)?, view.stock(ids.ss)?, view.stock(ids.sm)?);
        // load change per tile, kg/m2
        let mut dl = vec![0f64; n];
        let mut ice_sum = vec![0f32; BANDS];
        for i in 0..n {
            for b in 0..BANDS { ice_sum[b] = ice[b][i]; }
            let l = Crust::load_of(sea[i], &ice_sum, sg[i] + ss[i] + sm[i]);
            dl[i] = (l - self.base_load[i]) as f64;
        }
        // area-weighted mean onto the coarse cells
        let w_area: Vec<f64> = area.iter().map(|&a| a as f64).collect();
        let num: Vec<f64> = (0..n).map(|i| dl[i] * w_area[i]).collect();
        let cs = co.sum_up(&num).map_err(|e| WorldError::Unknown(e.to_string()))?;
        let ca = co.sum_up(&w_area).map_err(|e| WorldError::Unknown(e.to_string()))?;
        let load_c: Vec<f64> = (0..co.len()).map(|c| if ca[c] > 0.0 { cs[c] / ca[c] } else { 0.0 }).collect();
        // equilibrium sag, m: weighted sum of loads over the reach, divided by rho_m
        let mut w_eq_c = vec![0f64; co.len()];
        for c in 0..co.len() {
            // the kernel weights were normalised per source cell; use them as a smoother of the load
            let s: f64 = self.kernel[c].iter().map(|&(j, w)| w as f64 * load_c[j as usize]).sum();
            w_eq_c[c] = s / RHO_MANTLE;
        }
        let relax = 1.0 - (-(ctx.dt_months as f64 / 12.0) / TAU_YEARS).exp();
        let mut up = vec![0f32; n]; let mut down = vec![0f32; n];
        for i in 0..n {
            let target = w_eq_c[co.parent[i] as usize];
            let cur = (defl[i] - DEFL_OFFSET) as f64;
            let d = (target - cur) * relax;
            if d >= 0.0 { up[i] = d as f32; } else { down[i] = (-d) as f32; }
        }
        // bigger sag draws from the mantle account into the stock
        out.dense(Endpoint::Boundary(self.env.ph.b_mantle), Endpoint::Stock(ids.defl), n).copy_from_slice(&up);
        out.dense(Endpoint::Stock(ids.defl), Endpoint::Boundary(self.env.ph.b_mantle), n).copy_from_slice(&down);
        let _ = ids.reg;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kei_matches_tabulated_values() {
        // kei(0) = -pi/4, kei(1) = -0.4950, kei(2) = -0.2024 (Abramowitz and Stegun table 9.12)
        assert!((kei(0.0) + std::f64::consts::FRAC_PI_4).abs() < 1e-9);
        assert!((kei(1.0) + 0.49499).abs() < 2e-3, "{}", kei(1.0));
        assert!((kei(2.0) + 0.20240).abs() < 2e-3, "{}", kei(2.0));
    }

    #[test]
    fn the_shape_dips_then_lifts_a_forebulge() {
        assert!((shape(0.0) - 1.0).abs() < 1e-9);
        assert!(shape(1.0) > 0.0 && shape(1.0) < 1.0);
        let bulge = (30..120).map(|k| shape(k as f64 * 0.1)).fold(f64::MAX, f64::min);
        assert!(bulge < 0.0, "kei turns positive, which is the forebulge: min {bulge}");
    }

    #[test]
    fn flexural_length_is_about_130_km() {
        let a = flexural_length_m() / 1000.0;
        assert!(a > 110.0 && a < 150.0, "{a}");
    }
}
