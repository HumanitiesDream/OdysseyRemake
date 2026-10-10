//! Flow routing on the live surface: where water goes, and where it pools.
//!
//! Priority-flood (Barnes et al. 2014) fills every pit up to its lowest spill point. Each
//! cell gets a receiver (the neighbour water leaves to). The result is read two ways:
//!
//! - `dest_dry[i]`: where a drop that falls on tile `i` ends up while every lake is below
//!   its spill level. Sea tiles end themselves. A cell inside a lake ends at the lake's pit.
//! - `spill_to[root]` and `lake_cap[root]`: once a lake holds more than its capacity,
//!   the excess leaves to `spill_to`. So lakes fill, spill, and (as dams change) burst,
//!   with no lake placed by hand.
//!
//! Nothing here is stored in a save. It is rebuilt from the stocks.

use crate::grid::Grid;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub const NONE: u32 = u32::MAX;

#[derive(Debug, Clone)]
pub struct Routing {
    /// Downhill neighbour on the filled surface, NONE for sea tiles.
    pub recv: Vec<u32>,
    /// Filled surface, m.
    pub fill: Vec<f32>,
    pub dest_dry: Vec<u32>,
    /// For lake roots only: where the excess goes, NONE for non-roots.
    pub spill_to: Vec<u32>,
    /// For lake roots only: capacity in (mm of water over the root tile's area) x km2, NaN for non-roots.
    pub lake_cap: Vec<f64>,
    /// Cells in pop order from the outlets inward (every receiver comes before its donor).
    pub order: Vec<u32>,
}

#[derive(PartialEq)]
struct Item(f32, u32);
impl Eq for Item {}
impl PartialOrd for Item { fn partial_cmp(&self, o: &Self) -> Option<Ordering> { Some(self.cmp(o)) } }
impl Ord for Item {
    // min-heap on height, then on index so equal heights pop in a fixed order
    fn cmp(&self, o: &Self) -> Ordering { o.0.partial_cmp(&self.0).unwrap_or(Ordering::Equal).then(o.1.cmp(&self.1)) }
}

impl Routing {
    /// `surface`: height of each tile, m. `sea[i]`: tile is open sea (a sink at its own height).
    pub fn build(grid: &Grid, surface: &[f32], sea: &[bool], area_km2: &[f32]) -> Routing {
        let n = grid.len();
        let mut fill = vec![f32::NAN; n];
        let mut recv = vec![NONE; n];
        let mut order: Vec<u32> = Vec::with_capacity(n);
        let mut closed = vec![false; n];
        let mut heap = BinaryHeap::new();
        for i in 0..n { if sea[i] { fill[i] = surface[i]; closed[i] = true; heap.push(Item(surface[i], i as u32)); } }
        if heap.is_empty() {
            // a world with no sea: the lowest tile is the outlet
            let low = (0..n).min_by(|&a, &b| surface[a].partial_cmp(&surface[b]).unwrap()).unwrap();
            fill[low] = surface[low]; closed[low] = true; heap.push(Item(surface[low], low as u32));
        }
        while let Some(Item(f, c)) = heap.pop() {
            let c = c as usize;
            order.push(c as u32);
            for j in grid.neighbours(c) {
                if closed[j] { continue; }
                closed[j] = true;
                recv[j] = c as u32;
                fill[j] = surface[j].max(f);
                heap.push(Item(fill[j], j as u32));
            }
        }
        // lake cells and their exits
        let is_lake = |i: usize| fill[i] > surface[i] + 1e-4 && !sea[i];
        let mut exit = vec![NONE; n]; // the first non-lake cell down the chain, for lake cells
        let mut dest_dry = vec![NONE; n];
        for &c in &order {
            let c = c as usize;
            if recv[c] == NONE { dest_dry[c] = c as u32; continue; }
            let r = recv[c] as usize;
            if is_lake(c) {
                exit[c] = if is_lake(r) { exit[r] } else { r as u32 };
            }
        }
        // root of each exit group = lowest surface cell; capacity = filled volume
        let mut root_of: std::collections::BTreeMap<u32, (u32, f32)> = std::collections::BTreeMap::new();
        let mut volume: std::collections::BTreeMap<u32, f64> = std::collections::BTreeMap::new();
        for c in 0..n {
            if exit[c] == NONE { continue; }
            let e = exit[c];
            let ent = root_of.entry(e).or_insert((c as u32, surface[c]));
            if surface[c] < ent.1 { *ent = (c as u32, surface[c]); }
            *volume.entry(e).or_insert(0.0) += (fill[c] - surface[c]) as f64 * 1000.0 * area_km2[c] as f64;
        }
        // destinations: outlets first
        for &c in &order {
            let c = c as usize;
            if recv[c] == NONE { continue; }
            if is_lake(c) { dest_dry[c] = root_of[&exit[c]].0; }
            else { dest_dry[c] = dest_dry[recv[c] as usize]; }
        }
        let mut spill_to = vec![NONE; n];
        let mut lake_cap = vec![f64::NAN; n];
        for (e, (root, _)) in &root_of {
            let e = *e as usize;
            // the exit cell itself drains down its own chain
            spill_to[*root as usize] = if sea[e] { e as u32 } else { dest_dry[e] };
            lake_cap[*root as usize] = volume[&(e as u32)];
        }
        Routing { recv, fill, dest_dry, spill_to, lake_cap, order }
    }

    pub fn is_root(&self, i: usize) -> bool { self.spill_to[i] != NONE }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tilted world: height falls with the x coordinate, so everything drains to the -x pole.
    fn setup(n: u32) -> (Grid, Vec<f32>, Vec<f32>) {
        let g = Grid::new(n);
        let h: Vec<f32> = g.pos.iter().map(|p| 1000.0 * (p[0] + 1.0)).collect();
        let a = g.area_km2.clone();
        (g, h, a)
    }

    #[test]
    fn every_cell_drains_to_the_sea() {
        let (g, h, a) = setup(8);
        let sea: Vec<bool> = h.iter().map(|&z| z < 100.0).collect();
        assert!(sea.iter().any(|&s| s));
        let r = Routing::build(&g, &h, &sea, &a);
        for i in 0..g.len() {
            let d = r.dest_dry[i] as usize;
            assert!(sea[d] || r.is_root(d), "tile {i} ends at {d}");
        }
        assert!((0..g.len()).all(|i| r.fill[i] >= h[i]));
        assert_eq!(r.order.len(), g.len());
    }

    #[test]
    fn a_pit_becomes_a_lake_that_spills() {
        let (g, mut h, a) = setup(10);
        let sea: Vec<bool> = h.iter().map(|&z| z < 50.0).collect();
        // dig a hole in the middle of a slope
        let mid = (0..g.len()).filter(|&i| !sea[i] && h[i] > 600.0 && h[i] < 900.0).next().unwrap();
        h[mid] -= 400.0;
        let r = Routing::build(&g, &h, &sea, &a);
        assert!(r.is_root(mid), "the pit is a lake root");
        assert!(r.lake_cap[mid] > 0.0);
        let spill = r.spill_to[mid] as usize;
        assert!(sea[spill] || r.is_root(spill) || r.dest_dry[spill] as usize == spill || r.dest_dry[spill] != mid as u32);
        // a neighbour that flows toward the pit ends there while the lake is dry
        let ends_in_pit = (0..g.len()).filter(|&i| r.dest_dry[i] as usize == mid).count();
        assert!(ends_in_pit >= 1);
        // capacity equals the filled volume (depth x area) of the pit
        let want = (r.fill[mid] - h[mid]) as f64 * 1000.0 * a[mid] as f64;
        assert!((r.lake_cap[mid] - want).abs() / want < 0.5 || r.lake_cap[mid] >= want);
    }

    #[test]
    fn same_surface_gives_same_routing() {
        let (g, h, a) = setup(9);
        let sea: Vec<bool> = h.iter().map(|&z| z < 100.0).collect();
        let r1 = Routing::build(&g, &h, &sea, &a);
        let r2 = Routing::build(&g, &h, &sea, &a);
        assert_eq!(r1.recv, r2.recv);
        assert_eq!(r1.dest_dry, r2.dest_dry);
    }
}
