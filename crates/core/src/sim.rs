//! The scheduler.
//!
//! Each month, every due module reads the same start-of-month stocks, so order cannot
//! bias the result and the due modules can run on separate threads. Their flows are
//! then checked and applied in a fixed order (phase, then registration), so thread
//! timing cannot change the result either.
//!
//! The plan asks for "modules that share no writes run at the same time". Flows are
//! additive deltas here, so write conflicts cannot happen and every due module may
//! run at once. What can still go wrong is two modules each drawing most of one
//! stock. That shows up as a negative-stock violation, never as a silent clamp.
//!
//! A module that returns an error or panics is logged and skipped for the month.
//! Its flows are dropped. The run goes on.

use crate::flows::FlowOut;
use crate::module::{Detail, Module, StepCtx, View};
use crate::world::{Resolved, World, WorldError};
use std::panic::{catch_unwind, AssertUnwindSafe};

pub struct Slot {
    pub module: Box<dyn Module>,
    pub res: Resolved,
    pub last_run: Option<u64>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Parallel {
    Off,
    On,
    /// On when more than one module is due and the grid has at least this many cells.
    Auto(usize),
}

pub struct Sim {
    pub world: World,
    pub slots: Vec<Slot>,
    pub parallel: Parallel,
    pub detail: Detail,
    /// Count of months a module was skipped because of an error.
    pub skipped: u64,
    /// When set, running totals of every flow lane by module, in (density unit x km2). For diagnosis.
    pub trace: Option<std::collections::BTreeMap<String, f64>>,
}

type Outcome = (usize, Result<FlowOut, WorldError>);

impl Sim {
    pub fn new(world: World) -> Sim { Sim { world, slots: Vec::new(), parallel: Parallel::Auto(5000), detail: Detail::Full, skipped: 0, trace: None } }

    pub fn add_module(&mut self, mut module: Box<dyn Module>) -> Result<(), WorldError> {
        let spec = module.spec();
        if self.slots.iter().any(|s| s.res.name == spec.name) { return Err(WorldError::Unknown(format!("module {} is already registered", spec.name))); }
        let res = Resolved::new(&spec, &self.world.reg)?;
        module.bind(&self.world.reg, &self.world.grid)?;
        self.slots.push(Slot { module, res, last_run: None });
        Ok(())
    }

    fn use_threads(&self, due: usize) -> bool {
        match self.parallel {
            Parallel::Off => false,
            Parallel::On => due > 1,
            Parallel::Auto(min) => due > 1 && self.world.grid.len() >= min,
        }
    }

    /// Run one month: step the due modules, apply their flows, advance the clock.
    pub fn step_month(&mut self) {
        let month = self.world.clock.month_index;
        let mut due: Vec<usize> = (0..self.slots.len()).filter(|&i| self.slots[i].res.due(month)).collect();
        due.sort_by_key(|&i| (self.slots[i].res.phase, i));
        let threads = self.use_threads(due.len());
        let detail = self.detail;

        let mut outcomes: Vec<Outcome> = Vec::with_capacity(due.len());
        {
            let Sim { world, slots, .. } = self;
            let world: &World = world;
            let ctx_for = |dt: u32| StepCtx {
                month_index: month, month_of_year: world.clock.month_of_year(), year: world.clock.year(),
                dt_months: dt, detail, seed: world.seed, grid: &world.grid,
            };
            // Disjoint &mut access to the due slots.
            let mut picked: Vec<(usize, &mut Slot)> = Vec::new();
            for (i, s) in slots.iter_mut().enumerate() { if due.contains(&i) { picked.push((i, s)); } }
            picked.sort_by_key(|(i, _)| due.iter().position(|d| d == i));

            let run = |i: usize, slot: &mut Slot| -> Outcome {
                let dt = slot.last_run.map(|l| (month - l) as u32).unwrap_or(slot.res.period);
                slot.last_run = Some(month);
                let ctx = ctx_for(dt);
                let view = View { world, allowed: &slot.res.reads, module: &slot.res.name };
                let name = slot.res.name.clone();
                let mut out = FlowOut::new();
                let r = catch_unwind(AssertUnwindSafe(|| slot.module.step(&ctx, &view, &mut out)));
                match r {
                    Ok(Ok(())) => (i, Ok(out)),
                    Ok(Err(e)) => (i, Err(e)),
                    Err(_) => (i, Err(WorldError::Panicked { module: name })),
                }
            };

            if threads {
                std::thread::scope(|sc| {
                    let handles: Vec<_> = picked.into_iter().map(|(i, slot)| { let run = &run; sc.spawn(move || run(i, slot)) }).collect();
                    for (h, &i) in handles.into_iter().zip(due.iter()) {
                        outcomes.push(h.join().unwrap_or_else(|_| (i, Err(WorldError::Panicked { module: "thread".into() }))));
                    }
                });
            } else {
                for (i, slot) in picked { outcomes.push(run(i, slot)); }
            }
        }

        let mut good: Vec<FlowOut> = Vec::new();
        for (i, outcome) in outcomes {
            let verdict = outcome.and_then(|out| self.world.validate(&self.slots[i].res, &out).map(|_| out));
            match verdict {
                Ok(out) => {
                    if let Some(tr) = self.trace.as_mut() {
                        let reg = &self.world.reg;
                        let name = |e: &crate::flows::Endpoint| match e { crate::flows::Endpoint::Stock(s) => reg.def(*s).name.clone(), crate::flows::Endpoint::Boundary(b) => format!("[{}]", reg.bdef(*b).name), crate::flows::Endpoint::Transit => "(transit)".into() };
                        for ((f, t), v) in &out.lanes { *tr.entry(format!("{}: {} > {}", self.slots[i].res.name, name(f), name(t))).or_insert(0.0) += v.iter().map(|e| e.amount).sum::<f64>(); }
                        for ((f, t), v) in &out.dense { *tr.entry(format!("{}: {} > {}", self.slots[i].res.name, name(f), name(t))).or_insert(0.0) += v.iter().zip(self.world.grid.area_km2.iter()).map(|(&d, &a)| d as f64 * a as f64).sum::<f64>(); }
                    }
                    good.push(out)
                }
                Err(e) => {
                    self.skipped += 1;
                    if self.world.log.len() < 1000 { self.world.log.push(format!("month {}: {}", month, e)); }
                }
            }
        }
        for o in &good { for e in &o.events { self.world.event_count += 1; if self.world.events.len() < 200_000 { self.world.events.push(*e); } } }
        let refs: Vec<&FlowOut> = good.iter().collect();
        self.world.apply(&refs);
        self.world.clock.month_index += 1;
    }

    pub fn run_months(&mut self, months: u64) { for _ in 0..months { self.step_month(); } }
    pub fn run_years(&mut self, years: u64) { self.run_months(years * 12); }

    /// World fingerprint plus module state, for proving speed-ups and saves exact.
    pub fn fingerprint(&self) -> u64 {
        let mut h = self.world.fingerprint();
        for s in &self.slots {
            if let Some(b) = s.module.save_state() { h = (h ^ crate::rng::fnv64(&b)).wrapping_mul(0x0000_0100_0000_01b3); }
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flows::Endpoint;
    use crate::grid::Grid;
    use crate::module::{ModuleSpec, Phase};
    use crate::stocks::{Group, Registry, Storage, StockId};
    use std::sync::Arc;

    /// Moves a share of `a` into `b` on every tile.
    struct Mover { name: String, from: &'static str, to: &'static str, a: StockId, b: StockId, share: f64, period: u32, undeclared: bool }
    impl Mover {
        fn new(name: &str, share: f64, period: u32) -> Box<Mover> {
            Box::new(Mover { name: name.into(), from: "a", to: "b", a: StockId(0), b: StockId(1), share, period, undeclared: false })
        }
    }
    impl Module for Mover {
        fn spec(&self) -> ModuleSpec {
            let mut s = ModuleSpec::new(&self.name, Phase::Water).reads(&["a"]).writes(&[self.from, self.to]).every(self.period);
            if self.undeclared { s.writes = vec!["a".into()]; }
            s
        }
        fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> {
            self.a = reg.stock(self.from).unwrap(); self.b = reg.stock(self.to).unwrap(); Ok(())
        }
        fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
            let a = view.stock(self.a)?;
            let area = &ctx.grid.area_km2;
            for i in 0..a.len() { out.local(self.a, self.b, i, a[i] as f64 * self.share * area[i] as f64); }
            Ok(())
        }
    }
    struct Sneak(StockId);
    impl Module for Sneak {
        fn spec(&self) -> ModuleSpec { ModuleSpec::new("sneak", Phase::Water).reads(&["a"]).writes(&["a"]) }
        fn bind(&mut self, reg: &Registry, _: &Grid) -> Result<(), WorldError> { self.0 = reg.stock("b").unwrap(); Ok(()) }
        fn step(&mut self, _: &StepCtx, view: &View, _: &mut FlowOut) -> Result<(), WorldError> { view.stock(self.0).map(|_| ()) }
    }
    struct Boom;
    impl Module for Boom {
        fn spec(&self) -> ModuleSpec { ModuleSpec::new("boom", Phase::Water) }
        fn step(&mut self, _: &StepCtx, _: &View, _: &mut FlowOut) -> Result<(), WorldError> { panic!("bad state") }
    }

    fn sim(n: u32) -> Sim {
        let mut reg = Registry::new();
        reg.add_stock("a", "mm", Group::Water, Storage::Exact);
        reg.add_stock("b", "mm", Group::Water, Storage::Exact);
        let mut w = World::new(Arc::new(Grid::new(n)), reg, 3, 0);
        for (i, v) in w.stock_mut(StockId(0)).iter_mut().enumerate() { *v = 10.0 + (i % 7) as f32; }
        w.reset_ledger();
        Sim::new(w)
    }

    #[test]
    fn modules_run_and_budget_closes() {
        let mut s = sim(8);
        s.add_module(Mover::new("m", 0.1, 1)).unwrap();
        s.run_months(24);
        assert!(s.world.audit()[0].residual.abs() < 1e-6);
        assert_eq!(s.world.violation_count, 0);
        assert!(s.world.stock(StockId(1)).iter().all(|&v| v > 0.0));
    }

    #[test]
    fn period_is_respected_and_dt_is_passed() {
        use std::sync::{Arc as A, Mutex};
        struct Rec(A<Mutex<Vec<(u64, u32)>>>);
        impl Module for Rec {
            fn spec(&self) -> ModuleSpec { ModuleSpec::new("rec", Phase::Water).every(12) }
            fn step(&mut self, c: &StepCtx, _: &View, _: &mut FlowOut) -> Result<(), WorldError> { self.0.lock().unwrap().push((c.month_index, c.dt_months)); Ok(()) }
        }
        let log = A::new(Mutex::new(Vec::new()));
        let mut s = sim(6);
        s.add_module(Box::new(Rec(log.clone()))).unwrap();
        s.add_module(Mover::new("slow", 0.5, 12)).unwrap();
        s.run_months(1);
        let moved = s.world.stock(StockId(1))[0];
        assert!(moved > 0.0, "a yearly module runs in its first month");
        s.run_months(11);
        assert_eq!(s.world.stock(StockId(1))[0], moved, "ran before its month");
        s.run_months(1);
        assert!(s.world.stock(StockId(1))[0] > moved);
        assert_eq!(*log.lock().unwrap(), vec![(0, 12), (12, 12)]);
    }

    #[test]
    fn threads_do_not_change_the_result() {
        let build = |p: Parallel| {
            let mut s = sim(30);
            s.parallel = p;
            for (k, name) in ["m1", "m2", "m3"].iter().enumerate() { s.add_module(Mover::new(name, 0.05 + 0.02 * k as f64, 1)).unwrap(); }
            s.run_months(36);
            s.fingerprint()
        };
        assert_eq!(build(Parallel::Off), build(Parallel::On));
    }

    #[test]
    fn undeclared_write_drops_the_modules_output() {
        let mut s = sim(6);
        let mut m = Mover::new("liar", 0.5, 1);
        m.undeclared = true;
        s.add_module(m).unwrap();
        let before = s.world.fingerprint();
        s.run_months(1);
        assert_eq!(s.skipped, 1);
        assert!(s.world.log[0].contains("without declaring"));
        // the clock moved, the stocks did not
        assert_ne!(before, s.world.fingerprint());
        assert!(s.world.stock(StockId(1)).iter().all(|&v| v == 0.0));
    }

    #[test]
    fn undeclared_read_is_an_error_and_a_panic_is_survived() {
        let mut s = sim(6);
        s.add_module(Box::new(Sneak(StockId(0)))).unwrap();
        s.add_module(Box::new(Boom)).unwrap();
        s.add_module(Mover::new("ok", 0.1, 1)).unwrap();
        s.run_months(3);
        assert_eq!(s.skipped, 6);
        assert!(s.world.log.iter().any(|l| l.contains("read b without declaring")));
        assert!(s.world.log.iter().any(|l| l.contains("panicked")));
        assert!(s.world.stock(StockId(1))[0] > 0.0, "the healthy module kept running");
    }

    #[test]
    fn duplicate_and_unknown_are_refused() {
        let mut s = sim(6);
        s.add_module(Mover::new("m", 0.1, 1)).unwrap();
        assert!(s.add_module(Mover::new("m", 0.1, 1)).is_err());
        struct Ghost;
        impl Module for Ghost {
            fn spec(&self) -> ModuleSpec { ModuleSpec::new("ghost", Phase::Water).reads(&["nope"]) }
            fn step(&mut self, _: &StepCtx, _: &View, _: &mut FlowOut) -> Result<(), WorldError> { Ok(()) }
        }
        assert!(matches!(s.add_module(Box::new(Ghost)), Err(WorldError::Unknown(_))));
        let _ = Endpoint::Transit;
    }
}
