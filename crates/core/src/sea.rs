//! The sea: ice that follows the heat, and a level that follows the water.
//!
//! `SeaIce` keeps the sea ice water stock equal to what the ocean tile's stored heat says is
//! frozen (latent heat, 0.334 MJ/kg), moving water between the sea and the ice.
//!
//! `SeaLevel` finds the one level that all the ocean water fills, given the live shape of the
//! sea floor and the shore, and moves water between tiles to match. Ice on land takes water
//! out of the sea, so the level falls. No sea level history is ever read. Only seas connected
//! to the open ocean fill: a basin below sea level behind a ridge stays dry unless rivers fill it.

use crate::earth::BANDS;
use crate::env::*;
use crate::flows::{Endpoint, FlowOut};
use crate::grid::Grid;
use crate::hydro::waterfill;
use crate::module::*;
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;

pub struct SeaIce { env: Env, heat: StockId, ice: StockId, sea: StockId }
impl SeaIce {
    pub fn new(env: Env) -> Self { SeaIce { env, heat: StockId(0), ice: StockId(0), sea: StockId(0) } }
}
impl Module for SeaIce {
    fn spec(&self) -> ModuleSpec { ModuleSpec::new("seaice", Phase::Water).reads(&["heat", "w_seaice", "w_sea"]).writes(&["w_seaice", "w_sea"]) }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.heat = g("heat")?; self.ice = g("w_seaice")?; self.sea = g("w_sea")?;
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let n = self.env.n();
        let (heat, ice, sea) = (view.stock(self.heat)?, view.stock(self.ice)?, view.stock(self.sea)?);
        let (mut freeze, mut melt) = (vec![0f32; n], vec![0f32; n]);
        for i in 0..n {
            if !self.env.ocean0[i] { continue; }
            let target = self.env.sea_ice_mass(i, heat[i]);
            let d = target - ice[i];
            // never freeze more water than the tile holds
            if d > 0.0 { freeze[i] = d.min(0.9 * sea[i].max(0.0)).max(0.0); } else { melt[i] = (-d).min(ice[i].max(0.0)); }
        }
        out.dense_local(self.sea, self.ice, n).copy_from_slice(&freeze);
        out.dense_local(self.ice, self.sea, n).copy_from_slice(&melt);
        let _ = ctx;
        Ok(())
    }
}

pub struct SeaLevel {
    env: Env,
    ids: Option<Ids>,
    level: f32,
}
struct Ids { sea: StockId, rock: StockId, reg: StockId, sg: StockId, ss: StockId, sm: StockId, defl: StockId, ice: Vec<StockId> }

impl SeaLevel {
    pub fn new(env: Env) -> Self { SeaLevel { env, ids: None, level: 0.0 } }
}

/// Mean water depth (m) over a tile whose bands sit at `z` for sea level `l`.
pub fn depth_at(z: &[f32], l: f32) -> f32 { z.iter().map(|&b| (l - b).max(0.0)).sum::<f32>() / z.len() as f32 }

impl Module for SeaLevel {
    fn spec(&self) -> ModuleSpec {
        ModuleSpec::new("sealevel", Phase::Water).every(12)
            .reads(&["w_sea", "rock", "reg", "sed_g", "sed_s", "sed_m", "defl"]).writes(&["w_sea"])
            .also_reads(&(0..BANDS).map(|b| format!("w_ice{b}")).collect::<Vec<_>>())
    }
    fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(Ids { sea: g("w_sea")?, rock: g("rock")?, reg: g("reg")?, sg: g("sed_g")?, ss: g("sed_s")?, sm: g("sed_m")?, defl: g("defl")?, ice: (0..BANDS).map(|b| g(&format!("w_ice{b}"))).collect::<Result<_, _>>()? });
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("sealevel not bound".into()))?;
        let ice: Vec<&[f32]> = ids.ice.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let env = &self.env;
        let n = env.n();
        let grid = ctx.grid;
        let area = &grid.area_km2;
        let w_sea = view.stock(ids.sea)?;
        let (rock, regl, sg, ss, sm, defl) = (view.stock(ids.rock)?, view.stock(ids.reg)?, view.stock(ids.sg)?, view.stock(ids.ss)?, view.stock(ids.sm)?, view.stock(ids.defl)?);
        let (z, ice_tile) = geometry(env, rock, regl, sg, ss, sm, defl, &ice);
        let volume: f64 = (0..n).map(|i| w_sea[i] as f64 * area[i] as f64).sum();
        if volume <= 0.0 { return Ok(()); }
        let Some((l, c)) = solve_level(grid, &z, &ice_tile, w_sea, self.level) else { return Ok(()) };
        self.level = l;
        let target: Vec<f64> = (0..n).map(|i| if c[i] { depth_at(&z[i], l) as f64 * 1000.0 } else { 0.0 }).collect();
        let v_t: f64 = (0..n).map(|i| target[i] * area[i] as f64).sum();
        let scale = if v_t > 0.0 { volume / v_t } else { return Ok(()) };
        for i in 0..n {
            let d = (target[i] * scale - w_sea[i] as f64) * area[i] as f64;
            if d > 0.0 { out.push(Endpoint::Transit, Endpoint::Stock(ids.sea), 0, i, d); }
            else if d < 0.0 { out.push(Endpoint::Stock(ids.sea), Endpoint::Transit, i, 0, -d); }
        }
        Ok(())
    }
    fn save_state(&self) -> Option<Vec<u8>> { Some(self.level.to_le_bytes().to_vec()) }
    fn load_state(&mut self, b: &[u8]) -> Result<(), WorldError> {
        if b.len() == 4 { self.level = f32::from_le_bytes(b.try_into().unwrap()); Ok(()) } else { Err(WorldError::Save("sealevel state".into())) }
    }
}

/// Band bed heights and mean ice thickness per tile.
#[allow(clippy::too_many_arguments)]
pub fn geometry(env: &Env, rock: &[f32], regl: &[f32], sg: &[f32], ss: &[f32], sm: &[f32], defl: &[f32], ice: &[&[f32]]) -> (Vec<[f32; BANDS]>, Vec<f32>) {
    let n = env.n();
    let mut z = vec![[0f32; BANDS]; n];
    for i in 0..n {
        let sh = env.bed_shift(i, rock[i], regl[i], sg[i], ss[i], sm[i], defl[i]);
        for b in 0..BANDS { z[i][b] = env.band_bed(i, b, sh); }
    }
    let ice_tile = (0..n).map(|i| (0..BANDS).map(|b| Env::ice_m(ice[b][i])).sum::<f32>() / BANDS as f32).collect();
    (z, ice_tile)
}

/// Find the one level that the ocean water fills. Returns (level, connected ocean tiles).
/// `prev` is the last level, used to guess how far the shore may have moved.
pub fn solve_level(grid: &Grid, z: &[[f32; BANDS]], ice_tile: &[f32], w_sea: &[f32], prev: f32) -> Option<(f32, Vec<bool>)> {
    let n = z.len();
    let area = &grid.area_km2;
    let volume: f64 = (0..n).map(|i| w_sea[i] as f64 * area[i] as f64).sum();
    // grounded ice sits on its bed and keeps the ocean out: thick enough not to float
    let grounded = |i: usize, level: f32| {
        let bed = z[i].iter().sum::<f32>() / BANDS as f32;
        ice_tile[i] > 20.0 && ice_tile[i] > 1.12 * (level - bed).max(0.0)
    };
    let seeds: Vec<usize> = (0..n).filter(|&i| w_sea[i] > 0.0 && waterfill(&z[i], w_sea[i] / 1000.0).1 * 2 >= BANDS && !grounded(i, 0.0)).collect();
    if seeds.is_empty() { return None; }
    let connected = |level: f32| -> Vec<bool> {
        let mut c = vec![false; n];
        let mut stack: Vec<usize> = Vec::new();
        for &s in &seeds { c[s] = true; stack.push(s); }
        while let Some(t) = stack.pop() {
            for j in grid.neighbours(t) { if !c[j] && z[j][0] < level && !grounded(j, level) { c[j] = true; stack.push(j); } }
        }
        c
    };
    let solve = |c: &[bool]| -> f32 {
        let (mut lo, mut hi) = (-1000.0f32, 9000.0f32);
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            let v: f64 = (0..n).filter(|&i| c[i]).map(|i| depth_at(&z[i], mid) as f64 * 1000.0 * area[i] as f64).sum();
            if v < volume { lo = mid } else { hi = mid }
        }
        0.5 * (lo + hi)
    };
    let mut c = connected(prev + 30.0);
    let mut l = solve(&c);
    c = connected(l);
    l = solve(&c);
    Some((l, c))
}

/// Sea level (m) implied by the water now in the sea stock, for tests and readouts.
pub fn sea_level_of(env: &Env, w: &crate::world::World) -> f32 {
    let ph = &env.ph;
    let ice: Vec<&[f32]> = ph.w_ice.iter().map(|&s| w.stock(s)).collect();
    let (z, ice_tile) = geometry(env, w.stock(ph.rock), w.stock(ph.reg), w.stock(ph.sed_g), w.stock(ph.sed_s), w.stock(ph.sed_m), w.stock(ph.defl), &ice);
    solve_level(&w.grid, &z, &ice_tile, w.stock(ph.w_sea), 0.0).map_or(0.0, |(l, _)| l)
}
