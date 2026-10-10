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
