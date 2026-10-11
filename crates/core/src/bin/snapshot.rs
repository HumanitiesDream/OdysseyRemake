//! Turn saved worlds into map rasters for a viewer.
//!
//! snapshot --n 48 --state a.state [--state b.state] --out maps.json

use odyssey_core::build::*;
use odyssey_core::sim::Parallel;
use odyssey_core::{save, view};

fn main() {
    let (mut n, mut states, mut out_path) = (48u32, Vec::<String>::new(), "maps.json".to_string());
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut v = || args.next().expect("value");
        match a.as_str() { "--n" => n = v().parse().unwrap(), "--state" => states.push(v()), "--out" => out_path = v(), o => panic!("unknown {o}") }
    }
    let mut b = build(&Setup::today(n)).unwrap();
    b.sim.parallel = Parallel::Off;
    let pix = view::pixel_tiles(&b);
    let mut json = String::from("{\"w\":360,\"h\":180,\"snapshots\":[");
    for (si, path) in states.iter().enumerate() {
        save::read_into(&mut b.sim, &std::fs::read(path).unwrap()).unwrap();
        if si > 0 { json.push(','); }
        let one = view::maps_json(&b, &pix);
        // add the file name as the label
        json.push_str(&one.replacen("{", &format!("{{\"label\":\"{}\",", std::path::Path::new(path).file_name().unwrap().to_string_lossy()), 1));
    }
    json.push_str("]}");
    std::fs::write(&out_path, &json).unwrap();
    println!("wrote {out_path}: {} bytes", json.len());
}
