//! Climate tests: stable at rest, responds to CO2 and orbit.
use odyssey_core::build::*;
use odyssey_core::grid::Grid;
use odyssey_core::sim::Parallel;

fn mean_t(b: &Built) -> f64 {
    let w = &b.sim.world;
    let h = w.stock(b.env.ph.heat);
    let a = &w.grid.area_km2;
    (0..h.len()).map(|i| b.env.t_from_h(i, h[i]) as f64 * a[i] as f64).sum::<f64>() / w.grid.total_area_km2()
}
fn mean_obs(b: &Built, m: usize) -> f64 {
    let a = &b.sim.world.grid.area_km2;
    (0..a.len()).map(|i| b.env.tobs[m][i] as f64 * a[i] as f64).sum::<f64>() / b.sim.world.grid.total_area_km2()
}

#[test]
fn base_climate_stays_put() {
    let _ = Grid::new(2);
    let mut st = Setup::physical(24);
    st.only = Some(vec!["climate".into()]);
    let mut b = build(&st).unwrap();
    b.sim.parallel = Parallel::Off;
    b.sim.run_years(30);
    assert_eq!(b.sim.skipped, 0, "{:?}", &b.sim.world.log[..b.sim.world.log.len().min(3)]);
    let drift = mean_t(&b) - mean_obs(&b, 0);
    assert!(drift.abs() < 0.1, "drift after 30 years {drift} K");
    let h = b.sim.world.stock(b.env.ph.heat);
    let worst = (0..h.len()).map(|i| (b.env.t_from_h(i, h[i]) - b.env.tobs[0][i]).abs()).fold(0.0f32, f32::max);
    assert!(worst < 1.0, "worst tile departure {worst} K");
    assert!(b.sim.world.audit().iter().all(|r| r.residual.abs() < 1e-6));
}

#[test]
fn doubling_co2_warms_by_a_plausible_amount() {
    let mut s = Setup::physical(24);
    s.only = Some(vec!["climate".into()]);
    let mut b0 = build(&s).unwrap();
    s.co2_ppm = 700.0;
    let mut b1 = build(&s).unwrap();
    for b in [&mut b0, &mut b1] { b.sim.parallel = Parallel::Off; b.sim.run_years(300); }
    let dt = mean_t(&b1) - mean_t(&b0);
    // equilibrium sensitivity of a model with B = 1.5 plus albedo feedback; the deep ocean is still catching up
    println!("warming {dt:.2} K after 300 years");
    assert!(dt > 1.5 && dt < 4.5, "warming {dt} K for 350 -> 700 ppm after 300 years");
}

#[test]
fn baked_earth_has_no_holes() {
    let e = load_earth(24).unwrap();
    for m in 0..12 {
        assert!(e.tair[m].iter().all(|v| v.is_finite()), "tair month {m}");
        assert!(e.prec[m].iter().all(|v| v.is_finite()), "prec month {m}");
    }
    assert!(e.zq.iter().all(|b| b.iter().all(|v| v.is_finite())));
    let first = e.tair[0].iter().position(|v| !v.is_finite());
    assert_eq!(first, None);
}

#[test]
#[ignore]
fn debug_blowup() {
    let mut b = build(&Setup::physical(24)).unwrap();
    b.sim.parallel = Parallel::Off;
    for y in 0..40 {
        b.sim.run_months(12);
        let h = b.sim.world.stock(b.env.ph.heat);
        let (mut worst, mut wi) = (0f32, 0usize);
        for i in 0..h.len() { let d = (b.env.t_from_h(i, h[i]) - b.env.tobs[0][i]).abs(); if d > worst || d.is_nan() { worst = d; wi = i; } }
        println!("year {y}: worst departure {worst} at tile {wi} (lat {}, ocean {}), h {} skipped {}", b.env.p.lat_deg[wi], b.env.ocean0[wi], h[wi], b.sim.skipped);
    }
}

#[test]
#[ignore]
fn debug_tile() {
    let mut b = build(&Setup::physical(24)).unwrap();
    b.sim.parallel = Parallel::Off;
    let h0 = b.sim.world.stock(b.env.ph.heat).to_vec();
    let mut worst = 0usize; 
    for m in 0..4 {
        b.sim.run_months(1);
        let h = b.sim.world.stock(b.env.ph.heat);
        let mut best = (0f32, 0usize);
        for i in 0..h.len() { let d = (h[i] - h0[i]).abs(); if d > best.0 { best = (d, i); } }
        worst = best.1;
        println!("month {m}: worst dh {} at tile {} lat {} ocean {} tobs {} -> {}  h {} t {}", best.0, best.1, b.env.p.lat_deg[best.1], b.env.ocean0[best.1], b.env.tobs[m][best.1], b.env.tobs[m+1][best.1], h[best.1], b.env.t_from_h(best.1, h[best.1]));
    }
    let _ = worst;
}

#[test]
fn whole_model_climate_stays_near_the_base() {
    let mut b = build(&Setup::physical(24)).unwrap();
    b.sim.parallel = Parallel::Off;
    b.sim.run_years(30);
    let h = b.sim.world.stock(b.env.ph.heat);
    let a = &b.sim.world.grid.area_km2;
    let mut mean = 0f64;
    let (mut worst, mut wi) = (0f32, 0usize);
    for i in 0..h.len() {
        let d = b.env.t_from_h(i, h[i]) - b.env.tobs[0][i];
        mean += d as f64 * a[i] as f64;
        if d.abs() > worst { worst = d.abs(); wi = i; }
    }
    mean /= b.sim.world.grid.total_area_km2();
    println!("whole model: mean departure {mean:.2} K, worst {worst:.1} K at tile {wi} lat {:.0} ocean {}", b.env.p.lat_deg[wi], b.env.ocean0[wi]);
    assert!(mean.abs() < 0.5, "global mean departure {mean}");
    assert!(worst < 6.0, "worst tile {worst}");
}

#[test]
#[ignore]
fn debug_antarctic_tile() {
    let mut b = build(&Setup::physical(24)).unwrap();
    b.sim.parallel = Parallel::Off;
    let i = 3801usize;
    println!("tile {i}: lat {} bed0 {} ice0 {} ocean0 {}", b.env.p.lat_deg[i], b.env.p.bed0[i], b.env.p.earth.ice[i], b.env.ocean0[i]);
    for y in 0..32 {
        let w = &b.sim.world;
        let ice: Vec<f32> = (0..12).map(|k| odyssey_core::env::Env::ice_m(w.stock(b.env.ph.w_ice[k])[i])).collect();
        let snow: f32 = (0..12).map(|k| w.stock(b.env.ph.w_snow[k])[i]).sum();
        let t = b.env.t_from_h(i, w.stock(b.env.ph.heat)[i]);
        if y % 4 == 0 || y > 26 { println!("y{y}: T {:.1} (obs {:.1}) ice m {:?} snow {:.0} sea {:.0}", t, b.env.tobs[0][i], ice.iter().map(|x| x.round()).collect::<Vec<_>>(), snow, w.stock(b.env.ph.w_sea)[i]); }
        b.sim.run_years(1);
    }
}
