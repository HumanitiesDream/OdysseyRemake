//! The carbon cycle: steady at rest, and a pulse is taken up by the sea over decades.
use odyssey_core::build::*;
use odyssey_core::carbon::*;
use odyssey_core::climate::PPM_PER_KGC_M2;
use odyssey_core::sim::Parallel;

fn ppm(b: &Built) -> f64 {
    let w = &b.sim.world;
    let c = w.stock(b.env.ph.c_air);
    let a = &w.grid.area_km2;
    c.iter().zip(a.iter()).map(|(&c, &a)| c as f64 * a as f64).sum::<f64>() / w.grid.total_area_km2() * PPM_PER_KGC_M2
}

#[test]
fn co2_holds_still_at_rest() {
    let mut b = build(&Setup::physical(24)).unwrap();
    b.sim.parallel = Parallel::Off;
    let p0 = ppm(&b);
    b.sim.run_years(60);
    assert_eq!(b.sim.skipped, 0, "{:?}", &b.sim.world.log[..b.sim.world.log.len().min(3)]);
    let p1 = ppm(&b);
    println!("CO2 {p0:.1} -> {p1:.1} ppm after 60 years");
    assert!((p1 - p0).abs() < 8.0);
    for r in b.sim.world.audit() { assert!(r.residual.abs() < 1e-5, "{:?} {}", r.group, r.residual); }
}

#[test]
fn a_pulse_of_carbon_is_taken_up() {
    let mut b = build(&Setup::physical(24)).unwrap();
    b.sim.parallel = Parallel::Off;
    b.sim.run_years(5);
    let base = ppm(&b);
    // add 100 GtC to the air: 1e14 kg over 5.1e14 m2
    let add = (1.0e14 / 5.1e14) as f32;
    for v in b.sim.world.stock_mut(b.env.ph.c_air) { *v += add; }
    b.sim.world.reset_ledger();
    let start = ppm(&b);
    assert!((start - base - 47.0).abs() < 3.0);
    b.sim.run_years(100);
    let after = (ppm(&b) - base) / (start - base);
    println!("after 100 years {:.0} percent of the pulse is still in the air", after * 100.0);
    assert!(after > 0.30 && after < 0.75, "airborne fraction {after}");
    let _ = (SURF_DIC, DEEP_DIC);
}
