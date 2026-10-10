//! The module interface.
//!
//! A module declares what it reads and what it may write. The world enforces it:
//! reading an undeclared stock or writing to one returns an error and the module's
//! output for that step is dropped. A step reads the start-of-month stocks only.
//! It returns flows. The world applies them after every due module has run, in a
//! fixed order, so threads cannot change the result.

use crate::flows::FlowOut;
use crate::grid::Grid;
use crate::stocks::{Registry, StockId};
use crate::world::{World, WorldError};

/// Order of application within a month. Ties break by registration order.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase { Climate, Water, SoilPlants, Animals, People }

/// Cheap or full version of a module. Tests run `Full`. A slow machine may pick `Cheap`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Detail { Cheap, Full }

#[derive(Clone, Debug)]
pub struct ModuleSpec {
    pub name: String,
    pub phase: Phase,
    /// Runs when `month_index % period_months == offset`. One means every month.
    pub period_months: u32,
    pub offset: u32,
    pub reads: Vec<String>,
    /// Stocks the module's flows may touch, at either end.
    pub writes: Vec<String>,
    /// Outside accounts the module may use.
    pub boundaries: Vec<String>,
}

impl ModuleSpec {
    pub fn new(name: &str, phase: Phase) -> Self {
        ModuleSpec { name: name.into(), phase, period_months: 1, offset: 0, reads: vec![], writes: vec![], boundaries: vec![] }
    }
    pub fn reads(mut self, names: &[&str]) -> Self { self.reads = names.iter().map(|s| s.to_string()).collect(); self }
    pub fn also_reads(mut self, names: &[String]) -> Self { self.reads.extend(names.iter().cloned()); self }
    pub fn writes(mut self, names: &[&str]) -> Self { self.writes = names.iter().map(|s| s.to_string()).collect(); self }
    pub fn boundaries(mut self, names: &[&str]) -> Self { self.boundaries = names.iter().map(|s| s.to_string()).collect(); self }
    pub fn every(mut self, months: u32) -> Self { self.period_months = months.max(1); self }
}

/// Read access to the start-of-month stocks, limited to the declared reads.
pub struct View<'a> {
    pub(crate) world: &'a World,
    pub(crate) allowed: &'a [StockId],
    pub(crate) module: &'a str,
}

impl<'a> View<'a> {
    pub fn stock(&self, id: StockId) -> Result<&'a [f32], WorldError> {
        if !self.allowed.contains(&id) {
            return Err(WorldError::UndeclaredRead { module: self.module.into(), stock: self.world.reg.def(id).name.clone() });
        }
        Ok(&self.world.cur[id.0 as usize])
    }
    pub fn grid(&self) -> &'a Grid { &self.world.grid }
    pub fn registry(&self) -> &'a Registry { &self.world.reg }
}

pub struct StepCtx<'a> {
    pub month_index: u64,
    pub month_of_year: u32,
    pub year: i64,
    /// Months since this module last ran, so a yearly module books twelve months of flow.
    pub dt_months: u32,
    pub detail: Detail,
    pub seed: u64,
    pub grid: &'a Grid,
}

impl<'a> StepCtx<'a> {
    /// A random stream for this module and step. Independent of thread order.
    pub fn rng(&self, module: &str) -> crate::rng::Rng { crate::rng::Rng::derive(self.seed, module, self.month_index) }
}

pub trait Module: Send {
    fn spec(&self) -> ModuleSpec;
    /// Called once at registration. Resolve stock ids and build grids here.
    fn bind(&mut self, _reg: &Registry, _grid: &Grid) -> Result<(), WorldError> { Ok(()) }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError>;
    /// Private state that is not a stock (for example an RNG cursor). Saved with the world.
    fn save_state(&self) -> Option<Vec<u8>> { None }
    fn load_state(&mut self, _bytes: &[u8]) -> Result<(), WorldError> { Ok(()) }
}
