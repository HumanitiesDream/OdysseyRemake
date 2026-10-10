//! Climate: baked observation plus physics-driven change.
//!
//! The monthly temperature of today's Earth is the base. What the model computes is the
//! departure from it. Each month the surface heat of every tile changes by
//!
//! - the seasonal cycle of the base (booked to the `season` account, nets to zero over a year),
//! - the change in sunlight from the orbit (Berger 1978) against today's,
//! - the change in surface brightness (snow, land ice, sea ice) against today's,
//! - the greenhouse forcing of the CO2 in the air, 5.35 ln(C/C0) W/m2,
//! - a restoring term B x departure, which stands for water vapour, lapse rate and clouds
//!   held fixed (a known risk, see docs/DECISIONS.md),
//! - exchange with the deep ocean (the memory of the climate),
//! - diffusion of the departure on a fixed 5-degree coarse grid, solved implicitly.
//!
//! Sea ice is not a separate state. The ocean tile's heat is an enthalpy: below the freezing
//! point it holds latent heat, and the ice mass follows from it. `seaice` keeps the water
//! stock in step. At the base state the model returns today's temperature exactly.

use crate::env::*;
use crate::flows::{Endpoint, FlowOut};
use crate::grid::{Coarse, Grid, EARTH_RADIUS_KM};
use crate::module::*;
use crate::orbit;
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;

pub const PPM_PER_KGC_M2: f64 = 4.0 * std::f64::consts::PI * (EARTH_RADIUS_KM * 1000.0) * (EARTH_RADIUS_KM * 1000.0) / 2.124e12;
pub const C_DEEP: f32 = 4100.0; // MJ/m2/K, about 1000 m of water
pub const HD_OFF: f32 = C_DEEP * 270.0;

#[derive(Clone, Debug)]
pub struct ClimateParams {
    /// Restoring slope, W/m2/K. With 3.7 W/m2 per CO2 doubling this gives 2.5 K before the ice albedo feedback.
    pub b: f32,
    /// Diffusion of departures, W/m2/K on the unit sphere (North et al. 1981 give 0.44 to 0.65).
    pub d: f32,
    /// Surface to deep ocean exchange, W/m2/K (about the ocean heat uptake efficiency).
    pub gamma: f32,
    /// CO2 the base climate corresponds to, ppm.
    pub co2_ref_ppm: f32,
    /// Share of a surface albedo change that reaches the top of the air.
    pub albedo_reach: f32,
    pub coarse_n: u32,
    /// How fast a tile's departure from its coarse-cell mean dies, W/m2/K. Heat moves fast at
    /// tile scale; without this the snow and ice feedbacks run away on single tiles.
    pub k_local: f32,
}
impl Default for ClimateParams {
    fn default() -> Self { ClimateParams { b: 1.5, d: 0.55, gamma: 0.7, co2_ref_ppm: 350.0, albedo_reach: 0.4, coarse_n: 20, k_local: 25.0 } }
}

/// Sea ice mass that fully covers a tile, mm w.e. Wide on purpose: a tile holds a range of ice.
pub const SEA_ICE_SCALE_MM: f32 = 2000.0;
/// Temperature range over which a tile goes from snow-free to snow-covered, K.
pub const SNOW_WIDTH_K: f32 = 16.0;
pub const ALB_OCEAN: f32 = 0.07;
pub const ALB_SEAICE: f32 = 0.60;
pub const ALB_GROUND: f32 = 0.20;
pub const ALB_SNOW: f32 = 0.70;
pub const ALB_ICE: f32 = 0.60;

/// Snow cover implied by the air temperature (all snow below -9 C, none above +1 C, wide because a tile holds a range of heights).
fn snow_cover(t: f32) -> f32 { ((274.15 - t) / SNOW_WIDTH_K).clamp(0.0, 1.0) }

/// Surface albedo from temperature, land ice share and sea ice (as stored heat).
pub fn surface_albedo(env: &Env, i: usize, t: f32, h: f32, f_landice: f32) -> f32 {
    if env.ocean0[i] {
        let f_si = (env.sea_ice_mass(i, h) / SEA_ICE_SCALE_MM).clamp(0.0, 1.0);
        ALB_OCEAN * (1.0 - f_si) + ALB_SEAICE * f_si
    } else {
        let s = snow_cover(t);
        (1.0 - f_landice) * (ALB_GROUND + (ALB_SNOW - ALB_GROUND) * s) + f_landice * ALB_ICE
    }
}

/// Monthly insolation by latitude (181 steps, -90..90) for one year's orbit.
pub struct InsolTable { pub year: i64, q: Vec<[f32; 12]> }
impl InsolTable {
    pub fn build(year: i64) -> InsolTable {
        let o = orbit::orbit_at(year as f64);
        let q = (0..=180).map(|k| { let lat = k as f64 - 90.0; let mut r = [0f32; 12]; for m in 0..12 { r[m] = orbit::monthly_insolation(lat, m, o) as f32; } r }).collect();
        InsolTable { year, q }
    }
    pub fn at(&self, lat: f32, m: usize) -> f32 {
        let x = (lat + 90.0).clamp(0.0, 180.0);
        let k = (x.floor() as usize).min(179);
        let t = x - k as f32;
        self.q[k][m] * (1.0 - t) + self.q[k + 1][m] * t
    }
}

pub struct Climate {
    env: Env,
    pub prm: ClimateParams,
    ids: Option<Ids>,
    mix: Option<Coarse>,
    now: Option<InsolTable>,
    reference: Option<InsolTable>,
    /// Ice-free ocean tiles never carry land ice, so only land tiles use this.
    f_landice0: Vec<f32>,
}
struct Ids { heat: StockId, deep: StockId, c_air: StockId, w_ice: Vec<StockId> }

impl Climate {
    pub fn new(env: Env, prm: ClimateParams) -> Climate {
        let f_landice0 = env.p.earth.ice.iter().map(|&t| if t > 50.0 { 1.0 } else { 0.0 }).collect();
        Climate { env, prm, ids: None, mix: None, now: None, reference: None, f_landice0 }
    }
    fn f_landice(&self, i: usize, ice: &[&[f32]]) -> f32 {
        // land ice share: bands with more than 10 m of ice count as covered
        let mut c = 0;
        for b in 0..ice.len() { if Env::ice_m(ice[b][i]) > 10.0 { c += 1; } }
        c as f32 / ice.len() as f32
    }
}

impl Module for Climate {
    fn spec(&self) -> ModuleSpec {
        let ph = &self.env.ph;
        let mut reads = vec!["heat", "heat_deep", "c_air"];
        let ice: Vec<String> = (0..ph.w_ice.len()).map(|b| format!("w_ice{b}")).collect();
        let mut s = ModuleSpec::new("climate", Phase::Climate);
        s.reads = reads.drain(..).map(String::from).chain(ice).collect();
        s.writes = vec!["heat".into(), "heat_deep".into()];
        s.boundaries = vec!["sun".into(), "space".into(), "season".into(), "greenhouse".into()];
        s
    }
    fn bind(&mut self, reg: &Registry, grid: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(Ids { heat: g("heat")?, deep: g("heat_deep")?, c_air: g("c_air")?, w_ice: (0..crate::earth::BANDS).map(|b| g(&format!("w_ice{b}"))).collect::<Result<_, _>>()? });
        let cn = if grid.n > self.prm.coarse_n * 2 { self.prm.coarse_n } else { (grid.n / 2).max(2) };
        self.mix = Some(Coarse::new(grid, cn));
        self.reference = Some(InsolTable::build(1950));
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("climate not bound".into()))?;
        let ph = &self.env.ph;
        let n = self.env.n();
        let m = ctx.month_of_year as usize;
        let m2 = (m + 1) % 12;
        if self.now.as_ref().map_or(true, |t| (t.year - ctx.year).abs() >= 25) { self.now = Some(InsolTable::build(ctx.year)); }
        let (now, refr) = (self.now.as_ref().unwrap(), self.reference.as_ref().unwrap());
        let heat = view.stock(ids.heat)?;
        let deep = view.stock(ids.deep)?;
        let c_air = view.stock(ids.c_air)?;
        let ice: Vec<&[f32]> = ids.w_ice.iter().map(|&s| view.stock(s)).collect::<Result<_, _>>()?;
        let grid = ctx.grid;
        let area = &grid.area_km2;

        // CO2 in ppm from the air carbon (area-weighted mean)
        let c_mean: f64 = c_air.iter().zip(area.iter()).map(|(&c, &a)| c as f64 * a as f64).sum::<f64>() / grid.total_area_km2();
        let ppm = (c_mean * PPM_PER_KGC_M2) as f32;
        let f_co2 = 5.35 * (ppm / self.prm.co2_ref_ppm).max(1e-3).ln();

        let tobs = &self.env.tobs;
        let secs = (MJ_PER_WM2_MONTH * ctx.dt_months as f64) as f32;
        // one signed lane per account: positive goes in, negative goes out
        let mut sun = Signed::new(n); let mut spc = Signed::new(n); let mut sea = Signed::new(n);
        let mut ghg = Signed::new(n); let mut dpl = Signed::new(n);
        let mut theta_pred = vec![0f64; n];
        let mut cap = vec![0f64; n];
        let mut h_new_v = vec![0f64; n];
        for i in 0..n {
            let h = heat[i];
            let t = self.env.t_from_h(i, h);
            let t0 = tobs[m][i];
            let theta = t - t0;
            let lat = self.env.p.lat_deg[i];
            let (qn, qr) = (now.at(lat, m), refr.at(lat, m));
            let f_li = if self.env.ocean0[i] { 0.0 } else { self.f_landice(i, &ice) };
            let a_now = surface_albedo(&self.env, i, t, h, f_li);
            let a_ref = surface_albedo(&self.env, i, t0, self.env.h_from_t(i, t0), self.f_landice0[i]);
            let f_sun = (1.0 - 0.30) * (qn - qr) - qn * self.prm.albedo_reach * (a_now - a_ref);
            let f_rest = -self.prm.b * theta;
            let f_deep = if self.env.ocean0[i] { -self.prm.gamma * (theta - (deep[i] - HD_OFF) / C_DEEP) } else { 0.0 };
            // Semi-implicit step. The albedo feedback can be steeper than the restoring term, which
            // makes a plain monthly step blow up. Slope of the net flux against temperature, W/m2/K:
            let a_up = surface_albedo(&self.env, i, t + 0.5, self.env.h_from_t(i, t + 0.5), f_li);
            let d_alb = qn * self.prm.albedo_reach * (a_now - a_up) / 0.5; // positive: warming adds heat
            let gslope = (self.prm.b + if self.env.ocean0[i] { self.prm.gamma } else { 0.0 } - d_alb).max(0.5);
            let damp = 1.0 / (1.0 + secs * gslope / self.env.cap_eff(i, h));
            let (f_sun, f_co2_i, f_rest, f_deep) = (f_sun * damp, f_co2 * damp, f_rest * damp, f_deep * damp);
            sun.add(i, f_sun * secs); ghg.add(i, f_co2_i * secs); spc.add(i, f_rest * secs); dpl.add(i, f_deep * secs);
            let ds = self.env.h_from_t(i, tobs[m2][i]) - self.env.h_from_t(i, t0);
            sea.add(i, ds);
            let h_new = h as f64 + ds as f64 + ((f_sun + f_co2_i + f_rest + f_deep) * secs) as f64;
            h_new_v[i] = h_new;
            cap[i] = self.env.cap_eff(i, h_new as f32) as f64;
            theta_pred[i] = self.env.t_from_h(i, h_new as f32) as f64 - tobs[m2][i] as f64;
        }
        let hs = ph.heat; let hd = ph.heat_deep;
        sun.emit(out, Endpoint::Boundary(ph.b_sun), Endpoint::Stock(hs));
        spc.emit(out, Endpoint::Boundary(ph.b_space), Endpoint::Stock(hs));
        sea.emit(out, Endpoint::Boundary(ph.b_season), Endpoint::Stock(hs));
        ghg.emit(out, Endpoint::Boundary(ph.b_greenhouse), Endpoint::Stock(hs));
        dpl.emit(out, Endpoint::Stock(hd), Endpoint::Stock(hs));

        // diffusion of the departure on the coarse grid, implicit
        let co = self.mix.as_ref().unwrap();
        let ca_fine: Vec<f64> = (0..n).map(|i| cap[i] * area[i] as f64).collect();
        let ca = co.sum_up(&ca_fine).map_err(|e| WorldError::Unknown(e.to_string()))?;
        let th_fine: Vec<f64> = (0..n).map(|i| theta_pred[i] * ca_fine[i]).collect();
        let th_sum = co.sum_up(&th_fine).map_err(|e| WorldError::Unknown(e.to_string()))?;
        let t0c: Vec<f64> = (0..co.len()).map(|c| th_sum[c] / ca[c]).collect();
        let r_m = EARTH_RADIUS_KM * 1000.0;
        let kappa = self.prm.d as f64 * r_m * r_m / 3f64.sqrt();
        let k = kappa * SEC_PER_MONTH * ctx.dt_months as f64 / 1e12;
        let mut t = t0c.clone();
        for _ in 0..40 {
            for c in 0..co.len() {
                let (mut sum, mut deg) = (0.0, 0.0);
                for j in co.grid.neighbours(c) { sum += t[j]; deg += 1.0; }
                t[c] = (ca[c] * t0c[c] + k * sum) / (ca[c] + k * deg);
            }
        }
        // Tile departures from their coarse-cell mean relax quickly (same factor for every member of
        // a cell, so the cell total is untouched). Then remove the solver residual from the world sum.
        let mut amount = vec![0f64; n];
        for c in 0..co.len() {
            let area_c: f64 = co.members_of(c).iter().map(|&i| area[i as usize] as f64).sum();
            let r = 1.0 / (1.0 + (self.prm.k_local as f64) * SEC_PER_MONTH * ctx.dt_months as f64 * 1e-6 / (ca[c] / area_c));
            for &i in co.members_of(c) {
                let i = i as usize;
                let target = t[c] + r * (theta_pred[i] - t0c[c]);
                // through the enthalpy curve, so a tile that crosses the freezing point is handled exactly
                let h_target = self.env.h_from_t(i, tobs[m2][i] + target as f32) as f64;
                amount[i] = (h_target - h_new_v[i]) * area[i] as f64;
            }
        }
        let mean = amount.iter().sum::<f64>() / ca_fine.iter().sum::<f64>();
        for i in 0..n { amount[i] -= mean * ca_fine[i]; }
        for (i, &x) in amount.iter().enumerate() {
            if x > 0.0 { out.push(Endpoint::Transit, Endpoint::Stock(hs), 0, i, x); }
            else if x < 0.0 { out.push(Endpoint::Stock(hs), Endpoint::Transit, i, 0, -x); }
        }
        Ok(())
    }
}

/// A per-tile signed flux. Positive values flow `from -> to`, negative values flow back.
pub struct Signed { pos: Vec<f32>, neg: Vec<f32>, used_pos: bool, used_neg: bool }
impl Signed {
    pub fn new(n: usize) -> Self { Signed { pos: vec![0.0; n], neg: vec![0.0; n], used_pos: false, used_neg: false } }
    pub fn add(&mut self, i: usize, v: f32) {
        if v > 0.0 { self.pos[i] = v; self.used_pos = true; } else if v < 0.0 { self.neg[i] = -v; self.used_neg = true; }
    }
    /// Positive values go `from -> to`; negative values go `to -> from`.
    pub fn emit(self, out: &mut FlowOut, from: Endpoint, to: Endpoint) {
        let n = self.pos.len();
        if self.used_pos { out.dense(from, to, n).copy_from_slice(&self.pos); }
        if self.used_neg { out.dense(to, from, n).copy_from_slice(&self.neg); }
    }
}
