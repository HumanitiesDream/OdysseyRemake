//! Assemble a world: load the baked Earth, register stocks, set the starting state, add modules.

use crate::climate::{Climate, ClimateParams, PPM_PER_KGC_M2, HD_OFF};
use crate::earth::{Earth, BANDS};
use crate::env::*;
use crate::grid::Grid;
use crate::planet::*;
use crate::sim::Sim;
use crate::stocks::Registry;
use crate::world::World;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Where baked data lives: `$ODYSSEY_DATA/baked`, else the repository's `data/baked`.
pub fn data_dir() -> PathBuf {
    std::env::var("ODYSSEY_DATA").map(PathBuf::from).unwrap_or_else(|_| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data"))
}

#[derive(Clone, Debug)]
pub struct Setup {
    pub n: u32,
    /// Calendar year the clock starts at (negative before common era). Sets the orbit.
    pub start_year: i64,
    pub seed: u64,
    pub co2_ppm: f32,
    pub climate: ClimateParams,
}
impl Setup {
    pub fn today(n: u32) -> Setup { Setup { n, start_year: 1950, seed: 1, co2_ppm: 350.0, climate: ClimateParams::default() } }
}

pub struct Built {
    pub sim: Sim,
    pub env: Env,
}

pub fn load_earth(n: u32) -> Result<Earth, String> {
    Earth::load(&data_dir().join("baked").join(format!("earth-n{n}.bin")))
}

pub fn build(s: &Setup) -> Result<Built, String> {
    let earth = load_earth(s.n)?;
    let grid = Arc::new(Grid::new(s.n));
    let planet = Arc::new(Planet::new(grid.clone(), earth));
    let n = planet.n;
    let mut reg = Registry::new();
    let ph = Phys::register(&mut reg);
    let ocean0: Vec<bool> = (0..n).map(|i| !planet.earth.is_land(i)).collect();
    let rock0: Vec<f32> = (0..n).map(|i| ROCK_DATUM + planet.bed0[i]).collect();
    let tobs = Arc::new(planet.earth.tair.clone());
    let env = Env { p: planet.clone(), ph: ph.clone(), base: Arc::new(rock0.clone()), ocean0: Arc::new(ocean0), tobs };

    let mut world = World::new(grid.clone(), reg, s.seed, s.start_year);
    let area = &grid.area_km2;
    for i in 0..n {
        world.stock_mut(ph.heat)[i] = env.h_from_t(i, env.tobs[0][i]);
        world.stock_mut(ph.heat_deep)[i] = if env.ocean0[i] { HD_OFF } else { 0.0 };
        world.stock_mut(ph.c_air)[i] = (s.co2_ppm as f64 / PPM_PER_KGC_M2) as f32;
        world.stock_mut(ph.rock)[i] = rock0[i];
        world.stock_mut(ph.defl)[i] = DEFL_OFFSET;
    }
    // today's ice sheets and glaciers: the mean thickness of the tile in every band
    for i in 0..n {
        let thick = planet.earth.ice[i];
        if thick > 20.0 { for b in 0..BANDS { world.stock_mut(ph.w_ice[b])[i] = thick * RHO_ICE; } }
    }
    let _ = area;
    world.reset_ledger();
    let mut sim = Sim::new(world);
    sim.add_module(Box::new(Climate::new(env.clone(), s.climate.clone()))).map_err(|e| e.to_string())?;
    Ok(Built { sim, env })
}
