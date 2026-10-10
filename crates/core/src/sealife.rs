//! Life in the sea: plankton, small fish and large fish, with phosphorus as the limit.
//!
//! One surface layer and one deep layer per ocean tile, like the carbon cycle. Phosphorus is the nutrient
//! that limits plankton. Every living marine thing holds phosphorus at the Redfield ratio (106 mol C to 1 mol P,
//! 41 by mass), so the P in life is always C / 41 and moves with every carbon flow. Plankton grow with
//! light, warmth (Eppley 1972) and P, and are stopped by sea ice. They die or get eaten. Fish rates scale as
//! body mass to the minus one quarter, the same rule as on land. Dead matter splits: most is remineralised in the
//! surface layer, a part sinks into the deep layer. That sinking is the biological pump.
//!
//! Between layers P mixes like carbon does (surface to deep in 15 yr, deep to surface in 500 yr). Between
//! neighbours, plankton, fish and surface P spread slowly, a stand-in for currents. Polar blooms and tropical
//! deserts come out of light and P, not a map.
//!
//! Not yet: iron and nitrogen limits, coastal upwelling, river P and burial (ocean P is a closed box),
//! diatom and coccolithophore groups, oxygen, migration of whales and tuna.

use crate::eco::*;
use crate::env::*;
use crate::climate::{InsolTable, SEA_ICE_SCALE_MM};
use crate::flows::{Endpoint, FlowOut};
use crate::grid::Grid;
use crate::module::*;
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;

pub const CP_SEA: f64 = 41.0;
pub const SUB: usize = 10;
pub const EXPORT_FRAC: f64 = 0.12;
pub const PLANKTON_LOSS_PER_DAY: f64 = 0.13;
pub const K_P: f64 = 3.0e-4; // kgP/m2
pub const K_PLANKTON: f64 = 0.002; // kgC/m2
pub const K_FISH: f64 = 0.003; // kgC/m2
pub const MIX_RATE: f64 = 0.08; // per month, between neighbours
pub const P_TAU_SURF: f64 = 15.0;
pub const P_TAU_DEEP: f64 = 500.0;
pub const P_SURF0: f32 = 1.5e-3;
pub const P_DEEP0: f32 = 0.34;

pub struct SeaLife { env: Env, gs: GuildSet, eco: Eco, ids: Option<Ids>, table: Option<InsolTable> }
struct Ids { heat: StockId, w_sea: StockId, c_ocean: StockId, c_deep: StockId }

impl SeaLife {
    pub fn new(env: Env, gs: GuildSet, eco: Eco) -> SeaLife { SeaLife { env, gs, eco, ids: None, table: None } }
}

impl Module for SeaLife {
    fn spec(&self) -> ModuleSpec {
        let mut r: Vec<String> = ["heat", "w_sea", "c_ocean", "c_deep", "c_plankton", "p_sea", "p_deep_sea", "p_marine"].iter().map(|s| s.to_string()).collect();
        for g in &self.gs.sea { r.push(format!("c_fauna_{}", g.id)); }
        let mut s = ModuleSpec::new("sealife", Phase::Animals).also_reads(&r);
        s.writes = r.clone();
        s.reads = r;
        s
    }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(Ids { heat: g("heat")?, w_sea: g("w_sea")?, c_ocean: g("c_ocean")?, c_deep: g("c_deep")? });
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("sealife not bound".into()))?;
        if self.table.as_ref().map_or(true, |t| (ctx.year - t.year).abs() >= 100) { self.table = Some(InsolTable::build(ctx.year)); }
        let table = self.table.as_ref().unwrap();
        let (env, gs, eco) = (&self.env, &self.gs, &self.eco);
        let n = env.n();
        let grid = ctx.grid;
        let area = &grid.area_km2;
        let m = ctx.month_of_year as usize;
        let dt = ctx.dt_months as f64 / 12.0;
        let dh = ctx.dt_months as f64 * 30.44 / SUB as f64; // days per sub-step
        let heat = view.stock(ids.heat)?; let w_sea = view.stock(ids.w_sea)?;
        let plank = view.stock(eco.plankton)?; let psea = view.stock(eco.p_sea)?; let pdeep = view.stock(eco.p_deep_sea)?;
        let fs = view.stock(eco.sea_fauna[0])?; let fl = view.stock(eco.sea_fauna[1])?;
        let (gsmall, glarge) = (&gs.sea[0], &gs.sea[1]);
        let mut ocean = vec![false; n];
        let (mut b_end, mut s_end, mut l_end, mut p_end) = (vec![0f32; n], vec![0f32; n], vec![0f32; n], vec![0f32; n]);
        // per-tile dense lanes
        let mut l_up = vec![0f32; n]; // c_ocean -> plankton
        let mut pl_oc = vec![0f32; n]; let mut pl_dp = vec![0f32; n]; // plankton -> ocean, deep
        let mut sm_oc = vec![0f32; n]; let mut sm_dp = vec![0f32; n];
        let mut lg_oc = vec![0f32; n]; let mut lg_dp = vec![0f32; n];
        let mut pl_sm = vec![0f32; n]; let mut sm_lg = vec![0f32; n];
        let mut pup = vec![0f32; n]; let mut pback = vec![0f32; n]; let mut pdown = vec![0f32; n]; // p_sea -> marine, marine -> p_sea, marine -> p_deep
        let mut p_mix_dn = vec![0f32; n]; let mut p_mix_up = vec![0f32; n];
        for i in 0..n {
            if !env.ocean0[i] || w_sea[i] <= 0.0 { continue; }
            ocean[i] = true;
            let t_k = env.t_from_h(i, heat[i]);
            let tc = (t_k - 273.15) as f64;
            let f_si = (env.sea_ice_mass(i, heat[i]) / SEA_ICE_SCALE_MM).clamp(0.0, 1.0) as f64;
            let q = table.at(env.p.lat_deg[i], m) as f64;
            let light = q / (q + 120.0);
            let mu = (0.59 * (0.0633 * tc.clamp(-2.0, 32.0)).exp()).min(2.5);
            let (mut b, mut p, mut sm, mut lg) = (plank[i].max(0.0) as f64, psea[i].max(0.0) as f64, fs[i].max(0.0) as f64, fl[i].max(0.0) as f64);
            let (mut up_c, mut pl_o, mut pl_d, mut sm_o, mut sm_d, mut lg_o, mut lg_d, mut g_ps, mut g_sl) = (0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 0f64);
            let q10 = 2f64.powf((tc - 15.0) / 10.0).clamp(0.1, 3.0);
            let (mut p_to_mar, mut p_from_mar_surf, mut p_from_mar_deep) = (0f64, 0f64, 0f64);
            let mut o_prev = 0f64;
            for _ in 0..SUB {
                let g = mu * light * (1.0 - f_si) * p / (p + K_P);
                let want = b * ((g * dh).exp() - 1.0);
                let cap = 0.8 * CP_SEA * p;
                let up = want.min(cap).max(0.0);
                b += up; p -= up / CP_SEA; up_c += up; p_to_mar += up / CP_SEA;
                // loss: natural death, then grazing by small fish
                let dead = b * (1.0 - (-PLANKTON_LOSS_PER_DAY * dh).exp());
                b -= dead; pl_o += dead * (1.0 - EXPORT_FRAC); pl_d += dead * EXPORT_FRAC;
                let graze = (gsmall.i_max as f64 * q10 * sm * dh / 365.0 * b / (b + K_PLANKTON)).min(0.3 * b);
                b -= graze; sm += gsmall.assim as f64 * graze; g_ps += gsmall.assim as f64 * graze;
                let feces = (1.0 - gsmall.assim as f64) * graze; pl_o += feces * (1.0 - EXPORT_FRAC); pl_d += feces * EXPORT_FRAC;
                let eat = (glarge.i_max as f64 * q10 * lg * dh / 365.0 * sm / (sm + K_FISH)).min(0.3 * sm);
                sm -= eat; lg += glarge.assim as f64 * eat; g_sl += glarge.assim as f64 * eat;
                let feces2 = (1.0 - glarge.assim as f64) * eat; sm_o += feces2 * (1.0 - EXPORT_FRAC); sm_d += feces2 * EXPORT_FRAC;
                for (x, gd, o, d) in [(&mut sm, gsmall, &mut sm_o, &mut sm_d), (&mut lg, glarge, &mut lg_o, &mut lg_d)] {
                    let (kr, km) = (gd.resp as f64 * q10, gd.mort as f64 * q10);
                    let tot = *x * (1.0 - (-(kr + km) * dh / 365.0).exp());
                    let r = tot * kr / (kr + km); let dth = tot - r;
                    *x -= tot; *o += r + dth * (1.0 - EXPORT_FRAC); *d += dth * EXPORT_FRAC;
                }
                // what the living gave back to the surface layer returns its phosphorus within the month
                let o_now = pl_o + sm_o + lg_o; p += (o_now - o_prev) / CP_SEA; o_prev = o_now;
            }
            // phosphorus: all flows living -> sea carry C/41, split as the carbon splits
            p_from_mar_surf += (pl_o + sm_o + lg_o) / CP_SEA;
            p_from_mar_deep += (pl_d + sm_d + lg_d) / CP_SEA;
            // the surface P left after uptake, plus what came back, is the new surface P: handled by the flows
            l_up[i] = up_c as f32; pl_oc[i] = pl_o as f32; pl_dp[i] = pl_d as f32; sm_oc[i] = sm_o as f32; sm_dp[i] = sm_d as f32;
            lg_oc[i] = lg_o as f32; lg_dp[i] = lg_d as f32; pl_sm[i] = g_ps as f32; sm_lg[i] = g_sl as f32;
            pup[i] = p_to_mar as f32; pback[i] = p_from_mar_surf as f32; pdown[i] = p_from_mar_deep as f32;
            // physical mixing of phosphorus between the layers
            let s0 = psea[i].max(0.0) as f64; let d0 = pdeep[i].max(0.0) as f64;
            p_mix_dn[i] = (s0 / P_TAU_SURF * dt).min(0.3 * s0) as f32; p_mix_up[i] = (d0 / P_TAU_DEEP * dt).min(0.3 * d0) as f32;
            b_end[i] = b as f32; s_end[i] = sm as f32; l_end[i] = lg as f32; p_end[i] = p as f32;
        }
        let _ = (b_end, l_end, p_end);
        // carbon flows (within the module's own tile: dissolved carbon to life and back)
        out.dense_local(ids.c_ocean, eco.plankton, n).copy_from_slice(&l_up);
        out.dense_local(eco.plankton, ids.c_ocean, n).copy_from_slice(&pl_oc);
        out.dense_local(eco.plankton, ids.c_deep, n).copy_from_slice(&pl_dp);
        out.dense_local(eco.plankton, eco.sea_fauna[0], n).copy_from_slice(&pl_sm);
        out.dense_local(eco.sea_fauna[0], ids.c_ocean, n).copy_from_slice(&sm_oc);
        out.dense_local(eco.sea_fauna[0], ids.c_deep, n).copy_from_slice(&sm_dp);
        out.dense_local(eco.sea_fauna[0], eco.sea_fauna[1], n).copy_from_slice(&sm_lg);
        out.dense_local(eco.sea_fauna[1], ids.c_ocean, n).copy_from_slice(&lg_oc);
        out.dense_local(eco.sea_fauna[1], ids.c_deep, n).copy_from_slice(&lg_dp);
        // phosphorus follows
        out.dense_local(eco.p_sea, eco.p_marine, n).copy_from_slice(&pup);
        out.dense_local(eco.p_marine, eco.p_sea, n).copy_from_slice(&pback);
        out.dense_local(eco.p_marine, eco.p_deep_sea, n).copy_from_slice(&pdown);
        out.dense_local(eco.p_sea, eco.p_deep_sea, n).copy_from_slice(&p_mix_dn);
        out.dense_local(eco.p_deep_sea, eco.p_sea, n).copy_from_slice(&p_mix_up);
        // slow spreading between neighbours
        let mut spread = |stock: StockId, vals: &[f32], ratio: f64| {
            for i in 0..n {
                if !ocean[i] { continue; }
                let v = vals[i].max(0.0) as f64;
                for j in grid.neighbours(i) {
                    if !ocean[j] { continue; }
                    let dv = v - vals[j].max(0.0) as f64;
                    if dv <= 0.0 { continue; }
                    let amt = ratio * MIX_RATE * dt * dv / 6.0 * area[i] as f64;
                    out.push(Endpoint::Stock(stock), Endpoint::Stock(stock), i, j, amt);
                    if stock != eco.p_sea { out.push(Endpoint::Stock(eco.p_marine), Endpoint::Stock(eco.p_marine), i, j, amt / CP_SEA); }
                }
            }
        };
        spread(eco.plankton, plank, 1.0);
        spread(eco.sea_fauna[0], fs, 1.0);
        spread(eco.sea_fauna[1], fl, 0.5);
        spread(eco.p_sea, psea, 1.0);
        Ok(())
    }
}
