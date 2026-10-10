//! Shared read-only environment for the physical modules: the planet, the stock ids,
//! and small pure functions that turn stocks into temperatures, heights and bands.
//! Pure functions only. Every module recomputes what it needs from the stocks.

use crate::planet::*;
use std::sync::Arc;

pub const SEA_C: f32 = 210.0; // MJ/m2/K, a 50 m mixed layer (4.2 MJ/m3/K)
pub const LAND_C: f32 = 14.0; // MJ/m2/K: the air column (about 10) plus a few metres of soil
pub const ICE_C: f32 = 15.0; // MJ/m2/K for sea ice plus snow, placeholder
/// Stored enthalpy offsets so the stock stays positive.
pub const H_OFF_SEA: f32 = 1500.0;
pub const T_LAND_REF: f32 = 150.0;
pub const SEC_PER_MONTH: f64 = crate::units::SECONDS_PER_MONTH;
/// W/m2 held for one month, in MJ/m2.
pub const MJ_PER_WM2_MONTH: f64 = SEC_PER_MONTH * 1e-6;

#[derive(Clone)]
pub struct Env {
    pub p: Arc<Planet>,
    pub ph: Phys,
    /// rock + regolith at the start, m. The land surface is this plus what has moved since.
    pub base: Arc<Vec<f32>>,
    /// True where the tile was mostly sea at the start (sets heat capacity).
    pub ocean0: Arc<Vec<bool>>,
    /// Monthly air temperature of the baked Earth, K.
    pub tobs: Arc<Vec<Vec<f32>>>,
}

impl Env {
    pub fn n(&self) -> usize { self.p.n }

    pub fn cap(&self, i: usize) -> f32 { if self.ocean0[i] { SEA_C } else { LAND_C } }

    /// Heat capacity right now: an ice-covered ocean tile responds like ice, not like open water.
    pub fn cap_eff(&self, i: usize, h: f32) -> f32 {
        if self.ocean0[i] { if h >= H_OFF_SEA { SEA_C } else { ICE_C } } else { LAND_C }
    }

    /// Stored heat for a surface air or mixed-layer temperature.
    pub fn h_from_t(&self, i: usize, t: f32) -> f32 {
        if self.ocean0[i] {
            let rel = if t >= T_FREEZE_SEA { SEA_C * (t - T_FREEZE_SEA) } else { ICE_C * (t - T_FREEZE_SEA) };
            rel + H_OFF_SEA
        } else {
            LAND_C * (t - T_LAND_REF)
        }
    }

    pub fn t_from_h(&self, i: usize, h: f32) -> f32 {
        if self.ocean0[i] {
            let rel = h - H_OFF_SEA;
            if rel >= 0.0 { T_FREEZE_SEA + rel / SEA_C } else { T_FREEZE_SEA + rel / ICE_C }
        } else {
            T_LAND_REF + h / LAND_C
        }
    }

    /// Ice-covered ocean tile? (enthalpy below the freezing point)
    pub fn sea_ice_mass(&self, i: usize, h: f32) -> f32 {
        // water-equivalent mm of sea ice that holds this much latent heat
        if self.ocean0[i] && h < H_OFF_SEA { ((H_OFF_SEA - h) as f64 / L_FUSION_MJ_PER_KG) as f32 } else { 0.0 }
    }

    /// Shift of the tile's mean bed since the start, m: moved matter minus extra sag.
    pub fn bed_shift(&self, i: usize, rock: f32, reg: f32, sg: f32, ss: f32, sm: f32, defl: f32) -> f32 {
        (rock + reg + sg + ss + sm) - (defl - DEFL_OFFSET) - self.base[i]
    }

    /// Bed height of band b, m, given the tile shift.
    pub fn band_bed(&self, i: usize, b: usize, shift: f32) -> f32 { self.p.z0_band(i, b) + shift }

    /// Ice thickness of a band from its water equivalent, m.
    pub fn ice_m(w: f32) -> f32 { w / RHO_ICE }

    /// Surface air temperature of band b: the tile temperature, moved by the lapse rate to the band's height.
    pub fn band_temp(&self, i: usize, t_tile: f32, z_band_surface: f32) -> f32 {
        t_tile + LAPSE * (self.p.zref[i] - z_band_surface)
    }
}

/// Smooth rain/snow split: all snow below -1.5 C, all rain above +2.5 C.
pub fn snow_fraction(t_k: f32) -> f32 {
    ((275.65 - t_k) / 4.0).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::earth::Earth;
    use crate::grid::Grid;

    fn env() -> Env {
        let g = Arc::new(Grid::new(4));
        let n = g.len();
        let e = Earth { n: 4, zq: (0..BANDS).map(|b| vec![b as f32 * 10.0; n]).collect(), land_frac: (0..n).map(|i| (i % 2) as f32).collect(), wc_elev: vec![0.0; n], ice: vec![0.0; n],
            tair: vec![vec![280.0; n]; 12], sst: vec![vec![290.0; n]; 12], prec: vec![vec![50.0; n]; 12] };
        let p = Arc::new(Planet::new(g, e));
        let mut reg = crate::stocks::Registry::new();
        let ph = Phys::register(&mut reg);
        Env { ocean0: Arc::new((0..n).map(|i| i % 2 == 0).collect()), base: Arc::new(vec![0.0; n]), tobs: Arc::new(vec![vec![280.0; n]; 12]), p, ph }
    }

    #[test]
    fn enthalpy_round_trips_and_stays_positive() {
        let e = env();
        for i in [0usize, 1] {
            for t in [200.0f32, 250.0, 271.35, 273.15, 288.0, 305.0] {
                if i == 1 && t < T_LAND_REF { continue; }
                let h = e.h_from_t(i, t);
                assert!(h > 0.0, "tile {i} T {t} gives {h}");
                assert!((e.t_from_h(i, h) - t).abs() < 1e-3);
            }
        }
    }

    #[test]
    fn sea_ice_mass_matches_latent_heat() {
        let e = env();
        // 0.334 MJ per kg: 100 MJ/m2 below freezing holds about 299 mm w.e. of ice
        let m = e.sea_ice_mass(0, H_OFF_SEA - 100.0);
        assert!((m - 299.4).abs() < 1.0, "{m}");
        assert_eq!(e.sea_ice_mass(0, H_OFF_SEA + 5.0), 0.0);
    }

    #[test]
    fn rain_snow_split() {
        assert_eq!(snow_fraction(260.0), 1.0);
        assert_eq!(snow_fraction(285.0), 0.0);
        assert!((snow_fraction(273.65) - 0.5).abs() < 1e-5);
    }

    #[test]
    fn lapse_cools_high_bands() {
        let e = env();
        assert!((e.band_temp(0, 280.0, 1000.0) - (280.0 + 0.0065 * (55.0 - 1000.0))).abs() < 1e-3);
    }
}
