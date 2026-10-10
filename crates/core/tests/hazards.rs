//! Quakes and eruptions match the record in the large, without the record driving them.
use odyssey_core::build::*;
use odyssey_core::sim::Parallel;
use odyssey_core::EventKind;

fn catalog_counts() -> (f64, f64, f64) {
    let text = std::fs::read_to_string(data_dir().join("hazards/quakes_m55.csv")).unwrap();
    let (mut m6, mut m7, mut m8) = (0, 0, 0);
    for l in text.lines().skip(1) {
        let mag: f64 = l.rsplit(',').next().unwrap().parse().unwrap();
        if mag >= 6.0 { m6 += 1 } if mag >= 7.0 { m7 += 1 } if mag >= 8.0 { m8 += 1 }
    }
    (m6 as f64 / 125.0, m7 as f64 / 125.0, m8 as f64 / 125.0)
}

#[test]
fn quake_counts_by_size_sit_near_the_record() {
    let mut s = Setup::today(48);
    s.only = Some(vec!["quakes".into(), "volcanoes".into()]);
    let mut b = build(&s).unwrap();
    b.sim.parallel = Parallel::Off;
    let years = 400;
    b.sim.run_years(years);
    assert_eq!(b.sim.skipped, 0, "{:?}", &b.sim.world.log[..b.sim.world.log.len().min(3)]);
    for r in b.sim.world.audit() { assert!(r.residual.abs() < 1e-6, "{:?} {}", r.group, r.residual); }
    let sizes: Vec<f64> = b.sim.world.events.iter().filter(|e| e.kind == EventKind::Quake).map(|e| e.size as f64).collect();
    let rate = |m: f64| sizes.iter().filter(|&&x| x >= m).count() as f64 / years as f64;
    let (c6, c7, c8) = catalog_counts();
    println!("per year, model vs record: M6+ {:.1} vs {:.1}, M7+ {:.1} vs {:.1}, M8+ {:.2} vs {:.2}", rate(6.0), c6, rate(7.0), c7, rate(8.0), c8);
    assert!(rate(6.0) > c6 / 2.5 && rate(6.0) < c6 * 2.5);
    assert!(rate(7.0) > c7 / 2.5 && rate(7.0) < c7 * 2.5);
    assert!(rate(8.0) > c8 / 3.0 && rate(8.0) < c8 * 3.0);
}

#[test]
fn eruptions_come_in_every_size_and_the_big_ones_are_rare() {
    let mut s = Setup::today(48);
    s.only = Some(vec!["volcanoes".into()]);
    let mut b = build(&s).unwrap();
    b.sim.parallel = Parallel::Off;
    let years = 6000;
    b.sim.run_years(years);
    let v: Vec<f64> = b.sim.world.events.iter().filter(|e| e.kind == EventKind::Eruption).map(|e| e.size as f64).collect();
    let n_of = |lo: f64| v.iter().filter(|&&x| x >= lo).count();
    println!("eruptions in {years} yr: total {}, VEI>=4 {}, VEI>=5 {}, VEI>=6 {}", v.len(), n_of(4.0), n_of(5.0), n_of(6.0));
    assert!(v.len() > 100);
    assert!(n_of(6.0) < n_of(4.0) && n_of(4.0) < v.len());
    for r in b.sim.world.audit() { assert!(r.residual.abs() < 1e-6, "{:?} {}", r.group, r.residual); }
}
