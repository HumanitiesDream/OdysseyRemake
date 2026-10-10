//! Whole-world tests on the demo modules: the checks the plan asks of every system.

use odyssey_core::sim::Parallel;
use odyssey_core::{demo, save};

fn worst_residual(d: &demo::Demo) -> f64 { d.sim.world.audit().iter().map(|r| r.residual.abs()).fold(0.0, f64::max) }

#[test]
fn budgets_close_and_nothing_goes_negative() {
    let mut d = demo::build(12, 1, false);
    d.sim.parallel = Parallel::Off;
    d.sim.run_years(200);
    assert_eq!(d.sim.skipped, 0, "{:?}", d.sim.world.log);
    assert_eq!(d.sim.world.violation_count, 0, "{:?}", d.sim.world.violations.first());
    assert!(worst_residual(&d) < 1e-5, "{:?}", d.sim.world.audit());
    let heat = d.sim.world.audit()[1].clone();
    assert!(heat.boundary_net.abs() > 0.0, "sun and space were used");
}

#[test]
fn the_demo_climate_settles_near_earth() {
    let mut d = demo::build(12, 1, false);
    d.sim.parallel = Parallel::Off;
    d.sim.run_years(150);
    let t = demo::mean_temperature(&d);
    // loose: the albedo, clouds and heat transport are placeholders
    assert!(t > 280.0 && t < 298.0, "mean temperature {t}");
}

#[test]
fn same_seed_same_world_threads_or_not() {
    let run = |seed: u64, p: Parallel| {
        let mut d = demo::build(40, seed, false);
        d.sim.parallel = p;
        d.sim.run_years(5);
        d.sim.fingerprint()
    };
    let a = run(7, Parallel::Off);
    assert_eq!(a, run(7, Parallel::Off));
    assert_eq!(a, run(7, Parallel::On), "threads must not change the result");
    assert_ne!(a, run(8, Parallel::Off), "a different seed gives a different world");
}

#[test]
fn save_and_resume_equals_never_stopping() {
    let mut straight = demo::build(14, 3, false);
    straight.sim.parallel = Parallel::Off;
    straight.sim.run_years(40);

    let mut first = demo::build(14, 3, false);
    first.sim.parallel = Parallel::Off;
    first.sim.run_years(17);
    first.sim.run_months(5);
    let bytes = save::write(&first.sim).unwrap();

    let mut resumed = demo::build(14, 3, false);
    resumed.sim.parallel = Parallel::Off;
    let rep = save::read_into(&mut resumed.sim, &bytes).unwrap();
    assert!(rep.defaulted_stocks.is_empty() && rep.unknown_stocks.is_empty());
    assert_eq!(first.sim.fingerprint(), resumed.sim.fingerprint(), "load itself must be exact");
    resumed.sim.run_months(40 * 12 - 17 * 12 - 5);
    assert_eq!(straight.sim.fingerprint(), resumed.sim.fingerprint());
    assert!(worst_residual(&resumed) < 1e-5, "the ledger survives a save");
}

#[test]
fn global_result_does_not_depend_on_tile_size() {
    // A world of sea only, at two resolutions. Rates are per km2, so the large-scale
    // answer must agree. Tolerance is loose because the coarse mixing grid also changes.
    let mean = |n: u32| {
        let mut d = demo::build(n, 1, true);
        d.sim.parallel = Parallel::Off;
        d.sim.run_years(40);
        demo::mean_temperature(&d)
    };
    let (a, b) = (mean(12), mean(24));
    assert!((a - b).abs() < 1.5, "n12 {a:.2} K vs n24 {b:.2} K");
}

#[test]
#[ignore = "10,000 simulated years; run with --ignored --release"]
fn ten_thousand_years_two_seeds() {
    for seed in [1u64, 2] {
        let mut d = demo::build(10, seed, false);
        d.sim.parallel = Parallel::Off;
        d.sim.run_years(10_000);
        let rows = d.sim.world.audit();
        println!("seed {seed}: T {:.2} K, residuals {:?}, violations {}", demo::mean_temperature(&d), rows.iter().map(|r| r.residual).collect::<Vec<_>>(), d.sim.world.violation_count);
        assert_eq!(d.sim.world.violation_count, 0);
        assert!(worst_residual(&d) < 1e-9, "{rows:?}");
    }
}
