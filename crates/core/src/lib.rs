//! Odyssey rebuild, stage 1 (groundwork).
//!
//! The world is a set of stocks on one grid family. Modules never touch state
//! directly. They read last month's stocks and return flows. Every flow is booked
//! on both sides, so nothing appears or vanishes. See `docs/ENGINE.md`.

pub mod content;
pub mod demo;
pub mod flows;
pub mod grid;
pub mod module;
pub mod rng;
pub mod save;
pub mod sim;
pub mod stocks;
pub mod units;
pub mod world;

pub use flows::{Endpoint, FlowOut};
pub use grid::{Coarse, Grid};
// sim re-export added below
pub use stocks::{BoundaryId, Group, Registry, Storage, StockId};
pub use world::{World, WorldError};
pub use module::{Detail, Module, ModuleSpec, Phase, StepCtx, View};
