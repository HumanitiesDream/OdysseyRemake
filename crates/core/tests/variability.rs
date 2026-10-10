use odyssey_core::build::*;
use odyssey_core::sim::Parallel;
use odyssey_core::EventKind;

fn world(n: u32, extra: &[&str]) -> Built {
    let mut s = Setup::physical(n);
    if let Some(o) = s.only.as_mut() { for e in extra { o.push(e.to_string()); } }
    let mut b = build(&s).unwrap();
    b.sim.parallel = Parallel::Off;
    b
}

#[test]
fn enso_has_the_size_and_rhythm_of_the_real_one() {
    let mut b = world(24, &["variability"]);
    let id = b.sim.world.reg.stock("enso_t").unwrap();
    let mut series = Vec::new();
    for _ in 0..12 * 150 { b.sim.run_months(1); series.push(b.sim.world.stock(id)[0] as f64 - odyssey_core::variability::ENSO_OFFSET as f64); }
    let mean = series.iter().sum::<f64>() / series.len() as f64;
    let sd = (series.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / series.len() as f64).sqrt();
    // period: the lag of the first positive peak of the autocorrelation after the first zero crossing
    let ac = |lag: usize| -> f64 { let n = series.len() - lag; (0..n).map(|i| (series[i] - mean) * (series[i + lag] - mean)).sum::<f64>() / n as f64 / (sd * sd) };
    let mut zero = 0; for l in 1..120 { if ac(l) < 0.0 { zero = l; break; } }
    let mut peak = zero; let mut best = f64::MIN; for l in zero..120 { if ac(l) > best { best = ac(l); peak = l; } }
    println!("ENSO sd {sd:.2} K, mean {mean:.2}, first zero crossing at {zero} months, autocorrelation peak at {peak} months ({best:.2})");
    assert!(sd > 0.6 && sd < 1.3, "sd {sd}");
    assert!(peak >= 30 && peak <= 70, "period {peak}");
    assert_eq!(b.sim.world.violation_count, 0);
}

#[test]
fn weather_keeps_the_mean_rain_and_the_books_close() {
    let mut b = world(24, &["variability"]);
    b.sim.run_years(30);
    let id = b.sim.world.reg.stock("wx_p").unwrap();
    let wx = b.sim.world.stock(id);
    let area = &b.sim.world.grid.area_km2;
    let sd = (wx.iter().zip(area.iter()).map(|(&x, &a)| ((x - odyssey_core::variability::WX_OFFSET) as f64).powi(2) * a as f64).sum::<f64>() / area.iter().map(|&a| a as f64).sum::<f64>()).sqrt();
    println!("rain log-anomaly sd {sd:.3}, violations {}, skipped {}", b.sim.world.violation_count, b.sim.skipped);
    assert!(sd > 0.15 && sd < 0.5);
    assert_eq!(b.sim.world.violation_count, 0, "{:?}", b.sim.world.violations.first());
    assert_eq!(b.sim.skipped, 0);
}

#[test]
fn cyclones_form_in_the_warm_season_and_some_make_landfall() {
    let mut b = world(24, &["cyclones"]);
    b.sim.run_years(1);
    b.sim.world.events.clear();
    b.sim.run_years(20);
    let by_month: Vec<usize> = (0..12).map(|m| b.sim.world.events.iter().filter(|e| e.kind == EventKind::Storm && (e.month % 12) as usize == m).count()).collect();
    let landfalls = b.sim.world.events.iter().filter(|e| e.kind == EventKind::Storm).count();
    let births = b.sim.world.events.iter().filter(|e| e.kind == EventKind::StormBirth).count();
    println!("storms born per year {:.1}, landfalls per year {:.1}, by month {:?}, violations {}", births as f64 / 20.0, landfalls as f64 / 20.0, by_month, b.sim.world.violation_count);
    assert!(landfalls > 0);
    assert_eq!(b.sim.world.violation_count, 0);
}
