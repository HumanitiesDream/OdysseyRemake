//! The planet: fixed facts about the grid and the baked Earth, plus the physical stock catalogue.
//!
//! Nothing here changes during a run. Everything that changes is a stock.

use crate::earth::{Earth, BANDS};
use crate::grid::{Grid, EARTH_RADIUS_KM};
use crate::stocks::{BoundaryId, Group, Registry, Storage, StockId};
use std::sync::Arc;

/// Reference level of the bedrock stock: bedrock "thickness" above a datum 12 km down.
pub const ROCK_DATUM: f32 = 12_000.0;
/// Crust deflection is stored with this offset so the stock stays positive (m).
pub const DEFL_OFFSET: f32 = 4_000.0;
pub const RHO_ICE: f32 = 917.0;
pub const L_FUSION_MJ_PER_KG: f64 = 0.334;
/// Lapse rate, K per m (the textbook 6.5 K/km).
pub const LAPSE: f32 = 0.0065;
pub const T_FREEZE_SEA: f32 = 271.35;
pub const T_FREEZE: f32 = 273.15;

pub struct Planet {
    pub grid: Arc<Grid>,
    pub earth: Earth,
    pub n: usize,
    pub lat_deg: Vec<f32>,
    /// Bed elevation of band b in tile i at the start: index i*BANDS + b, metres.
    pub z0: Vec<f32>,
    pub bed0: Vec<f32>,
    /// Baseline mean surface height of the tile (bed + today's ice), m.
    pub zref: Vec<f32>,
    /// Today's ice per tile, as water equivalent mm, spread over the bands that hold it.
    pub tile_km: Vec<f32>,
    /// Distance to each neighbour, km (same slots as `grid.nbr`).
    pub nbr_km: Vec<[f32; 6]>,
}

impl Planet {
    pub fn new(grid: Arc<Grid>, earth: Earth) -> Planet {
        assert_eq!(grid.len(), earth.len(), "Earth data and grid must match");
        let n = grid.len();
        let mut z0 = vec![0f32; n * BANDS];
        for i in 0..n { for b in 0..BANDS { z0[i * BANDS + b] = earth.zq[b][i]; } }
        let bed0: Vec<f32> = (0..n).map(|i| earth.mean_bed(i)).collect();
        let zref: Vec<f32> = (0..n).map(|i| bed0[i] + earth.ice[i]).collect();
        let tile_km = grid.area_km2.iter().map(|a| a.sqrt()).collect();
        let mut nbr_km = vec![[0f32; 6]; n];
        for i in 0..n {
            for (k, &j) in grid.nbr[i].iter().enumerate() {
                if j < 0 { continue; }
                let (p, q) = (grid.pos[i], grid.pos[j as usize]);
                let d = (p[0] * q[0] + p[1] * q[1] + p[2] * q[2]).clamp(-1.0, 1.0) as f64;
                nbr_km[i][k] = (d.acos() * EARTH_RADIUS_KM) as f32;
            }
        }
        let lat_deg = grid.lat.iter().map(|l| l.to_degrees()).collect();
        Planet { grid, earth, n, lat_deg, z0, bed0, zref, tile_km, nbr_km }
    }
    pub fn z0_band(&self, i: usize, b: usize) -> f32 { self.z0[i * BANDS + b] }
}

/// Ids of the physical stocks and outside accounts.
#[derive(Clone, Debug)]
pub struct Phys {
    // water, mm = kg/m2
    pub w_sea: StockId,
    pub w_seaice: StockId,
    pub w_air: StockId,
    pub w_land: StockId,
    pub w_ground: StockId,
    pub w_snow: [StockId; BANDS],
    pub w_ice: [StockId; BANDS],
    // heat, MJ/m2
    pub heat: StockId,
    pub heat_deep: StockId,
    // carbon, kg C/m2
    pub c_air: StockId,
    pub c_ocean: StockId,
    // solid matter, m of rock equivalent
    pub rock: StockId,
    pub reg: StockId,
    pub sed_g: StockId,
    pub sed_s: StockId,
    pub sed_m: StockId,
    // crust
    pub defl: StockId,
    // outside accounts
    pub b_sun: BoundaryId,
    pub b_space: BoundaryId,
    pub b_season: BoundaryId,
    pub b_mantle: BoundaryId,
    pub b_uplift: BoundaryId,
    pub b_abyss: BoundaryId,
    pub b_greenhouse: BoundaryId,
    pub b_volcano: BoundaryId,
}

impl Phys {
    pub fn register(reg: &mut Registry) -> Phys {
        let w = |reg: &mut Registry, n: &str, s| reg.add_stock(n, "mm", Group::Water, s);
        let w_sea = w(reg, "w_sea", Storage::Exact);
        let w_seaice = w(reg, "w_seaice", Storage::Sparse);
        let w_air = w(reg, "w_air", Storage::Sparse);
        let w_land = w(reg, "w_land", Storage::Exact);
        let w_ground = w(reg, "w_ground", Storage::Exact);
        let mut w_snow = [StockId(0); BANDS];
        let mut w_ice = [StockId(0); BANDS];
        for b in 0..BANDS { w_snow[b] = w(reg, &format!("w_snow{b}"), Storage::Sparse); }
        for b in 0..BANDS { w_ice[b] = w(reg, &format!("w_ice{b}"), Storage::Sparse); }
        let heat = reg.add_stock("heat", "MJ/m2", Group::Heat, Storage::Exact);
        let heat_deep = reg.add_stock("heat_deep", "MJ/m2", Group::Heat, Storage::Exact);
        let c_air = reg.add_stock("c_air", "kgC/m2", Group::Carbon, Storage::Exact);
        let c_ocean = reg.add_stock("c_ocean", "kgC/m2", Group::Carbon, Storage::Exact);
        let m = |reg: &mut Registry, n: &str, s| reg.add_stock(n, "m", Group::Sediment, s);
        let rock = m(reg, "rock", Storage::Exact);
        let regl = m(reg, "reg", Storage::Exact);
        let sed_g = m(reg, "sed_g", Storage::Sparse);
        let sed_s = m(reg, "sed_s", Storage::Sparse);
        let sed_m = m(reg, "sed_m", Storage::Sparse);
        let defl = reg.add_stock("defl", "m", Group::Crust, Storage::Exact);
        Phys {
            w_sea, w_seaice, w_air, w_land, w_ground, w_snow, w_ice, heat, heat_deep, c_air, c_ocean,
            rock, reg: regl, sed_g, sed_s, sed_m, defl,
            b_sun: reg.add_boundary("sun", Group::Heat),
            b_space: reg.add_boundary("space", Group::Heat),
            b_season: reg.add_boundary("season", Group::Heat),
            b_mantle: reg.add_boundary("mantle", Group::Crust),
            b_uplift: reg.add_boundary("uplift", Group::Sediment),
            b_abyss: reg.add_boundary("abyss", Group::Sediment),
            b_greenhouse: reg.add_boundary("greenhouse", Group::Heat),
            b_volcano: reg.add_boundary("volcano", Group::Carbon),
        }
    }

    pub fn snow_ids(&self) -> Vec<StockId> { self.w_snow.to_vec() }
    pub fn ice_ids(&self) -> Vec<StockId> { self.w_ice.to_vec() }
}
