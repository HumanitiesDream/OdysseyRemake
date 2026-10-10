//! The world: grid, stock arrays, and the books.
//!
//! Flows are applied here and nowhere else. Each flow takes amount/area from the
//! source tile and adds amount/area to the target tile. So the total (density x area)
//! per group changes only by what crosses a boundary account. `audit` proves it.
//!
//! No clamping. A stock that goes negative is reported as a violation, because a
//! silent clamp would hide a leak.

use crate::flows::{Endpoint, FlowOut};
use crate::grid::Grid;
use crate::module::ModuleSpec;
use crate::stocks::{BoundaryId, Group, Registry, StockId, GROUPS, NGROUPS};
use crate::units::Clock;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum WorldError {
    UndeclaredRead { module: String, stock: String },
    UndeclaredWrite { module: String, what: String },
    GroupMismatch { module: String, from: String, to: String },
    BadAmount { module: String },
    BadTile { module: String, tile: u32 },
    TransitImbalance { module: String, group: &'static str, residual: f64 },
    Unknown(String),
    Panicked { module: String },
    Content(String),
    Save(String),
}

impl std::fmt::Display for WorldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use WorldError::*;
        match self {
            UndeclaredRead { module, stock } => write!(f, "module {module} read {stock} without declaring it"),
            UndeclaredWrite { module, what } => write!(f, "module {module} wrote {what} without declaring it"),
            GroupMismatch { module, from, to } => write!(f, "module {module} moved {from} into {to}, which are different kinds of matter"),
            BadAmount { module } => write!(f, "module {module} returned a negative or non-finite amount"),
            BadTile { module, tile } => write!(f, "module {module} used tile {tile}, outside the grid"),
            TransitImbalance { module, group, residual } => write!(f, "module {module} left {residual:e} of {group} in transit"),
            Unknown(s) => write!(f, "unknown name: {s}"),
            Panicked { module } => write!(f, "module {module} panicked"),
            Content(s) => write!(f, "content error: {s}"),
            Save(s) => write!(f, "save error: {s}"),
        }
    }
}
impl std::error::Error for WorldError {}

#[derive(Debug, Clone)]
pub struct Violation { pub month: u64, pub stock: String, pub tile: usize, pub value: f32 }

#[derive(Debug, Clone)]
pub struct AuditRow {
    pub group: Group,
    pub total: f64,
    pub initial: f64,
    /// Net amount that came in from outside accounts since the ledger started.
    pub boundary_net: f64,
    /// (total - initial - boundary_net) / max(|initial|, |total|, tiny). Should be near f32 rounding.
    pub residual: f64,
}

/// Values this far below zero are rounding (plus a share of the old value, see `commit`).
/// Anything lower is a violation.
pub const NEG_EPS: f64 = 1e-6;

/// A module's declared access, resolved to ids.
#[derive(Clone, Debug)]
pub struct Resolved {
    pub name: String,
    pub phase: crate::module::Phase,
    pub period: u32,
    pub offset: u32,
    pub reads: Vec<StockId>,
    pub writes: Vec<StockId>,
    pub bounds: Vec<BoundaryId>,
}

impl Resolved {
    pub fn new(spec: &ModuleSpec, reg: &Registry) -> Result<Resolved, WorldError> {
        let st = |n: &String| reg.stock(n).ok_or_else(|| WorldError::Unknown(format!("stock {n} (module {})", spec.name)));
        Ok(Resolved {
            name: spec.name.clone(),
            phase: spec.phase,
            period: spec.period_months.max(1),
            offset: spec.offset,
            reads: spec.reads.iter().map(st).collect::<Result<_, _>>()?,
            writes: spec.writes.iter().map(st).collect::<Result<_, _>>()?,
            bounds: spec.boundaries.iter().map(|n| reg.boundary(n).ok_or_else(|| WorldError::Unknown(format!("boundary {n} (module {})", spec.name)))).collect::<Result<_, _>>()?,
        })
    }
    pub fn due(&self, month_index: u64) -> bool { month_index % self.period as u64 == self.offset as u64 % self.period as u64 }
}

pub struct World {
    pub grid: Arc<Grid>,
    pub reg: Registry,
    /// One array per stock, indexed by `StockId`. Density per km2.
    pub cur: Vec<Vec<f32>>,
    pub clock: Clock,
    pub seed: u64,
    pub violations: Vec<Violation>,
    pub violation_count: u64,
    pub log: Vec<String>,
    /// Rounding left over when a stock was last stored as f32 (true value = cur + resid).
    /// Carried into the next step, so f32 storage cannot leak or gain matter over time.
    pub(crate) resid: Vec<Vec<f32>>,
    pub(crate) initial: [f64; NGROUPS],
    pub(crate) boundary_net: [f64; NGROUPS],
}

impl World {
    pub fn new(grid: Arc<Grid>, reg: Registry, seed: u64, start_year: i64) -> World {
        let n = grid.len();
        let ns = reg.stocks.len();
        World {
            cur: vec![vec![0f32; n]; ns],
            resid: vec![vec![0f32; n]; ns],
            grid, reg, clock: Clock::new(start_year), seed,
            violations: Vec::new(), violation_count: 0, log: Vec::new(),
            initial: [0.0; NGROUPS], boundary_net: [0.0; NGROUPS],
        }
    }

    pub fn stock(&self, id: StockId) -> &[f32] { &self.cur[id.0 as usize] }
    pub fn stock_mut(&mut self, id: StockId) -> &mut [f32] { &mut self.cur[id.0 as usize] }

    /// Totals per group in f64: sum of density x area.
    pub fn totals(&self) -> [f64; NGROUPS] {
        let mut t = [0f64; NGROUPS];
        for (s, def) in self.reg.stocks.iter().enumerate() {
            let sum: f64 = self.cur[s].iter().zip(self.resid[s].iter()).zip(self.grid.area_km2.iter()).map(|((&v, &r), &a)| (v as f64 + r as f64) * a as f64).sum();
            t[def.group.idx()] += sum;
        }
        t
    }

    /// Start the books from the current state. Call after setting initial conditions.
    pub fn reset_ledger(&mut self) {
        self.initial = self.totals();
        self.boundary_net = [0.0; NGROUPS];
    }

    pub fn audit(&self) -> Vec<AuditRow> {
        let now = self.totals();
        GROUPS.iter().map(|&g| {
            let i = g.idx();
            let scale = self.initial[i].abs().max(now[i].abs()).max(1e-30);
            AuditRow { group: g, total: now[i], initial: self.initial[i], boundary_net: self.boundary_net[i],
                residual: (now[i] - self.initial[i] - self.boundary_net[i]) / scale }
        }).collect()
    }

    /// Check one module's output without changing anything.
    pub fn validate(&self, r: &Resolved, out: &FlowOut) -> Result<(), WorldError> {
        let me = || r.name.clone();
        if out.bad > 0 { return Err(WorldError::BadAmount { module: me() }); }
        let n = self.grid.len() as u32;
        let mut transit = [0f64; NGROUPS];
        let mut transit_gross = [0f64; NGROUPS];
        for ((from, to), entries) in &out.lanes {
            let mut groups = [None, None];
            for (k, ep) in [from, to].into_iter().enumerate() {
                match *ep {
                    Endpoint::Stock(s) => {
                        if !r.writes.contains(&s) { return Err(WorldError::UndeclaredWrite { module: me(), what: self.reg.def(s).name.clone() }); }
                        groups[k] = Some(self.reg.def(s).group);
                    }
                    Endpoint::Boundary(b) => {
                        if !r.bounds.contains(&b) { return Err(WorldError::UndeclaredWrite { module: me(), what: format!("boundary {}", self.reg.bdef(b).name) }); }
                        groups[k] = Some(self.reg.bdef(b).group);
                    }
                    Endpoint::Transit => {}
                }
            }
            let g = match (groups[0], groups[1]) {
                (Some(a), Some(b)) if a != b => {
                    return Err(WorldError::GroupMismatch { module: me(), from: format!("{from:?}"), to: format!("{to:?}") });
                }
                (Some(a), _) | (_, Some(a)) => a,
                (None, None) => return Err(WorldError::Unknown(format!("module {} moved transit to transit", r.name))),
            };
            for e in entries {
                let need_src = matches!(from, Endpoint::Stock(_));
                let need_dst = matches!(to, Endpoint::Stock(_));
                if (need_src && e.src >= n) || (need_dst && e.dst >= n) {
                    return Err(WorldError::BadTile { module: me(), tile: if need_src && e.src >= n { e.src } else { e.dst } });
                }
                if *to == Endpoint::Transit { transit[g.idx()] += e.amount; transit_gross[g.idx()] += e.amount; }
                if *from == Endpoint::Transit { transit[g.idx()] -= e.amount; transit_gross[g.idx()] += e.amount; }
            }
        }
        for ((from, to), d) in &out.dense {
            if d.len() != n as usize { return Err(WorldError::BadTile { module: me(), tile: d.len() as u32 }); }
            let mut groups = [None, None];
            for (k, ep) in [from, to].into_iter().enumerate() {
                match *ep {
                    Endpoint::Stock(s) => {
                        if !r.writes.contains(&s) { return Err(WorldError::UndeclaredWrite { module: me(), what: self.reg.def(s).name.clone() }); }
                        groups[k] = Some(self.reg.def(s).group);
                    }
                    Endpoint::Boundary(b) => {
                        if !r.bounds.contains(&b) { return Err(WorldError::UndeclaredWrite { module: me(), what: format!("boundary {}", self.reg.bdef(b).name) }); }
                        groups[k] = Some(self.reg.bdef(b).group);
                    }
                    Endpoint::Transit => return Err(WorldError::Unknown(format!("module {} used transit in a dense lane", r.name))),
                }
            }
            if groups[0].is_none() && groups[1].is_none() { return Err(WorldError::Unknown(format!("module {} dense lane without a stock", r.name))); }
            if let (Some(a), Some(b)) = (groups[0], groups[1]) {
                if a != b { return Err(WorldError::GroupMismatch { module: me(), from: format!("{from:?}"), to: format!("{to:?}") }); }
            }
            if d.iter().any(|&x| !(x >= 0.0) || !x.is_finite()) { return Err(WorldError::BadAmount { module: me() }); }
        }
        for g in GROUPS {
            let (res, gross) = (transit[g.idx()], transit_gross[g.idx()]);
            if res.abs() > 1e-9 * gross + 1e-12 {
                return Err(WorldError::TransitImbalance { module: me(), group: g.name(), residual: res });
            }
        }
        Ok(())
    }

    /// Apply validated module outputs, in the order given. Stock by stock, so only one
    /// f64 scratch array exists at a time.
    pub fn apply(&mut self, outs: &[&FlowOut]) {
        let n = self.grid.len();
        let ns = self.cur.len();
        let mut touched = vec![false; ns];
        for out in outs {
            for ((from, to), entries) in &out.lanes {
                for (ep, is_from) in [(from, true), (to, false)] {
                    match *ep {
                        Endpoint::Stock(s) => touched[s.0 as usize] = true,
                        Endpoint::Boundary(b) => {
                            let g = self.reg.bdef(b).group.idx();
                            let sum: f64 = entries.iter().map(|e| e.amount).sum();
                            self.boundary_net[g] += if is_from { sum } else { -sum };
                        }
                        Endpoint::Transit => {}
                    }
                }
            }
            for ((from, to), d) in &out.dense {
                for (ep, is_from) in [(from, true), (to, false)] {
                    match *ep {
                        Endpoint::Stock(s) => touched[s.0 as usize] = true,
                        Endpoint::Boundary(b) => {
                            let g = self.reg.bdef(b).group.idx();
                            let sum: f64 = d.iter().zip(self.grid.area_km2.iter()).map(|(&x, &a)| x as f64 * a as f64).sum();
                            self.boundary_net[g] += if is_from { sum } else { -sum };
                        }
                        Endpoint::Transit => {}
                    }
                }
            }
        }
        let grid = self.grid.clone();
        let area = &grid.area_km2;
        let mut scratch = vec![0f64; n];
        for s in 0..ns {
            if !touched[s] { continue; }
            scratch.iter_mut().for_each(|v| *v = 0.0);
            let me = Endpoint::Stock(StockId(s as u16));
            for out in outs {
                for ((from, to), entries) in &out.lanes {
                    if *from == me { for e in entries { scratch[e.src as usize] -= e.amount / area[e.src as usize] as f64; } }
                    if *to == me { for e in entries { scratch[e.dst as usize] += e.amount / area[e.dst as usize] as f64; } }
                }
                for ((from, to), d) in &out.dense {
                    if *from == me { for (sc, &x) in scratch.iter_mut().zip(d.iter()) { *sc -= x as f64; } }
                    if *to == me { for (sc, &x) in scratch.iter_mut().zip(d.iter()) { *sc += x as f64; } }
                }
            }
            self.commit_stock(s, &scratch);
        }
    }

    fn commit_stock(&mut self, s: usize, delta: &[f64]) {
        let month = self.clock.month_index;
        let name = &self.reg.stocks[s].name;
        let (cur, resid) = (&mut self.cur[s], &mut self.resid[s]);
        for i in 0..cur.len() {
            if delta[i] == 0.0 { continue; }
            let v = cur[i] as f64 + resid[i] as f64 + delta[i];
            // f32 rounding of the old value is the most a module can have overdrawn by mistake
            let tol = NEG_EPS + 4e-7 * (cur[i] as f64).abs();
            if v < 0.0 && v >= -tol {
                // rounding debt: modules see zero, the books keep the exact amount
                cur[i] = 0.0;
                resid[i] = v as f32;
                continue;
            }
            if v < -tol {
                self.violation_count += 1;
                if self.violations.len() < 100 {
                    self.violations.push(Violation { month, stock: name.clone(), tile: i, value: v as f32 });
                }
            }
            cur[i] = v as f32;
            resid[i] = (v - cur[i] as f64) as f32;
        }
    }

    /// FNV-1a over month, seed, stock names and every stock bit, plus the ledger.
    pub fn fingerprint(&self) -> u64 {
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        let mut feed = |b: &[u8]| { for &x in b { h = (h ^ x as u64).wrapping_mul(0x0000_0100_0000_01b3); } };
        feed(&self.clock.month_index.to_le_bytes());
        feed(&self.seed.to_le_bytes());
        for (s, def) in self.reg.stocks.iter().enumerate() {
            feed(def.name.as_bytes());
            for &v in &self.cur[s] { feed(&v.to_bits().to_le_bytes()); }
            for &v in &self.resid[s] { feed(&v.to_bits().to_le_bytes()); }
        }
        for v in self.boundary_net { feed(&v.to_bits().to_le_bytes()); }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module::{ModuleSpec, Phase};
    use crate::stocks::Storage;

    fn tiny() -> (World, StockId, StockId, BoundaryId) {
        let mut reg = Registry::new();
        let a = reg.add_stock("a", "mm", Group::Water, Storage::Exact);
        let b = reg.add_stock("b", "mm", Group::Water, Storage::Exact);
        let sun = reg.add_boundary("sun", Group::Water);
        let mut w = World::new(Arc::new(Grid::new(4)), reg, 1, 0);
        w.stock_mut(a).iter_mut().for_each(|v| *v = 10.0);
        w.reset_ledger();
        (w, a, b, sun)
    }
    fn spec(w: &World, writes: &[&str], bounds: &[&str]) -> Resolved {
        Resolved::new(&ModuleSpec::new("t", Phase::Water).writes(writes).boundaries(bounds), &w.reg).unwrap()
    }

    #[test]
    fn local_flow_moves_density_and_keeps_total() {
        let (mut w, a, b, _) = tiny();
        let r = spec(&w, &["a", "b"], &[]);
        let mut f = FlowOut::new();
        let area = w.grid.area_km2[5] as f64;
        f.local(a, b, 5, 4.0 * area);
        w.validate(&r, &f).unwrap();
        w.apply(&[&f]);
        assert!((w.stock(a)[5] - 6.0).abs() < 1e-5);
        assert!((w.stock(b)[5] - 4.0).abs() < 1e-5);
        assert!(w.audit().iter().all(|r| r.residual.abs() < 1e-6));
    }

    #[test]
    fn lateral_flow_between_tiles_of_different_area_conserves() {
        let (mut w, a, _, _) = tiny();
        let r = spec(&w, &["a"], &[]);
        let j = w.grid.neighbours(0).next().unwrap();
        let mut f = FlowOut::new();
        f.lateral(a, 0, j, 3.0 * w.grid.area_km2[0] as f64);
        w.validate(&r, &f).unwrap();
        w.apply(&[&f]);
        assert!(w.audit()[0].residual.abs() < 1e-6);
    }

    #[test]
    fn boundary_flow_is_tallied() {
        let (mut w, a, _, sun) = tiny();
        let r = spec(&w, &["a"], &["sun"]);
        let mut f = FlowOut::new();
        f.push(Endpoint::Boundary(sun), Endpoint::Stock(a), 0, 0, 100.0);
        w.validate(&r, &f).unwrap();
        w.apply(&[&f]);
        let row = &w.audit()[0];
        assert!((row.boundary_net - 100.0).abs() < 1e-9);
        assert!(row.residual.abs() < 1e-6);
    }

    #[test]
    fn undeclared_write_is_refused() {
        let (w, a, b, _) = tiny();
        let r = spec(&w, &["a"], &[]);
        let mut f = FlowOut::new();
        f.local(a, b, 0, 1.0);
        assert!(matches!(w.validate(&r, &f), Err(WorldError::UndeclaredWrite { .. })));
    }

    #[test]
    fn mixing_kinds_of_matter_is_refused() {
        let mut reg = Registry::new();
        let a = reg.add_stock("a", "mm", Group::Water, Storage::Exact);
        let h = reg.add_stock("h", "MJ/m2", Group::Heat, Storage::Exact);
        let w = World::new(Arc::new(Grid::new(3)), reg, 1, 0);
        let r = Resolved::new(&ModuleSpec::new("t", Phase::Water).writes(&["a", "h"]), &w.reg).unwrap();
        let mut f = FlowOut::new();
        f.local(a, h, 0, 1.0);
        assert!(matches!(w.validate(&r, &f), Err(WorldError::GroupMismatch { .. })));
    }

    #[test]
    fn unbalanced_transit_is_refused() {
        let (w, a, _, _) = tiny();
        let r = spec(&w, &["a"], &[]);
        let mut f = FlowOut::new();
        f.push(Endpoint::Stock(a), Endpoint::Transit, 0, 0, 5.0);
        f.push(Endpoint::Transit, Endpoint::Stock(a), 0, 1, 4.0);
        assert!(matches!(w.validate(&r, &f), Err(WorldError::TransitImbalance { .. })));
        let mut ok = FlowOut::new();
        ok.push(Endpoint::Stock(a), Endpoint::Transit, 0, 0, 5.0);
        ok.push(Endpoint::Transit, Endpoint::Stock(a), 0, 1, 5.0);
        assert!(w.validate(&r, &ok).is_ok());
    }

    #[test]
    fn overdraw_is_a_violation_not_a_clamp() {
        let (mut w, a, b, _) = tiny();
        let r = spec(&w, &["a", "b"], &[]);
        let mut f = FlowOut::new();
        f.local(a, b, 2, 20.0 * w.grid.area_km2[2] as f64);
        w.validate(&r, &f).unwrap();
        w.apply(&[&f]);
        assert_eq!(w.violation_count, 1);
        assert!(w.stock(a)[2] < 0.0);
    }

    #[test]
    fn rounding_sized_overdraw_is_kept_as_debt_not_flagged() {
        let (mut w, a, b, _) = tiny();
        let r = spec(&w, &["a", "b"], &[]);
        let area = w.grid.area_km2[1] as f64;
        let mut f = FlowOut::new();
        f.local(a, b, 1, (10.0 + 5e-7) * area); // 5e-7 past empty, below f32 rounding of 10
        w.validate(&r, &f).unwrap();
        w.apply(&[&f]);
        assert_eq!(w.violation_count, 0);
        assert_eq!(w.stock(a)[1], 0.0);
        assert!(w.audit()[0].residual.abs() < 1e-12, "the debt stays on the books");
    }

    #[test]
    fn out_of_range_tile_is_refused() {
        let (w, a, b, _) = tiny();
        let r = spec(&w, &["a", "b"], &[]);
        let mut f = FlowOut::new();
        f.push(Endpoint::Stock(a), Endpoint::Stock(b), 0, 9_999_999, 1.0);
        assert!(matches!(w.validate(&r, &f), Err(WorldError::BadTile { .. })));
    }
}
