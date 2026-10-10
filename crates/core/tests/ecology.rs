//! Plants and soil: biomes form on their own and the carbon, nitrogen and phosphorus books close.
use odyssey_core::build::*;
use odyssey_core::sim::Parallel;

fn land_totals(b: &Built, stocks: &[odyssey_core::StockId]) -> f64 {
    let w = &b.sim.world;
    stocks.iter().map(|&s| w.stock(s).iter().zip(w.grid.area_km2.iter()).map(|(&v, &a)| v as f64 * a as f64).sum::<f64>()).sum()
}

#[test]
fn plants_grow_and_books_close() {
    let mut s = Setup::today(24);
    let mut b = build(&s).unwrap();
    s.only = None;
    b.sim.parallel = Parallel::Off;
    let t = std::time::Instant::now();
    b.sim.run_years(60);
    println!("60 years in {:.1}s, skipped {} first log {:?}", t.elapsed().as_secs_f64(), b.sim.skipped, b.sim.world.log.first());
    assert_eq!(b.sim.skipped, 0, "{:?}", &b.sim.world.log[..b.sim.world.log.len().min(3)]);
    for r in b.sim.world.audit() { assert!(r.residual.abs() < 1e-5, "{:?} {}", r.group, r.residual); }
    println!("violations {} {:?}", b.sim.world.violation_count, b.sim.world.violations.first());
}

#[test]
#[ignore]
fn debug_first_bad_month() {
    let mut b = build(&Setup::today(24)).unwrap();
    b.sim.parallel = Parallel::Off;
    for _ in 0..200 {
        b.sim.run_months(1);
        if b.sim.skipped > 0 { break; }
    }
    println!("month {} log {:?}", b.sim.world.clock.month_index, b.sim.world.log.first());
    let w = &b.sim.world;
    let mut shown = 0;
    for (s, def) in w.reg.stocks.iter().enumerate() {
        for i in 0..w.grid.len() {
            let v = w.cur[s][i];
            if !v.is_finite() || v < -1e-3 { if shown < 12 { println!("stock {} tile {} value {} lat {}", def.name, i, v, b.env.p.lat_deg[i]); shown += 1; } }
        }
    }
    println!("violations {} {:?}", w.violation_count, w.violations.first());
}

#[test]
#[ignore]
fn debug_equilibrium_report() {
    let mut b = build(&Setup::today(24)).unwrap();
    b.sim.parallel = Parallel::Off;
    let ph = b.env.ph.clone();
    let area = b.sim.world.grid.area_km2.clone();
    let content = odyssey_core::content::Content::load_dir(&data_dir()).unwrap();
    let set = odyssey_core::eco::PlantSet::from_content(&content).unwrap();
    let n = area.len();
    let reg = b.sim.world.reg.clone();
    let st = |name: &str| reg.stock(name).unwrap();
    for block in 0..6 {
        b.sim.trace = Some(Default::default());
        b.sim.run_years(50);
        let tr = b.sim.trace.take().unwrap();
        let npp: f64 = tr.iter().filter(|(k, _)| k.starts_with("plants: c_air > c_")).map(|(_, v)| *v).sum::<f64>() * 1e-6 / 50.0;
        let w = &b.sim.world;
        let tot = |name: &str| -> f64 { let s = w.stock(st(name)); (0..n).map(|i| s[i] as f64 * area[i] as f64).sum::<f64>() * 1e-6 };
        let mut veg = 0.0; let mut by_type = vec![];
        for t in &set.types { let c = tot(&format!("c_leaf_{}", t.id)) + tot(&format!("c_wood_{}", t.id)) + tot(&format!("c_root_{}", t.id)); veg += c; by_type.push((t.id.clone(), (c * 10.0).round() / 10.0)); }
        let soil = tot("c_litter") + tot("c_sfast") + tot("c_sslow") + tot("c_spass");
        println!("year {}: NPP {:.1} PgC/yr, vegetation {:.0} PgC, soil+litter {:.0} PgC, n_min {:.2e} p_min {:.2e}", (block + 1) * 50, npp, veg, soil, tot("n_min"), tot("p_min"));
        if block == 5 {
            println!("by type: {:?}", by_type);
            // climate classes from today's observed climate
            let names = ["tropical forest", "savanna", "desert", "temperate", "boreal", "tundra", "other"];
            let mut sum = [0f64; 7]; let mut ar = [0f64; 7]; let mut npp_cls = [0f64; 7];
            let _ = &mut npp_cls;
            for i in 0..n {
                if b.env.ocean0[i] || b.env.p.earth.ice[i] > 100.0 { continue; }
                let t: Vec<f32> = (0..12).map(|m| b.env.tobs[m][i] - 273.15).collect();
                let tc = t.iter().cloned().fold(f32::MAX, f32::min); let tw = t.iter().cloned().fold(f32::MIN, f32::max);
                let pr: f32 = (0..12).map(|m| b.env.p.earth.prec[m][i]).sum();
                let cls = if pr < 250.0 { 2 } else if tw < 10.0 { 5 } else if tc > 15.5 { if pr > 1500.0 { 0 } else { 1 } } else if tc < -3.0 { 4 } else if pr > 500.0 { 3 } else { 6 };
                let c: f64 = set.types.iter().map(|t| ["c_leaf_", "c_wood_", "c_root_"].iter().map(|k| w.stock(st(&format!("{k}{}", t.id)))[i] as f64).sum::<f64>()).sum();
                sum[cls] += c * area[i] as f64; ar[cls] += area[i] as f64;
            }
            // does the dominant type fit the climate class?
            let mut agree = 0f64; let mut total = 0f64;
            for i in 0..n {
                if b.env.ocean0[i] || b.env.p.earth.ice[i] > 100.0 { continue; }
                let t: Vec<f32> = (0..12).map(|m| b.env.tobs[m][i] - 273.15).collect();
                let tc = t.iter().cloned().fold(f32::MAX, f32::min); let tw = t.iter().cloned().fold(f32::MIN, f32::max);
                let pr: f32 = (0..12).map(|m| b.env.p.earth.prec[m][i]).sum();
                if pr < 250.0 { continue; }
                let mut best = (0usize, 0f32);
                for (p, tp) in set.types.iter().enumerate() { let c: f32 = ["c_leaf_", "c_wood_", "c_root_"].iter().map(|k| w.stock(st(&format!("{k}{}", tp.id)))[i]).sum(); if c > best.1 { best = (p, c); } }
                let id = set.types[best.0].id.as_str();
                let ok = if tw < 10.0 { matches!(id, "moss_lichen" | "c3_grass" | "shrub" | "boreal_needle" | "boreal_summergreen") }
                    else if tc > 15.5 { matches!(id, "tropical_evergreen" | "tropical_dry" | "c4_grass") }
                    else if tc < -3.0 { matches!(id, "boreal_needle" | "boreal_summergreen" | "c3_grass" | "shrub" | "moss_lichen" | "temperate_broadleaf" | "temperate_needle") && !matches!(id, "temperate_broadleaf") }
                    else { matches!(id, "temperate_broadleaf" | "temperate_needle" | "c3_grass" | "boreal_needle" | "boreal_summergreen" | "shrub") };
                total += area[i] as f64; if ok { agree += area[i] as f64; }
            }
            println!("dominant type fits its climate class on {:.0} percent of the vegetated land", 100.0 * agree / total);
            for k in 0..7 { if ar[k] > 0.0 { println!("{:16} mean vegetation carbon {:5.1} kgC/m2 over {:.1} million km2", names[k], sum[k] / ar[k], ar[k] / 1e6); } }
        }
    }
    let _ = (ph, n);
}

#[test]
#[ignore]
fn debug_tile_growth() {
    let mut b = build(&Setup::today(24)).unwrap();
    b.sim.parallel = Parallel::Off;
    // a temperate tile near 45 N
    let i = (0..b.sim.world.grid.len()).filter(|&i| !b.env.ocean0[i] && (b.env.p.lat_deg[i] - 45.0).abs() < 4.0 && b.sim.world.grid.lon[i].to_degrees().abs() < 100.0).min_by_key(|&i| (b.sim.world.grid.lon[i].to_degrees().abs() * 100.0) as i32).unwrap();
    println!("tile {i} lat {} lon {} mean T {:.1}", b.env.p.lat_deg[i], b.sim.world.grid.lon[i].to_degrees(), (0..12).map(|m| b.env.tobs[m][i]).sum::<f32>() / 12.0 - 273.15);
    std::env::set_var("ODY_TILE", i.to_string());
    b.sim.run_years(80);
}

#[test]
#[ignore]
fn debug_sample_tiles() {
    let mut b = build(&Setup::today(24)).unwrap();
    b.sim.parallel = Parallel::Off;
    b.sim.run_years(200);
    let content = odyssey_core::content::Content::load_dir(&data_dir()).unwrap();
    let set = odyssey_core::eco::PlantSet::from_content(&content).unwrap();
    let reg = b.sim.world.reg.clone();
    let w = &b.sim.world;
    let n = w.grid.len();
    let mut shown = 0;
    for i in (0..n).step_by(37) {
        if b.env.ocean0[i] || b.env.p.earth.ice[i] > 100.0 { continue; }
        let t: Vec<f32> = (0..12).map(|m| b.env.tobs[m][i] - 273.15).collect();
        let tc = t.iter().cloned().fold(f32::MAX, f32::min); let tw = t.iter().cloned().fold(f32::MIN, f32::max);
        let pr: f32 = (0..12).map(|m| b.env.p.earth.prec[m][i]).sum();
        if tc < -30.0 || tc > 12.0 || pr < 400.0 { continue; }
        let comp: Vec<String> = set.types.iter().filter_map(|tp| { let c: f32 = ["c_leaf_", "c_wood_", "c_root_"].iter().map(|k| w.stock(reg.stock(&format!("{k}{}", tp.id)).unwrap())[i]).sum(); if c > 0.05 { Some(format!("{}:{:.1}", tp.id, c)) } else { None } }).collect();
        println!("tile {i} lat {:.0} Tcold {:.0} Twarm {:.0} P {:.0}  w_land {:.0} n_min {:.4}  {:?}", b.env.p.lat_deg[i], tc, tw, pr, w.stock(b.env.ph.w_land)[i], w.stock(reg.stock("n_min").unwrap())[i], comp);
        shown += 1; if shown > 25 { break; }
    }
}

#[test]
#[ignore]
fn debug_tile_by_id() {
    let id: usize = std::env::var("TILE").unwrap().parse().unwrap();
    let mut b = build(&Setup::today(24)).unwrap();
    b.sim.parallel = Parallel::Off;
    println!("tile {id} lat {} T {:?}", b.env.p.lat_deg[id], (0..12).map(|m| (b.env.tobs[m][id] - 273.15).round()).collect::<Vec<_>>());
    std::env::set_var("ODY_TILE", id.to_string());
    b.sim.run_years(120);
}

fn spun(n: u32, years: u64) -> Built {
    let mut s = Setup::today(n);
    s.spin_up_years = years;
    let mut b = build(&s).unwrap();
    b.sim.parallel = Parallel::Off;
    spin_up(&mut b);
    b
}

#[test]
#[ignore]
fn debug_fire_budget() {
    let n: u32 = std::env::var("N").ok().and_then(|v| v.parse().ok()).unwrap_or(24);
    let mut b = spun(n, 150);
    println!("spin-up done, skipped {}, violations {}", b.sim.skipped, b.sim.world.violation_count);
    let t = std::time::Instant::now();
    b.sim.world.events.clear();
    b.sim.run_years(60);
    let sizes: Vec<f64> = b.sim.world.events.iter().filter(|e| e.kind == odyssey_core::EventKind::Fire).map(|e| e.size as f64).collect();
    let total: f64 = sizes.iter().sum();
    let mut sorted = sizes.clone(); sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top1: f64 = sorted.iter().take((sorted.len() / 100).max(1)).sum();
    println!("60 years in {:.1}s: {} fires, burned {:.2} Mha/yr (GFED4: 348), largest {:.0} km2, top 1 percent of fires = {:.0} percent of area", t.elapsed().as_secs_f64(), sizes.len(), total / 60.0 / 1e4, sorted.first().cloned().unwrap_or(0.0), 100.0 * top1 / total.max(1.0));
    println!("skipped {} log {:?} violations {} first {:?}", b.sim.skipped, b.sim.world.log.first(), b.sim.world.violation_count, b.sim.world.violations.first());
}

#[test]
#[ignore]
fn debug_after_spinup() {
    let mut b = spun(24, 150);
    let w = &b.sim.world;
    let c = w.stock(b.env.ph.c_air);
    println!("c_air tile0 {} mean {}", c[0], c.iter().sum::<f32>() / c.len() as f32);
    b.sim.run_months(3);
    println!("skipped {} log {:?}", b.sim.skipped, &b.sim.world.log[..b.sim.world.log.len().min(2)]);
    let w = &b.sim.world;
    for (s, def) in w.reg.stocks.iter().enumerate() {
        let bad = w.cur[s].iter().position(|v| !v.is_finite());
        if let Some(i) = bad { println!("non-finite in {} at {}", def.name, i); }
    }
}

#[test]
#[ignore]
fn debug_n_consistency() {
    let b = spun(24, 150);
    let content = odyssey_core::content::Content::load_dir(&data_dir()).unwrap();
    let set = odyssey_core::eco::PlantSet::from_content(&content).unwrap();
    let w = &b.sim.world; let reg = &w.reg;
    let st = |n: &str| w.stock(reg.stock(n).unwrap());
    let (mut tot_act, mut tot_exp, mut low) = (0f64, 0f64, 0);
    let mut cnt = 0;
    for i in 0..w.grid.len() {
        if b.env.ocean0[i] || b.env.p.earth.ice[i] > 100.0 { continue; }
        let mut exp = st("c_litter")[i] / 45.0 + st("c_sfast")[i] / 10.0 + st("c_sslow")[i] / 12.0 + st("c_spass")[i] / 9.0;
        for t in &set.types { exp += st(&format!("c_leaf_{}", t.id))[i] / 30.0 + st(&format!("c_wood_{}", t.id))[i] / 330.0 + st(&format!("c_root_{}", t.id))[i] / 30.0; }
        let act = st("n_org")[i];
        tot_act += act as f64; tot_exp += exp as f64; cnt += 1;
        if act < 0.7 * exp { low += 1; }
    }
    println!("n_org actual {:.3e} expected from carbon {:.3e} (ratio {:.2}); {} of {} tiles below 70 percent", tot_act, tot_exp, tot_act / tot_exp, low, cnt);
}

/// Prints animal carbon by guild after a spin-up and a run.
#[test]
#[ignore]
fn debug_fauna_report() {
    let n: u32 = std::env::var("N").ok().and_then(|v| v.parse().ok()).unwrap_or(24);
    let yrs: u64 = std::env::var("YRS").ok().and_then(|v| v.parse().ok()).unwrap_or(100);
    let mut b = spun(n, 150);
    println!("spin-up done, skipped {}, violations {}", b.sim.skipped, b.sim.world.violation_count);
    let names: Vec<String> = { let c = odyssey_core::content::Content::load_dir(&data_dir()).unwrap(); let ps = odyssey_core::eco::PlantSet::from_content(&c).unwrap(); odyssey_core::eco::GuildSet::from_content(&c, &ps).unwrap().guilds.iter().map(|g| format!("c_fauna_{}", g.id)).collect() };
    let area = b.sim.world.grid.area_km2.clone();
    for chunk in 0..(yrs / 20).max(1) {
        b.sim.run_years(20);
        let mut line = format!("yr {:>4}:", 150 + 20 * (chunk + 1));
        let mut tot = 0f64;
        for nm in &names {
            let id = b.sim.world.reg.stock(nm).unwrap();
            let s = b.sim.world.stock(id);
            let t: f64 = (0..s.len()).map(|i| s[i] as f64 * area[i] as f64 * 1e6).sum::<f64>() / 1e12; // MtC? kgC -> PgC is 1e12
            tot += t; line += &format!(" {}={:.4}", nm.trim_start_matches("c_fauna_"), t);
        }
        println!("{line} total={tot:.4} PgC viol={} skipped={} first={:?}", b.sim.world.violation_count, b.sim.skipped, b.sim.world.violations.first());
    }
}

/// Prints sea life totals and the ocean carbon change.
#[test]
#[ignore]
fn debug_sea_report() {
    let n: u32 = std::env::var("N").ok().and_then(|v| v.parse().ok()).unwrap_or(24);
    let yrs: u64 = std::env::var("YRS").ok().and_then(|v| v.parse().ok()).unwrap_or(100);
    let mut b = spun(n, 150);
    let area = b.sim.world.grid.area_km2.clone();
    let lat = b.env.p.lat_deg.clone();
    let tot = |b: &Built, nm: &str| -> f64 { let s = b.sim.world.stock(b.sim.world.reg.stock(nm).unwrap()); (0..s.len()).map(|i| s[i] as f64 * area[i] as f64 * 1e6).sum::<f64>() / 1e12 };
    for k in 0..(yrs / 10).max(1) {
        b.sim.run_years(10);
        let id = b.sim.world.reg.stock("c_plankton").unwrap();
        let pl = b.sim.world.stock(id);
        let (mut eq, mut pol, mut ne, mut na) = (0f64, 0f64, 0f64, 0f64);
        for i in 0..pl.len() { if lat[i].abs() < 20.0 { eq += pl[i] as f64; ne += 1.0; } else if lat[i].abs() > 60.0 { pol += pl[i] as f64; na += 1.0; } }
        println!("yr {:>4}: plankton {:.3} fish {:.3} large {:.4} PgC | P surf {:.3e} deep {:.3e} marine {:.3e} Tg | deepC {:.1} PgC airC {:.1} | plankton kgC/m2 tropics {:.4} polar {:.4} | viol {}",
            20 + 10 * (k + 1), tot(&b, "c_plankton"), tot(&b, "c_fauna_fish"), tot(&b, "c_fauna_large_fish"),
            tot(&b, "p_sea") * 1e3, tot(&b, "p_deep_sea") * 1e3, tot(&b, "p_marine") * 1e3, tot(&b, "c_deep"), tot(&b, "c_air"), eq / ne.max(1.0), pol / na.max(1.0), b.sim.world.violation_count);
    }
    println!("first violation {:?} skipped {} log {:?}", b.sim.world.violations.first(), b.sim.skipped, b.sim.world.log.first());
}

/// Prints where the carbon is, every 25 years after the spin-up.
#[test]
#[ignore]
fn debug_carbon_budget() {
    let n: u32 = std::env::var("N").ok().and_then(|v| v.parse().ok()).unwrap_or(24);
    let spin: u64 = std::env::var("SPIN").ok().and_then(|v| v.parse().ok()).unwrap_or(150);
    let yrs: u64 = std::env::var("YRS").ok().and_then(|v| v.parse().ok()).unwrap_or(100);
    let mut b = spun(n, spin);
    let area = b.sim.world.grid.area_km2.clone();
    let tot = |b: &Built, nm: &str| -> f64 { let s = b.sim.world.stock(b.sim.world.reg.stock(nm).unwrap()); (0..s.len()).map(|i| s[i] as f64 * area[i] as f64 * 1e6).sum::<f64>() / 1e12 };
    let names: Vec<String> = b.sim.world.reg.stock_names();
    let veg: Vec<&String> = names.iter().filter(|s| s.starts_with("c_leaf_") || s.starts_with("c_wood_") || s.starts_with("c_root_")).collect();
    let fau: Vec<&String> = names.iter().filter(|s| s.starts_with("c_fauna_")).collect();
    let step: u64 = std::env::var("STEP").ok().and_then(|v| v.parse().ok()).unwrap_or(25);
    for k in 0..=(yrs / step) {
        if k > 0 { b.sim.run_years(step); }
        let v: f64 = veg.iter().map(|s| tot(&b, s)).sum();
        let f: f64 = fau.iter().map(|s| tot(&b, s)).sum::<f64>() + tot(&b, "c_plankton");
        println!("t+{:>5}: air {:.1} veg {:.1} litter {:.1} fast {:.1} slow {:.1} pass {:.1} carcass {:.3} fauna {:.2} surf {:.1} deep {:.1} | Tg: nmin {:.0} norg {:.0} pmin {:.1} porg {:.0} pocc {:.0}", step * k, tot(&b, "c_air"), v, tot(&b, "c_litter"), tot(&b, "c_sfast"), tot(&b, "c_sslow"), tot(&b, "c_spass"), tot(&b, "c_carcass"), f, tot(&b, "c_ocean"), tot(&b, "c_deep"), tot(&b, "n_min") * 1e3, tot(&b, "n_org") * 1e3, tot(&b, "p_min") * 1e3, tot(&b, "p_org") * 1e3, tot(&b, "p_occ") * 1e3);
    }
}

/// How long a simulated year takes at a size, with and without threads.
#[test]
#[ignore]
fn debug_speed() {
    let n: u32 = std::env::var("N").ok().and_then(|v| v.parse().ok()).unwrap_or(48);
    for par in [Parallel::Off, Parallel::Auto(1)] {
        let mut b = build(&Setup::today(n)).unwrap();
        b.sim.parallel = par;
        let t = std::time::Instant::now();
        b.sim.run_years(3);
        println!("n{n} {:?}: {:.2} s per simulated year", par, t.elapsed().as_secs_f64() / 3.0);
    }
    let mut b = build(&Setup::today(n)).unwrap();
    b.sim.parallel = Parallel::Off;
    b.sim.timings = Some(Default::default());
    b.sim.run_years(2);
    let mut v: Vec<_> = b.sim.timings.clone().unwrap().into_iter().collect();
    v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (k, t) in v { println!("  {k:>12}: {:.2} s per simulated year", t / 2.0); }
}

/// Prints the yearly totals of the nitrogen and phosphorus lanes after a spin-up.
#[test]
#[ignore]
fn debug_np_flows() {
    let mut b = spun(24, 150);
    b.sim.trace = Some(Default::default());
    b.sim.run_years(10);
    for (k, v) in b.sim.trace.as_ref().unwrap() {
        if k.contains("nfix") || k.contains("nloss") || k.contains("ploss") || k.contains("rockp") || k.contains("n_min") || k.contains("p_min") || k.contains("n_org") && k.contains("[") { println!("{k}: {:.1} Tg/yr", v / 10.0 * 1e6 / 1e9); }
    }
}

/// Where does the litter pile up? Prints litter and soil water by latitude band after a long run.
#[test]
#[ignore]
fn debug_litter_map() {
    let yrs: u64 = std::env::var("YRS").ok().and_then(|v| v.parse().ok()).unwrap_or(400);
    let mut b = spun(24, 150);
    b.sim.run_years(yrs);
    let w = &b.sim.world;
    let lit = w.stock(w.reg.stock("c_litter").unwrap()); let wl = w.stock(w.reg.stock("w_land").unwrap());
    let ws = w.stock(w.reg.stock("w_sea").unwrap()); let heat = w.stock(b.env.ph.heat);
    let area = &w.grid.area_km2;
    for lo in (-90..90).step_by(15) {
        let (mut c, mut a, mut wsum, mut tsum, mut nl) = (0f64, 0f64, 0f64, 0f64, 0);
        for i in 0..lit.len() {
            let lat = b.env.p.lat_deg[i];
            if lat < lo as f32 || lat >= (lo + 15) as f32 || (ws[i] > 50.0 && b.env.ocean0[i]) { continue; }
            c += lit[i] as f64 * area[i] as f64; a += area[i] as f64; wsum += wl[i] as f64 * area[i] as f64; tsum += (b.env.t_from_h(i, heat[i]) - 273.15) as f64 * area[i] as f64; nl += 1;
        }
        if a > 0.0 { println!("lat {lo:>4}..{:>3}: tiles {nl:>4} litter {:.2} kgC/m2  total {:.1} PgC  soil water {:.0} mm  T {:.1} C", lo + 15, c / a, c * 1e6 / 1e12, wsum / a, tsum / a); }
    }
}

/// Mean land soil water after some years, with modules left out one at a time.
#[test]
#[ignore]
fn debug_leave_one_out() {
    let all: Vec<String> = ["quakes", "volcanoes", "climate", "seaice", "hydro", "plants", "soilbgc", "fire", "fauna", "sealife", "variability", "amoc", "cyclones", "dispersal", "iceflow", "crust", "airmix", "oceancarbon", "landscape", "sealevel"].iter().map(|s| s.to_string()).collect();
    for skip in ["none", "cyclones", "variability", "amoc", "fire"] {
        let mut s = Setup::today(24);
        s.only = Some(all.iter().filter(|m| m.as_str() != skip).cloned().collect());
        let mut b = build(&s).unwrap();
        b.sim.parallel = Parallel::Off;
        b.sim.run_years(60);
        let w = &b.sim.world;
        let wl = w.stock(w.reg.stock("w_land").unwrap()); let ws = w.stock(w.reg.stock("w_sea").unwrap());
        let (mut sum, mut a) = (0f64, 0f64); let mut mx = 0f32;
        for i in 0..wl.len() { if ws[i] > 50.0 && b.env.ocean0[i] { continue; } sum += wl[i] as f64 * w.grid.area_km2[i] as f64; a += w.grid.area_km2[i] as f64; mx = mx.max(wl[i]); }
        println!("without {skip}: mean soil water {:.0} mm, max {:.0} mm, violations {}", sum / a, mx, w.violation_count);
    }
}

/// Land water stores over time.
#[test]
#[ignore]
fn debug_water_stores() {
    let mut b = spun(24, 20);
    for k in 0..8 {
        if k > 0 { b.sim.run_years(50); }
        let w = &b.sim.world;
        let get = |nm: &str| w.stock(w.reg.stock(nm).unwrap()).to_vec();
        let (wl, wg, ws) = (get("w_land"), get("w_ground"), get("w_sea"));
        let area = &w.grid.area_km2;
        let mut line = format!("yr {:>4}:", 20 + 50 * k);
        for (lo, hi) in [(-15.0f32, 15.0f32), (30.0, 60.0)] {
            let (mut sl, mut sg, mut a, mut mx) = (0f64, 0f64, 0f64, 0f32);
            for i in 0..wl.len() { let lat = b.env.p.lat_deg[i]; if lat < lo || lat >= hi || (ws[i] > 50.0 && b.env.ocean0[i]) { continue; } sl += wl[i] as f64 * area[i] as f64; sg += wg[i] as f64 * area[i] as f64; a += area[i] as f64; mx = mx.max(wl[i]); }
            line += &format!(" | lat {lo}..{hi}: land {:.0} ground {:.0} max {:.0}", sl / a, sg / a, mx);
        }
        println!("{line}");
    }
}

/// The tiles that hold the most litter carbon.
#[test]
#[ignore]
fn debug_top_litter() {
    let mut b = spun(24, 150);
    b.sim.run_years(200);
    let w = &b.sim.world;
    let get = |nm: &str| w.stock(w.reg.stock(nm).unwrap()).to_vec();
    let (lit, wl, ws, pass) = (get("c_litter"), get("w_land"), get("w_sea"), get("c_spass"));
    let area = &w.grid.area_km2; let heat = w.stock(b.env.ph.heat);
    let mut idx: Vec<usize> = (0..lit.len()).filter(|&i| !(ws[i] > 50.0 && b.env.ocean0[i])).collect();
    idx.sort_by(|&a, &b| (lit[b] * area[b]).partial_cmp(&(lit[a] * area[a])).unwrap());
    let tot: f64 = idx.iter().map(|&i| lit[i] as f64 * area[i] as f64).sum::<f64>() * 1e6 / 1e12;
    println!("total litter {tot:.0} PgC over {} land tiles", idx.len());
    let mut cum = 0.0;
    for &i in idx.iter().take(25) {
        cum += lit[i] as f64 * area[i] as f64 * 1e6 / 1e12;
        println!("tile {i:>5} lat {:>6.1} lon {:>7.1} litter {:>6.2} kgC/m2 pass {:>6.2} w_land {:>8.0} T {:>5.1} cum {:.0}", b.env.p.lat_deg[i], w.grid.lon[i].to_degrees(), lit[i], pass[i], wl[i], b.env.t_from_h(i, heat[i]) - 273.15, cum);
    }
}

/// Yearly ice and snow flows, Tt/yr w.e. (1 Gt = 1e12 kg = 1 km3 w.e.), after a short run.
#[test]
#[ignore]
fn debug_ice_flows() {
    let mut b = spun(24, 20);
    b.sim.trace = Some(Default::default());
    b.sim.run_years(5);
    let mut rows: Vec<(String, f64)> = b.sim.trace.as_ref().unwrap().iter().filter(|(k, _)| k.contains("w_ice") || k.contains("w_snow") || k.contains("[melt]")).map(|(k, v)| (k.clone(), v / 5.0 / 1e6)).collect();
    let agg = |pre: &str, from: &str, to: &str| -> f64 { rows.iter().filter(|(k, _)| k.starts_with(pre) && { let (a, b) = k.split_once(": ").unwrap().1.split_once(" > ").unwrap(); a.starts_with(from) && b.starts_with(to) }).map(|(_, v)| v).sum() };
    println!("snow in (air>snow): {:.0}", agg("hydro", "w_air", "w_snow"));
    println!("snow to ice (snow>ice): {:.0}", agg("hydro", "w_snow", "w_ice"));
    println!("air to ice directly: {:.0}", agg("hydro", "w_air", "w_ice"));
    println!("snow melt (snow>land): {:.0}", agg("hydro", "w_snow", "w_land"));
    println!("ice melt (ice>land): {:.0}", agg("hydro", "w_ice", "w_land"));
    println!("ice to sea in hydro: {:.0}", agg("hydro", "w_ice", "w_sea"));
    println!("iceflow ice>sea (calving): {:.0}", agg("iceflow", "w_ice", "w_sea"));
    println!("iceflow ice>ice (sliding): {:.0}", agg("iceflow", "w_ice", "w_ice"));
    println!("snow>sea: {:.0}", agg("hydro", "w_snow", "w_sea"));
    println!("all other ice rows:"); for (k, v) in rows.iter().filter(|(k, _)| k.contains("w_ice") && !k.starts_with("hydro: w_ice") && !k.starts_with("iceflow")).take(10) { println!("  {k}: {v:.0}"); }
}
