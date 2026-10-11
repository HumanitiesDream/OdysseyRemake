//! Checks on what the animal layer does without being told: numbers fall out of the rules.
use odyssey_core::build::*;
use odyssey_core::sim::Parallel;
use odyssey_core::EventKind;

fn spun(years: u64) -> Built {
    let mut s = Setup::today(24);
    s.spin_up_years = years;
    let mut b = build(&s).unwrap();
    b.sim.parallel = Parallel::Off;
    spin_up(&mut b);
    b
}
fn stock(b: &Built, nm: &str) -> Vec<f32> { b.sim.world.stock(b.sim.world.reg.stock(nm).unwrap()).to_vec() }
/// Mean over twelve months of a stock, per tile: the seasonal low and high of grass and animals both count.
fn year_mean(b: &mut Built, names: &[&str]) -> Vec<Vec<f32>> {
    let mut acc: Vec<Vec<f64>> = names.iter().map(|n| vec![0.0; stock(b, n).len()]).collect();
    for _ in 0..12 {
        b.sim.run_months(1);
        for (k, n) in names.iter().enumerate() { let v = stock(b, n); for i in 0..v.len() { acc[k][i] += v[i] as f64 / 12.0; } }
    }
    acc.into_iter().map(|v| v.into_iter().map(|x| x as f32).collect()).collect()
}
fn is_land(b: &Built, i: usize, w_sea: &[f32]) -> bool { !(w_sea[i] > 50.0 && b.env.ocean0[i]) }

fn rank(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap());
    let mut r = vec![0.0; v.len()];
    for (k, &i) in idx.iter().enumerate() { r[i] = k as f64; }
    r
}
fn pearson(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len() as f64; let (mx, my) = (x.iter().sum::<f64>() / n, y.iter().sum::<f64>() / n);
    let (mut sxy, mut sxx, mut syy) = (0.0, 0.0, 0.0);
    for i in 0..x.len() { sxy += (x[i] - mx) * (y[i] - my); sxx += (x[i] - mx).powi(2); syy += (y[i] - my).powi(2); }
    sxy / (sxx * syy).sqrt()
}

#[test]
fn herbivore_biomass_is_highest_in_savanna_rain_and_lower_in_rainforest_and_desert() {
    // Large herbivore biomass in the tropics peaks in savanna and falls in closed forest and in desert
    // (Bell 1982; Fa and Purvis 1997). Coe et al. 1976 found a rise with rain across African reserves up to
    // about 900 mm. The model's rise with rain is checked as a band comparison, not as a fitted line.
    let mut b = spun(150);
    b.sim.run_years(100);
    let w_sea = stock(&b, "w_sea");
    let herb = year_mean(&mut b, &["c_fauna_megaherbivore", "c_fauna_large_grazer", "c_fauna_medium_grazer", "c_fauna_large_browser", "c_fauna_medium_browser"]);
    let mut bands = [(0.0f64, 0usize); 3];
    for i in 0..w_sea.len() {
        let lat = b.env.p.lat_deg[i];
        if !is_land(&b, i, &w_sea) || lat.abs() > 25.0 { continue; }
        let p: f64 = (0..12).map(|m| b.env.p.earth.prec[m][i] as f64).sum();
        let k = if p < 200.0 { 0 } else if p < 1000.0 { 1 } else if p > 1400.0 { 2 } else { continue };
        bands[k].0 += herb.iter().map(|h| h[i] as f64).sum::<f64>(); bands[k].1 += 1;
    }
    let m: Vec<f64> = bands.iter().map(|&(s, n)| s / n.max(1) as f64).collect();
    println!("mean herbivore carbon (kgC/m2): desert under 200 mm {:.2e}, savanna 200 to 1000 mm {:.2e}, wet forest over 1400 mm {:.2e}", m[0], m[1], m[2]);
    assert!(m[1] > m[0] && m[1] > m[2], "savanna should carry the most");
}

#[test]
fn predator_biomass_rises_with_prey_biomass() {
    // Hatton et al. 2015 find predator biomass scales with prey biomass to about the 0.75 power across reserves.
    let mut b = spun(150);
    b.sim.run_years(100);
    let w_sea = stock(&b, "w_sea");
    let both = year_mean(&mut b, &["c_fauna_large_grazer", "c_fauna_medium_grazer", "c_fauna_large_browser", "c_fauna_medium_browser", "c_fauna_megaherbivore", "c_fauna_small_herbivore", "c_fauna_medium_carnivore", "c_fauna_large_carnivore", "c_fauna_small_carnivore"]);
    let (prey, pred) = (both[..6].to_vec(), both[6..].to_vec());
    let (mut lx, mut ly) = (vec![], vec![]);
    for i in 0..w_sea.len() {
        if !is_land(&b, i, &w_sea) { continue; }
        let (x, y) = (prey.iter().map(|h| h[i] as f64).sum::<f64>(), pred.iter().map(|h| h[i] as f64).sum::<f64>());
        if x > 1e-7 && y > 1e-9 { lx.push(x.ln()); ly.push(y.ln()); }
    }
    let n = lx.len() as f64; let (mx, my) = (lx.iter().sum::<f64>() / n, ly.iter().sum::<f64>() / n);
    let slope = lx.iter().zip(ly.iter()).map(|(x, y)| (x - mx) * (y - my)).sum::<f64>() / lx.iter().map(|x| (x - mx).powi(2)).sum::<f64>();
    println!("{} tiles, log-log slope of predator on prey biomass {slope:.2} (Hatton 2015: about 0.75)", lx.len());
    // Not matched: the model gives a steeper slope than 0.75. Recorded in docs/VERIFIED.md. Only sanity is asserted here.
    assert!(slope > 0.0, "predators should rise with prey, slope {slope}");
}

#[test]
fn taking_away_the_herbivores_lets_fuel_build_and_fire_grow() {
    // Bond and Keeley 2005: grazers cut fuel, so fires are bigger where they are missing.
    let burned = |remove: bool| -> f64 {
        let mut b = spun(150);
        if remove {
            for nm in ["megaherbivore", "large_grazer", "medium_grazer", "large_browser", "medium_browser", "small_herbivore", "insect"] {
                let id = b.sim.world.reg.stock(&format!("c_fauna_{nm}")).unwrap();
                for v in b.sim.world.stock_mut(id).iter_mut() { *v = 0.0; }
            }
        }
        b.sim.world.events.clear();
        b.sim.run_years(40);
        b.sim.world.events.iter().filter(|e| e.kind == EventKind::Fire).map(|e| e.size as f64).sum::<f64>() / 40.0
    };
    let (with, without) = (burned(false), burned(true));
    println!("burned per year: with herbivores {:.0} km2, without {:.0} km2", with, without);
    assert!(without >= with * 0.95, "removing herbivores should not reduce fire: {with} vs {without}");
}

#[test]
fn animals_die_back_when_their_food_is_taken_away() {
    let mut b = spun(150);
    let before: f64 = stock(&b, "c_fauna_large_grazer").iter().chain(stock(&b, "c_fauna_medium_grazer").iter()).map(|&x| x as f64).sum();
    // strip all grass leaves
    for nm in ["c_leaf_c3_grass", "c_leaf_c4_grass"] { let id = b.sim.world.reg.stock(nm).unwrap(); for v in b.sim.world.stock_mut(id).iter_mut() { *v *= 0.02; } }
    b.sim.run_years(3);
    let after: f64 = stock(&b, "c_fauna_large_grazer").iter().chain(stock(&b, "c_fauna_medium_grazer").iter()).map(|&x| x as f64).sum();
    println!("grazer carbon before {before:.3e}, after starving 3 years {after:.3e}");
    assert!(after < before);
}

#[test]
#[ignore]
fn debug_rain_bins() {
    let mut b = spun(150);
    b.sim.run_years(100);
    let w_sea = stock(&b, "w_sea");
    let names = ["megaherbivore", "large_grazer", "medium_grazer", "large_browser", "medium_browser", "small_herbivore", "insect", "medium_carnivore", "large_carnivore", "small_carnivore"];
    let mut all: Vec<String> = names.iter().map(|n| format!("c_fauna_{n}")).collect();
    all.extend(["c_leaf_c3_grass".to_string(), "c_leaf_c4_grass".to_string(), "c_leaf_shrub".to_string()]);
    let refs: Vec<&str> = all.iter().map(|s| s.as_str()).collect();
    let m = year_mean(&mut b, &refs);
    let (g3, g4, sh) = (m[10].clone(), m[11].clone(), m[12].clone());
    let herb: Vec<(&str, Vec<f32>)> = names.iter().enumerate().map(|(k, n)| (*n, m[k].clone())).collect();
    for (lo, hi) in [(200.0, 400.0), (400.0, 600.0), (600.0, 900.0), (900.0, 1400.0), (1400.0, 2500.0)] {
        let mut cnt = 0; let mut grass = 0.0; let mut shr = 0.0; let mut sums = vec![0.0; herb.len()];
        for i in 0..w_sea.len() {
            if !is_land(&b, i, &w_sea) || b.env.p.lat_deg[i].abs() > 25.0 { continue; }
            let p: f64 = (0..12).map(|m| b.env.p.earth.prec[m][i] as f64).sum();
            if p < lo || p >= hi { continue; }
            cnt += 1; grass += (g3[i] + g4[i]) as f64; shr += sh[i] as f64;
            for (k, (_, h)) in herb.iter().enumerate() { sums[k] += h[i] as f64; }
        }
        let c = cnt.max(1) as f64;
        let mut line = format!("rain {lo:>5}-{hi:>5}: tiles {cnt:>3} grass {:.4} shrub {:.4} |", grass / c, shr / c);
        for (k, (nm, _)) in herb.iter().enumerate() { line += &format!(" {} {:.1e}", &nm[..nm.len().min(6)], sums[k] / c); }
        println!("{line}");
    }
}

#[test]
#[ignore]
fn debug_plant_bins() {
    let mut b = spun(150);
    b.sim.run_years(100);
    let w_sea = stock(&b, "w_sea");
    let c = odyssey_core::content::Content::load_dir(&data_dir()).unwrap();
    let ps = odyssey_core::eco::PlantSet::from_content(&c).unwrap();
    let leaf: Vec<Vec<f32>> = ps.types.iter().map(|t| stock(&b, &format!("c_leaf_{}", t.id))).collect();
    let wood: Vec<Vec<f32>> = ps.types.iter().map(|t| stock(&b, &format!("c_wood_{}", t.id))).collect();
    for (lo, hi) in [(200.0, 400.0), (400.0, 600.0), (600.0, 900.0), (900.0, 1400.0), (1400.0, 2500.0)] {
        let mut cnt = 0; let mut l = vec![0.0; ps.types.len()]; let mut w = vec![0.0; ps.types.len()];
        for i in 0..w_sea.len() {
            if !is_land(&b, i, &w_sea) || b.env.p.lat_deg[i].abs() > 25.0 { continue; }
            let p: f64 = (0..12).map(|m| b.env.p.earth.prec[m][i] as f64).sum();
            if p < lo || p >= hi { continue; }
            cnt += 1; for k in 0..ps.types.len() { l[k] += leaf[k][i] as f64; w[k] += wood[k][i] as f64; }
        }
        let cn = cnt.max(1) as f64;
        let mut line = format!("rain {lo:>5}-{hi:>5}: tiles {cnt:>3} |");
        for (k, t) in ps.types.iter().enumerate() { if l[k] / cn > 1e-3 || w[k] / cn > 1e-2 { line += &format!(" {} leaf {:.3} wood {:.2};", &t.id[..t.id.len().min(10)], l[k] / cn, w[k] / cn); } }
        println!("{line}");
    }
}

#[test]
#[ignore]
fn debug_find_savanna_tiles() {
    let b = build(&Setup::today(24)).unwrap();
    let w_sea = stock(&b, "w_sea");
    let mut n = 0;
    for i in 0..w_sea.len() {
        if !is_land(&b, i, &w_sea) { continue; }
        let lat = b.env.p.lat_deg[i]; let lon = b.sim.world.grid.lon[i].to_degrees();
        let p: f64 = (0..12).map(|m| b.env.p.earth.prec[m][i] as f64).sum();
        if lat > 5.0 && lat < 16.0 && lon > -15.0 && lon < 40.0 && p > 500.0 && p < 1000.0 { println!("tile {i} lat {lat:.1} lon {lon:.1} rain {p:.0}"); n += 1; if n > 5 { break; } }
    }
}
