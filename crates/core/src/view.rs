//! Map rasters and headline numbers for a viewer (the snapshot tool and the desktop app engine use these).

use crate::build::Built;
use crate::earth::{nearest, unit};

pub const W: usize = 360;
pub const H: usize = 180;

pub fn b64(bytes: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut s = String::with_capacity(bytes.len() * 4 / 3 + 4);
    for c in bytes.chunks(3) {
        let n = (c[0] as u32) << 16 | (*c.get(1).unwrap_or(&0) as u32) << 8 | *c.get(2).unwrap_or(&0) as u32;
        s.push(T[(n >> 18 & 63) as usize] as char); s.push(T[(n >> 12 & 63) as usize] as char);
        s.push(if c.len() > 1 { T[(n >> 6 & 63) as usize] as char } else { '=' });
        s.push(if c.len() > 2 { T[(n & 63) as usize] as char } else { '=' });
    }
    s
}

/// The tile under each pixel of a one-degree map, north up, longitude -180 to 180.
pub fn pixel_tiles(b: &Built) -> Vec<usize> {
    let grid = &b.sim.world.grid;
    let mut pix = vec![0usize; W * H];
    let mut hint = 0;
    for y in 0..H { for x in 0..W { hint = nearest(grid, unit(89.5 - y as f64, -179.5 + x as f64), hint); pix[y * W + x] = hint; } }
    pix
}

/// All map layers of the current world as JSON: {"year":..,"fields":{..},"mask":{..}}.
pub fn maps_json(b: &Built, pix: &[usize]) -> String {
    let w = &b.sim.world;
    let grid = &w.grid;
    let get = |nm: &str| -> Vec<f32> { w.stock(w.reg.stock(nm).unwrap()).to_vec() };
    let sum = |names: &[String]| -> Vec<f32> { let mut t = vec![0f32; grid.len()]; for nm in names { let v = get(nm); for i in 0..t.len() { t[i] += v[i]; } } t };
    let names = w.reg.stock_names();
    let pick = |pre: &[&str]| -> Vec<String> { names.iter().filter(|s| pre.iter().any(|p| s.starts_with(p))).cloned().collect() };
    let heat = get("heat"); let w_sea = get("w_sea"); let sea_ice = get("w_seaice");
    let ice_bands: Vec<String> = (0..12).map(|k| format!("w_ice{k}")).collect();
    let ice_mm = sum(&ice_bands);
    let veg = sum(&pick(&["c_leaf_", "c_wood_", "c_root_"]));
    let soil = sum(&["c_litter".to_string(), "c_sfast".to_string(), "c_sslow".to_string(), "c_spass".to_string()]);
    let fa = |l: &[&str]| sum(&l.iter().map(|s| format!("c_fauna_{s}")).collect::<Vec<_>>());
    let herb = fa(&["megaherbivore", "large_grazer", "medium_grazer", "large_browser", "medium_browser", "small_herbivore"]);
    let carn = fa(&["small_carnivore", "medium_carnivore", "large_carnivore"]);
    let plank = get("c_plankton");
    let fish = fa(&["fish", "large_fish"]);
    let is_sea: Vec<bool> = (0..grid.len()).map(|i| b.env.ocean0[i] && w_sea[i] > 50.0).collect();
    let temp: Vec<f32> = (0..grid.len()).map(|i| b.env.t_from_h(i, heat[i]) - 273.15).collect();
    let elev: Vec<f32> = (0..grid.len()).map(|i| b.env.p.bed0[i]).collect();
    let depth: Vec<f32> = (0..grid.len()).map(|i| w_sea[i] / 1000.0).collect();
    let ice_m: Vec<f32> = ice_mm.iter().map(|x| x / 917.0).collect();
    let x1000 = |v: Vec<f32>| -> Vec<f32> { v.iter().map(|x| x * 1000.0).collect() };
    // name, label, unit, values, min, max, land (true) or sea (false)
    let fields: Vec<(&str, &str, &str, Vec<f32>, f32, f32, bool)> = vec![
        ("temp", "Surface temperature", "C", temp, -40.0, 35.0, true),
        ("veg", "Plant carbon", "kgC/m2", veg, 0.0, 15.0, true),
        ("soil", "Soil carbon", "kgC/m2", soil, 0.0, 30.0, true),
        ("ice", "Land ice thickness", "m", ice_m, 0.0, 3000.0, true),
        ("herb", "Plant-eating animals", "gC/m2", x1000(herb), 0.0, 2.0, true),
        ("carn", "Meat-eating animals", "gC/m2", x1000(carn), 0.0, 0.2, true),
        ("plank", "Plankton", "gC/m2", x1000(plank), 0.0, 5.0, false),
        ("fish", "Fish", "gC/m2", x1000(fish), 0.0, 3.0, false),
        ("seaice", "Sea ice", "mm w.e.", sea_ice.clone(), 0.0, 2000.0, false),
        ("depth", "Sea depth", "m", depth, 0.0, 6000.0, false),
        ("elev", "Land height", "m", elev, 0.0, 5000.0, true),
    ];
    let mut json = format!("{{\"year\":{},\"fields\":{{", w.clock.year());
    for (k, (name, label, unit_s, vals, lo, hi, land)) in fields.iter().enumerate() {
        let mut bytes = vec![0u8; W * H];
        for p in 0..W * H {
            let t = pix[p];
            if *land == is_sea[t] { continue; }
            let f = ((vals[t] - lo) / (hi - lo)).clamp(0.0, 1.0);
            bytes[p] = 1 + (f * 254.0) as u8;
        }
        if k > 0 { json.push(','); }
        json.push_str(&format!("\"{name}\":{{\"label\":\"{label}\",\"unit\":\"{unit_s}\",\"min\":{lo},\"max\":{hi},\"data\":\"{}\"}}", b64(&bytes)));
    }
    let mask: Vec<u8> = (0..W * H).map(|p| if is_sea[pix[p]] { 0 } else { 1 }).collect();
    json.push_str(&format!("}},\"mask\":{{\"data\":\"{}\"}}}}", b64(&mask)));
    json
}

/// Headline numbers: mean temperature (C), CO2 (ppm), plant carbon, soil carbon, animal carbon (PgC), land ice (m of sea level), violations.
pub fn stats_json(b: &Built) -> String {
    let w = &b.sim.world;
    let area = &w.grid.area_km2;
    let total: f64 = area.iter().map(|&a| a as f64).sum();
    let tot = |nm: &str| -> f64 { let s = w.stock(w.reg.stock(nm).unwrap()); (0..s.len()).map(|i| s[i] as f64 * area[i] as f64 * 1e6).sum::<f64>() / 1e12 };
    let names = w.reg.stock_names();
    let sum_pre = |pre: &[&str]| -> f64 { names.iter().filter(|s| pre.iter().any(|p| s.starts_with(p))).map(|s| tot(s)).sum() };
    let h = w.stock(b.env.ph.heat);
    let t_mean: f64 = (0..h.len()).map(|i| b.env.t_from_h(i, h[i]) as f64 * area[i] as f64).sum::<f64>() / total - 273.15;
    let ppm = w.stock(b.env.ph.c_air).iter().zip(area.iter()).map(|(&c, &a)| c as f64 * a as f64).sum::<f64>() / total * crate::climate::PPM_PER_KGC_M2;
    let ocean_area: f64 = (0..area.len()).filter(|&i| b.env.ocean0[i]).map(|i| area[i] as f64).sum();
    let ice_km3: f64 = (0..12).map(|k| { let s = w.stock(b.env.ph.w_ice[k]); (0..s.len()).map(|i| s[i] as f64 * area[i] as f64).sum::<f64>() }).sum::<f64>() / 1e6;
    let sle = ice_km3 * 1e9 / (ocean_area * 1e6);
    format!("{{\"year\":{},\"t_mean\":{:.3},\"co2\":{:.2},\"veg\":{:.1},\"soil\":{:.1},\"fauna\":{:.3},\"ice_sle\":{:.2},\"violations\":{},\"skipped\":{}}}",
        w.clock.year(), t_mean, ppm, sum_pre(&["c_leaf_", "c_wood_", "c_root_"]), sum_pre(&["c_litter", "c_sfast", "c_sslow", "c_spass", "c_carcass"]), sum_pre(&["c_fauna_"]), sle, w.violation_count, b.sim.skipped)
}
