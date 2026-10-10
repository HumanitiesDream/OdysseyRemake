//! Headless runner for the demo world.
//!
//! world [--n 20] [--years 100] [--seed 1] [--threads] [--save file] [--load file]

use odyssey_core::{demo, save, sim::Parallel};
use std::time::Instant;

fn main() {
    let mut n = 20u32;
    let mut years = 100u64;
    let mut seed = 1u64;
    let mut threads = false;
    let mut save_to: Option<String> = None;
    let mut load_from: Option<String> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut val = |name: &str| args.next().unwrap_or_else(|| { eprintln!("{name} needs a value"); std::process::exit(2) });
        match a.as_str() {
            "--n" => n = val("--n").parse().expect("--n is a number"),
            "--years" => years = val("--years").parse().expect("--years is a number"),
            "--seed" => seed = val("--seed").parse().expect("--seed is a number"),
            "--threads" => threads = true,
            "--save" => save_to = Some(val("--save")),
            "--load" => load_from = Some(val("--load")),
            other => { eprintln!("unknown option {other}"); std::process::exit(2) }
        }
    }
    let mut d = demo::build(n, seed, false);
    d.sim.parallel = if threads { Parallel::On } else { Parallel::Off };
    if let Some(p) = &load_from {
        let bytes = std::fs::read(p).expect("read save");
        let rep = save::read_into(&mut d.sim, &bytes).expect("load save");
        println!("loaded {p}: {rep:?}");
    }
    println!("grid n{} = {} cells, mean T {:.2} K", n, d.sim.world.grid.len(), demo::mean_temperature(&d));
    let t0 = Instant::now();
    d.sim.run_years(years);
    let dt = t0.elapsed().as_secs_f64();
    println!("ran {years} years in {dt:.2} s ({:.1} ms a month)", dt * 1000.0 / (years * 12).max(1) as f64);
    println!("mean T {:.2} K", demo::mean_temperature(&d));
    for row in d.sim.world.audit() {
        if row.initial != 0.0 || row.total != 0.0 {
            println!("{:>8}: total {:.6e}  boundary {:+.3e}  residual {:+.2e}", row.group.name(), row.total, row.boundary_net, row.residual);
        }
    }
    println!("violations {}  skipped {}  fingerprint {:016x}", d.sim.world.violation_count, d.sim.skipped, d.sim.fingerprint());
    for l in d.sim.world.log.iter().take(5) { println!("log: {l}"); }
    if let Some(p) = &save_to {
        std::fs::write(p, save::write(&d.sim).expect("write save")).expect("write file");
        println!("saved {p}");
    }
}
