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

pub struct Layer { pub name: &'static str, pub label: &'static str, pub unit: &'static str, pub lo: f32, pub hi: f32, pub land: bool, pub both: bool, pub vals: Vec<f32> }

/// Sea or land flag per tile (true when the tile is open sea).
pub fn sea_mask(b: &Built) -> Vec<bool> {
    let w = &b.sim.world;
    let w_sea = w.stock(w.reg.stock("w_sea").unwrap());
    (0..w.grid.len()).map(|i| b.env.ocean0[i] && w_sea[i] > 50.0).collect()
}

/// Every layer's value on every tile of the simulation grid.
pub fn layers(b: &Built) -> Vec<Layer> {
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
    let temp: Vec<f32> = (0..grid.len()).map(|i| b.env.t_from_h(i, heat[i]) - 273.15).collect();
    let elev: Vec<f32> = (0..grid.len()).map(|i| b.env.p.bed0[i]).collect();
    let depth: Vec<f32> = (0..grid.len()).map(|i| w_sea[i] / 1000.0).collect();
    let ice_m: Vec<f32> = ice_mm.iter().map(|x| x / 917.0).collect();
    let x1000 = |v: Vec<f32>| -> Vec<f32> { v.iter().map(|x| x * 1000.0).collect() };
    vec![
        Layer { name: "temp", label: "Surface temperature", unit: "C", lo: -40.0, hi: 35.0, land: true, both: true, vals: temp },
        Layer { name: "veg", label: "Plant carbon", unit: "kgC/m2", lo: 0.0, hi: 15.0, land: true, both: false, vals: veg },
        Layer { name: "soil", label: "Soil carbon", unit: "kgC/m2", lo: 0.0, hi: 30.0, land: true, both: false, vals: soil },
        Layer { name: "ice", label: "Land ice thickness", unit: "m", lo: 0.0, hi: 3000.0, land: true, both: false, vals: ice_m },
        Layer { name: "herb", label: "Plant-eating animals", unit: "gC/m2", lo: 0.0, hi: 2.0, land: true, both: false, vals: x1000(herb) },
        Layer { name: "carn", label: "Meat-eating animals", unit: "gC/m2", lo: 0.0, hi: 0.2, land: true, both: false, vals: x1000(carn) },
        Layer { name: "elev", label: "Land height", unit: "m", lo: 0.0, hi: 5000.0, land: true, both: false, vals: elev },
        Layer { name: "plank", label: "Plankton", unit: "gC/m2", lo: 0.0, hi: 5.0, land: false, both: false, vals: x1000(plank) },
        Layer { name: "fish", label: "Fish", unit: "gC/m2", lo: 0.0, hi: 3.0, land: false, both: false, vals: x1000(fish) },
        Layer { name: "seaice", label: "Sea ice", unit: "mm w.e.", lo: 0.0, hi: 2000.0, land: false, both: false, vals: sea_ice },
        Layer { name: "depth", label: "Sea depth", unit: "m", lo: 0.0, hi: 6000.0, land: false, both: false, vals: depth },
    ]
}

/// One layer, one byte per simulation tile (0 means no value there), as a JSON event.
pub fn layer_event(b: &Built, name: &str) -> Option<String> {
    let sea = sea_mask(b);
    let l = layers(b).into_iter().find(|l| l.name == name)?;
    let bytes: Vec<u8> = (0..sea.len()).map(|i| if !l.both && l.land == sea[i] { 0 } else { 1 + (((l.vals[i] - l.lo) / (l.hi - l.lo)).clamp(0.0, 1.0) * 254.0) as u8 }).collect();
    Some(format!("{{\"event\":\"layer\",\"name\":\"{}\",\"label\":\"{}\",\"unit\":\"{}\",\"min\":{},\"max\":{},\"year\":{},\"data\":\"{}\"}}", l.name, l.label, l.unit, l.lo, l.hi, b.sim.world.clock.year(), b64(&bytes)))
}

/// 1 byte per tile: 1 for open sea, 0 for land.
pub fn sea_event(b: &Built) -> String {
    let bytes: Vec<u8> = sea_mask(b).iter().map(|&s| s as u8).collect();
    format!("{{\"event\":\"sea\",\"data\":\"{}\"}}", b64(&bytes))
}

/// Everything known about one tile, for an inspector.
pub fn tile_json(b: &Built, id: usize) -> String {
    let sea = sea_mask(b);
    let g = &b.sim.world.grid;
    let mut rows = String::new();
    for (k, l) in layers(b).iter().enumerate() {
        if !l.both && l.land == sea[id] { continue; }
        if k > 0 && !rows.is_empty() { rows.push(','); }
        rows.push_str(&format!("{{\"label\":\"{}\",\"unit\":\"{}\",\"value\":{:.4}}}", l.label, l.unit, l.vals[id]));
    }
    format!("{{\"event\":\"tile\",\"id\":{id},\"lat\":{:.2},\"lon\":{:.2},\"area_km2\":{:.0},\"sea\":{},\"rows\":[{rows}]}}",
        g.lat[id].to_degrees(), g.lon[id].to_degrees(), g.area_km2[id], sea[id])
}

/// For each cell of a finer grid, the simulation tile it lies in (nearest centre), as little-endian u32.
pub fn parents_event(sim_n: u32, view_n: u32) -> String {
    let fine = crate::grid::Grid::new(view_n);
    let co = crate::grid::Coarse::new(&fine, sim_n);
    let mut bytes = Vec::with_capacity(co.parent.len() * 4);
    for p in &co.parent { bytes.extend_from_slice(&p.to_le_bytes()); }
    format!("{{\"event\":\"parents\",\"view\":{view_n},\"data\":\"{}\"}}", b64(&bytes))
}

/// All map layers of the current world as JSON: {"year":..,"fields":{..},"mask":{..}}.
pub fn maps_json(b: &Built, pix: &[usize]) -> String {
    let is_sea = sea_mask(b);
    let grid = &b.sim.world.grid;
    let _ = grid;
    let mut json = format!("{{\"year\":{},\"fields\":{{", b.sim.world.clock.year());
    for (k, l) in layers(b).iter().enumerate() {
        let mut bytes = vec![0u8; W * H];
        for p in 0..W * H {
            let t = pix[p];
            if !l.both && l.land == is_sea[t] { continue; }
            let f = ((l.vals[t] - l.lo) / (l.hi - l.lo)).clamp(0.0, 1.0);
            bytes[p] = 1 + (f * 254.0) as u8;
        }
        if k > 0 { json.push(','); }
        json.push_str(&format!("\"{}\":{{\"label\":\"{}\",\"unit\":\"{}\",\"min\":{},\"max\":{},\"data\":\"{}\"}}", l.name, l.label, l.unit, l.lo, l.hi, b64(&bytes)));
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
