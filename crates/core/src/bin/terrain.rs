//! Bake bed elevation (metres, i16) for every cell of a grid from the 3 arc-minute ETOPO1 raster.
//!
//! terrain <n> <prep dir with topo3.f32> <out file>
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
}
