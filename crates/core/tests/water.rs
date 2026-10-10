//! Water cycle tests on the baked Earth.
use odyssey_core::build::*;
use odyssey_core::sea::sea_level_of;
use odyssey_core::sim::Parallel;

fn totals(b: &Built) -> (f64, f64, f64, f64) {
    let w = &b.sim.world; let ph = &b.env.ph;
    let a = &w.grid.area_km2;
    let sum = |s| w.stock(s).iter().zip(a.iter()).map(|(&v, &x)| v as f64 * x as f64).sum::<f64>();
    let ice: f64 = ph.w_ice.iter().map(|&s| sum(s)).sum();
    let snow: f64 = ph.w_snow.iter().map(|&s| sum(s)).sum();
    (sum(ph.w_sea), ice, snow, sum(ph.w_land))
}

#[test]
fn water_closes_and_sea_level_holds() {
    let mut b = build(&Setup::physical(24)).unwrap();
    b.sim.parallel = Parallel::Off;
    let l0 = sea_level_of(&b.env, &b.sim.world);
    let t0 = totals(&b);
    b.sim.run_years(20);
    assert_eq!(b.sim.skipped, 0, "{:?}", &b.sim.world.log[..b.sim.world.log.len().min(3)]);
    let rows = b.sim.world.audit();
    for r in &rows { assert!(r.residual.abs() < 1e-6, "{:?} {:?}", r.group, r.residual); }
    println!("violations {} first {:?}", b.sim.world.violation_count, b.sim.world.violations.first());
    let l1 = sea_level_of(&b.env, &b.sim.world);
    println!("sea level {l0} -> {l1}");
    let (sea, ice, snow, land) = totals(&b);
    println!("ice change {:.3e} (start {:.3e}); sea change {:.3e}; snow start {:.3e}", ice - t0.1, t0.1, sea - t0.0, t0.2);
    println!("sea {sea:.3e} ice {ice:.3e} snow {snow:.3e} land {land:.3e}");
    assert!((l1 - l0).abs() < 10.0, "sea level drifted {l0} -> {l1}");
}

#[test]
#[ignore]
fn debug_ice_loss() {
    let mut b = build(&Setup::physical(24)).unwrap();
    b.sim.parallel = Parallel::Off;
    let ph = b.env.ph.clone(); let env = b.env.clone(); let grid = b.sim.world.grid.clone();
    let region = |i: usize| { let la = env.p.lat_deg[i]; let lo = grid.lon[i].to_degrees(); if la < -60.0 { 0 } else if la > 58.0 && lo > -75.0 && lo < -10.0 { 1 } else { 2 } };
    let sum_by = |b: &Built| { let w = &b.sim.world; let mut r = [0f64; 3]; for i in 0..w.grid.len() { let m: f64 = (0..12).map(|k| w.stock(ph.w_ice[k])[i] as f64).sum(); r[region(i)] += m * w.grid.area_km2[i] as f64; } r };
    let r0 = sum_by(&b);
    for y in 0..8 { b.sim.run_years(5); let r = sum_by(&b); println!("year {y}: antarctica {:.3e} ({:+.1}%) greenland {:.3e} ({:+.1}%) other {:.3e} ({:+.1}%)", r[0], 100.0*(r[0]/r0[0]-1.0), r[1], 100.0*(r[1]/r0[1]-1.0), r[2], 100.0*(r[2]/r0[2]-1.0)); }
}

#[test]
#[ignore]
fn debug_ice_monthly() {
    let mut b = build(&Setup::physical(24)).unwrap();
    b.sim.parallel = Parallel::Off;
    let ph = b.env.ph.clone(); let env = b.env.clone(); let grid = b.sim.world.grid.clone();
    let ant = |b: &Built| { let w = &b.sim.world; let mut ice = 0f64; let mut snow = 0f64; let mut land = 0f64; for i in 0..w.grid.len() { if env.p.lat_deg[i] < -60.0 { let a = grid.area_km2[i] as f64; ice += (0..12).map(|k| w.stock(ph.w_ice[k])[i] as f64).sum::<f64>() * a; snow += (0..12).map(|k| w.stock(ph.w_snow[k])[i] as f64).sum::<f64>() * a; land += w.stock(ph.w_land)[i] as f64 * a; } } (ice, snow, land) };
    let mut prev = ant(&b);
    for m in 0..26 { b.sim.run_months(1); let c = ant(&b); println!("m{m:02}: d_ice {:+.2e} d_snow {:+.2e} d_land {:+.2e}", c.0 - prev.0, c.1 - prev.1, c.2 - prev.2); prev = c; }
}

#[test]
#[ignore]
fn debug_ice_flows() {
    let mut b = build(&Setup::physical(24)).unwrap();
    b.sim.parallel = Parallel::Off;
    b.sim.trace = Some(Default::default());
    b.sim.run_years(5);
    let tr = b.sim.trace.as_ref().unwrap();
    let agg = |pre: &str, post: &str| -> f64 { tr.iter().filter(|(k, _)| k.starts_with(pre) && k.contains(post)).map(|(_, v)| *v).sum() };
    println!("AGG hydro ice>land {:.3e}", agg("hydro: w_ice", "> w_land"));
    println!("AGG hydro ice>sea  {:.3e}", agg("hydro: w_ice", "> w_sea"));
    println!("AGG hydro snow>ice {:.3e}", agg("hydro: w_snow", "> w_ice"));
    println!("AGG hydro snow>land {:.3e}", agg("hydro: w_snow", "> w_land"));
    println!("AGG hydro air>snow {:.3e}", agg("hydro: w_air", "> w_snow"));
    println!("AGG iceflow ice>ice {:.3e}", agg("iceflow: w_ice", "> w_ice"));
    println!("AGG iceflow ice>sea {:.3e}", agg("iceflow: w_ice", "> w_sea"));
}
