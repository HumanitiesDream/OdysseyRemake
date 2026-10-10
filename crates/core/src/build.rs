//! Assemble a world: load the baked Earth, register stocks, set the starting state, add modules.

use crate::carbon::{AirMix, OceanCarbon, DEEP_DIC, SURF_DIC};
use crate::climate::{Climate, ClimateParams, PPM_PER_KGC_M2, HD_OFF};
use crate::hazards::{HazardMap, Quakes, Volcanoes};
use crate::hydro::Hydro;
use crate::content::Content;
use crate::crust::Crust;
use crate::dispersal::Dispersal;
use crate::fire::Fire;
use crate::eco::{Eco, PlantSet, NPT, CN, CP};
use crate::plants::Plants;
use crate::soilbgc::{initial_pools, SoilBgc};
use crate::ice::IceFlow;
use crate::landscape::{initial_regolith, Landscape};
use crate::sea::{SeaIce, SeaLevel};
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
    /// If set, only these modules are added (by name). For module tests.
    pub only: Option<Vec<String>>,
    /// Years of spin-up with CO2 held at `co2_ppm`. 0: none. Run it with `spin_up`.
    pub spin_up_years: u64,
}

/// The modules of the physical world, without plants, soil carbon and animals.
pub const PHYSICAL: &[&str] = &["quakes", "volcanoes", "climate", "seaice", "hydro", "iceflow", "crust", "airmix", "oceancarbon", "landscape", "sealevel"];

impl Setup {
    /// Today's Earth with the physical modules only.
    pub fn physical(n: u32) -> Setup { let mut s = Setup::today(n); s.only = Some(PHYSICAL.iter().map(|x| x.to_string()).collect()); s }
}
impl Setup {
    pub fn today(n: u32) -> Setup { Setup { n, start_year: 1950, seed: 1, co2_ppm: 350.0, climate: ClimateParams::default(), only: None, spin_up_years: 0 } }
}

pub struct Built {
    pub sim: Sim,
    pub env: Env,
    pub spin_up_months: u64,
}

/// Run the spin-up and close the books: the carbon the nudge supplied is part of the starting state.
pub fn spin_up(b: &mut Built) {
    b.sim.run_months(b.spin_up_months);
    b.sim.world.reset_ledger();
    b.sim.skipped = 0;
    b.sim.world.log.clear();
    b.sim.world.violation_count = 0;
    b.sim.world.violations.clear();
}

/// Tile that starts as open sea (ocean connected or not: inland seas are handled by `SeaLevel`).
fn ocean_tile(env: &Env, i: usize) -> bool { env.ocean0[i] }

pub fn load_earth(n: u32) -> Result<Earth, String> {
    Earth::load(&data_dir().join("baked").join(format!("earth-n{n}.bin")))
}

pub fn build(s: &Setup) -> Result<Built, String> {
    let mut s = s.clone();
    if s.spin_up_years > 0 && s.climate.veg_baseline_month == u64::MAX { s.climate.veg_baseline_month = s.spin_up_years * 12; }
    let s = &s;
    let earth = load_earth(s.n)?;
    let grid = Arc::new(Grid::new(s.n));
    let planet = Arc::new(Planet::new(grid.clone(), earth));
    let n = planet.n;
    let mut reg = Registry::new();
    let ph = Phys::register(&mut reg);
    let content = Content::load_dir(&data_dir())?;
    let set = PlantSet::from_content(&content)?;
    let guilds = crate::eco::GuildSet::from_content(&content, &set)?;
    let eco = Eco::register(&mut reg, &set, &guilds);
    // open sea at the start: not land, and not a marine basin filled by grounded ice
    // (ice that is too thin to hold its bed down floats: it belongs to the sea, not to the ice sheet)
    let grounded: Vec<bool> = (0..n).map(|i| planet.earth.ice[i] > 20.0 && planet.earth.ice[i] > 1.12 * (-planet.bed0[i]).max(0.0)).collect();
    let ocean0: Vec<bool> = (0..n).map(|i| !planet.earth.is_land(i) && !grounded[i]).collect();
    let tobs = Arc::new(planet.earth.tair.clone());
    let mut env = Env { p: planet.clone(), ph: ph.clone(), base: Arc::new(vec![0.0; n]), ocean0: Arc::new(ocean0), tobs };
    // today's bed height already holds today's soil: rock + regolith = datum + bed
    let reg0 = initial_regolith(&env);
    let rock0: Vec<f32> = (0..n).map(|i| ROCK_DATUM + planet.bed0[i] - reg0[i]).collect();
    env.base = Arc::new((0..n).map(|i| rock0[i] + reg0[i]).collect());

    let mut world = World::new(grid.clone(), reg, s.seed, s.start_year);
    let area = &grid.area_km2;
    for i in 0..n {
        world.stock_mut(ph.heat)[i] = env.h_from_t(i, env.tobs[0][i]);
        world.stock_mut(ph.heat_deep)[i] = if env.ocean0[i] { HD_OFF } else { 0.0 };
        if env.ocean0[i] {
            world.stock_mut(ph.c_ocean)[i] = SURF_DIC as f32; world.stock_mut(ph.c_deep)[i] = DEEP_DIC as f32;
            // sea life: a seed of plankton and fish, and the phosphorus the sea starts with (about 0.5 and 2.5 umol/L)
            let (cp, cs, cl) = (2.0e-3f32, 5.0e-4f32, 1.0e-4f32);
            world.stock_mut(eco.plankton)[i] = cp; world.stock_mut(eco.sea_fauna[0])[i] = cs; world.stock_mut(eco.sea_fauna[1])[i] = cl;
            world.stock_mut(eco.p_marine)[i] = (cp + cs + cl) / crate::sealife::CP_SEA as f32;
            world.stock_mut(eco.p_sea)[i] = crate::sealife::P_SURF0; world.stock_mut(eco.p_deep_sea)[i] = crate::sealife::P_DEEP0;
        }
        world.stock_mut(ph.c_air)[i] = (s.co2_ppm as f64 / PPM_PER_KGC_M2) as f32;
        world.stock_mut(ph.rock)[i] = rock0[i];
        world.stock_mut(ph.reg)[i] = reg0[i];
        world.stock_mut(ph.defl)[i] = DEFL_OFFSET;
    }
    // today's ice sheets and glaciers: the mean thickness of the tile in every band
    for i in 0..n {
        let thick = planet.earth.ice[i];
        if grounded[i] { for b in 0..BANDS { world.stock_mut(ph.w_ice[b])[i] = thick * RHO_ICE / BANDS as f32; } }
    }
    // sea: fill the open ocean to sea level 0 (m); soil water and groundwater start moist
    for i in 0..n {
        let zb: Vec<f32> = (0..BANDS).map(|b| planet.z0_band(i, b)).collect();
        // sea water in every low band that lies under sea level 0, except where grounded ice sits
        let d = crate::sea::depth_at(&zb, 0.0);
        if !grounded[i] { world.stock_mut(ph.w_sea)[i] = d * 1000.0; }
        if !ocean_tile(&env, i) { world.stock_mut(ph.w_land)[i] = 150.0; world.stock_mut(ph.w_ground)[i] = 75.0; }
    }
    let _ = area;
    // the living land: soil pools by climate, and a seed of every type that the climate can carry
    for i in 0..n {
        if env.ocean0[i] || grounded[i] { continue; }
        let t_ann: f64 = (0..12).map(|m| env.tobs[m][i] as f64).sum::<f64>() / 12.0 - 273.15;
        let prec: f64 = (0..12).map(|m| planet.earth.prec[m][i] as f64).sum();
        let pools = initial_pools(t_ann, prec);
        let ids = [eco.litter, eco.sfast, eco.sslow, eco.spass];
        let (mut n_org, mut p_org) = (0f32, 0f32);
        for k in 0..4 { world.stock_mut(ids[k])[i] = pools[k]; n_org += pools[k] / CN[k]; p_org += pools[k] / CP[k]; }
        let tcold = (0..12).map(|m| env.tobs[m][i]).fold(f32::MAX, f32::min) - 273.15;
        let tmax = (0..12).map(|m| env.tobs[m][i]).fold(f32::MIN, f32::max) - 273.15;
        let mut seeded = 0f32;
        for (p, t) in set.types.iter().enumerate() {
            let fits = tcold >= t.cold_min && tcold <= t.cold_max && tmax > t.t_min + 5.0 && prec > t.aprec_min_mm() as f64;
            if fits { world.stock_mut(eco.leaf[p])[i] = 0.02; world.stock_mut(eco.root[p])[i] = 0.02; seeded += 0.04 / crate::eco::CN_LEAF; }
        }
        let mut fauna_c = 0f32;
        if seeded > 0.0 { for g in 0..guilds.guilds.len() { world.stock_mut(eco.fauna[g])[i] = 2.0e-5; fauna_c += 2.0e-5; } }
        world.stock_mut(eco.n_org)[i] = n_org + seeded + fauna_c / crate::fauna::CN_FAUNA as f32;
        world.stock_mut(eco.p_org)[i] = p_org + seeded * crate::eco::CN_LEAF / crate::eco::CP_LEAF + fauna_c / crate::fauna::CP_FAUNA as f32;
        world.stock_mut(eco.n_min)[i] = 0.003;
        world.stock_mut(eco.p_min)[i] = 4.0e-5;
        world.stock_mut(eco.p_occ)[i] = 4.0e-4;
    }
    let _ = NPT;
    world.reset_ledger();
    let mut sim = Sim::new(world);
    let keep = |name: &str| s.only.as_ref().map_or(true, |v| v.iter().any(|x| x == name));
    let add = |sim: &mut Sim, name: &str, m: Box<dyn crate::module::Module>| if keep(name) { sim.add_module(m).map_err(|e| e.to_string()) } else { Ok(()) };
    // what the crust carried at the start, kg/m2
    let base_load: Vec<f32> = (0..n).map(|i| {
        let ice: Vec<f32> = (0..BANDS).map(|b| sim.world.stock(ph.w_ice[b])[i]).collect();
        Crust::load_of(sim.world.stock(ph.w_sea)[i], &ice, 0.0)
    }).collect();
    let hz = Arc::new(HazardMap::load(&grid, &data_dir().join("hazards"))?);
    sim.world.stock_mut(ph.strain).copy_from_slice(&hz.initial_strain(&grid, s.seed));
    sim.world.stock_mut(ph.magma).copy_from_slice(&hz.initial_magma(s.seed));
    sim.world.stock_mut(ph.amoc_s)[0] = crate::amoc::S0;
    sim.world.stock_mut(ph.enso_t)[0] = crate::variability::ENSO_OFFSET;
    sim.world.stock_mut(ph.enso_h)[0] = crate::variability::ENSO_OFFSET;
    for v in sim.world.stock_mut(ph.wx_p).iter_mut() { *v = crate::variability::WX_OFFSET + 0.5 * crate::variability::RAIN_SIGMA * crate::variability::RAIN_SIGMA; }
    sim.world.reset_ledger();
    add(&mut sim, "quakes", Box::new(Quakes::new(env.clone(), hz.clone())))?;
    add(&mut sim, "volcanoes", Box::new(Volcanoes::new(env.clone(), hz.clone())))?;
    add(&mut sim, "climate", Box::new(Climate::new(env.clone(), s.climate.clone(), set.types.iter().map(|t| t.sla).collect(), set.types.iter().map(|t| format!("c_leaf_{}", t.id)).collect())))?;
    add(&mut sim, "seaice", Box::new(SeaIce::new(env.clone())))?;
    add(&mut sim, "hydro", Box::new(Hydro::new(env.clone(), set.clone(), eco.clone())))?;
    add(&mut sim, "plants", Box::new(Plants::new(env.clone(), set.clone(), eco.clone())))?;
    add(&mut sim, "soilbgc", Box::new(SoilBgc::new(env.clone(), eco.clone())))?;
    add(&mut sim, "fire", Box::new(Fire::new(env.clone(), set.clone(), eco.clone())))?;
    add(&mut sim, "fauna", Box::new(crate::fauna::Fauna::new(env.clone(), set.clone(), guilds.clone(), eco.clone())))?;
    add(&mut sim, "sealife", Box::new(crate::sealife::SeaLife::new(env.clone(), guilds.clone(), eco.clone())))?;
    add(&mut sim, "variability", Box::new(crate::variability::Variability::new(env.clone())))?;
    add(&mut sim, "amoc", Box::new(crate::amoc::Amoc::new(env.clone())))?;
    add(&mut sim, "cyclones", Box::new(crate::cyclones::Cyclones::new(env.clone())))?;
    add(&mut sim, "dispersal", Box::new(Dispersal::new(env.clone(), set.clone(), eco.clone())))?;
    add(&mut sim, "iceflow", Box::new(IceFlow::new(env.clone())))?;
    add(&mut sim, "crust", Box::new(Crust::new(env.clone(), base_load)))?;
    let share: Vec<f32> = { let w: Vec<f64> = (0..n).map(|i| hz.magma_m[i] * grid.area_km2[i] as f64).collect(); let sum: f64 = w.iter().sum::<f64>().max(1e-30); w.iter().map(|&x| (x / sum) as f32).collect() };
    if s.spin_up_years > 0 { add(&mut sim, "nudge", Box::new(crate::carbon::Nudge::new(env.clone(), s.co2_ppm as f64, s.spin_up_years * 12)))?; }
    add(&mut sim, "airmix", Box::new(AirMix::new(env.clone())))?;
    let with_sea = s.only.as_ref().map_or(true, |o| o.iter().any(|m| m == "sealife"));
    add(&mut sim, "oceancarbon", Box::new(if with_sea { OceanCarbon::with_sea_life(env.clone()) } else { OceanCarbon::new(env.clone()) }))?;
    add(&mut sim, "landscape", Box::new(Landscape::new(env.clone(), share)))?;
    add(&mut sim, "sealevel", Box::new(SeaLevel::new(env.clone())))?;
    Ok(Built { sim, env, spin_up_months: s.spin_up_years * 12 })
}
