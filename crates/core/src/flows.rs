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

#[derive(Default, Debug)]
pub struct FlowOut {
    /// Keyed by (from, to). A BTreeMap keeps the apply order the same on every run.
    pub(crate) lanes: BTreeMap<(Endpoint, Endpoint), Vec<Entry>>,
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

    pub fn is_empty(&self) -> bool { self.lanes.is_empty() }
    pub fn entry_count(&self) -> usize { self.lanes.values().map(|v| v.len()).sum() }
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
