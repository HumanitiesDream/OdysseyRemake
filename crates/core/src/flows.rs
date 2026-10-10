//! Flows. A module returns amounts moved between stocks. It never edits a stock.
//!
//! An amount is in (density unit x km2), the same on both ends. The world turns it
//! into a density change on each side by dividing by that tile's area.
//!
//! Endpoints:
//! - `Stock`: a stock on a tile.
//! - `Boundary`: an outside account (sun, space). Tallied, so the budget still closes.
//! - `Transit`: a module-local junction for splitting and joining flows across grids.
//!   Per group, what goes in must come out, or the world rejects the module's output.

use crate::stocks::{BoundaryId, StockId};
use std::collections::BTreeMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Endpoint { Stock(StockId), Boundary(BoundaryId), Transit }

#[derive(Copy, Clone, Debug)]
pub struct Entry {
    /// Tile the amount leaves (used when `from` is a stock).
    pub src: u32,
    /// Tile the amount enters (used when `to` is a stock).
    pub dst: u32,
    pub amount: f64,
}

/// What happened, as a readout. Never a cause: events are written down after the stocks moved.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EventKind { Quake, Tsunami, Eruption, Landslide, Fire, Flood, LakeBurst, Storm, Drought, Calving, Crash, Outbreak }

#[derive(Copy, Clone, Debug)]
pub struct Event { pub month: u64, pub kind: EventKind, pub tile: u32, pub size: f32 }

#[derive(Default, Debug)]
pub struct FlowOut {
    pub events: Vec<Event>,
    /// Keyed by (from, to). A BTreeMap keeps the apply order the same on every run.
    pub(crate) lanes: BTreeMap<(Endpoint, Endpoint), Vec<Entry>>,
    /// Same-tile flows as one density per tile, for modules that touch every tile.
    /// The same f32 leaves one stock and enters the other, so these close exactly.
    pub(crate) dense: BTreeMap<(Endpoint, Endpoint), Vec<f32>>,
    pub(crate) bad: u32,
}

impl FlowOut {
    pub fn new() -> Self { Self::default() }

    /// Move `amount` (>= 0) from `from` at tile `src` to `to` at tile `dst`.
    /// Zero is skipped. A negative or non-finite amount marks the whole output bad.
    pub fn push(&mut self, from: Endpoint, to: Endpoint, src: usize, dst: usize, amount: f64) {
        if !amount.is_finite() || amount < 0.0 { self.bad += 1; return; }
        if amount == 0.0 { return; }
        self.lanes.entry((from, to)).or_default().push(Entry { src: src as u32, dst: dst as u32, amount });
    }

    /// Move between two stocks on the same tile.
    pub fn local(&mut self, from: StockId, to: StockId, tile: usize, amount: f64) {
        self.push(Endpoint::Stock(from), Endpoint::Stock(to), tile, tile, amount);
    }

    /// Move between tiles of one stock.
    pub fn lateral(&mut self, stock: StockId, src: usize, dst: usize, amount: f64) {
        self.push(Endpoint::Stock(stock), Endpoint::Stock(stock), src, dst, amount);
    }

    /// A per-tile lane of density flow (in the stocks' unit) from `from` to `to` on each tile.
    /// Write non-negative numbers into it. Both ends need a stock on the same tile, except
    /// that one end may be a boundary account. Allocated zeroed on first use.
    pub fn dense(&mut self, from: Endpoint, to: Endpoint, tiles: usize) -> &mut [f32] {
        self.dense.entry((from, to)).or_insert_with(|| vec![0f32; tiles])
    }
    pub fn dense_local(&mut self, from: StockId, to: StockId, tiles: usize) -> &mut [f32] {
        self.dense(Endpoint::Stock(from), Endpoint::Stock(to), tiles)
    }

    pub fn is_empty(&self) -> bool { self.lanes.is_empty() && self.dense.is_empty() }
    pub fn entry_count(&self) -> usize { self.lanes.values().map(|v| v.len()).sum::<usize>() + self.dense.values().map(|v| v.iter().filter(|&&x| x != 0.0).count()).sum::<usize>() }
    pub fn gross(&self) -> f64 { self.lanes.values().flat_map(|v| v.iter()).map(|e| e.amount).sum() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zero_is_skipped_and_negative_is_bad() {
        let mut f = FlowOut::new();
        f.local(StockId(0), StockId(1), 3, 0.0);
        assert!(f.is_empty());
        f.local(StockId(0), StockId(1), 3, -1.0);
        f.local(StockId(0), StockId(1), 3, f64::NAN);
        assert_eq!(f.bad, 2);
        f.local(StockId(0), StockId(1), 3, 2.5);
        assert_eq!(f.entry_count(), 1);
        assert_eq!(f.gross(), 2.5);
    }
}
