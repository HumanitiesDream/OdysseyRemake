//! Animals that eat what the land grows, and each other.
//!
//! Twelve land guilds (sea life is in `sealife`) live on the same tiles as the plants. Each month a tile's
//! guilds are stepped together in three ten-day sub-steps, in a fixed order (plant eaters first):
//!
//! - Intake follows a Holling type II curve: I = Imax B F / (F + K). F is the food the guild can reach:
//!   leaves (trees only down to the guild's reach), litter, carcasses, or prey that has not hidden
//!   (tree cover is refuge for small prey). What is eaten is gone from the plants or the prey.
//! - Rates follow metabolic theory (Brown et al. 2004): intake, respiration and natural death all scale as
//!   body mass to the minus one quarter, so slow megafauna and fast rodents come from one rule.
//! - Part of the intake is assimilated into body carbon (the guild's assimilation efficiency). The rest
//!   is dung, to the litter. Respiration goes to the air. Deaths go to the carcass pool (most of it) and
//!   the litter. Scavengers eat the carcass pool. What is left rots into the litter at 30 percent a month.
//! - Crowding adds disease: death rises with the square of the density.
//! - Movement: animals move toward neighbours with more to eat, up to the guild's monthly range. They do
//!   not cross the sea. Herds that follow the green season come out of this.
//!
//! There is no capacity map and no cap on kills. Overshoot, crashes and predator cycles come from the
//! numbers. Nitrogen and phosphorus follow carbon with the animals' own C:N of 5 and C:P of 60.
//! Not yet: islands colonised by swimming or rafting, seed dispersal by animals, fish.

use crate::eco::*;
use crate::env::*;
use crate::flows::{Endpoint, Event, EventKind, FlowOut};
use crate::grid::Grid;
use crate::module::*;
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;

pub const CN_FAUNA: f64 = 5.0;
pub const CP_FAUNA: f64 = 60.0;
pub const SUB: usize = 3;
const CARCASS_ROT_PER_MONTH: f64 = 0.3;
const MAX_NG: usize = 16;

pub struct Fauna { env: Env, set: PlantSet, gs: GuildSet, eco: Eco, ids: Option<Ids> }
struct Ids { heat: StockId, w_sea: StockId, c_air: StockId }

impl Fauna {
    pub fn new(env: Env, set: PlantSet, gs: GuildSet, eco: Eco) -> Fauna { Fauna { env, set, gs, eco, ids: None } }
}

impl Module for Fauna {
    fn spec(&self) -> ModuleSpec {
        let mut s = ModuleSpec::new("fauna", Phase::Animals);
        let mut reads: Vec<String> = ["heat", "w_sea", "c_litter", "c_carcass", "n_org", "p_org", "n_min", "p_min"].iter().map(|x| x.to_string()).collect();
        let mut writes: Vec<String> = ["c_air", "c_litter", "c_carcass", "n_org", "n_min", "p_org", "p_min"].iter().map(|x| x.to_string()).collect();
        for t in &self.set.types { reads.push(format!("c_leaf_{}", t.id)); writes.push(format!("c_leaf_{}", t.id)); reads.push(format!("c_wood_{}", t.id)); writes.push(format!("c_wood_{}", t.id)); }
        for g in &self.gs.guilds { reads.push(format!("c_fauna_{}", g.id)); writes.push(format!("c_fauna_{}", g.id)); }
        s.reads = reads; s.writes = writes;
        s
    }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(Ids { heat: g("heat")?, w_sea: g("w_sea")?, c_air: g("c_air")? });
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("fauna not bound".into()))?;
        let (env, set, gs, eco) = (&self.env, &self.set, &self.gs, &self.eco);
        let n = env.n();
        let ng = gs.guilds.len();
        assert!(ng <= MAX_NG);
        let grid = ctx.grid;
        let area = &grid.area_km2;
        let dt = ctx.dt_months as f64 / 12.0;
        let h = dt / SUB as f64;
        let heat = view.stock(ids.heat)?; let w_sea = view.stock(ids.w_sea)?;
        let litter = view.stock(eco.litter)?; let carcass = view.stock(eco.carcass)?;
        let (n_org, p_org, n_min, p_min) = (view.stock(eco.n_org)?, view.stock(eco.p_org)?, view.stock(eco.n_min)?, view.stock(eco.p_min)?);
        let leaf: Vec<&[f32]> = eco.leaf.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let wood: Vec<&[f32]> = eco.wood.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let fa: Vec<&[f32]> = eco.fauna.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let slas: Vec<f32> = set.types.iter().map(|t| t.sla).collect();
        let woody: Vec<bool> = set.types.iter().map(|t| t.woody).collect();
        let mega = gs.guilds.iter().position(|g| g.id == "megaherbivore");

        // final biomass per tile and guild, and the attractiveness for movement
        let mut b_end = vec![[0f32; MAX_NG]; n];
        let mut attract = vec![[0f32; MAX_NG]; n];
        let mut active = vec![false; n];
        let (mut dn_net, mut dp_net) = (vec![0f64; n], vec![0f64; n]);
        let mut events: Vec<Event> = Vec::new();

        for i in 0..n {
            if (w_sea[i] > 50.0 && env.ocean0[i]) || env.p.earth.ice[i] > 100.0 { continue; }
            let any: f32 = (0..ng).map(|g| fa[g][i]).sum();
            let food: f32 = (0..NPT).map(|p| leaf[p][i]).sum();
            if any < 1e-12 && food < 1e-6 { continue; }
            active[i] = true;
            let t_c = env.t_from_h(i, heat[i]) as f64 - 273.15;
            let cover = { let l: Vec<f32> = (0..NPT).map(|p| leaf[p][i]).collect(); cover_of(&slas, &l) as f64 };
            // local state
            let mut b = [0f64; MAX_NG];
            for g in 0..ng { b[g] = fa[g][i].max(0.0) as f64; }
            let mut lf = [0f64; NPT]; let mut wd = [0f64; NPT];
            for p in 0..NPT { lf[p] = leaf[p][i].max(0.0) as f64; wd[p] = wood[p][i].max(0.0) as f64; }
            let (mut lit, mut carc) = (litter[i].max(0.0) as f64, carcass[i].max(0.0) as f64);
            let (lit0, carc0) = (lit, carc);
            // cumulative flows over the month
            let mut eat_leaf = [[0f64; NPT]; MAX_NG];
            let mut eat_wood = [0f64; NPT];
            let mut eat_prey = [[0f64; MAX_NG]; MAX_NG];
            let mut eat_lit = [0f64; MAX_NG]; let mut eat_carc = [0f64; MAX_NG];
            let mut resp = [0f64; MAX_NG]; let mut death_c = [0f64; MAX_NG]; let mut death_l = [0f64; MAX_NG];
            let mut rot = 0f64;
            let mut disease_hit = false;
            for _s in 0..SUB {
                for g in 0..ng {
                    let gd = &gs.guilds[g];
                    if b[g] <= 0.0 { continue; }
                    let mut gain_c = 0.0; // carbon eaten this sub-step
                    // plants
                    if !gd.plants.is_empty() {
                        let f: f64 = gd.plants.iter().map(|&(p, acc)| acc as f64 * lf[p]).sum();
                        if f > 1e-12 {
                            let want = gd.i_max as f64 * b[g] * h * f / (f + gs.k_half as f64);
                            let take = want.min(0.15 * f);
                            for &(p, acc) in &gd.plants {
                                let d = take * acc as f64 * lf[p] / f;
                                lf[p] -= d; eat_leaf[g][p] += d; gain_c += d; lit += (1.0 - gd.assim as f64) * d;
                            }
                            // megaherbivores also push over and eat trees
                            if Some(g) == mega {
                                for p in 0..NPT { if woody[p] { let d = (0.02 * take * wd[p] / (wd[p] + 1.0)).min(0.02 * wd[p]); wd[p] -= d; eat_wood[p] += d; gain_c += d; } }
                            }
                        }
                    }
                    // litter and carcass
                    if gd.eats_litter {
                        let want = gd.i_max as f64 * b[g] * h * (0.05 * lit) / (0.05 * lit + gs.k_half as f64);
                        let d = want.min(0.2 * lit); lit -= gd.assim as f64 * d; eat_lit[g] += d; gain_c += d;
                    }
                    if gd.eats_carcass {
                        let want = gd.i_max as f64 * b[g] * h * carc / (carc + 0.05 * gs.k_half as f64);
                        let d = want.min(0.5 * carc); carc -= d; lit += (1.0 - gd.assim as f64) * d; eat_carc[g] += d; gain_c += d;
                    }
                    // prey
                    if !gd.prey.is_empty() {
                        let mut avail = [0f64; MAX_NG]; let mut tot = 0.0;
                        for &(q, v) in &gd.prey {
                            let refuge = match gs.guilds[q].id.as_str() { "small_herbivore" | "insectivore" | "bird" | "insect" => 0.6 * cover, _ => 0.2 * cover };
                            avail[q] = v as f64 * b[q] * (1.0 - refuge); tot += avail[q];
                        }
                        if tot > 1e-14 {
                            let want = gd.i_max as f64 * b[g] * h * tot / (tot + gs.k_prey as f64);
                            let take = want.min(0.4 * tot);
                            for &(q, _) in &gd.prey {
                                let share = avail[q] / tot;
                                let d = (take * share).min(0.5 * b[q]);
                                b[q] -= d; eat_prey[g][q] += d; gain_c += d; lit += (1.0 - gd.assim as f64) * d;
                            }
                        }
                    }
                    b[g] += gd.assim as f64 * gain_c;
                }
                // upkeep, deaths, rot
                for g in 0..ng {
                    let gd = &gs.guilds[g];
                    if b[g] <= 0.0 { continue; }
                    let q10 = if gd.endotherm { 1.0 } else { 2f64.powf((t_c - 20.0) / 10.0).clamp(0.05, 3.0) };
                    let dis = 0.4 * (b[g] / gs.disease as f64).powi(2).min(4.0);
                    if dis > 0.1 { disease_hit = true; }
                    let (kr, km) = (gd.resp as f64 * q10, gd.mort as f64 * q10 + dis);
                    let tot = b[g] * (1.0 - (-(kr + km) * h).exp());
                    let r = tot * kr / (kr + km); let dth = tot - r;
                    b[g] -= tot;
                                        resp[g] += r; death_c[g] += 0.85 * dth; death_l[g] += 0.15 * dth; carc += 0.85 * dth; lit += 0.15 * dth;
                }
                let r = carc * (1.0 - (-CARCASS_ROT_PER_MONTH * 12.0 * h).exp());
                carc -= r; lit += r; rot += r;
            }
            let _ = (lit0, carc0);
            if disease_hit { events.push(Event { month: ctx.month_index, kind: EventKind::Outbreak, tile: i as u32, size: b.iter().sum::<f64>() as f32 }); }
            // book the month. Dung (the part not assimilated) goes from the food to the litter.
            let a = area[i] as f64;
            let mut dn = 0f64; let mut dp = 0f64;
            let fau = |g: usize| Endpoint::Stock(eco.fauna[g]);
            let litter_e = Endpoint::Stock(eco.litter);
            for g in 0..ng {
                let al = gs.guilds[g].assim as f64;
                for p in 0..NPT {
                    let d = eat_leaf[g][p];
                    if d > 0.0 {
                        out.push(Endpoint::Stock(eco.leaf[p]), fau(g), i, i, al * d * a);
                        out.push(Endpoint::Stock(eco.leaf[p]), litter_e, i, i, (1.0 - al) * d * a);
                        dn += al * d * (1.0 / CN_FAUNA - 1.0 / CN_LEAF as f64) + (1.0 - al) * d * (1.0 / CN[0] as f64 - 1.0 / CN_LEAF as f64);
                        dp += al * d * (1.0 / CP_FAUNA - 1.0 / CP_LEAF as f64) + (1.0 - al) * d * (1.0 / CP[0] as f64 - 1.0 / CP_LEAF as f64);
                    }
                }
                for q in 0..ng {
                    let d = eat_prey[g][q];
                    if d > 0.0 {
                        out.push(fau(q), fau(g), i, i, al * d * a);
                        out.push(fau(q), litter_e, i, i, (1.0 - al) * d * a);
                        dn += (1.0 - al) * d * (1.0 / CN[0] as f64 - 1.0 / CN_FAUNA); dp += (1.0 - al) * d * (1.0 / CP[0] as f64 - 1.0 / CP_FAUNA);
                    }
                }
                if eat_lit[g] > 0.0 {
                    let d = eat_lit[g]; out.push(litter_e, fau(g), i, i, al * d * a);
                    // the part not assimilated stays litter: no flow
                    dn += al * d * (1.0 / CN_FAUNA - 1.0 / CN[0] as f64); dp += al * d * (1.0 / CP_FAUNA - 1.0 / CP[0] as f64);
                }
                if eat_carc[g] > 0.0 {
                    let d = eat_carc[g];
                    out.push(Endpoint::Stock(eco.carcass), fau(g), i, i, al * d * a);
                    out.push(Endpoint::Stock(eco.carcass), litter_e, i, i, (1.0 - al) * d * a);
                    dn += (1.0 - al) * d * (1.0 / CN[0] as f64 - 1.0 / CN_FAUNA); dp += (1.0 - al) * d * (1.0 / CP[0] as f64 - 1.0 / CP_FAUNA);
                }
                if resp[g] > 0.0 { out.push(fau(g), Endpoint::Stock(ids.c_air), i, i, resp[g] * a); dn -= resp[g] / CN_FAUNA; dp -= resp[g] / CP_FAUNA; }
                if death_c[g] > 0.0 { out.push(fau(g), Endpoint::Stock(eco.carcass), i, i, death_c[g] * a); }
                if death_l[g] > 0.0 { out.push(fau(g), litter_e, i, i, death_l[g] * a); dn += death_l[g] * (1.0 / CN[0] as f64 - 1.0 / CN_FAUNA); dp += death_l[g] * (1.0 / CP[0] as f64 - 1.0 / CP_FAUNA); }
            }
            for p in 0..NPT { if eat_wood[p] > 0.0 { out.push(Endpoint::Stock(eco.wood[p]), fau(mega.unwrap()), i, i, 0.45 * eat_wood[p] * a); out.push(Endpoint::Stock(eco.wood[p]), litter_e, i, i, 0.55 * eat_wood[p] * a); } }
            if rot > 0.0 { out.push(Endpoint::Stock(eco.carcass), litter_e, i, i, rot * a); }
            dn_net[i] = dn.min(0.3 * n_min[i].max(0.0) as f64).max(-0.15 * n_org[i].max(0.0) as f64);
            dp_net[i] = dp.min(0.3 * p_min[i].max(0.0) as f64).max(-0.15 * p_org[i].max(0.0) as f64);
            for g in 0..ng { b_end[i][g] = b[g].max(0.0) as f32; }
            // what the guilds would find to eat here, for the neighbours' decisions
            for g in 0..ng {
                let gd = &gs.guilds[g];
                let mut f = 0.0f64; let mut k = gs.k_half as f64;
                if !gd.plants.is_empty() { f += gd.plants.iter().map(|&(p, acc)| acc as f64 * lf[p]).sum::<f64>(); }
                if gd.eats_litter { f += 0.05 * lit; }
                if gd.eats_carcass { f += carc; }
                if !gd.prey.is_empty() { f += gd.prey.iter().map(|&(q, v)| v as f64 * b[q]).sum::<f64>(); k = gs.k_prey as f64; }
                attract[i][g] = (f / (f + k)) as f32;
            }
        }
        for e in events { out.events.push(e); }
        // nitrogen and phosphorus follow the carbon
        {
            let (mut n_up, mut n_dn, mut p_up, mut p_dn) = (vec![0f32; n], vec![0f32; n], vec![0f32; n], vec![0f32; n]);
            for i in 0..n {
                if dn_net[i] >= 0.0 { n_up[i] = dn_net[i] as f32 } else { n_dn[i] = (-dn_net[i]) as f32 }
                if dp_net[i] >= 0.0 { p_up[i] = dp_net[i] as f32 } else { p_dn[i] = (-dp_net[i]) as f32 }
            }
            out.dense_local(eco.n_min, eco.n_org, n).copy_from_slice(&n_up);
            out.dense_local(eco.n_org, eco.n_min, n).copy_from_slice(&n_dn);
            out.dense_local(eco.p_min, eco.p_org, n).copy_from_slice(&p_up);
            out.dense_local(eco.p_org, eco.p_min, n).copy_from_slice(&p_dn);
        }
        // movement toward better food, from what is left at the end of the month
        for g in 0..ng {
            let gd = &gs.guilds[g];
            for i in 0..n {
                if !active[i] || b_end[i][g] < 1e-10 { continue; }
                let mut wsum = 0f32; let mut ws = [0f32; 6];
                for (k, j) in grid.nbr[i].iter().enumerate() {
                    if *j < 0 { continue; }
                    let j = *j as usize;
                    if !active[j] { continue; }
                    let w = (attract[j][g] - attract[i][g]).max(0.0);
                    ws[k] = w; wsum += w;
                }
                if wsum <= 0.0 { continue; }
                let frac = (gd.move_km as f64 / env.p.tile_km[i] as f64).min(0.4) * 0.9;
                let giving = b_end[i][g] as f64 * frac;
                for (k, j) in grid.nbr[i].iter().enumerate() {
                    if ws[k] <= 0.0 { continue; }
                    let amt = giving * ws[k] as f64 / wsum as f64 * area[i] as f64;
                    out.push(Endpoint::Stock(eco.fauna[g]), Endpoint::Stock(eco.fauna[g]), i, *j as usize, amt);
                    // the body's nutrients move along: nitrogen and phosphorus ride with the carbon
                    out.push(Endpoint::Stock(eco.n_org), Endpoint::Stock(eco.n_org), i, *j as usize, (amt / CN_FAUNA).min(0.1 * n_org[i].max(0.0) as f64 * area[i] as f64));
                    out.push(Endpoint::Stock(eco.p_org), Endpoint::Stock(eco.p_org), i, *j as usize, (amt / CP_FAUNA).min(0.1 * p_org[i].max(0.0) as f64 * area[i] as f64));
                }
            }
        }
        Ok(())
    }
}

