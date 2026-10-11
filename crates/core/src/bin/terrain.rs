//! Bake bed elevation (metres, i16) for every cell of a grid from the 3 arc-minute ETOPO1 raster.
//!
//! terrain <n> <prep dir with topo3.f32> <out file> [look out file]
//! With a fourth argument it also bakes today's natural look (one byte a cell, packed as the engine's "look"
//! layer: 1 + white * 16 + green) from WorldClim rain and temperature and the ice thickness. The title globe uses it.
//! The globe uses it to draw 16 km coasts and relief while the simulation runs on a coarser grid.

use odyssey_core::grid::Grid;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let n: u32 = a[1].parse().unwrap();
    let raw = std::fs::read(format!("{}/topo3.f32", a[2])).expect("topo3.f32");
    let topo: Vec<f32> = raw.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
    let (w, h) = (7200usize, 3600usize);
    assert_eq!(topo.len(), w * h);
    let g = Grid::new(n);
    let mut out = Vec::with_capacity(g.len() * 2);
    for i in 0..g.len() {
        let (lat, lon) = (g.lat[i].to_degrees() as f64, g.lon[i].to_degrees() as f64);
        let px = (((lon + 180.0) / 360.0 * w as f64) as isize).rem_euclid(w as isize) as usize;
        let py = (((90.0 - lat) / 180.0 * h as f64) as isize).clamp(0, h as isize - 1) as usize;
        let mut s = 0.0f64; let mut c = 0.0;
        for dy in -1..=1isize { for dx in -1..=1isize {
            let x = (px as isize + dx).rem_euclid(w as isize) as usize; let y = (py as isize + dy).clamp(0, h as isize - 1) as usize;
            s += topo[y * w + x] as f64; c += 1.0;
        } }
        out.extend_from_slice(&((s / c).round().clamp(-32000.0, 32000.0) as i16).to_le_bytes());
    }
    std::fs::write(&a[3], out).unwrap();
    println!("wrote {} cells to {}", g.len(), a[3]);
    if let Some(look_out) = a.get(4) { bake_look(&g, &a[2], look_out, &topo); }
}

fn read_f32(path: String) -> Vec<f32> {
    let raw = std::fs::read(&path).unwrap_or_else(|_| panic!("{path}"));
    raw.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

fn bake_look(g: &Grid, prep: &str, out_path: &str, topo: &[f32]) {
    let (ww, wh) = (2160usize, 1080usize);
    let tavg = read_f32(format!("{prep}/wc_tavg.f32"));
    let prec = read_f32(format!("{prep}/wc_prec.f32"));
    let ice = read_f32(format!("{prep}/ice3.f32"));
    let mut out = Vec::with_capacity(g.len());
    for i in 0..g.len() {
        let (lat, lon) = (g.lat[i].to_degrees() as f64, g.lon[i].to_degrees() as f64);
        let at = |w: usize, h: usize| -> usize {
            let x = (((lon + 180.0) / 360.0 * w as f64) as isize).rem_euclid(w as isize) as usize;
            let y = (((90.0 - lat) / 180.0 * h as f64) as isize).clamp(0, h as isize - 1) as usize;
            y * w + x
        };
        let k3 = at(7200, 3600);
        if topo[k3] <= 0.0 && ice[k3] < 1.0 { out.push(1u8); continue; }
        let k = at(ww, wh);
        let (mut p, mut tsum, mut tmax, mut ok) = (0.0f64, 0.0f64, -99.0f64, true);
        for m in 0..12 {
            let t = tavg[m * ww * wh + k] as f64; let r = prec[m * ww * wh + k] as f64;
            if !t.is_finite() || !r.is_finite() { ok = false; break; }
            p += r; tsum += t; tmax = tmax.max(t);
        }
        let tmean = tsum / 12.0;
        let thick = ice[k3] as f64;
        let (green, white) = if thick > 20.0 || !ok && lat.abs() > 60.0 { (0.0, 1.0) } else if !ok { (0.3, 0.0) } else {
            // rain against the water warm air can take up; trees need a warm month above 10 C
            let wet = p / (250.0 + 45.0 * tmean.max(0.0) + 4.0 * tmean.max(0.0).powi(2));
            let mut gr = (wet / 1.4).clamp(0.0, 1.0).sqrt();
            if tmax < 10.0 { gr = gr.min(0.35 * (tmax / 10.0).max(0.0)); }
            let wh = if tmax < 0.0 { 0.9 } else if tmean < -10.0 { 0.35 } else { 0.0 };
            (gr, wh)
        };
        let gq = (green * 15.0).round() as u8; let wq = (white * 14.0f64).round() as u8;
        out.push(1 + wq * 16 + gq);
    }
    std::fs::write(out_path, &out).unwrap();
    println!("wrote look for {} cells to {out_path}", g.len());
}
