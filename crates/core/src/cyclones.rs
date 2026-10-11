//! Tropical cyclones.
//!
//! A storm forms over warm ocean (28 C or more here, between 5 and 35 degrees of latitude) with a chance that
//! grows with the heat above that threshold. It then walks tile to tile: west and a little poleward in the trade
//! winds, east and poleward past 28 degrees. Over warm water it gains strength, over cold water and land it loses
//! it, and it ends below 18 m/s.
//!
//! The storm lifts water from the sea tiles it crosses and drops it on the land it hits. That water is a real flow
//! from the sea stock to the land stock, and the sea tile gives up the latent heat to space. A strong storm that
//! never reaches land lifts nothing in the books: its rain falls back where it was lifted.
//! The tile of each landfall gets a Storm event, sized by the wind. The people layer will read these.
//!
//! The genesis rate is a placeholder constant set so that the world has about 80 storms a year (Emanuel 2003,
//! from memory, flagged). Not yet: wind damage to plants, storm surge, steering by large-scale flow, eyewall
//! physics. The step of a storm is one tile, so at coarse resolution tracks are coarse (300 km at n24).

use crate::env::*;
use crate::flows::{Endpoint, Event, EventKind, FlowOut};
use crate::grid::Grid;
use crate::module::*;
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;

pub const SST_MIN_C: f32 = 28.0;
/// genesis per km2 per month at 4 K above the threshold
pub const GENESIS: f64 = 4.4e-7;
pub const WIND_MIN: f32 = 18.0;
const LIFT_MM: f64 = 8.0; // mm of water lifted from a sea tile per pass of a strong storm, over its swath
const RAIN_MM: f64 = 25.0;
const LATENT_MJ_PER_MM: f64 = 2.5;
const SWATH_KM: f64 = 400.0;

pub struct Cyclones { env: Env, ids: Option<Ids> }
struct Ids { heat: StockId, w_sea: StockId, w_land: StockId }
impl Cyclones { pub fn new(env: Env) -> Cyclones { Cyclones { env, ids: None } } }

impl Module for Cyclones {
    fn spec(&self) -> ModuleSpec { ModuleSpec::new("cyclones", Phase::Climate).reads(&["heat", "w_sea", "w_land"]).writes(&["heat", "w_sea", "w_land"]).boundaries(&["space"]) }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(Ids { heat: g("heat")?, w_sea: g("w_sea")?, w_land: g("w_land")? });
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("cyclones not bound".into()))?;
        let env = &self.env;
        let n = env.n();
        let grid = ctx.grid;
        let area = &grid.area_km2;
        let heat = view.stock(ids.heat)?; let w_sea = view.stock(ids.w_sea)?; let w_land = view.stock(ids.w_land)?;
        let is_sea = |i: usize| env.ocean0[i] && w_sea[i] > 50.0;
        let sst: Vec<f32> = (0..n).map(|i| if is_sea(i) { env.t_from_h(i, heat[i]) - 273.15 } else { -99.0 }).collect();
        let mut rng = ctx.rng("cyclones");
        let months = ctx.dt_months as f64;
        let (mut lifted_total, mut lost_water) = (vec![0f64; n], vec![0f64; n]); // per sea tile: water lifted (mm x km2)
        let mut rain_land = vec![0f64; n];
        let mut events: Vec<Event> = Vec::new();
        let mut paths: Vec<(Vec<(usize, f64)>, usize, f64)> = Vec::new(); // (sea tiles with their lifted amounts, landfall tile, wanted rain)
        for s in 0..n {
            let lat = grid.lat[s].to_degrees();
            if !is_sea(s) || lat.abs() < 5.0 || lat.abs() > 35.0 || sst[s] < SST_MIN_C { continue; }
            let lam = GENESIS * area[s] as f64 * ((sst[s] - SST_MIN_C + 0.5) / 4.0).min(1.5) as f64 * months;
            let mut k = 0; let mut l = (-lam).exp(); let mut cum = l; let u = rng.f64();
            while u > cum && k < 3 { k += 1; l *= lam / k as f64; cum += l; }
            for _ in 0..k {
                events.push(Event { month: ctx.month_index, kind: EventKind::StormBirth, tile: s as u32, size: sst[s] });
                let mut cur = s; let mut wind = 20.0f32;
                let mut sea_path: Vec<(usize, f64)> = Vec::new();
                let mut land_hit: Option<(usize, f64)> = None;
                for _step in 0..40 {
                    let la = grid.lat[cur].to_degrees();
                    if la.abs() > 48.0 { break; }
                    let (ex, ny) = if la.abs() < 28.0 { (-1.0f32, 0.35 * la.signum()) } else { (0.8, 0.7 * la.signum()) };
                    // pick the neighbour that points most nearly the way the storm is steered
                    let mut best = usize::MAX; let mut bs = f32::MIN;
                    for j in grid.neighbours(cur) {
                        let dlon = { let mut d = grid.lon[j].to_degrees() - grid.lon[cur].to_degrees(); while d > 180.0 { d -= 360.0 } while d < -180.0 { d += 360.0 } d } * la.to_radians().cos();
                        let dlat = grid.lat[j].to_degrees() - grid.lat[cur].to_degrees();
                        let norm = (dlon * dlon + dlat * dlat).sqrt().max(1e-6);
                        let sc = (dlon * ex + dlat * ny) / norm + 0.02 * ((rng.f64() as f32) - 0.5);
                        if sc > bs { bs = sc; best = j; }
                    }
                    if best == usize::MAX { break; }
                    cur = best;
                    let swath = (SWATH_KM / env.p.tile_km[cur] as f64).min(1.0);
                    if is_sea(cur) {
                        wind += 5.0 * (sst[cur] - SST_MIN_C).clamp(-4.0, 3.0) - 1.0;
                        wind = wind.clamp(0.0, 70.0);
                        if wind >= 25.0 { sea_path.push((cur, LIFT_MM * swath * (wind as f64 / 40.0).min(1.5))); }
                        if wind < WIND_MIN { break; }
                    } else {
                        wind *= 0.7;
                        if wind < WIND_MIN { break; }
                        if land_hit.is_none() { land_hit = Some((cur, wind as f64)); }
                        let rain = RAIN_MM * swath * (wind as f64 / 33.0).powf(1.5);
                        paths.push((sea_path.clone(), cur, rain));
                    }
                }
                if let Some((t, w)) = land_hit { events.push(Event { month: ctx.month_index, kind: EventKind::Storm, tile: t as u32, size: w as f32 }); }
            }
        }
        // book the water: each landfall rain draws on the sea tiles the storm crossed, newest first
        for (sea, land, want) in paths {
            let mut need = want * area[land] as f64;
            for &(t, amt) in sea.iter().rev() {
                if need <= 0.0 { break; }
                let avail = (0.9 * w_sea[t] as f64 * area[t] as f64 - lost_water[t]).max(0.0).min(amt * area[t] as f64 - lifted_total[t]);
                let take = avail.min(need);
                if take <= 0.0 { continue; }
                lifted_total[t] += take; lost_water[t] += take; need -= take;
                rain_land[land] += take;
                out.push(Endpoint::Stock(ids.w_sea), Endpoint::Stock(ids.w_land), t, land, take);
                let q = take * LATENT_MJ_PER_MM;
                out.push(Endpoint::Stock(ids.heat), Endpoint::Boundary(env.ph.b_space), t, t, q.min(0.1 * heat[t].max(0.0) as f64 * area[t] as f64));
            }
        }
        let _ = (rain_land, w_land);
        out.events.extend(events);
        Ok(())
    }
}
