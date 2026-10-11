//! Long run. Builds the full world, spins it up, then runs it for many years and writes one report line per interval.
//!
//! proof --n 48 --seed 1 --spin 3000 --years 10000 --every 100 [--load state] [--save state] [--out file]

use odyssey_core::build::*;
use odyssey_core::sim::Parallel;
use odyssey_core::{save, EventKind};
use std::io::Write;

fn main() {
    let (mut n, mut seed, mut spin, mut years, mut every) = (48u32, 1u64, 3000u64, 10000u64, 100u64);
    let (mut load, mut save_to, mut out_path): (Option<String>, Option<String>, Option<String>) = (None, None, None);
    let mut start_year: i64 = 1950;
    let mut reseed: Option<u64> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut v = |name: &str| args.next().unwrap_or_else(|| { eprintln!("{name} needs a value"); std::process::exit(2) });
        match a.as_str() {
            "--n" => n = v("--n").parse().unwrap(),
            "--seed" => seed = v("--seed").parse().unwrap(),
            "--spin" => spin = v("--spin").parse().unwrap(),
            "--years" => years = v("--years").parse().unwrap(),
            "--every" => every = v("--every").parse().unwrap(),
            "--start-year" => start_year = v("--start-year").parse().unwrap(),
            "--load" => load = Some(v("--load")),
            "--reseed" => reseed = Some(v("--reseed").parse().unwrap()),
            "--save" => save_to = Some(v("--save")),
            "--out" => out_path = Some(v("--out")),
            o => { eprintln!("unknown option {o}"); std::process::exit(2) }
        }
    }
    let mut s = Setup::today(n);
    s.seed = seed; s.start_year = start_year;
    s.spin_up_years = if load.is_some() { 0 } else { spin };
    let mut b = build(&s).expect("build");
    b.sim.parallel = Parallel::Auto(1);
    let mut out: Box<dyn Write> = match &out_path { Some(p) => Box::new(std::fs::File::create(p).expect("create out")), None => Box::new(std::io::stdout()) };
    if let Some(p) = &load {
        let rep = save::read_into(&mut b.sim, &std::fs::read(p).expect("read state")).expect("load");
        writeln!(out, "# loaded {p}: {rep:?}").unwrap();
        if let Some(r) = reseed { b.sim.world.seed = r; writeln!(out, "# reseeded to {r}").unwrap(); }
    } else {
        let t = std::time::Instant::now();
        spin_up(&mut b);
        writeln!(out, "# spin-up of {spin} years took {:.0} s", t.elapsed().as_secs_f64()).unwrap();
        if let Some(p) = &save_to { std::fs::write(format!("{p}.spun"), save::write(&b.sim).expect("save")).expect("write"); }
    }
    let area = b.sim.world.grid.area_km2.clone();
    let reg_names = b.sim.world.reg.stock_names();
    let sum = |b: &Built, nm: &str| -> f64 { let s = b.sim.world.stock(b.sim.world.reg.stock(nm).unwrap()); (0..s.len()).map(|i| s[i] as f64 * area[i] as f64 * 1e6).sum::<f64>() / 1e12 };
    let group_sum = |b: &Built, pre: &[&str]| -> f64 { reg_names.iter().filter(|nm| pre.iter().any(|p| nm.starts_with(p))).map(|nm| sum(b, nm)).sum() };
    let total_area: f64 = area.iter().map(|&a| a as f64).sum();
    let ocean_area: f64 = (0..area.len()).filter(|&i| b.env.ocean0[i]).map(|i| area[i] as f64).sum();
    let v_sea = |b: &Built| -> f64 { let w = b.sim.world.stock(b.env.ph.w_sea); (0..w.len()).map(|i| w[i] as f64 * area[i] as f64).sum::<f64>() / 1e6 }; // km3
    let v_ice = |b: &Built| -> f64 { (0..12).map(|k| { let w = b.sim.world.stock(b.env.ph.w_ice[k]); (0..w.len()).map(|i| w[i] as f64 * area[i] as f64).sum::<f64>() }).sum::<f64>() / 1e6 };
    let sea0 = v_sea(&b); let ice0 = v_ice(&b);
    writeln!(out, "year,T_mean_K,CO2_ppm,sea_level_m,ice_m_sle,veg_PgC,soil_PgC,ocean_surf_PgC,ocean_deep_PgC,fauna_PgC,plankton_PgC,amoc_s,enso_sd,quakes,eruptions,fires,storms,floods,violations,skipped").unwrap();
    let t_run = std::time::Instant::now();
    let steps = years / every;
    let mut enso_acc: Vec<f64> = Vec::new();
    for k in 0..=steps {
        if k > 0 {
            // run the interval month by month so the ENSO sample is real
            for _ in 0..every * 12 { b.sim.run_months(1); if b.sim.world.clock.month_of_year() == 0 { enso_acc.push(b.sim.world.stock(b.env.ph.enso_t)[0] as f64 - odyssey_core::variability::ENSO_OFFSET as f64); } }
        }
        let t_mean: f64 = { let h = b.sim.world.stock(b.env.ph.heat); (0..h.len()).map(|i| b.env.t_from_h(i, h[i]) as f64 * area[i] as f64).sum::<f64>() / total_area };
        let co2 = sum(&b, "c_air") * 1e12 / total_area / 1e6 * odyssey_core::climate::PPM_PER_KGC_M2 / 1e6 * 1e6 / 1e0;
        let co2_ppm = b.sim.world.stock(b.env.ph.c_air).iter().zip(area.iter()).map(|(&c, &a)| c as f64 * a as f64).sum::<f64>() / total_area * odyssey_core::climate::PPM_PER_KGC_M2;
        let _ = co2;
        let sl = -((v_sea(&b) - sea0) * 1e9) / (ocean_area * 1e6); // loss of ocean water raises nothing: sea level is water gained
        let sl = -sl;
        let ice = (v_ice(&b) - ice0) * 1e9 / (ocean_area * 1e6);
        let veg = group_sum(&b, &["c_leaf_", "c_wood_", "c_root_"]);
        let soil = group_sum(&b, &["c_litter", "c_sfast", "c_sslow", "c_spass", "c_carcass"]);
        let fauna = group_sum(&b, &["c_fauna_"]);
        let (mut q, mut e, mut f, mut st, mut fl) = (0, 0, 0, 0, 0);
        for ev in b.sim.world.events.iter() { match ev.kind { EventKind::Quake => q += 1, EventKind::Eruption => e += 1, EventKind::Fire => f += 1, EventKind::Storm => st += 1, EventKind::Flood => fl += 1, _ => {} } }
        b.sim.world.events.clear();
        let sd = if enso_acc.len() > 5 { let m = enso_acc.iter().sum::<f64>() / enso_acc.len() as f64; (enso_acc.iter().map(|x| (x - m).powi(2)).sum::<f64>() / enso_acc.len() as f64).sqrt() } else { 0.0 };
        enso_acc.clear();
        writeln!(out, "{},{:.3},{:.2},{:.2},{:.2},{:.1},{:.1},{:.1},{:.1},{:.3},{:.3},{:.3},{:.2},{},{},{},{},{},{},{}",
            b.sim.world.clock.year(), t_mean, co2_ppm, sl, ice, veg, soil, sum(&b, "c_ocean"), sum(&b, "c_deep"), fauna, sum(&b, "c_plankton"), b.sim.world.stock(b.env.ph.amoc_s)[0], sd, q, e, f, st, fl, b.sim.world.violation_count, b.sim.skipped).unwrap();
        out.flush().unwrap();
        let _ = sl;
    }
    writeln!(out, "# ran {years} years in {:.0} s", t_run.elapsed().as_secs_f64()).unwrap();
    for row in b.sim.world.audit() {
        if row.initial != 0.0 || row.total != 0.0 { writeln!(out, "# {:>10}: total {:.6e}  boundary {:+.3e}  residual {:+.2e}", row.group.name(), row.total, row.boundary_net, row.residual).unwrap(); }
    }
    writeln!(out, "# violations {}  skipped {}  fingerprint {:016x}", b.sim.world.violation_count, b.sim.skipped, b.sim.fingerprint()).unwrap();
    { let mut by: std::collections::BTreeMap<String, usize> = Default::default(); for v in b.sim.world.violations.iter() { *by.entry(v.stock.clone()).or_default() += 1; } writeln!(out, "# recorded violations by stock: {by:?}").unwrap(); }
    for v in b.sim.world.violations.iter().take(5) { writeln!(out, "# violation: {v:?}").unwrap(); }
    for l in b.sim.world.log.iter().take(5) { writeln!(out, "# log: {l}").unwrap(); }
    if let Some(p) = &save_to { std::fs::write(p, save::write(&b.sim).expect("save")).expect("write"); writeln!(out, "# saved {p}").unwrap(); }
}
