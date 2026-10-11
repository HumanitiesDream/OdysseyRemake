//! The grid family.
//!
//! Fine grid: a Goldberg hex sphere, the dual of an icosahedron cut into n x n
//! triangles per face. It has 10 n^2 + 2 cells, 12 of them pentagons. Same geometry
//! as the v0.49 renderer grid (`grid.js`), ported to Rust.
//!
//! Coarse grids are the same kind of grid with a smaller n. Each fine cell belongs
//! to exactly one coarse cell (its nearest centre). Moving an amount up sums the
//! fine cells. Moving it down splits it by weight. Both are exact in f64, so a
//! flow handed between grids loses nothing to interpolation.
//!
//! Standard size: n160 (about 45 km, 256,002 cells). The 16 km level (n480) was dropped: it has no use at the
//! speeds the model runs.
//! Tile width is about sqrt(510e6 / (10 n^2 + 2)) km: n24 300 km, n48 150 km, n80 90 km, n160 45 km.

pub const EARTH_RADIUS_KM: f64 = 6371.0;
pub const N160: u32 = 160;

#[derive(Debug)]
pub struct Grid {
    pub n: u32,
    /// Unit vectors.
    pub pos: Vec<[f32; 3]>,
    /// Neighbour indices, -1 for an unused slot (pentagons have five).
    pub nbr: Vec<[i32; 6]>,
    pub area_km2: Vec<f32>,
    /// Radians. lat = asin(y), lon = atan2(-z, x), as in the renderer.
    pub lat: Vec<f32>,
    pub lon: Vec<f32>,
}

#[derive(Debug, PartialEq)]
pub enum GridError {
    /// A coarse cell with no fine members was asked to hold an amount.
    EmptyTarget(usize),
    /// Slice length does not match the grid.
    Length { want: usize, got: usize },
}

impl std::fmt::Display for GridError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GridError::EmptyTarget(c) => write!(f, "coarse cell {c} has no fine members"),
            GridError::Length { want, got } => write!(f, "length {got}, expected {want}"),
        }
    }
}
impl std::error::Error for GridError {}

pub fn cell_count(n: u32) -> usize { 10 * (n as usize) * (n as usize) + 2 }

fn add_nbr(nbr: &mut [[i32; 6]], a: usize, b: usize) {
    let row = &mut nbr[a];
    for s in row.iter_mut() {
        if *s == b as i32 { return; }
        if *s < 0 { *s = b as i32; return; }
    }
    unreachable!("a hex cell has at most six neighbours");
}

/// Solid angle of the spherical triangle abc (unit vectors), Van Oosterom and Strackee.
fn tri_solid_angle(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> f64 {
    let dot = |p: [f64; 3], q: [f64; 3]| p[0] * q[0] + p[1] * q[1] + p[2] * q[2];
    let cross = |p: [f64; 3], q: [f64; 3]| [p[1] * q[2] - p[2] * q[1], p[2] * q[0] - p[0] * q[2], p[0] * q[1] - p[1] * q[0]];
    let num = dot(a, cross(b, c)).abs();
    let den = 1.0 + dot(a, b) + dot(b, c) + dot(c, a);
    2.0 * num.atan2(den)
}

impl Grid {
    pub fn new(n: u32) -> Grid {
        assert!(n >= 1, "grid frequency must be at least 1");
        let nn = n as usize;
        let v_count = cell_count(n);
        let t = (1.0 + 5f64.sqrt()) / 2.0;
        let raw: [[f64; 3]; 12] = [
            [-1.0, t, 0.0], [1.0, t, 0.0], [-1.0, -t, 0.0], [1.0, -t, 0.0], [0.0, -1.0, t], [0.0, 1.0, t],
            [0.0, -1.0, -t], [0.0, 1.0, -t], [t, 0.0, -1.0], [t, 0.0, 1.0], [-t, 0.0, -1.0], [-t, 0.0, 1.0],
        ];
        let iv: Vec<[f64; 3]> = raw.iter().map(|v| { let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt(); [v[0] / l, v[1] / l, v[2] / l] }).collect();
        let faces: [[usize; 3]; 20] = [
            [0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11], [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8],
            [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9], [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1],
        ];
        let mut edge_idx = [[-1i64; 12]; 12];
        let mut ec = 0i64;
        for f in &faces {
            for k in 0..3 {
                let (a, b) = (f[k], f[(k + 1) % 3]);
                let (lo, hi) = (a.min(b), a.max(b));
                if edge_idx[lo][hi] < 0 { edge_idx[lo][hi] = ec; ec += 1; }
            }
        }
        let edge_base = 12usize;
        let face_base = 12 + 30 * (nn - 1);
        let per_face = (nn - 1) * nn.saturating_sub(2) / 2;
        let mut row_start = vec![0usize; nn + 2];
        for u in 1..nn { row_start[u + 1] = row_start[u] + (nn - 1 - u); }
        let ev = |x: usize, y: usize, s: usize| -> usize {
            let e = edge_idx[x.min(y)][x.max(y)] as usize;
            let start = edge_base + e * (nn - 1);
            if x < y { start + (s - 1) } else { start + (nn - 1 - s) }
        };

        let mut pos = vec![[0f64; 3]; v_count];
        let mut written = vec![false; v_count];
        let mut nbr = vec![[-1i32; 6]; v_count];
        let mut area = vec![0f64; v_count];

        // Positions first for the whole face, then triangles. Edge vertices are shared,
        // so a position is written once and every face sees the same value.
        let mut tris: Vec<[usize; 3]> = Vec::new();
        for (fi, f) in faces.iter().enumerate() {
            let (a, b, c) = (f[0], f[1], f[2]);
            let (pa, pb, pc) = (iv[a], iv[b], iv[c]);
            let fb = face_base + fi * per_face;
            let vid = |u: usize, v: usize| -> usize {
                if u == 0 && v == 0 { return a; }
                if u == nn { return b; }
                if v == nn { return c; }
                if v == 0 { return ev(a, b, u); }
                if u == 0 { return ev(a, c, v); }
                if u + v == nn { return ev(b, c, v); }
                fb + row_start[u] + (v - 1)
            };
            for u in 0..=nn {
                for v in 0..=(nn - u) {
                    let id = vid(u, v);
                    if written[id] { continue; }
                    written[id] = true;
                    // atan warp of the barycentrics evens out the cell sizes
                    let wa = (((nn - u - v) as f64) / nn as f64).atan();
                    let wb = (u as f64 / nn as f64).atan();
                    let wc = (v as f64 / nn as f64).atan();
                    let x = pa[0] * wa + pb[0] * wb + pc[0] * wc;
                    let y = pa[1] * wa + pb[1] * wb + pc[1] * wc;
                    let z = pa[2] * wa + pb[2] * wb + pc[2] * wc;
                    let l = (x * x + y * y + z * z).sqrt();
                    pos[id] = [x / l, y / l, z / l];
                }
            }
            for u in 0..nn {
                for v in 0..(nn - u) {
                    tris.push([vid(u, v), vid(u + 1, v), vid(u, v + 1)]);
                    if u + v < nn - 1 { tris.push([vid(u + 1, v), vid(u + 1, v + 1), vid(u, v + 1)]); }
                }
            }
        }
        debug_assert!(written.iter().all(|&w| w));
        let r2 = EARTH_RADIUS_KM * EARTH_RADIUS_KM;
        for tri in &tris {
            let w = tri_solid_angle(pos[tri[0]], pos[tri[1]], pos[tri[2]]) / 3.0 * r2;
            for k in 0..3 {
                area[tri[k]] += w;
                add_nbr(&mut nbr, tri[k], tri[(k + 1) % 3]);
                add_nbr(&mut nbr, tri[(k + 1) % 3], tri[k]);
            }
        }
        let lat = pos.iter().map(|p| p[1].asin() as f32).collect();
        let lon = pos.iter().map(|p| (-p[2]).atan2(p[0]) as f32).collect();
        Grid {
            n,
            pos: pos.iter().map(|p| [p[0] as f32, p[1] as f32, p[2] as f32]).collect(),
            nbr,
            area_km2: area.iter().map(|&a| a as f32).collect(),
            lat,
            lon,
        }
    }

    pub fn len(&self) -> usize { self.pos.len() }
    pub fn is_empty(&self) -> bool { self.pos.is_empty() }
    pub fn neighbours(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        self.nbr[i].iter().filter(|&&j| j >= 0).map(|&j| j as usize)
    }
    /// Total area in f64, from the f64 sum of the stored f32 areas.
    pub fn total_area_km2(&self) -> f64 { self.area_km2.iter().map(|&a| a as f64).sum() }

    fn dot(&self, i: usize, p: [f32; 3]) -> f64 {
        let q = self.pos[i];
        q[0] as f64 * p[0] as f64 + q[1] as f64 * p[1] as f64 + q[2] as f64 * p[2] as f64
    }
}

/// A coarse level of the grid family and the exact map from fine cells to it.
#[derive(Debug)]
pub struct Coarse {
    pub grid: Grid,
    /// Fine cell -> coarse cell.
    pub parent: Vec<u32>,
    /// CSR: members of coarse cell c are `members[start[c]..start[c+1]]`.
    pub start: Vec<u32>,
    pub members: Vec<u32>,
}

impl Coarse {
    pub fn new(fine: &Grid, coarse_n: u32) -> Coarse {
        assert!(coarse_n < fine.n, "a coarse grid needs a smaller frequency than the fine grid");
        let grid = Grid::new(coarse_n);
        let mut parent = vec![0u32; fine.len()];
        let mut cur = 0usize;
        for i in 0..fine.len() {
            let p = fine.pos[i];
            let mut best = grid.dot(cur, p);
            loop {
                let mut next = cur;
                for j in grid.neighbours(cur) {
                    let d = grid.dot(j, p);
                    if d > best { best = d; next = j; }
                }
                if next == cur { break; }
                cur = next;
            }
            parent[i] = cur as u32;
        }
        let cn = grid.len();
        let mut start = vec![0u32; cn + 1];
        for &p in &parent { start[p as usize + 1] += 1; }
        for c in 0..cn { start[c + 1] += start[c]; }
        let mut fill = start.clone();
        let mut members = vec![0u32; fine.len()];
        for (i, &p) in parent.iter().enumerate() {
            members[fill[p as usize] as usize] = i as u32;
            fill[p as usize] += 1;
        }
        Coarse { grid, parent, start, members }
    }

    pub fn len(&self) -> usize { self.grid.len() }
    pub fn is_empty(&self) -> bool { self.grid.is_empty() }
    pub fn members_of(&self, c: usize) -> &[u32] { &self.members[self.start[c] as usize..self.start[c + 1] as usize] }

    /// Sum fine amounts into coarse amounts. Exact in f64.
    pub fn sum_up(&self, fine: &[f64]) -> Result<Vec<f64>, GridError> {
        if fine.len() != self.parent.len() { return Err(GridError::Length { want: self.parent.len(), got: fine.len() }); }
        let mut out = vec![0f64; self.len()];
        for (i, &v) in fine.iter().enumerate() { out[self.parent[i] as usize] += v; }
        Ok(out)
    }

    /// Split coarse amounts over the fine members by weight. `weights` are per fine cell
    /// (for example area). A coarse cell whose members all have zero weight splits evenly.
    /// The member sum equals the coarse amount to f64 rounding.
    pub fn split_down(&self, coarse: &[f64], weights: &[f64]) -> Result<Vec<f64>, GridError> {
        if coarse.len() != self.len() { return Err(GridError::Length { want: self.len(), got: coarse.len() }); }
        if weights.len() != self.parent.len() { return Err(GridError::Length { want: self.parent.len(), got: weights.len() }); }
        let mut out = vec![0f64; self.parent.len()];
        for c in 0..self.len() {
            let m = self.members_of(c);
            if m.is_empty() {
                if coarse[c] != 0.0 { return Err(GridError::EmptyTarget(c)); }
                continue;
            }
            let wsum: f64 = m.iter().map(|&i| weights[i as usize]).sum();
            for &i in m {
                let share = if wsum > 0.0 { weights[i as usize] / wsum } else { 1.0 / m.len() as f64 };
                out[i as usize] = coarse[c] * share;
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_count_and_pentagons() {
        for n in [1u32, 2, 5, 12] {
            let g = Grid::new(n);
            assert_eq!(g.len(), 10 * (n as usize).pow(2) + 2);
            let pent = g.nbr.iter().filter(|r| r.iter().filter(|&&j| j >= 0).count() == 5).count();
            let hex = g.nbr.iter().filter(|r| r.iter().filter(|&&j| j >= 0).count() == 6).count();
            assert_eq!(pent, 12, "n={n}");
            assert_eq!(pent + hex, g.len(), "n={n}");
        }
    }

    #[test]
    fn neighbours_are_symmetric() {
        let g = Grid::new(9);
        for i in 0..g.len() {
            for j in g.neighbours(i) { assert!(g.neighbours(j).any(|k| k == i), "{i} -> {j}"); }
        }
    }

    #[test]
    fn areas_sum_to_the_sphere() {
        let g = Grid::new(20);
        let want = 4.0 * std::f64::consts::PI * EARTH_RADIUS_KM * EARTH_RADIUS_KM;
        assert!((g.total_area_km2() - want).abs() / want < 1e-5);
        let (mn, mx) = g.area_km2.iter().fold((f32::MAX, 0f32), |(a, b), &x| (a.min(x), b.max(x)));
        assert!(mx / mn < 1.5, "cell sizes spread {mn}..{mx}");
    }

    #[test]
    fn standard_size_is_48km() {
        let g = Grid::new(N160);
        assert_eq!(g.len(), 256_002);
        let mean = g.total_area_km2() / g.len() as f64;
        assert!((mean.sqrt() - 45.0).abs() < 6.0, "mean cell edge {}", mean.sqrt());
    }

    #[test]
    fn lat_lon_ranges() {
        let g = Grid::new(6);
        assert!(g.lat.iter().all(|&l| l.abs() <= std::f32::consts::FRAC_PI_2 + 1e-6));
        assert!(g.lon.iter().all(|&l| l.abs() <= std::f32::consts::PI + 1e-6));
    }

    #[test]
    fn coarse_parent_is_the_nearest_centre() {
        let fine = Grid::new(24);
        let c = Coarse::new(&fine, 6);
        for i in 0..fine.len() {
            let p = fine.pos[i];
            let best = (0..c.len()).max_by(|&a, &b| c.grid.dot(a, p).partial_cmp(&c.grid.dot(b, p)).unwrap()).unwrap();
            // an exact tie between two centres may go either way
            let gap = c.grid.dot(best, p) - c.grid.dot(c.parent[i] as usize, p);
            assert!(gap.abs() < 1e-9, "fine cell {i}: gap {gap}");
        }
        assert!((0..c.len()).all(|k| !c.members_of(k).is_empty()));
        assert_eq!(c.members.len(), fine.len());
    }

    #[test]
    fn up_and_down_lose_nothing() {
        let fine = Grid::new(24);
        let c = Coarse::new(&fine, 6);
        let amounts: Vec<f64> = (0..fine.len()).map(|i| ((i * 7919) % 101) as f64 * 0.37).collect();
        let up = c.sum_up(&amounts).unwrap();
        assert!((up.iter().sum::<f64>() - amounts.iter().sum::<f64>()).abs() < 1e-9);
        let w: Vec<f64> = fine.area_km2.iter().map(|&a| a as f64).collect();
        let down = c.split_down(&up, &w).unwrap();
        let again = c.sum_up(&down).unwrap();
        for k in 0..c.len() { assert!((again[k] - up[k]).abs() <= 1e-12 * up[k].abs().max(1.0)); }
        assert!((down.iter().sum::<f64>() - amounts.iter().sum::<f64>()).abs() < 1e-9);
    }

    #[test]
    fn length_errors_are_reported() {
        let fine = Grid::new(12);
        let c = Coarse::new(&fine, 3);
        assert!(matches!(c.sum_up(&[1.0]), Err(GridError::Length { .. })));
    }
}
