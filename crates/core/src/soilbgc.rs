//! Soil carbon, nitrogen and phosphorus: CENTURY in small (Parton et al. 1987).
//!
//! Four carbon pools: litter, fast, slow and passive. Each decays at its own rate, faster when warm
//! (Q10 = 2) and moist, hardly at all when frozen or waterlogged, so cold wet ground builds peat and
//! permafrost carbon. What decays either goes to the air (heterotrophic respiration) or into the next pool.
//!
//! Organic nitrogen and phosphorus follow the carbon at each pool's own C:N and C:P. A pool with a
//! lower ratio than the one feeding it needs mineral nitrogen to build. If the mineral pool is short,
//! decay slows to fit. Mineral nitrogen enters by fixation and leaves by leaching and denitrification.
//! Mineral phosphorus enters by rock weathering (in the landscape module), sticks to soil as an occluded
//! pool, and is washed away slowly.

use crate::earth::BANDS;
use crate::eco::*;
use crate::env::*;
use crate::flows::{Endpoint, FlowOut};
use crate::grid::Grid;
use crate::hydro::FIELD_CAP;
use crate::module::*;
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;

/// 1/yr at 10 C and ideal moisture
pub const K: [f64; 4] = [1.0, 0.8, 0.03, 0.002];
/// (to air, to litter-slot targets) as shares: [air, fast, slow, passive]
pub const FRAC: [[f64; 4]; 4] = [
    [0.60, 0.35, 0.05, 0.00], // litter
    [0.55, 0.00, 0.40, 0.05], // fast
    [0.50, 0.45, 0.00, 0.05], // slow
    [0.50, 0.50, 0.00, 0.00], // passive
];
pub const NFIX_KG_M2_YR: f64 = 4.0e-4;
pub const N_LOSS_PER_YR: f64 = 0.15;
pub const P_SORB_PER_YR: f64 = 0.2;
pub const P_DESORB_PER_YR: f64 = 0.002;
pub const P_LOSS_PER_YR: f64 = 0.05;

pub fn moisture_factor(w: f64) -> f64 {
    let dry = (w / FIELD_CAP as f64).clamp(0.05, 1.0);
    let wet = 1.0 / (1.0 + ((w - 250.0) / 150.0).max(0.0).powi(2));
    dry * wet
}
pub fn temp_factor_soil(tc: f64) -> f64 {
    let f = 2f64.powf((tc - 10.0) / 10.0);
    if tc < 0.0 { f * 0.1 } else { f }
}

pub struct SoilBgc { env: Env, eco: Eco, ids: Option<Ids> }
struct Ids { heat: StockId, w_land: StockId, w_sea: StockId, c_air: StockId }

impl SoilBgc { pub fn new(env: Env, eco: Eco) -> Self { SoilBgc { env, eco, ids: None } } }

impl Module for SoilBgc {
    fn spec(&self) -> ModuleSpec {
        ModuleSpec::new("soilbgc", Phase::SoilPlants)
            .reads(&["heat", "w_land", "w_sea", "c_air", "c_litter", "c_sfast", "c_sslow", "c_spass", "n_min", "p_min", "p_occ"])
            .writes(&["c_air", "c_litter", "c_sfast", "c_sslow", "c_spass", "n_min", "n_org", "p_min", "p_org", "p_occ"])
            .boundaries(&["nfix", "nloss", "ploss"])
    }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(Ids { heat: g("heat")?, w_land: g("w_land")?, w_sea: g("w_sea")?, c_air: g("c_air")? });
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("soilbgc not bound".into()))?;
        let (env, eco) = (&self.env, &self.eco);
        let n = env.n();
        let dt = ctx.dt_months as f64 / 12.0;
        let heat = view.stock(ids.heat)?; let w_land = view.stock(ids.w_land)?; let w_sea = view.stock(ids.w_sea)?;
        let pools = [eco.litter, eco.sfast, eco.sslow, eco.spass];
        let c: Vec<&[f32]> = pools.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let n_min = view.stock(eco.n_min)?; let p_min = view.stock(eco.p_min)?; let p_occ = view.stock(eco.p_occ)?;
        // lanes: pool -> pool (4x4 off-diagonal), pool -> air (4), n and p exchanges
        let mut to_air: Vec<Vec<f32>> = (0..4).map(|_| vec![0f32; n]).collect();
        let mut move_: Vec<Vec<Vec<f32>>> = (0..4).map(|_| (0..4).map(|_| Vec::new()).collect()).collect();
        let (mut n_in, mut n_out, mut p_in, mut p_out) = (vec![0f32; n], vec![0f32; n], vec![0f32; n], vec![0f32; n]); // mineral gains / losses from organic
        let (mut nfix, mut nloss) = (vec![0f32; n], vec![0f32; n]);
        let (mut sorb, mut desorb, mut ploss) = (vec![0f32; n], vec![0f32; n], vec![0f32; n]);
        for i in 0..n {
            if w_sea[i] > 50.0 && env.ocean0[i] { continue; }
            let tc = env.t_from_h(i, heat[i]) as f64 - 273.15;
            let w = w_land[i] as f64;
            let rate_f = temp_factor_soil(tc) * moisture_factor(w);
            // potential decay of each pool this step
            let mut dec = [0f64; 4];
            for k in 0..4 { dec[k] = (c[k][i] as f64 * (1.0 - (-K[k] * rate_f * dt).exp())).max(0.0); }
            // nitrogen and phosphorus balance of the whole step
            let (mut dn, mut dpp) = (0f64, 0f64);
            for k in 0..4 {
                let mut take_n = 0.0; let mut take_p = 0.0;
                for t in 1..4 { if FRAC[k][t] > 0.0 { take_n += FRAC[k][t] / CN[t] as f64; take_p += FRAC[k][t] / CP[t] as f64; } }
                dn += dec[k] * (1.0 / CN[k] as f64 - take_n);
                dpp += dec[k] * (1.0 / CP[k] as f64 - take_p);
            }
            // mineral shortage slows the decay that needs to build new pools
            let mut s = 1.0f64;
            if dn < 0.0 && -dn > 0.4 * n_min[i].max(0.0) as f64 { s = s.min(0.4 * n_min[i].max(0.0) as f64 / -dn); }
            if dpp < 0.0 && -dpp > 0.4 * p_min[i].max(0.0) as f64 { s = s.min(0.4 * p_min[i].max(0.0) as f64 / -dpp); }
            for k in 0..4 {
                let d = dec[k] * s;
                if d <= 0.0 { continue; }
                to_air[k][i] = (d * FRAC[k][0]) as f32;
                for t in 1..4 {
                    if FRAC[k][t] > 0.0 {
                        if move_[k][t].is_empty() { move_[k][t] = vec![0f32; n]; }
                        move_[k][t][i] = (d * FRAC[k][t]) as f32;
                    }
                }
            }
            if dn >= 0.0 { n_in[i] = (dn * s) as f32; } else { n_out[i] = (-dn * s) as f32; }
            if dpp >= 0.0 { p_in[i] = (dpp * s) as f32; } else { p_out[i] = (-dpp * s) as f32; }
            // inputs and losses of the mineral pools
            let f_in = ((tc + 5.0) / 25.0).clamp(0.0, 1.0) * (w / FIELD_CAP as f64).min(1.0);
            nfix[i] = (NFIX_KG_M2_YR * f_in * dt) as f32;
            nloss[i] = (n_min[i] as f64 * (1.0 - (-N_LOSS_PER_YR * dt).exp())) as f32;
            sorb[i] = (p_min[i] as f64 * (1.0 - (-P_SORB_PER_YR * dt).exp())) as f32;
            desorb[i] = (p_occ[i] as f64 * (1.0 - (-P_DESORB_PER_YR * dt).exp())) as f32;
            ploss[i] = (p_min[i] as f64 * (1.0 - (-P_LOSS_PER_YR * dt).exp())) as f32;
        }
        let air = ids.c_air;
        for k in 0..4 {
            out.dense_local(pools[k], air, n).copy_from_slice(&to_air[k]);
            for t in 1..4 { if !move_[k][t].is_empty() { out.dense_local(pools[k], pools[t], n).copy_from_slice(&move_[k][t]); } }
        }
        out.dense_local(eco.n_org, eco.n_min, n).copy_from_slice(&n_in);
        out.dense_local(eco.n_min, eco.n_org, n).copy_from_slice(&n_out);
        out.dense_local(eco.p_org, eco.p_min, n).copy_from_slice(&p_in);
        out.dense_local(eco.p_min, eco.p_org, n).copy_from_slice(&p_out);
        out.dense(Endpoint::Boundary(eco.b_nfix), Endpoint::Stock(eco.n_min), n).copy_from_slice(&nfix);
        out.dense(Endpoint::Stock(eco.n_min), Endpoint::Boundary(eco.b_nloss), n).copy_from_slice(&nloss);
        out.dense_local(eco.p_min, eco.p_occ, n).copy_from_slice(&sorb);
        out.dense_local(eco.p_occ, eco.p_min, n).copy_from_slice(&desorb);
        out.dense(Endpoint::Stock(eco.p_min), Endpoint::Boundary(eco.b_ploss), n).copy_from_slice(&ploss);
        let _ = BANDS;
        Ok(())
    }
}

/// Starting soil carbon (kgC/m2) for a tile: more where it is cold or wet.
pub fn initial_pools(t_ann_c: f64, precip_mm: f64) -> [f32; 4] {
    let cold = (1.0 + (10.0 - t_ann_c).max(0.0) / 8.0).min(4.0);
    let wet = (precip_mm / 800.0).clamp(0.3, 1.8);
    let base = cold * wet;
    [(0.4 * wet) as f32, (0.8 * base) as f32, (5.0 * base) as f32, (3.0 * base) as f32]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frozen_and_flooded_ground_hardly_decays() {
        let warm = temp_factor_soil(20.0) * moisture_factor(100.0);
        assert!(temp_factor_soil(-10.0) * moisture_factor(100.0) < 0.05 * warm);
        assert!(moisture_factor(600.0) < 0.5 * moisture_factor(120.0));
        assert!(moisture_factor(5.0) < 0.1);
    }
    #[test]
    fn partitions_sum_to_one() {
        for f in FRAC { assert!((f.iter().sum::<f64>() - 1.0).abs() < 1e-9); }
    }
}
