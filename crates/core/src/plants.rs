//! Plants that grow. Ten types, each with leaf, wood and root carbon on every land tile.
//!
//! The method follows the LPJ family (Sitch et al. 2003; parameters from LPJmL 6.1.9, see
//! `data/plant_types.json`), with a light-use form of photosynthesis in place of the full
//! Farquhar model:
//!
//! - Light: taller types see the sky first (Beer's law, layer by layer). Grass lives on what is left.
//! - Growth: GPP = 1 gC per MJ of absorbed PAR x warmth x CO2 x water x nutrients.
//!   Warmth is the type's own response between its temperature limits, averaged over four height
//!   groups of the tile, so a tile with a high ridge grows trees low down and not on top.
//! - Respiration: maintenance on leaves, roots and a little of the wood, rising 2x per 10 K, and a
//!   quarter of what is left as growth respiration. About half of GPP ends as NPP.
//! - Allocation: fixed shares to leaf, wood and root, moved toward roots when water is short, and
//!   cut off for leaves once the type's leaf area index is full.
//! - Loss: leaf, wood and root turn over to litter at the type's own rates. Cold or hot winters outside
//!   the type's limits kill it. Nothing is capped. A type that loses its climate dwindles to zero.
//! - Water: the type that holds leaves transpires. The water module computes the same transpiration from
//!   the same stocks, so the soil is drained by exactly what the plants draw.
//! - Nitrogen and phosphorus: new tissue takes N and P from the mineral pools at the tissue's C:N
//!   and C:P. Growth is slowed as the mineral pools run low.
//!
//! Spread to new ground is in `dispersal`. Fire and grazing take their share in their own modules.

use crate::climate::InsolTable;
use crate::earth::BANDS;
use crate::eco::*;
use crate::env::*;
use crate::flows::{Endpoint, FlowOut};
use crate::grid::Grid;
use crate::hydro::{pet_oudin, DAYS_PER_MONTH, FIELD_CAP};
use crate::module::*;
use crate::planet::LAPSE;
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;

pub const LUE_GC_PER_MJ: f32 = 1.0;
pub const PAR_SHARE: f32 = 0.22; // PAR reaching the canopy as a share of top-of-air insolation (cloud, air, PAR fraction)
pub const KN_HALF: f32 = 5.0e-4; // kgN/m2 at which nitrogen halves growth
pub const KP_HALF: f32 = 2.0e-5;
pub const MAINT_PER_MONTH: f32 = 0.02;

pub struct Plants {
    env: Env,
    set: PlantSet,
    eco: Eco,
    ids: Option<Ids>,
    insol: Option<InsolTable>,
    tcold_obs: Vec<f32>,
}
struct Ids {
    heat: StockId, w_land: StockId, w_sea: StockId, c_air: StockId,
    rock: StockId, reg: StockId, sg: StockId, ss: StockId, sm: StockId, defl: StockId,
    ice: Vec<StockId>,
}

impl Plants {
    pub fn new(env: Env, set: PlantSet, eco: Eco) -> Plants {
        let tcold_obs = (0..env.n()).map(|i| (0..12).map(|m| env.tobs[m][i]).fold(f32::MAX, f32::min) - 273.15).collect();
        Plants { env, set, eco, ids: None, insol: None, tcold_obs }
    }
}

impl Module for Plants {
    fn spec(&self) -> ModuleSpec {
        let mut s = ModuleSpec::new("plants", Phase::SoilPlants).boundaries(&[]);
        let mut reads: Vec<String> = ["heat", "w_land", "w_sea", "c_air", "rock", "reg", "sed_g", "sed_s", "sed_m", "defl", "n_min", "p_min", "n_org", "p_org"].iter().map(|x| x.to_string()).collect();
        for b in 0..BANDS { reads.push(format!("w_ice{b}")); }
        let mut writes: Vec<String> = ["c_air", "c_litter", "n_min", "n_org", "p_min", "p_org"].iter().map(|x| x.to_string()).collect();
        for t in &self.set.types { for k in ["leaf", "wood", "root"] { let n = format!("c_{k}_{}", t.id); reads.push(n.clone()); writes.push(n); } }
        s.reads = reads; s.writes = writes;
        s
    }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(Ids { heat: g("heat")?, w_land: g("w_land")?, w_sea: g("w_sea")?, c_air: g("c_air")?, rock: g("rock")?, reg: g("reg")?, sg: g("sed_g")?, ss: g("sed_s")?, sm: g("sed_m")?, defl: g("defl")?,
            ice: (0..BANDS).map(|b| g(&format!("w_ice{b}"))).collect::<Result<_, _>>()? });
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("plants not bound".into()))?;
        let (env, set, eco) = (&self.env, &self.set, &self.eco);
        let n = env.n();
        let m = ctx.month_of_year as usize;
        let grid = ctx.grid;
        if self.insol.as_ref().map_or(true, |t| t.year != InsolTable::block_year(ctx.year)) { self.insol = Some(InsolTable::build(InsolTable::block_year(ctx.year))); }
        let insol = self.insol.as_ref().unwrap();
        let days = DAYS_PER_MONTH * ctx.dt_months as f32;
        let dt_yr = ctx.dt_months as f32 / 12.0;

        let heat = view.stock(ids.heat)?;
        let w_land = view.stock(ids.w_land)?;
        let w_sea = view.stock(ids.w_sea)?;
        let (rock, regl, sg, ss, sm, defl) = (view.stock(ids.rock)?, view.stock(ids.reg)?, view.stock(ids.sg)?, view.stock(ids.ss)?, view.stock(ids.sm)?, view.stock(ids.defl)?);
        let ice: Vec<&[f32]> = ids.ice.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let c_air = view.stock(ids.c_air)?;
        let n_min = view.stock(eco.n_min)?;
        let p_min = view.stock(eco.p_min)?;
        let n_org = view.stock(eco.n_org)?;
        let p_org = view.stock(eco.p_org)?;
        let leaf: Vec<&[f32]> = eco.leaf.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let wood: Vec<&[f32]> = eco.wood.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let root: Vec<&[f32]> = eco.root.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let ppm = (c_air.iter().zip(grid.area_km2.iter()).map(|(&c, &a)| c as f64 * a as f64).sum::<f64>() / grid.total_area_km2() * crate::climate::PPM_PER_KGC_M2) as f32;
        let f_co2_c3 = ((ppm - 40.0) / (ppm + 80.0)) / ((350.0 - 40.0) / (350.0 + 80.0));

        const L: usize = 9; // lanes per type
        let mut lanes: Vec<Vec<f32>> = (0..NPT * L).map(|_| vec![0f32; n]).collect();
        let mut used = vec![false; NPT * L];
        let (mut n_net, mut p_net) = (vec![0f64; n], vec![0f64; n]);
        let mut any = false;

        for i in 0..n {
            // land only, and not under ice
            let shift = env.bed_shift(i, rock[i], regl[i], sg[i], ss[i], sm[i], defl[i]);
            if w_sea[i] > 0.0 && env.is_sea(i, shift, w_sea[i]) { continue; }
            let ice_m: f32 = (0..BANDS).map(|b| Env::ice_m(ice[b][i])).sum::<f32>() / BANDS as f32;
            if ice_m > 50.0 { continue; }
            let mut lf = [0f32; NPT]; let mut present = false;
            for p in 0..NPT {
                lf[p] = leaf[p][i];
                if leaf[p][i] + wood[p][i] + root[p][i] > 1e-9 { present = true; }
            }
            if !present { continue; }
            let t_tile = env.t_from_h(i, heat[i]);
            // four height groups
            let mut tq = [0f32; 4];
            for k in 0..4 {
                let mut s = 0.0;
                for b in 0..3 { s += env.band_temp(i, t_tile, env.band_bed(i, k * 3 + b, shift)); }
                tq[k] = s / 3.0 - 273.15;
            }
            let t_mean = tq.iter().sum::<f32>() / 4.0;
            let tcold = self.tcold_obs[i] + (t_tile - env.tobs[m][i]) - LAPSE * shift;
            let w = w_land[i];
            let w_ratio = (w / FIELD_CAP).min(1.0);
            let q = insol.at(env.p.lat_deg[i], m);
            let pet = pet_oudin(q, t_mean) * days;
            let cn = canopy(set, &lf, t_mean, w_ratio);
            let (_transp, f_w) = transpiration(pet, cn.fpar_tot, w);
            let par = PAR_SHARE * q * MJ_PER_WM2_MONTH as f32 * ctx.dt_months as f32;
            let (nm, pm) = (n_min[i].max(0.0), p_min[i].max(0.0));
            let f_n = nm / (nm + KN_HALF);
            let f_p = pm / (pm + KP_HALF);
            let f_nut = f_n.min(f_p);
            let resp_t = (2f32).powf((t_mean - 15.0) / 10.0).clamp(0.1, 4.0);
            let (mut dn, mut dp) = (0f64, 0f64);
            for p in 0..NPT {
                let (cl, cw, cr) = (leaf[p][i].max(0.0), wood[p][i].max(0.0), root[p][i].max(0.0));
                if cl + cw + cr < 1e-9 { continue; }
                let t = &set.types[p];
                let ft = (0..4).map(|k| temp_factor(t, tq[k])).sum::<f32>() / 4.0;
                let fc = if t.c4 { 1.0 } else { f_co2_c3 };
                let gpp = LUE_GC_PER_MJ * par * cn.fpar[p] * ft * fc * f_w * f_nut / 1000.0;
                let rm = MAINT_PER_MONTH * ctx.dt_months as f32 * resp_t * (cl + cr + 0.05 * cw);
                let net = gpp - rm;
                if std::env::var("ODY_TILE").ok().and_then(|v| v.parse::<usize>().ok()) == Some(i) && (ctx.month_index % 12 == 6 || (ctx.month_index >= 120 && ctx.month_index < 132)) {
                    eprintln!("y{} {} cl {:.3} cw {:.2} cr {:.3} t {:.1} ft {:.2} fw {:.2} fnut {:.2} fpar {:.2} gpp {:.3} rm {:.3} net {:.3} n {:.5} p {:.6}", ctx.month_index / 12, t.id, cl, cw, cr, t_mean, ft, f_w, f_nut, cn.fpar[p], gpp, rm, net, n_min[i], p_min[i]);
                }
                let base = p * L;
                if net > 0.0 {
                    let npp = net * 0.75;
                    let shift_root = 0.15 * (1.0 - f_w);
                    let mut al = (t.al - shift_root).max(0.05 * t.al);
                    let mut ar = t.ar + (t.al - al);
                    let mut aw = t.aw;
                    // Pipe model: the leaves a tree can hold rise with its wood. Below that target the leaves
                    // are filled first. At the target the share goes to wood and root.
                    let leaf_full = if t.woody { (t.lai_max / t.sla) * (1.0 - (-(cw + 0.1) / 2.0).exp()) } else { t.lai_max / t.sla };
                    if cl >= leaf_full {
                        let spare = al; al = 0.0;
                        if t.woody { aw += 0.6 * spare; ar += 0.4 * spare; } else { ar += spare; }
                    } else if t.woody {
                        let boost = 0.6f32.max(al);
                        let extra = boost - al;
                        al = boost; aw = (aw - 0.7 * extra).max(0.0); ar = (1.0 - al - aw).max(0.0);
                    }
                    lanes[base][i] = npp * al; lanes[base + 1][i] = npp * aw; lanes[base + 2][i] = npp * ar;
                    used[base] = true; used[base + 1] = true; used[base + 2] = true;
                    dn += (npp * (al / CN_LEAF + aw / CN_WOOD + ar / CN_ROOT)) as f64;
                    dp += (npp * (al / CP_LEAF + aw / CP_WOOD + ar / CP_ROOT)) as f64;
                } else {
                    // respiration exceeds what the leaves bring in: it draws on leaves and roots
                    let draw = (-net).min(0.25 * (cl + cr));
                    if cl + cr > 0.0 {
                        lanes[base + 3][i] = draw * cl / (cl + cr); lanes[base + 4][i] = draw * cr / (cl + cr);
                        used[base + 3] = true; used[base + 4] = true;
                        // the respired tissue gives its nitrogen and phosphorus back to the mineral pools
                        dn -= ((lanes[base + 3][i]) / CN_LEAF + (lanes[base + 4][i]) / CN_ROOT) as f64;
                        dp -= ((lanes[base + 3][i]) / CP_LEAF + (lanes[base + 4][i]) / CP_ROOT) as f64;
                    }
                }
                // Reserves in the roots: when the canopy is thin the roots feed new leaves (grass sprouting back after a fire
                // or a grazing, trees flushing leaves). Carbon moves root to leaf, with the same N and P per carbon.
                {
                    let leaf_target = if t.woody { (t.lai_max / t.sla) * (1.0 - (-(cw + 0.1) / 2.0).exp()) } else { t.lai_max / t.sla };
                    if cl < 0.6 * leaf_target && cr > 0.01 && f_w > 0.2 && ft > 0.2 {
                        let mv = (0.75 * cr * dt_yr).min(0.6 * leaf_target - cl).min(0.25 * cr);
                        if mv > 0.0 { lanes[base + 8][i] = mv; used[base + 8] = true; }
                    }
                }
                // turnover and death, to the litter
                // Wood is lost by death, not by sapwood turning to heartwood. Death rises with crowding
                // (growth efficiency in LPJ): a stand of 12 kgC/m2 loses 4 percent a year.
                let bg_wood = if t.woody { t.mort_max * (0.25 + 0.75 * (cw / 12.0).powi(2)).min(2.0) } else { 0.0 };
                let mut mort = 0.0f32;
                // outside its winter limits a type cannot hold on: a 3 K miss kills it in a year
                if tcold < t.cold_min { mort += ((t.cold_min - tcold) / 3.0).min(3.0); }
                if tcold > t.cold_max { mort += ((tcold - t.cold_max) / 5.0).min(2.0); }
                if net < 0.0 && f_w < 0.3 { mort += 0.3; }
                // evergreens cannot drop their leaves in a dry season, and pay for it
                if t.phen == Phen::Evergreen { mort += 1.5 * (1.0 - f_w).powi(2); }
                let k_leaf = (1.0 / t.turn_leaf.max(0.1) + mort).min(3.0) * dt_yr;
                let k_wood = (bg_wood + mort).min(3.0) * dt_yr;
                let k_root = (1.0 / t.turn_root.max(0.1) + mort).min(3.0) * dt_yr;
                lanes[base + 5][i] = cl * (1.0 - (-k_leaf).exp()); lanes[base + 6][i] = cw * (1.0 - (-k_wood).exp()); lanes[base + 7][i] = cr * (1.0 - (-k_root).exp());
                used[base + 5] = true; used[base + 6] = true; used[base + 7] = true;
                // dead tissue becomes litter, which has its own C:N and C:P: it frees or takes nutrients
                let (ll, lw, lr) = (lanes[base + 5][i] as f64, lanes[base + 6][i] as f64, lanes[base + 7][i] as f64);
                dn += ll * (1.0 / CN[0] as f64 - 1.0 / CN_LEAF as f64) + lw * (1.0 / CN[0] as f64 - 1.0 / CN_WOOD as f64) + lr * (1.0 / CN[0] as f64 - 1.0 / CN_ROOT as f64);
                dp += ll * (1.0 / CP[0] as f64 - 1.0 / CP_LEAF as f64) + lw * (1.0 / CP[0] as f64 - 1.0 / CP_WOOD as f64) + lr * (1.0 / CP[0] as f64 - 1.0 / CP_ROOT as f64);
                any = true;
            }
            // mineral nutrients for the new tissue, never more than the pool has
            // the soil module may take up to 40 percent for itself, so plants stop at 30
            // uptake is limited by the mineral pool, release by what the organic pool holds
            n_net[i] = dn.min(0.3 * nm as f64).max(-0.15 * n_org[i].max(0.0) as f64);
            p_net[i] = dp.min(0.3 * pm as f64).max(-0.15 * p_org[i].max(0.0) as f64);
        }
        if std::env::var("ODY_CHK").is_ok() {
            for (k, l) in lanes.iter().enumerate() { if let Some(i) = l.iter().position(|v| !v.is_finite() || *v < 0.0) { eprintln!("bad lane {} (type {} lane {}) tile {} value {}", k, k / L, k % L, i, l[i]); break; } }
            for (nm, v) in [("n_net", &n_net), ("p_net", &p_net)] { if let Some(i) = v.iter().position(|x| !x.is_finite()) { eprintln!("bad {} tile {}", nm, i); } }
        }
        if !any { return Ok(()); }
        for p in 0..NPT {
            let base = p * L;
            let air = eco_air(ids);
            let pools = [eco.leaf[p], eco.wood[p], eco.root[p]];
            for k in 0..3 { if used[base + k] { out.dense_local(air, pools[k], n).copy_from_slice(&lanes[base + k]); } }
            if used[base + 3] { out.dense_local(pools[0], air, n).copy_from_slice(&lanes[base + 3]); }
            if used[base + 8] { out.dense_local(pools[2], pools[0], n).copy_from_slice(&lanes[base + 8]); }
            if used[base + 4] { out.dense_local(pools[2], air, n).copy_from_slice(&lanes[base + 4]); }
            for k in 0..3 { if used[base + 5 + k] { out.dense_local(pools[k], eco.litter, n).copy_from_slice(&lanes[base + 5 + k]); } }
        }
        let (mut n_up, mut n_dn, mut p_up, mut p_dn) = (vec![0f32; n], vec![0f32; n], vec![0f32; n], vec![0f32; n]);
        for i in 0..n {
            if n_net[i] >= 0.0 { n_up[i] = n_net[i] as f32 } else { n_dn[i] = (-n_net[i]) as f32 }
            if p_net[i] >= 0.0 { p_up[i] = p_net[i] as f32 } else { p_dn[i] = (-p_net[i]) as f32 }
        }
        out.dense_local(eco.n_min, eco.n_org, n).copy_from_slice(&n_up);
        out.dense_local(eco.n_org, eco.n_min, n).copy_from_slice(&n_dn);
        out.dense_local(eco.p_min, eco.p_org, n).copy_from_slice(&p_up);
        out.dense_local(eco.p_org, eco.p_min, n).copy_from_slice(&p_dn);
        Ok(())
    }
}

fn eco_air(ids: &Ids) -> StockId { ids.c_air }

#[cfg(test)]
mod tests {
    #[test]
    fn par_share_and_lue_give_realistic_gpp() {
        // a tropical canopy: 420 W/m2 at the top of the air, 90 percent absorbed
        let par = super::PAR_SHARE * 420.0 * 2.63;
        let gpp_year = super::LUE_GC_PER_MJ * par * 0.9 * 12.0 / 1000.0;
        assert!(gpp_year > 2.0 && gpp_year < 4.0, "{gpp_year} kgC/m2/yr at full tilt");
    }
}
