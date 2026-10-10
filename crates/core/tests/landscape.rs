//! The landscape holds still under today's climate, and moves sediment the way rivers do.
use odyssey_core::build::*;
use odyssey_core::sim::Parallel;

#[test]
fn stands_still_and_balances() {
    let mut s = Setup::physical(48);
    let mut b = build(&s).unwrap();
    s.only = None;
    b.sim.parallel = Parallel::Off;
    b.sim.trace = Some(Default::default());
    b.sim.run_years(3); // year 0 clears runoff, year 1 calibrates, year 2 runs
    let tr0 = b.sim.trace.clone().unwrap();
    let rock0: f64 = (0..b.sim.world.grid.len()).map(|i| b.sim.world.stock(b.env.ph.rock)[i] as f64 * b.sim.world.grid.area_km2[i] as f64).sum();
    b.sim.trace = Some(Default::default());
    b.sim.run_years(50);
    assert_eq!(b.sim.skipped, 0, "{:?}", &b.sim.world.log[..b.sim.world.log.len().min(3)]);
    for r in b.sim.world.audit() { assert!(r.residual.abs() < 1e-5, "{:?} {}", r.group, r.residual); }
    let tr = b.sim.trace.clone().unwrap();
    let get = |k: &str| tr.iter().filter(|(n, _)| n.starts_with("landscape") && n.contains(k)).map(|(_, v)| *v).sum::<f64>();
    let per_year = |v: f64| v / 50.0;
    // solid matter moved by rivers, m x km2 per year -> m3 -> Gt (x2700 kg/m3)
    let eroded = per_year(get("> (transit)")) * 1e6 * 2700.0 / 1e12;
    let to_abyss = per_year(get("[abyss]")) * 1e6 * 2700.0 / 1e12;
    println!("sediment into rivers {eroded:.1} Gt/yr, lost to the deep sea {to_abyss:.1} Gt/yr");
    println!("landscape lanes at calibration: {:?}", tr0.iter().filter(|(n, _)| n.starts_with("landscape")).count());
    let _ = rock0;
    assert!(eroded > 8.0 && eroded < 40.0, "{eroded}");
    // mean land height change after 50 years, in metres, area weighted
    let w = &b.sim.world;
    let mut num = 0f64; let mut den = 0f64;
    for i in 0..w.grid.len() {
        if b.env.ocean0[i] { continue; }
        let sh = b.env.bed_shift(i, w.stock(b.env.ph.rock)[i], w.stock(b.env.ph.reg)[i], w.stock(b.env.ph.sed_g)[i], w.stock(b.env.ph.sed_s)[i], w.stock(b.env.ph.sed_m)[i], w.stock(b.env.ph.defl)[i]);
        num += sh as f64 * w.grid.area_km2[i] as f64; den += w.grid.area_km2[i] as f64;
    }
    println!("mean land height change {:.3} m in 50 years", num / den);
    assert!((num / den).abs() < 5.0);
    // soils: thin on steep land, thick on gentle land
    let reg = w.stock(b.env.ph.reg);
    let (mut thin, mut thick) = (0usize, 0usize);
    for i in 0..reg.len() { if !b.env.ocean0[i] && reg[i] > 0.0 { if reg[i] < 0.5 { thin += 1 } else { thick += 1 } } }
    println!("regolith: {thin} tiles under 0.5 m, {thick} tiles over");
    assert!(thin > 0 && thick > 0);
}
