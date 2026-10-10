//! bake <n> <prep_dir> <out_file>: sample the prepared rasters onto a grid of frequency n.
use odyssey_core::{earth, grid::Grid};
use std::path::Path;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() != 4 { eprintln!("usage: bake <n> <prep_dir> <out_file>"); std::process::exit(2); }
    let n: u32 = a[1].parse().expect("n is a number");
    let t0 = std::time::Instant::now();
    let grid = Grid::new(n);
    let e = earth::bake(&grid, Path::new(&a[2])).unwrap_or_else(|m| { eprintln!("{m}"); std::process::exit(1) });
    e.save(Path::new(&a[3])).expect("write");
    let land: f64 = (0..grid.len()).filter(|&i| e.is_land(i)).map(|i| grid.area_km2[i] as f64).sum::<f64>() / grid.total_area_km2();
    let t: f64 = (0..grid.len()).map(|i| (0..12).map(|m| e.tair[m][i] as f64).sum::<f64>() / 12.0 * grid.area_km2[i] as f64).sum::<f64>() / grid.total_area_km2();
    println!("n{n}: {} cells, land {:.1}%, mean air T {:.2} K, baked in {:.1}s", grid.len(), land * 100.0, t, t0.elapsed().as_secs_f64());
}
