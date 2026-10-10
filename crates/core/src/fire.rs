//! Fire as a front that burns through fuel.
//!
//! Lightning lights dry fuel (random, as nature is). A fire spreads tile to tile while the next tile has
//! enough connected fuel and is dry enough, and stops at wet ground, bare land or by chance. It has a time
//! budget of one month. A front still alive at the month's end is kept in the `fire` stock and goes on in the
//! next month, so fires can smoulder across months. Nothing sets a fire's size ahead of time: no area budget,
//! no ceiling, no per-tile loss. Old burns have no fuel, so they act as firebreaks, and long gaps build fuel for
//! big fires.
//!
//! Fuel is litter plus grass leaves plus a fifth of the tree leaves. A burned tile loses 90 percent of its
//! burned litter and grass leaves to the air (a tenth becomes charcoal in the passive soil pool), burns a
//! share of tree crowns, and kills young trees and shrubs far more than old trees. Three quarters of the
//! burned nitrogen goes to the air, the rest and all the phosphorus to the mineral pools.
//!
//! The strength of lightning is a stated placeholder, set so that the world burns about 350 Mha a year
//! (GFED4, Giglio et al. 2013). Not yet: peat that burns for months, fires set by people, fire weather from wind.

use crate::earth::BANDS;
use crate::eco::*;
use crate::env::*;
use crate::flows::{Endpoint, Event, EventKind, FlowOut};
use crate::grid::Grid;
use crate::hydro::FIELD_CAP;
use crate::module::*;
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;

/// ignitions per km2 per month at the strongest convective weather
pub const LIGHTNING_PER_KM2: f64 = 5.5e-8;
pub const FUEL_MIN: f32 = 0.04; // kgC/m2 below which a tile will not carry fire
pub const FUEL_FULL: f32 = 0.5;

pub struct Fire { env: Env, set: PlantSet, eco: Eco, ids: Option<Ids>, pub lightning: f64 }
struct Ids { heat: StockId, w_land: StockId, w_sea: StockId, fire: StockId, c_air: StockId }

impl Fire {
    pub fn new(env: Env, set: PlantSet, eco: Eco) -> Fire { Fire { env, set, eco, ids: None, lightning: LIGHTNING_PER_KM2 } }
}

impl Module for Fire {
    fn spec(&self) -> ModuleSpec {
        let mut s = ModuleSpec::new("fire", Phase::SoilPlants).boundaries(&["diag", "nloss"]);
        let mut reads: Vec<String> = ["heat", "w_land", "w_sea", "fire", "c_litter", "c_spass", "n_org", "p_org", "c_sfast", "c_sslow", "n_min", "p_min"].iter().map(|x| x.to_string()).collect();
        let mut writes: Vec<String> = ["c_air", "c_litter", "c_spass", "fire", "n_org", "n_min", "p_org", "p_min"].iter().map(|x| x.to_string()).collect();
        for t in &self.set.types { for k in ["leaf", "wood", "root"] { let n = format!("c_{k}_{}", t.id); reads.push(n.clone()); writes.push(n); } }
        s.reads = reads; s.writes = writes;
        s
    }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(Ids { heat: g("heat")?, w_land: g("w_land")?, w_sea: g("w_sea")?, fire: g("fire")?, c_air: g("c_air")? });
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("fire not bound".into()))?;
        let (env, set, eco) = (&self.env, &self.set, &self.eco);
        let n = env.n();
        let grid = ctx.grid;
        let area = &grid.area_km2;
        let m = ctx.month_of_year as usize;
        let days = 30.44 * ctx.dt_months as f32;
        let heat = view.stock(ids.heat)?; let w_land = view.stock(ids.w_land)?; let w_sea = view.stock(ids.w_sea)?;
        let front = view.stock(ids.fire)?;
        let litter = view.stock(eco.litter)?;
        let leaf: Vec<&[f32]> = eco.leaf.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let wood: Vec<&[f32]> = eco.wood.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let root: Vec<&[f32]> = eco.root.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let (n_org, p_org) = (view.stock(eco.n_org)?, view.stock(eco.p_org)?);
        let (n_min, p_min) = (view.stock(eco.n_min)?, view.stock(eco.p_min)?);
        let (cf, cs, cp) = (view.stock(eco.sfast)?, view.stock(eco.sslow)?, view.stock(eco.spass)?);
        let grass: Vec<usize> = set.types.iter().enumerate().filter(|(_, t)| !t.woody && t.id.contains("grass")).map(|(p, _)| p).collect();

        // fuel, dryness and the share of it that is grass, per land tile
        let mut fuel = vec![0f32; n]; let mut dry = vec![0f32; n]; let mut grass_share = vec![0f32; n];
        let mut land = vec![false; n];
        for i in 0..n {
            if (w_sea[i] > 50.0 && env.ocean0[i]) || env.p.earth.ice[i] > 100.0 { continue; }
            land[i] = true;
            let tc = env.t_from_h(i, heat[i]) - 273.15;
            let mut f = litter[i]; let mut g = 0.0;
            for p in 0..NPT {
                if grass.contains(&p) { f += leaf[p][i]; g += leaf[p][i]; } else { f += 0.2 * leaf[p][i]; }
            }
            fuel[i] = f;
            grass_share[i] = if f > 0.0 { (g + 0.5 * litter[i]).min(f) / f } else { 0.0 };
            dry[i] = if tc < 0.0 { 0.0 } else { (1.0 - w_land[i] / FIELD_CAP).clamp(0.0, 1.0) * (0.3 + 0.7 * ((tc - 5.0) / 20.0).clamp(0.0, 1.0)) };
        }
        let mut rng = ctx.rng("fire");
        let mut burned = vec![0f32; n];
        let mut queued = vec![false; n];
        let mut carry = vec![false; n];
        let carried: Vec<usize> = (0..n).filter(|&i| front[i] > 0.5 && land[i]).collect();
        let mut starts: Vec<usize> = Vec::new();
        // lightning: more where it is warm and wet enough to storm, and only where the fuel can burn
        for i in 0..n {
            if !land[i] || fuel[i] < FUEL_MIN { continue; }
            let tc = env.t_from_h(i, heat[i]) - 273.15;
            let conv = ((tc - 5.0) / 25.0).clamp(0.0, 1.0).powf(1.5) * (env.p.earth.prec[m][i] / 100.0).sqrt() as f32;
            let lambda = self.lightning * area[i] as f64 * conv as f64 * dry[i] as f64 * ctx.dt_months as f64;
            if lambda <= 0.0 { continue; }
            // Poisson with a small rate: at most a couple per tile and month
            let mut k = 0; let mut l = (-lambda).exp(); let mut cum = l; let u = rng.f64();
            while u > cum && k < 4 { k += 1; l *= lambda / k as f64; cum += l; }
            if k > 0 { starts.push(i); }
        }
        starts.sort_unstable(); starts.dedup();
        // a carried front spreads on from tiles that already burned; they do not burn twice
        for &c in &carried { queued[c] = true; }
        let burn_frac = |i: usize| (0.15 + 0.75 * dry[i]).min(0.95);
        let mut fires: Vec<(usize, f64)> = Vec::new();
        let mut jobs: Vec<(usize, bool)> = carried.iter().map(|&c| (c, true)).collect();
        for &s in &starts { if !queued[s] { jobs.push((s, false)); } }
        for (s, is_carried) in jobs {
            if !is_carried && (burned[s] > 0.0 || fuel[s] < FUEL_MIN) { continue; }
            let mut frontier = vec![s]; queued[s] = true;
            let mut time_left = days;
            let mut area_burn = 0f64;
            let mut first = true;
            while !frontier.is_empty() {
                let mut speed = 0.0f32;
                for &i in &frontier {
                    if !(first && is_carried) { burned[i] = burn_frac(i); area_burn += burned[i] as f64 * area[i] as f64; }
                    speed += (2.0 + 40.0 * grass_share[i]) * (0.3 + 0.7 * dry[i]);
                }
                first = false;
                speed /= frontier.len() as f32;
                let cost = env.p.tile_km[frontier[0]] / speed.max(0.2);
                if cost > time_left {
                    // a front still alive at the month's end goes on only if the weather keeps it alive
                    for &i in &frontier { if rng.f64() < (0.2 + 0.8 * dry[i]) as f64 * 0.7 { carry[i] = true; } }
                    break;
                }
                time_left -= cost;
                let mut next = Vec::new();
                for &i in &frontier {
                    for j in grid.neighbours(i) {
                        if queued[j] || !land[j] || fuel[j] < FUEL_MIN { continue; }
                        let p = ((fuel[j] / FUEL_FULL).min(1.0) * dry[j] * 1.1).clamp(0.0, 0.95);
                        if rng.f64() < p as f64 { queued[j] = true; next.push(j); }
                    }
                }
                frontier = next;
            }
            if area_burn > 0.0 { fires.push((s, area_burn)); }
        }
        for &(s, a) in &fires { out.events.push(Event { month: ctx.month_index, kind: EventKind::Fire, tile: s as u32, size: a as f32 }); }

        // clear the old front, mark the new one
        let old: Vec<f32> = front.to_vec();
        out.dense(Endpoint::Stock(ids.fire), Endpoint::Boundary(env.ph.b_diag), n).copy_from_slice(&old);
        let newf: Vec<f32> = carry.iter().map(|&c| if c { 1.0 } else { 0.0 }).collect();
        if carry.iter().any(|&c| c) { out.dense(Endpoint::Boundary(env.ph.b_diag), Endpoint::Stock(ids.fire), n).copy_from_slice(&newf); }

        // effects on every burned tile
        let tr = Endpoint::Stock;
        let air = tr(ids.c_air);
        for i in 0..n {
            let f = burned[i];
            if f <= 0.0 { continue; }
            let a = area[i] as f64;
            // N and P bookkeeping: burned carbon frees what it held (a share of the nitrogen goes to the air),
            // dead tissue that turns to litter frees or takes nutrients to match litter's own ratios
            let (mut n_burn, mut p_burn, mut n_adj, mut p_adj) = (0f64, 0f64, 0f64, 0f64);
            let lit = 0.9 * f as f64 * litter[i] as f64;
            if lit > 0.0 {
                out.push(tr(eco.litter), air, i, i, 0.9 * lit * a);
                out.push(tr(eco.litter), tr(eco.spass), i, i, 0.1 * lit * a);
                n_burn += 0.9 * lit / CN[0] as f64; p_burn += 0.9 * lit / CP[0] as f64;
                n_adj += 0.1 * lit * (1.0 / CN[3] as f64 - 1.0 / CN[0] as f64); p_adj += 0.1 * lit * (1.0 / CP[3] as f64 - 1.0 / CP[0] as f64);
            }
            for p in 0..NPT {
                let t = &set.types[p];
                let cl = leaf[p][i] as f64; let cw = wood[p][i] as f64; let cr = root[p][i] as f64;
                if cl + cw + cr < 1e-9 { continue; }
                let grassy = !t.woody && t.id.contains("grass");
                let crown = if grassy { 0.9 } else { 0.4 * t.flam as f64 + 0.1 };
                let leaf_burn = f as f64 * crown * cl;
                if leaf_burn > 0.0 {
                    out.push(tr(eco.leaf[p]), air, i, i, 0.9 * leaf_burn * a);
                    out.push(tr(eco.leaf[p]), tr(eco.spass), i, i, 0.1 * leaf_burn * a);
                    n_burn += 0.9 * leaf_burn / CN_LEAF as f64; p_burn += 0.9 * leaf_burn / CP_LEAF as f64;
                    n_adj += 0.1 * leaf_burn * (1.0 / CN[3] as f64 - 1.0 / CN_LEAF as f64); p_adj += 0.1 * leaf_burn * (1.0 / CP[3] as f64 - 1.0 / CP_LEAF as f64);
                }
                if t.woody {
                    let young = (-cw / 1.5).exp();
                    let kill = (f as f64 * (0.1 + 0.4 * t.flam as f64) * (1.0 + 2.5 * young)).min(0.95);
                    let wl = kill * cw; let rl = kill * cr; let ll = kill * (cl - leaf_burn).max(0.0);
                    if wl > 0.0 { out.push(tr(eco.wood[p]), tr(eco.litter), i, i, wl * a); n_adj += wl * (1.0 / CN[0] as f64 - 1.0 / CN_WOOD as f64); p_adj += wl * (1.0 / CP[0] as f64 - 1.0 / CP_WOOD as f64); }
                    if rl > 0.0 { out.push(tr(eco.root[p]), tr(eco.litter), i, i, rl * a); n_adj += rl * (1.0 / CN[0] as f64 - 1.0 / CN_ROOT as f64); p_adj += rl * (1.0 / CP[0] as f64 - 1.0 / CP_ROOT as f64); }
                    if ll > 0.0 { out.push(tr(eco.leaf[p]), tr(eco.litter), i, i, ll * a); n_adj += ll * (1.0 / CN[0] as f64 - 1.0 / CN_LEAF as f64); p_adj += ll * (1.0 / CP[0] as f64 - 1.0 / CP_LEAF as f64); }
                }
            }
            // burned nutrients leave organic matter; litter-ratio changes move the rest
            let nb = n_burn.min(0.4 * n_org[i] as f64);
            let pb = p_burn.min(0.4 * p_org[i] as f64);
            if nb > 0.0 {
                out.push(tr(eco.n_org), Endpoint::Boundary(eco.b_nloss), i, i, 0.75 * nb * a);
                out.push(tr(eco.n_org), tr(eco.n_min), i, i, 0.25 * nb * a);
            }
            if pb > 0.0 { out.push(tr(eco.p_org), tr(eco.p_min), i, i, pb * a); }
            if n_adj > 0.0 { out.push(tr(eco.n_min), tr(eco.n_org), i, i, n_adj.min(0.2 * n_min[i].max(0.0) as f64) * a); }
            else if n_adj < 0.0 { out.push(tr(eco.n_org), tr(eco.n_min), i, i, (-n_adj).min(0.2 * n_org[i] as f64) * a); }
            if p_adj > 0.0 { out.push(tr(eco.p_min), tr(eco.p_org), i, i, p_adj.min(0.2 * p_min[i].max(0.0) as f64) * a); }
            else if p_adj < 0.0 { out.push(tr(eco.p_org), tr(eco.p_min), i, i, (-p_adj).min(0.2 * p_org[i] as f64) * a); }
        }
        let _ = BANDS;
        Ok(())
    }
}
