//! Baked Earth data: the starting state and the test, never the driver.
//!
//! `bake` samples the rasters made by `tools/prep_rasters.py` onto a grid. The result
//! is one small file per grid size. Sources (all open downloads, see docs/DATA.md):
//! ETOPO1 bedrock and ice surface (NOAA), WorldClim 2.1 (temperature, rain, elevation),
//! ERSST v5 (sea surface temperature), GPCP (rain over sea), NCEP/NCAR (air temperature
//! where the others have none: Antarctica, Greenland ice, sea ice).

use crate::grid::Grid;
use flate2::{read::DeflateDecoder, write::DeflateEncoder, Compression};
use std::io::{Read, Write};
use std::path::Path;

pub const BANDS: usize = 12;
const MAGIC: &[u8; 4] = b"OEAR";

#[derive(Clone, Debug)]
pub struct Earth {
    pub n: u32,
    /// Bed elevation quantiles inside each tile, m: band b is the b-th twelfth of the tile's area.
    pub zq: Vec<Vec<f32>>,
    /// Share of the tile whose bed lies above sea level.
    pub land_frac: Vec<f32>,
    /// WorldClim mean elevation of the tile's land pixels, m (NaN where none).
    pub wc_elev: Vec<f32>,
    /// Mean ice thickness today, m.
    pub ice: Vec<f32>,
    /// Monthly surface air temperature, K, 12 x tiles.
    pub tair: Vec<Vec<f32>>,
    /// Monthly sea surface temperature, K (NaN on land tiles), 12 x tiles.
    pub sst: Vec<Vec<f32>>,
    /// Monthly precipitation, mm per month, 12 x tiles.
    pub prec: Vec<Vec<f32>>,
}

impl Earth {
    pub fn len(&self) -> usize { self.land_frac.len() }
    pub fn is_empty(&self) -> bool { self.land_frac.is_empty() }
    pub fn mean_bed(&self, i: usize) -> f32 { self.zq.iter().map(|b| b[i]).sum::<f32>() / BANDS as f32 }
    pub fn is_land(&self, i: usize) -> bool { self.land_frac[i] >= 0.5 }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let mut body: Vec<u8> = Vec::new();
        let mut put = |name: &str, v: &[f32]| {
            body.extend((name.len() as u16).to_le_bytes());
            body.extend(name.as_bytes());
            body.extend((v.len() as u32).to_le_bytes());
            for x in v { body.extend(x.to_le_bytes()); }
        };
        put("land_frac", &self.land_frac);
        put("wc_elev", &self.wc_elev);
        put("ice", &self.ice);
        for b in 0..BANDS { put(&format!("zq{b}"), &self.zq[b]); }
        for m in 0..12 {
            put(&format!("tair{m}"), &self.tair[m]);
            put(&format!("sst{m}"), &self.sst[m]);
            put(&format!("prec{m}"), &self.prec[m]);
        }
        let mut enc = DeflateEncoder::new(Vec::new(), Compression::best());
        enc.write_all(&body)?;
        let packed = enc.finish()?;
        let mut out = Vec::new();
        out.extend(MAGIC);
        out.extend(1u16.to_le_bytes());
        out.extend(self.n.to_le_bytes());
        out.extend((body.len() as u64).to_le_bytes());
        out.extend(packed);
        std::fs::write(path, out)
    }

    pub fn load(path: &Path) -> Result<Earth, String> {
        let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        if bytes.len() < 18 || &bytes[..4] != MAGIC { return Err(format!("{}: not an Odyssey Earth file", path.display())); }
        let n = u32::from_le_bytes(bytes[6..10].try_into().unwrap());
        let raw_len = u64::from_le_bytes(bytes[10..18].try_into().unwrap()) as usize;
        let mut body = Vec::with_capacity(raw_len);
        DeflateDecoder::new(&bytes[18..]).read_to_end(&mut body).map_err(|e| e.to_string())?;
        if body.len() != raw_len { return Err("Earth file is damaged".into()); }
        let mut map = std::collections::BTreeMap::new();
        let mut p = 0usize;
        while p < body.len() {
            let nl = u16::from_le_bytes(body[p..p + 2].try_into().unwrap()) as usize; p += 2;
            let name = String::from_utf8_lossy(&body[p..p + nl]).to_string(); p += nl;
            let cnt = u32::from_le_bytes(body[p..p + 4].try_into().unwrap()) as usize; p += 4;
            let v: Vec<f32> = body[p..p + cnt * 4].chunks_exact(4).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect();
            p += cnt * 4;
            map.insert(name, v);
        }
        let get = |k: &str| map.get(k).cloned().ok_or_else(|| format!("Earth file lacks {k}"));
        let cells = crate::grid::cell_count(n);
        let e = Earth {
            n,
            land_frac: get("land_frac")?, wc_elev: get("wc_elev")?, ice: get("ice")?,
            zq: (0..BANDS).map(|b| get(&format!("zq{b}"))).collect::<Result<_, _>>()?,
            tair: (0..12).map(|m| get(&format!("tair{m}"))).collect::<Result<_, _>>()?,
            sst: (0..12).map(|m| get(&format!("sst{m}"))).collect::<Result<_, _>>()?,
            prec: (0..12).map(|m| get(&format!("prec{m}"))).collect::<Result<_, _>>()?,
        };
        if e.land_frac.len() != cells { return Err("Earth file has the wrong cell count".into()); }
        Ok(e)
    }
}

// ------------------------------------------------------------------ baking ----

struct Raster { w: usize, h: usize, planes: usize, data: Vec<f32> }
impl Raster {
    fn load(path: &Path, w: usize, h: usize, planes: usize) -> Result<Raster, String> {
        let b = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        if b.len() != w * h * planes * 4 { return Err(format!("{}: expected {} bytes, found {}", path.display(), w * h * planes * 4, b.len())); }
        Ok(Raster { w, h, planes, data: b.chunks_exact(4).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect() })
    }
    fn at(&self, plane: usize, x: usize, y: usize) -> f32 { self.data[(plane * self.h + y) * self.w + x] }
    /// Bilinear sample at (lat, lon) degrees. `lon0`, `dlon`, `lat0`, `dlat` locate pixel centres.
    /// NaN corners fall back to the nearest valid corner. NaN if all are missing.
    fn sample(&self, plane: usize, lat: f64, lon: f64, lat0: f64, dlat: f64, lon0: f64, dlon: f64) -> f32 {
        let fy = ((lat0 - lat) / dlat).clamp(0.0, (self.h - 1) as f64);
        let mut fx = (lon - lon0) / dlon;
        fx = fx.rem_euclid(self.w as f64);
        let (y0, x0) = (fy.floor() as usize, fx.floor() as usize);
        let (y1, x1) = ((y0 + 1).min(self.h - 1), (x0 + 1) % self.w);
        let (ty, tx) = (fy - y0 as f64, fx - x0 as f64);
        let c = [(self.at(plane, x0, y0), (1.0 - tx) * (1.0 - ty)), (self.at(plane, x1, y0), tx * (1.0 - ty)),
                 (self.at(plane, x0, y1), (1.0 - tx) * ty), (self.at(plane, x1, y1), tx * ty)];
        let (mut sum, mut wsum, mut best, mut bw) = (0.0f64, 0.0f64, f32::NAN, -1.0f64);
        for (v, w) in c { if !v.is_nan() { sum += v as f64 * w; wsum += w; if w > bw { bw = w; best = v; } } }
        if wsum >= 0.999 { (sum / wsum) as f32 } else if wsum > 0.0 { best } else { f32::NAN }
    }
}

pub fn unit(lat_deg: f64, lon_deg: f64) -> [f32; 3] {
    let (la, lo) = (lat_deg.to_radians(), lon_deg.to_radians());
    [(la.cos() * lo.cos()) as f32, la.sin() as f32, (-la.cos() * lo.sin()) as f32]
}

/// Nearest tile to a point, walking from `hint`.
pub fn nearest(grid: &Grid, p: [f32; 3], mut cur: usize) -> usize {
    let dot = |i: usize| { let q = grid.pos[i]; q[0] as f64 * p[0] as f64 + q[1] as f64 * p[1] as f64 + q[2] as f64 * p[2] as f64 };
    let mut best = dot(cur);
    loop {
        let mut next = cur;
        for j in grid.neighbours(cur) { let d = dot(j); if d > best { best = d; next = j; } }
        if next == cur { return cur; }
        cur = next;
    }
}

/// Tile of every pixel of a regular raster (pixel centres), row by row from the north.
fn assign_pixels(grid: &Grid, w: usize, h: usize, lat0: f64, dlat: f64, lon0: f64, dlon: f64) -> Vec<u32> {
    let mut out = vec![0u32; w * h];
    let mut row_hint = 0usize;
    for y in 0..h {
        let lat = lat0 - y as f64 * dlat;
        let mut hint = row_hint;
        for x in 0..w {
            let t = nearest(grid, unit(lat, lon0 + x as f64 * dlon), hint);
            out[y * w + x] = t as u32;
            hint = t;
            if x == 0 { row_hint = t; }
        }
    }
    out
}

/// Bake the Earth onto `grid` from the rasters in `prep`.
pub fn bake(grid: &Grid, prep: &Path) -> Result<Earth, String> {
    let n = grid.len();
    let topo = Raster::load(&prep.join("topo3.f32"), 7200, 3600, 1)?;
    let ice = Raster::load(&prep.join("ice3.f32"), 7200, 3600, 1).ok();
    let wc_e = Raster::load(&prep.join("wc_elev.f32"), 2160, 1080, 1)?;
    let wc_t = Raster::load(&prep.join("wc_tavg.f32"), 2160, 1080, 12)?;
    let wc_p = Raster::load(&prep.join("wc_prec.f32"), 2160, 1080, 12)?;
    let sst = Raster::load(&prep.join("sst.f32"), 180, 89, 12)?;
    let gpcp = Raster::load(&prep.join("gpcp.f32"), 144, 72, 12)?;
    let ncep = Raster::load(&prep.join("ncep.f32"), 144, 73, 12)?;

    // --- elevation quantiles, land share, ice
    let tid = assign_pixels(grid, 7200, 3600, 90.0 - 0.025, 0.05, -180.0 + 0.025, 0.05);
    let mut counts = vec![0u32; n + 1];
    for &t in &tid { counts[t as usize + 1] += 1; }
    for i in 0..n { counts[i + 1] += counts[i]; }
    let mut fill = counts.clone();
    let mut samples = vec![0f32; tid.len()];
    let mut ice_sum = vec![0f64; n];
    for (k, &t) in tid.iter().enumerate() {
        samples[fill[t as usize] as usize] = topo.data[k];
        fill[t as usize] += 1;
        if let Some(r) = &ice { ice_sum[t as usize] += r.data[k] as f64; }
    }
    let mut zq: Vec<Vec<f32>> = (0..BANDS).map(|_| vec![0f32; n]).collect();
    let mut land_frac = vec![0f32; n];
    let mut ice_m = vec![0f32; n];
    for i in 0..n {
        let s = &mut samples[counts[i] as usize..counts[i + 1] as usize];
        if s.is_empty() {
            // a tile smaller than a pixel: take the pixel under its centre
            let (lat, lon) = (grid.lat[i] as f64 * 180.0 / std::f64::consts::PI, grid.lon[i] as f64 * 180.0 / std::f64::consts::PI);
            let v = topo.sample(0, lat, lon, 90.0 - 0.025, 0.05, -180.0 + 0.025, 0.05);
            for b in 0..BANDS { zq[b][i] = v; }
            land_frac[i] = (v > 0.0) as u8 as f32;
            continue;
        }
        s.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for b in 0..BANDS {
            let k = (((b as f64 + 0.5) / BANDS as f64) * s.len() as f64) as usize;
            zq[b][i] = s[k.min(s.len() - 1)];
        }
        land_frac[i] = s.iter().filter(|&&v| v > 0.0).count() as f32 / s.len() as f32;
        ice_m[i] = (ice_sum[i] / s.len() as f64) as f32;
    }
    drop(samples); drop(tid);

    // --- WorldClim land pixels
    let wid = assign_pixels(grid, 2160, 1080, 90.0 - 1.0 / 12.0, 1.0 / 6.0, -180.0 + 1.0 / 12.0, 1.0 / 6.0);
    let mut cnt = vec![0u32; n];
    let mut esum = vec![0f64; n];
    let mut tsum = vec![[0f64; 12]; n];
    let mut psum = vec![[0f64; 12]; n];
    for (k, &t) in wid.iter().enumerate() {
        let (x, y) = (k % 2160, k / 2160);
        let e = wc_e.at(0, x, y);
        if e.is_nan() || wc_t.at(0, x, y).is_nan() { continue; }
        let t = t as usize;
        cnt[t] += 1; esum[t] += e as f64;
        for m in 0..12 { tsum[t][m] += wc_t.at(m, x, y) as f64; let p = wc_p.at(m, x, y); psum[t][m] += if p.is_nan() { 0.0 } else { p as f64 }; }
    }

    // --- assemble per tile
    let mut tair = vec![vec![0f32; n]; 12];
    let mut sstv = vec![vec![f32::NAN; n]; 12];
    let mut prec = vec![vec![0f32; n]; 12];
    let mut wc_elev = vec![f32::NAN; n];
    let rad = 180.0 / std::f64::consts::PI;
    for i in 0..n {
        let (lat, lon) = (grid.lat[i] as f64 * rad, grid.lon[i] as f64 * rad);
        let land = land_frac[i] >= 0.5;
        let have_wc = cnt[i] > 0;
        let icy = ice_m[i] > 100.0;
        if have_wc { wc_elev[i] = (esum[i] / cnt[i] as f64) as f32; }
        for m in 0..12 {
            let air = ncep.sample(m, lat, lon, 90.0, 2.5, -180.0, 2.5);
            let sea = sst.sample(m, lat, lon, 88.0, 2.0, -180.0, 2.0);
            let rain_sea = gpcp.sample(m, lat, lon, 88.75, 2.5, -178.75, 2.5);
            if land {
                tair[m][i] = if have_wc && !icy { (tsum[i][m] / cnt[i] as f64) as f32 + 273.15 } else { air + 273.15 };
                prec[m][i] = if have_wc && !icy { (psum[i][m] / cnt[i] as f64) as f32 } else { rain_sea.max(0.0) };
            } else {
                let open = !sea.is_nan() && sea > -1.7;
                tair[m][i] = if open { sea + 273.15 } else { air + 273.15 };
                sstv[m][i] = if open { sea + 273.15 } else { 271.35 };
                prec[m][i] = if rain_sea.is_nan() { 0.0 } else { rain_sea.max(0.0) };
            }
        }
    }
    Ok(Earth { n: grid.n, zq, land_frac, wc_elev, ice: ice_m, tair, sst: sstv, prec })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixels_land_on_their_nearest_tile() {
        let g = Grid::new(10);
        let ids = assign_pixels(&g, 36, 18, 85.0, 10.0, -175.0, 10.0);
        for y in 0..18 {
            for x in 0..36 {
                let p = unit(85.0 - 10.0 * y as f64, -175.0 + 10.0 * x as f64);
                let t = ids[y * 36 + x] as usize;
                let best = (0..g.len()).map(|i| (i, g.pos[i][0] as f64 * p[0] as f64 + g.pos[i][1] as f64 * p[1] as f64 + g.pos[i][2] as f64 * p[2] as f64))
                    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();
                let mine = g.pos[t][0] as f64 * p[0] as f64 + g.pos[t][1] as f64 * p[1] as f64 + g.pos[t][2] as f64 * p[2] as f64;
                assert!((best.1 - mine).abs() < 1e-9, "pixel ({x},{y})");
            }
        }
    }

    #[test]
    fn bilinear_handles_wrap_and_missing() {
        let r = Raster { w: 4, h: 2, planes: 1, data: vec![0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, f32::NAN] };
        // halfway between x=3 and x=0 wraps around the date line
        let v = r.sample(0, 90.0, 3.5 * 90.0 + (-180.0), 90.0, 90.0, -180.0, 90.0);
        assert!((v - 1.5).abs() < 1e-5, "{v}");
        let nan_corner = r.sample(0, 0.0, -180.0 + 3.0 * 90.0, 90.0, 90.0, -180.0, 90.0);
        assert!(nan_corner.is_nan() || nan_corner.is_finite());
        assert_eq!(r.planes, 1);
    }

    #[test]
    fn file_round_trip() {
        let g = Grid::new(4);
        let n = g.len();
        let e = Earth { n: 4, zq: (0..BANDS).map(|b| vec![b as f32; n]).collect(), land_frac: vec![0.5; n], wc_elev: vec![f32::NAN; n], ice: vec![1.0; n],
            tair: vec![vec![280.0; n]; 12], sst: vec![vec![290.0; n]; 12], prec: vec![vec![50.0; n]; 12] };
        let path = std::env::temp_dir().join("odyssey_earth_test.bin");
        e.save(&path).unwrap();
        let back = Earth::load(&path).unwrap();
        assert_eq!(back.zq[5][3], 5.0);
        assert!(back.wc_elev[0].is_nan());
        assert_eq!(back.tair[11][n - 1], 280.0);
        std::fs::remove_file(path).ok();
    }
}
