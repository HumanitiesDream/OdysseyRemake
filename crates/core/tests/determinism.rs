//! The whole world repeats exactly, with or without threads, and a save resumes to the same state.
use odyssey_core::build::*;
use odyssey_core::save;
use odyssey_core::sim::Parallel;

fn fresh(par: Parallel) -> Built {
    let mut b = build(&Setup::today(24)).unwrap();
    b.sim.parallel = par;
    b
}

#[test]
fn same_seed_same_world_with_and_without_threads() {
    let mut a = fresh(Parallel::Off);
    let mut b = fresh(Parallel::On);
    a.sim.run_years(8);
    b.sim.run_years(8);
    assert_eq!(a.sim.fingerprint(), b.sim.fingerprint());
}

#[test]
fn a_different_seed_gives_a_different_world() {
    let mut a = fresh(Parallel::Off);
    let mut s = Setup::today(24); s.seed = 2;
    let mut b = build(&s).unwrap(); b.sim.parallel = Parallel::Off;
    a.sim.run_years(8);
    b.sim.run_years(8);
    assert_ne!(a.sim.fingerprint(), b.sim.fingerprint());
}

#[test]
fn save_and_resume_matches_running_straight_through() {
    let mut straight = fresh(Parallel::Off);
    straight.sim.run_years(9);
    let mut first = fresh(Parallel::Off);
    first.sim.run_years(5);
    let bytes = save::write(&first.sim).unwrap();
    let mut second = fresh(Parallel::Off);
    save::read_into(&mut second.sim, &bytes).unwrap();
    second.sim.run_years(4);
    assert_eq!(straight.sim.fingerprint(), second.sim.fingerprint(), "resume differs from a straight run");
}

#[test]
#[ignore]
fn debug_which_stocks_differ_after_resume() {
    let mut straight = fresh(Parallel::Off);
    straight.sim.run_years(9);
    let mut first = fresh(Parallel::Off);
    first.sim.run_years(5);
    let bytes = save::write(&first.sim).unwrap();
    let mut second = fresh(Parallel::Off);
    save::read_into(&mut second.sim, &bytes).unwrap();
    second.sim.run_years(4);
    for nm in straight.sim.world.reg.stock_names() {
        let id = straight.sim.world.reg.stock(&nm).unwrap();
        let (a, b) = (straight.sim.world.stock(id), second.sim.world.stock(id));
        let nd = a.iter().zip(b.iter()).filter(|(x, y)| x != y).count();
        if nd > 0 { println!("{nm}: {nd} tiles differ, first {:?}", a.iter().zip(b.iter()).position(|(x, y)| x != y)); }
    }
}

/// Scan a saved state for stocks that hold NaN, infinity or negative values.
#[test]
#[ignore]
fn debug_scan_state() {
    let path = std::env::var("STATE").unwrap();
    let mut b = build(&Setup::today(48)).unwrap();
    b.sim.parallel = Parallel::Off;
    save::read_into(&mut b.sim, &std::fs::read(path).unwrap()).unwrap();
    for nm in b.sim.world.reg.stock_names() {
        let v = b.sim.world.stock(b.sim.world.reg.stock(&nm).unwrap());
        let bad = v.iter().filter(|x| !x.is_finite()).count();
        let neg = v.iter().filter(|x| **x < -1e-9).count();
        let first = v.iter().position(|x| !x.is_finite() || *x < -1e-9);
        if bad > 0 || neg > 0 { println!("{nm}: {bad} non-finite, {neg} negative, first tile {first:?}"); }
    }
}

#[test]
#[ignore]
fn debug_dump_grid_n6() {
    let g = odyssey_core::grid::Grid::new(6);
    let v: Vec<[f32; 3]> = g.pos.clone();
    std::fs::write("/tmp/runs/rust_centers.json", format!("{:?}", v)).unwrap();
}
