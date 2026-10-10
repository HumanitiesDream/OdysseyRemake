//! Water on land and in the air, one booked cycle.
//!
//! Each month, per tile:
//! - Evaporation takes water from the sea or from soil water. Potential evaporation is the
//!   Oudin et al. (2005) form: sunlight times (T + 5)/100. Soil water limits it.
//! - Rain over land is today's observed rain, scaled by 7 percent per degree of departure
//!   (Clausius-Clapeyron). Rain over the sea is whatever closes the books: evaporation minus
//!   rain on land. So the air gains and loses exactly the same water every month.
//! - Each height band decides snow or rain from its own temperature (lapse 6.5 K/km),
//!   keeps a snowpack, melts it by degree-days (4.5 mm/K/day for snow, 7 for ice), and turns
//!   old snow into ice.
//! - Soil water drains to groundwater and to rivers. Rivers follow the live surface
//!   (`routing`) into the sea or into a lake that fills, spills and overflows.
//!
//! Missing on purpose for now: plants (they will take over transpiration), refreezing in
//! the snowpack, frozen ground.

use crate::climate::InsolTable;
use crate::earth::BANDS;
use crate::env::*;
use crate::flows::{Endpoint, FlowOut};
use crate::grid::Grid;
use crate::module::*;
use crate::routing::{Routing, NONE};
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;

pub const DAYS_PER_MONTH: f32 = 30.44;
pub const DDF_SNOW: f32 = 4.5; // mm per K per day, carried over from v0.49
pub const DDF_ICE: f32 = 7.0; // mm per K per day, typical 6 to 9 for glacier ice
pub const FIRN_LIMIT_MM: f32 = 300.0;
pub const W_HALF: f32 = 100.0;
pub const FIELD_CAP: f32 = 120.0;
pub const P_PER_K: f32 = 0.07;

/// Oudin et al. (2005): potential evaporation, mm per day, from daily mean insolation (W/m2)
/// and air temperature (C).
pub fn pet_oudin(q_wm2: f32, t_c: f32) -> f32 {
    if t_c + 5.0 <= 0.0 { 0.0 } else { (q_wm2 * 0.0864 / 2.45) * (t_c + 5.0) / 100.0 }
}

/// Water-filling: depth `d_m` of water over the bands (bed heights ascending) gives a level.
/// Returns (level, number of bands under water).
pub fn waterfill(z: &[f32], d_m: f32) -> (f32, usize) {
    if d_m <= 0.0 { return (z[0], 0); }
    let n = z.len();
    let mut acc = 0.0f64;
    for k in 1..=n {
        acc += z[k - 1] as f64;
        let level = (d_m as f64 * n as f64 + acc) / k as f64;
        if k == n || level <= z[k] as f64 { return (level as f32, k); }
    }
    unreachable!()
}

pub struct Hydro {
    env: Env,
    ids: Option<Ids>,
    insol: Option<InsolTable>,
    rt: Option<Routing>,
}
struct Ids {
    runoff_y: StockId, heat: StockId, w_air: StockId, w_sea: StockId, w_land: StockId, w_ground: StockId,
    snow: Vec<StockId>, ice: Vec<StockId>,
    rock: StockId, reg: StockId, sg: StockId, ss: StockId, sm: StockId, defl: StockId,
}

impl Hydro {
    pub fn new(env: Env) -> Hydro { Hydro { env, ids: None, insol: None, rt: None } }
}

impl Module for Hydro {
    fn spec(&self) -> ModuleSpec {
        let mut reads: Vec<String> = ["heat", "w_sea", "w_land", "w_ground", "rock", "reg", "sed_g", "sed_s", "sed_m", "defl"].iter().map(|s| s.to_string()).collect();
        let mut writes: Vec<String> = ["w_air", "w_sea", "w_land", "w_ground", "runoff_y"].iter().map(|s| s.to_string()).collect();
        for b in 0..BANDS { for k in ["w_snow", "w_ice"] { reads.push(format!("{k}{b}")); writes.push(format!("{k}{b}")); } }
        let mut s = ModuleSpec::new("hydro", Phase::Water);
        s.reads = reads; s.writes = writes;
        s.boundaries = vec!["diag".into()];
        s
    }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(Ids {
            runoff_y: g("runoff_y")?, heat: g("heat")?, w_air: g("w_air")?, w_sea: g("w_sea")?, w_land: g("w_land")?, w_ground: g("w_ground")?,
            snow: (0..BANDS).map(|b| g(&format!("w_snow{b}"))).collect::<Result<_, _>>()?,
            ice: (0..BANDS).map(|b| g(&format!("w_ice{b}"))).collect::<Result<_, _>>()?,
            rock: g("rock")?, reg: g("reg")?, sg: g("sed_g")?, ss: g("sed_s")?, sm: g("sed_m")?, defl: g("defl")?,
        });
        Ok(())
    }

    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("hydro not bound".into()))?;
        let env = &self.env;
        let n = env.n();
        let m = ctx.month_of_year as usize;
        let grid = ctx.grid;
        let area = &grid.area_km2;
        if self.insol.as_ref().map_or(true, |t| (t.year - ctx.year).abs() >= 25) { self.insol = Some(InsolTable::build(ctx.year)); }
        let insol = self.insol.as_ref().unwrap();

        let heat = view.stock(ids.heat)?;
        let w_sea = view.stock(ids.w_sea)?;
        let w_land = view.stock(ids.w_land)?;
        let w_ground = view.stock(ids.w_ground)?;
        let snow: Vec<&[f32]> = ids.snow.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let ice: Vec<&[f32]> = ids.ice.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let (rock, regl, sg, ss, sm, defl) = (view.stock(ids.rock)?, view.stock(ids.reg)?, view.stock(ids.sg)?, view.stock(ids.ss)?, view.stock(ids.sm)?, view.stock(ids.defl)?);

        // geometry and sea cover
        let mut shift = vec![0f32; n];
        let mut is_sea = vec![false; n];
        let mut surface = vec![0f32; n];
        for i in 0..n {
            shift[i] = env.bed_shift(i, rock[i], regl[i], sg[i], ss[i], sm[i], defl[i]);
            let mut zb = [0f32; BANDS];
            for b in 0..BANDS { zb[b] = env.band_bed(i, b, shift[i]); }
            if w_sea[i] > 0.0 {
                let (_, k) = waterfill(&zb, w_sea[i] / 1000.0);
                is_sea[i] = k * 2 >= BANDS;
            }
            let mean_ice: f32 = (0..BANDS).map(|b| Env::ice_m(ice[b][i])).sum::<f32>() / BANDS as f32;
            surface[i] = zb.iter().sum::<f32>() / BANDS as f32 + mean_ice;
        }
        if self.rt.is_none() || ctx.month_index % 12 == 0 {
            self.rt = Some(Routing::build(grid, &surface, &is_sea, area));
        }
        let rt = self.rt.as_ref().unwrap();

        let tobs = &env.tobs[m];
        let prec = &env.p.earth.prec[m];
        let days = DAYS_PER_MONTH * ctx.dt_months as f32;

        // per-tile arrays for dense lanes
        let mut e_land = vec![0f32; n]; let mut e_sea = vec![0f32; n];
        let mut rain_to_land = vec![0f32; n]; let mut p_sea = vec![0f32; n];
        let mut snow_in: Vec<Vec<f32>> = (0..BANDS).map(|_| vec![0f32; n]).collect();
        let mut melt_snow: Vec<Vec<f32>> = (0..BANDS).map(|_| vec![0f32; n]).collect();
        let mut melt_ice: Vec<Vec<f32>> = (0..BANDS).map(|_| vec![0f32; n]).collect();
        let mut firn: Vec<Vec<f32>> = (0..BANDS).map(|_| vec![0f32; n]).collect();
        let mut drain_deep = vec![0f32; n]; let mut base_flow = vec![0f32; n];
        let mut run = vec![0f32; n];
        let mut flood_land = vec![0f32; n];
        let mut sea_snow: Vec<Vec<f32>> = (0..BANDS).map(|_| vec![0f32; n]).collect();
        let mut sea_ice: Vec<Vec<f32>> = (0..BANDS).map(|_| vec![0f32; n]).collect();
        let mut p_land_tile = vec![0f32; n];
        let (mut sum_e, mut sum_p_land) = (0f64, 0f64);
        let mut sea_weight_sum = 0f64;

        for i in 0..n {
            let a = area[i] as f64;
            let t_tile = env.t_from_h(i, heat[i]);
            let lat = env.p.lat_deg[i];
            let q = insol.at(lat, m);
            if is_sea[i] {
                let pet = pet_oudin(q, t_tile - 273.15) * days;
                e_sea[i] = pet * 0.75; // ocean coefficient, placeholder tuned to about 1250 mm a year
                sum_e += e_sea[i] as f64 * a;
                sea_weight_sum += prec[i] as f64 * a;
                // anything that sat on this tile as land water or snow now joins the sea
                flood_land[i] = w_land[i];
                for b in 0..BANDS {
                    sea_snow[b][i] = snow[b][i] * 0.25;
                    sea_ice[b][i] = ice[b][i] * 0.25;
                }
                continue;
            }
            // land: band temperatures and the snow and ice budget
            let mut t_sum = 0f32; let mut f_ice = 0f32;
            let mut tb = [0f32; BANDS];
            for b in 0..BANDS {
                let ice_m = Env::ice_m(ice[b][i]);
                let z_surf = env.band_bed(i, b, shift[i]) + ice_m;
                tb[b] = env.band_temp(i, t_tile, z_surf);
                t_sum += tb[b];
                if ice_m > 1.0 { f_ice += 1.0 / BANDS as f32; }
            }
            let t_mean = t_sum / BANDS as f32;
            let theta = (t_mean - tobs[i]).clamp(-40.0, 15.0);
            let p_tile = prec[i] * (P_PER_K * theta).exp();
            p_land_tile[i] = p_tile;
            sum_p_land += p_tile as f64 * a;
            let pet = pet_oudin(q, t_mean - 273.15) * days;
            let w = w_land[i];
            e_land[i] = (pet * w / (w + W_HALF) * (1.0 - 0.9 * f_ice)).min(0.3 * w);
            sum_e += e_land[i] as f64 * a;

            let mut rain = 0f32;
            for b in 0..BANDS {
                let sf = snow_fraction(tb[b]);
                let p_b = p_tile; // local mm over the band; stock share is 1/BANDS of it
                snow_in[b][i] = p_b * sf / BANDS as f32;
                rain += p_b * (1.0 - sf) / BANDS as f32;
                // melt, in local depth then converted to stock share
                let pos = (tb[b] - 273.15).max(0.0);
                let pot = DDF_SNOW * pos * days;
                let s_loc = (snow[b][i] * BANDS as f32).max(0.0);
                let ms = pot.min(s_loc);
                melt_snow[b][i] = ms / BANDS as f32;
                let i_loc = (ice[b][i] * BANDS as f32).max(0.0);
                let mi = ((pot - ms) * DDF_ICE / DDF_SNOW).clamp(0.0, 0.5 * i_loc);
                melt_ice[b][i] = mi / BANDS as f32;
                let s_left = s_loc - ms;
                if s_left > FIRN_LIMIT_MM { firn[b][i] = (s_left - FIRN_LIMIT_MM) / 12.0 * ctx.dt_months as f32 / BANDS as f32; }
            }
            rain_to_land[i] = rain;
            // soil water: drainage to groundwater and quick runoff
            drain_deep[i] = 0.04 * w;
            run[i] = 0.4 * (w - FIELD_CAP).max(0.0) + 0.04 * w;
            base_flow[i] = 0.08 * w_ground[i];
        }

        // close the air: sea rain is what evaporation leaves after rain on land
        let mut p_land_scale = 1.0f64;
        let mut p_sea_total = sum_e - sum_p_land;
        if p_sea_total < 0.02 * sum_e {
            p_land_scale = (0.98 * sum_e) / sum_p_land.max(1e-30);
            p_sea_total = 0.02 * sum_e;
        }
        if p_land_scale != 1.0 {
            for i in 0..n { if !is_sea[i] { rain_to_land[i] *= p_land_scale as f32; p_land_tile[i] *= p_land_scale as f32; for b in 0..BANDS { snow_in[b][i] *= p_land_scale as f32; } } }
        }
        // sea rain, spread by the observed pattern (or by evaporation if there is none)
        {
            let mut wts = vec![0f64; n];
            let mut wsum = 0.0;
            for i in 0..n { if is_sea[i] { wts[i] = if sea_weight_sum > 0.0 { prec[i] as f64 * area[i] as f64 } else { e_sea[i] as f64 * area[i] as f64 }; wsum += wts[i]; } }
            for i in 0..n { if is_sea[i] { p_sea[i] = (p_sea_total * wts[i] / wsum.max(1e-30) / area[i] as f64) as f32; } }
        }
        // The books must close exactly, in the numbers that are actually moved. Total out of the
        // air (evaporation) against total into it (rain), both from the f32 lanes. Give the
        // difference to the wettest sea tile.
        let tile_in = |i: usize, p_sea: &[f32]| -> f64 {
            if is_sea[i] { p_sea[i] as f64 } else { rain_to_land[i] as f64 + (0..BANDS).map(|b| snow_in[b][i] as f64).sum::<f64>() }
        };
        let (mut tot_e, mut tot_p) = (0f64, 0f64);
        for i in 0..n { tot_e += (e_land[i] + e_sea[i]) as f64 * area[i] as f64; tot_p += tile_in(i, &p_sea) * area[i] as f64; }
        if let Some(k) = (0..n).filter(|&i| is_sea[i]).max_by(|&a, &b| p_sea[a].partial_cmp(&p_sea[b]).unwrap()) {
            p_sea[k] = (p_sea[k] as f64 + (tot_e - tot_p) / area[k] as f64).max(0.0) as f32;
        }

        // ---- flows
        let w_air = ids.w_air;
        let air = Endpoint::Stock(w_air);
        if e_land.iter().any(|&x| x > 0.0) { out.dense_local(ids.w_land, w_air, n).copy_from_slice(&e_land); }
        if e_sea.iter().any(|&x| x > 0.0) { out.dense_local(ids.w_sea, w_air, n).copy_from_slice(&e_sea); }
        for i in 0..n {
            let e = (e_land[i] + e_sea[i]) as f64 * area[i] as f64;
            if e > 0.0 { out.push(air, Endpoint::Transit, i, 0, e); }
            let p = tile_in(i, &p_sea) * area[i] as f64;
            if p > 0.0 { out.push(Endpoint::Transit, air, 0, i, p); }
        }
        out.dense_local(w_air, ids.w_sea, n).copy_from_slice(&p_sea);
        out.dense_local(w_air, ids.w_land, n).copy_from_slice(&rain_to_land);
        for b in 0..BANDS {
            out.dense_local(w_air, ids.snow[b], n).copy_from_slice(&snow_in[b]);
            out.dense_local(ids.snow[b], ids.w_land, n).copy_from_slice(&melt_snow[b]);
            out.dense_local(ids.ice[b], ids.w_land, n).copy_from_slice(&melt_ice[b]);
            out.dense_local(ids.snow[b], ids.ice[b], n).copy_from_slice(&firn[b]);
            out.dense_local(ids.snow[b], ids.w_sea, n).copy_from_slice(&sea_snow[b]);
            out.dense_local(ids.ice[b], ids.w_sea, n).copy_from_slice(&sea_ice[b]);
        }
        out.dense_local(ids.w_land, ids.w_ground, n).copy_from_slice(&drain_deep);
        out.dense_local(ids.w_ground, ids.w_land, n).copy_from_slice(&base_flow);
        out.dense_local(ids.w_land, ids.w_sea, n).copy_from_slice(&flood_land);

        // hand the local runoff to the landscape for the year's river erosion
        let river: Vec<f32> = (0..n).map(|i| run[i] + base_flow[i]).collect();
        out.dense(Endpoint::Boundary(env.ph.b_diag), Endpoint::Stock(ids.runoff_y), n).copy_from_slice(&river);

        // rivers and lakes
        let land_stock = |d: usize| if is_sea[d] { ids.w_sea } else { ids.w_land };
        for i in 0..n {
            if is_sea[i] || run[i] <= 0.0 { continue; }
            let d = rt.dest_dry[i] as usize;
            if d == i || d as u32 == NONE { continue; }
            out.push(Endpoint::Stock(ids.w_land), Endpoint::Stock(land_stock(d)), i, d, run[i] as f64 * area[i] as f64);
        }
        for p in 0..n {
            if !rt.is_root(p) || is_sea[p] { continue; }
            let vol = w_land[p] as f64 * area[p] as f64;
            if vol > rt.lake_cap[p] {
                let d = rt.spill_to[p] as usize;
                if d != p { out.push(Endpoint::Stock(ids.w_land), Endpoint::Stock(land_stock(d)), p, d, vol - rt.lake_cap[p]); }
            }
        }
        let _ = (&p_land_tile,);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oudin_is_zero_in_the_cold_and_rises_with_warmth() {
        assert_eq!(pet_oudin(300.0, -10.0), 0.0);
        let warm = pet_oudin(400.0, 25.0);
        assert!(warm > 3.5 && warm < 5.0, "{warm}");
        assert!(pet_oudin(400.0, 25.0) > pet_oudin(400.0, 10.0));
    }
    #[test]
    fn waterfill_levels() {
        let z = [-100.0f32, 0.0, 100.0, 200.0];
        let (l, k) = waterfill(&z, 0.0);
        assert_eq!((l, k), (-100.0, 0));
        // 50 m mean depth over four bands = 200 m total depth: fills the lowest two (100 + 0... ) check volume
        let (l, k) = waterfill(&z, 50.0);
        let vol: f32 = z.iter().map(|&b| (l - b).max(0.0)).sum();
        assert!((vol / 4.0 - 50.0).abs() < 1e-3, "level {l} bands {k} mean depth {}", vol / 4.0);
        assert_eq!(k, z.iter().filter(|&&b| b < l).count());
    }
}
