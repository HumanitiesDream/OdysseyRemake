//! Weather noise and ENSO.
//!
//! Each month this module does three things. All draws come from the world's seed and the month number, so a run
//! repeats exactly.
//!
//! 1. ENSO is a recharge oscillator (Jin 1997, Burgers et al. 2005 in their linear form): a warm-pool anomaly T
//!    and a heat content h pass energy back and forth with a period near four years, nudged each month by noise.
//!    The noise strength is set so the standard deviation of T is about 0.9 K, the observed size of the
//!    Nino 3.4 index (Trenberth 1997). The state lives in two stocks on tile 0, so it saves and resumes.
//! 2. Weather noise in rain: a persistent random field on a coarse grid (about 700 km across) that scales the
//!    month's rain tile by tile. Droughts and wet spells come from its persistence and from soil memory.
//! 3. Weather noise in heat (Hasselmann 1976): a random heat flux on each coarse cell, which the climate turns
//!    into red-noise temperature swings. ENSO adds a heat pattern: warm east Pacific, cool west Pacific, with the
//!    total at zero.
//!
//! ENSO changes rain through teleconnection boxes. The signs follow the usual maps (Ropelewski and Halpert
//! 1987). Their strengths are assumptions and are flagged as such in docs/VERIFIED.md.
//! Not yet: ENSO reacting to the mean state (warming, ice ages), seasonal phase locking, the Indian Ocean dipole.

use crate::env::*;
use crate::flows::{Endpoint, FlowOut};
use crate::grid::{Coarse, Grid};
use crate::module::*;
use crate::stocks::{Registry, StockId};
use crate::world::WorldError;

/// monthly standard deviation of the log of rain, per coarse cell
/// These stocks cannot be negative, so the signed values are stored with an offset.
pub const WX_OFFSET: f32 = 2.0;
pub const ENSO_OFFSET: f32 = 10.0;
pub const RAIN_SIGMA: f32 = 0.30;
pub const RAIN_RHO: f32 = 0.55;
/// heat noise per coarse cell, W/m2
pub const HEAT_SIGMA: f32 = 4.0;
/// per month
pub const ENSO_DAMP: f32 = 0.02;
pub const ENSO_FREQ: f32 = 0.13;
pub const ENSO_NOISE: f32 = 0.25;
/// W/m2 of heat forcing per K of ENSO state, at pattern value 1
pub const ENSO_HEAT: f32 = 2.2;

/// lat, lon, half-width in lat, half-width in lon, change of the log of rain per K of ENSO state
pub const TELECONNECTIONS: [(f32, f32, f32, f32, f32); 8] = [
    (-5.0, -80.0, 8.0, 10.0, 0.50),   // coastal Peru and Ecuador, wet in El Nino
    (30.0, -110.0, 7.0, 15.0, 0.25),  // south-west United States, wet
    (-28.0, -52.0, 10.0, 10.0, 0.25), // southern Brazil and Uruguay, wet
    (0.0, 38.0, 8.0, 8.0, 0.25),      // East Africa short rains, wet
    (-5.0, 125.0, 10.0, 25.0, -0.30), // Indonesia and northern Australia, dry
    (-18.0, 27.0, 10.0, 10.0, -0.25), // southern Africa, dry
    (-5.0, -50.0, 8.0, 12.0, -0.35),  // northern Brazil, dry
    (18.0, 80.0, 10.0, 10.0, -0.12),  // India, a little drier
];

fn bump(d: f32, w: f32) -> f32 { if d.abs() >= w { 0.0 } else { 0.5 * (1.0 + (std::f32::consts::PI * d / w).cos()) } }
fn wrap(d: f32) -> f32 { let mut x = d; while x > 180.0 { x -= 360.0 } while x < -180.0 { x += 360.0 } x }

/// Rain pattern per K of ENSO state at a place.
pub fn rain_pattern(lat: f32, lon: f32) -> f32 {
    TELECONNECTIONS.iter().map(|&(la, lo, wa, wo, a)| a * bump(lat - la, wa) * bump(wrap(lon - lo), wo)).sum()
}
/// Heat pattern of ENSO before it is made to sum to zero.
fn heat_raw(lat: f32, lon: f32) -> f32 {
    1.0 * bump(lat, 12.0) * bump(wrap(lon + 110.0), 35.0) - 0.8 * bump(lat, 12.0) * bump(wrap(lon - 135.0), 35.0)
}

pub struct Variability { env: Env, coarse: Option<Coarse>, ids: Option<Ids>, rain_pat: Vec<f32>, heat_pat: Vec<f32> }
struct Ids { wx: StockId, t: StockId, h: StockId, heat: StockId }

impl Variability {
    pub fn new(env: Env) -> Variability { Variability { env, coarse: None, ids: None, rain_pat: vec![], heat_pat: vec![] } }
}

impl Module for Variability {
    fn spec(&self) -> ModuleSpec {
        ModuleSpec::new("variability", Phase::Climate).reads(&["wx_p", "enso_t", "enso_h", "heat"]).writes(&["wx_p", "enso_t", "enso_h", "heat"]).boundaries(&["diag", "weather"])
    }
    fn bind(&mut self, reg: &Registry, grid: &Grid) -> Result<(), WorldError> {
        let g = |n: &str| reg.stock(n).ok_or_else(|| WorldError::Unknown(n.into()));
        self.ids = Some(Ids { wx: g("wx_p")?, t: g("enso_t")?, h: g("enso_h")?, heat: g("heat")? });
        let n = grid.len();
        self.coarse = Some(Coarse::new(grid, 20.min(grid.n.saturating_sub(1)).max(1)));
        self.rain_pat = (0..n).map(|i| rain_pattern(grid.lat[i].to_degrees(), grid.lon[i].to_degrees())).collect();
        let raw: Vec<f32> = (0..n).map(|i| if self.env.ocean0[i] { heat_raw(grid.lat[i].to_degrees(), grid.lon[i].to_degrees()) } else { 0.0 }).collect();
        let mean: f64 = raw.iter().zip(grid.area_km2.iter()).map(|(&r, &a)| r as f64 * a as f64).sum::<f64>() / grid.total_area_km2();
        // make the pattern sum to zero over the planet by removing a share proportional to the pattern's own magnitude
        let abs: f64 = raw.iter().zip(grid.area_km2.iter()).map(|(&r, &a)| r.abs() as f64 * a as f64).sum::<f64>() / grid.total_area_km2();
        self.heat_pat = raw.iter().map(|&r| if abs > 0.0 { r - (mean / abs) as f32 * r.abs() } else { r }).collect();
        Ok(())
    }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let ids = self.ids.as_ref().ok_or_else(|| WorldError::Unknown("variability not bound".into()))?;
        let n = self.env.n();
        let co = self.coarse.as_ref().unwrap();
        let mut rng = ctx.rng("variability");
        let months = ctx.dt_months.max(1);
        // ENSO
        let (t0, h0) = (view.stock(ids.t)?[0], view.stock(ids.h)?[0]);
        let (mut t, mut h) = (t0 - ENSO_OFFSET, h0 - ENSO_OFFSET);
        let t_prev = t0 - ENSO_OFFSET;
        for _ in 0..months {
            t += -ENSO_DAMP * t + ENSO_FREQ * h + ENSO_NOISE * rng.normal() as f32;
            h += -ENSO_FREQ * t - ENSO_DAMP * h;
        }
        let (dt_, dh_) = (t + ENSO_OFFSET - t0, h + ENSO_OFFSET - h0);
        let diag = Endpoint::Boundary(self.env.ph.b_diag);
        let a0 = ctx.grid.area_km2[0] as f64;
        let put = |out: &mut FlowOut, st: StockId, d: f32| { if d > 0.0 { out.push(diag, Endpoint::Stock(st), 0, 0, d as f64 * a0) } else if d < 0.0 { out.push(Endpoint::Stock(st), diag, 0, 0, -d as f64 * a0) } };
        put(out, ids.t, dt_); put(out, ids.h, dh_);
        // rain: last month's weather part, then one more step of the persistent field
        let wx = view.stock(ids.wx)?;
        let nc = co.grid.len();
        let mut sum = vec![0f64; nc]; let mut cnt = vec![0f64; nc];
        for i in 0..n { let c = co.parent[i] as usize; sum[c] += (wx[i] - WX_OFFSET - self.rain_pat[i] * t_prev) as f64; cnt[c] += 1.0; }
        let rho = RAIN_RHO.powi(months as i32);
        let innov = RAIN_SIGMA * (1.0 - rho * rho).sqrt();
        let cell: Vec<f32> = (0..nc).map(|c| { let prev = if cnt[c] > 0.0 { (sum[c] / cnt[c]) as f32 } else { 0.0 }; rho * prev + innov * rng.normal() as f32 }).collect();
        // heat noise per coarse cell (ocean and land alike)
        let sigma_f = HEAT_SIGMA;
        let hn: Vec<f32> = (0..nc).map(|_| sigma_f * rng.normal() as f32).collect();
        let (mut up, mut down) = (vec![0f32; n], vec![0f32; n]);
        let mj = (MJ_PER_WM2_MONTH * months as f64) as f32;
        let mut new_wx = vec![0f32; n];
        for i in 0..n {
            let c = co.parent[i] as usize;
            new_wx[i] = (cell[c] + self.rain_pat[i] * t).clamp(-1.5, 1.5) + WX_OFFSET;
            let f = hn[c] + ENSO_HEAT * t * self.heat_pat[i];
            let q = f * mj;
            if q >= 0.0 { up[i] = q } else { down[i] = -q }
        }
        // the stock holds the month's value: book the change
        let (mut a, mut b) = (vec![0f32; n], vec![0f32; n]);
        for i in 0..n { let d = new_wx[i] - wx[i]; if d >= 0.0 { a[i] = d } else { b[i] = -d } }
        out.dense(diag, Endpoint::Stock(ids.wx), n).copy_from_slice(&a);
        out.dense(Endpoint::Stock(ids.wx), diag, n).copy_from_slice(&b);
        out.dense(Endpoint::Boundary(self.env.ph.b_weather), Endpoint::Stock(ids.heat), n).copy_from_slice(&up);
        out.dense(Endpoint::Stock(ids.heat), Endpoint::Boundary(self.env.ph.b_weather), n).copy_from_slice(&down);
        Ok(())
    }
}
