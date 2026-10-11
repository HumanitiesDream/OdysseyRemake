//! Turn a saved world into map rasters for a viewer.
//!
//! snapshot --n 48 --state file.state [--state2 other.state] --out maps.json
//!
//! Each map is 360 x 180 (one degree), north up, longitude -180 to 180, stored as bytes between a minimum and a maximum.

use odyssey_core::build::*;
use odyssey_core::earth::{nearest, unit};
use odyssey_core::sim::Parallel;
use odyssey_core::save;
use std::io::Write;

const W: usize = 360;
const H: usize = 180;

fn b64(bytes: &[u8]) -> String {
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

fn main() {
    let (mut n, mut states, mut out_path) = (48u32, Vec::<String>::new(), "maps.json".to_string());
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut v = || args.next().expect("value");
        match a.as_str() { "--n" => n = v().parse().unwrap(), "--state" | "--state2" => states.push(v()), "--out" => out_path = v(), o => panic!("unknown {o}") }
    }
    let mut b = build(&Setup::today(n)).unwrap();
    b.sim.parallel = Parallel::Off;
    // which tile sits under each pixel
    let grid = b.sim.world.grid.clone();
    let mut pix = vec![0usize; W * H];
    let mut hint = 0;
    for y in 0..H {
        for x in 0..W {
            let (lat, lon) = (89.5 - y as f64, -179.5 + x as f64);
            hint = nearest(&grid, unit(lat, lon), hint);
            pix[y * W + x] = hint;
        }
    }
    let mut json = String::from("{\"w\":360,\"h\":180,\"snapshots\":[");
    for (si, path) in states.iter().enumerate() {
        save::read_into(&mut b.sim, &std::fs::read(path).unwrap()).unwrap();
        let w = &b.sim.world;
        let get = |nm: &str| -> Vec<f32> { w.stock(w.reg.stock(nm).unwrap()).to_vec() };
        let sum = |names: &[String]| -> Vec<f32> { let mut t = vec![0f32; grid.len()]; for nm in names { let v = get(nm); for i in 0..t.len() { t[i] += v[i]; } } t };
        let names = w.reg.stock_names();
        let pick = |pre: &[&str]| -> Vec<String> { names.iter().filter(|s| pre.iter().any(|p| s.starts_with(p))).cloned().collect() };
        let heat = get("heat"); let w_sea = get("w_sea"); let sea_ice = get("w_seaice");
        let ice_bands: Vec<String> = (0..12).map(|k| format!("w_ice{k}")).collect();
        let ice_mm = sum(&ice_bands);
        let veg = sum(&pick(&["c_leaf_", "c_wood_", "c_root_"]));
        let soil = sum(&["c_litter".to_string(), "c_sfast".to_string(), "c_sslow".to_string(), "c_spass".to_string()]);
        let herb = sum(&["megaherbivore", "large_grazer", "medium_grazer", "large_browser", "medium_browser", "small_herbivore"].iter().map(|s| format!("c_fauna_{s}")).collect::<Vec<_>>());
        let carn = sum(&["small_carnivore", "medium_carnivore", "large_carnivore"].iter().map(|s| format!("c_fauna_{s}")).collect::<Vec<_>>());
        let plank = get("c_plankton");
        let fish = sum(&["c_fauna_fish".to_string(), "c_fauna_large_fish".to_string()]);
        let is_sea: Vec<bool> = (0..grid.len()).map(|i| b.env.ocean0[i] && w_sea[i] > 50.0).collect();
        let temp: Vec<f32> = (0..grid.len()).map(|i| b.env.t_from_h(i, heat[i]) - 273.15).collect();
        let elev: Vec<f32> = (0..grid.len()).map(|i| b.env.p.bed0[i]).collect();
        let depth: Vec<f32> = (0..grid.len()).map(|i| w_sea[i] / 1000.0).collect();
        let ice_m: Vec<f32> = ice_mm.iter().map(|x| x / 917.0).collect();
        let mut fields: Vec<(&str, &str, &str, Vec<f32>, f32, f32, bool)> = Vec::new();
        // name, label, unit, values, min, max, land only (true) / sea only (false) handled by mask below
        fields.push(("temp", "Surface temperature", "C", temp, -40.0, 35.0, true));
        fields.push(("veg", "Plant carbon", "kgC/m2", veg, 0.0, 15.0, true));
        fields.push(("soil", "Soil carbon", "kgC/m2", soil, 0.0, 30.0, true));
        fields.push(("ice", "Land ice thickness", "m", ice_m, 0.0, 3000.0, true));
        fields.push(("herb", "Plant-eating animals", "kgC/m2 (x1000)", herb.iter().map(|x| x * 1000.0).collect(), 0.0, 2.0, true));
        fields.push(("carn", "Meat-eating animals", "kgC/m2 (x1000)", carn.iter().map(|x| x * 1000.0).collect(), 0.0, 0.2, true));
        fields.push(("plank", "Plankton", "kgC/m2 (x1000)", plank.iter().map(|x| x * 1000.0).collect(), 0.0, 5.0, false));
        fields.push(("fish", "Fish", "kgC/m2 (x1000)", fish.iter().map(|x| x * 1000.0).collect(), 0.0, 3.0, false));
        fields.push(("seaice", "Sea ice", "mm w.e.", sea_ice.clone(), 0.0, 2000.0, false));
        fields.push(("depth", "Sea depth", "m", depth, 0.0, 6000.0, false));
        fields.push(("elev", "Land height", "m", elev, 0.0, 5000.0, true));
        if si > 0 { json.push(','); }
        json.push_str(&format!("{{\"label\":\"{}\",\"year\":{},\"fields\":{{", std::path::Path::new(path).file_name().unwrap().to_string_lossy(), w.clock.year()));
        for (k, (name, label, unit_s, vals, lo, hi, land)) in fields.iter().enumerate() {
            let mut bytes = vec![0u8; W * H]; // 0 means no data
            for p in 0..W * H {
                let t = pix[p];
                let ok = if *land { !is_sea[t] } else { is_sea[t] };
                if !ok { continue; }
                let f = ((vals[t] - lo) / (hi - lo)).clamp(0.0, 1.0);
                bytes[p] = 1 + (f * 254.0) as u8;
            }
            if k > 0 { json.push(','); }
            json.push_str(&format!("\"{name}\":{{\"label\":\"{label}\",\"unit\":\"{unit_s}\",\"min\":{lo},\"max\":{hi},\"data\":\"{}\"}}", b64(&bytes)));
        }
        // a land/sea mask for the base layer
        let mask: Vec<u8> = (0..W * H).map(|p| if is_sea[pix[p]] { 0 } else { 1 }).collect();
        json.push_str(&format!(",\"mask\":{{\"data\":\"{}\"}}}}}}", b64(&mask)));
    }
    json.push_str("]}");
    std::fs::File::create(&out_path).unwrap().write_all(json.as_bytes()).unwrap();
    println!("wrote {out_path}: {} bytes", json.len());
}
