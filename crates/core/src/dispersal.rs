//! Seeds spread to new ground. Once a year each type's leaf carbon diffuses between neighbouring tiles
//! at a rate set by its seed diffusion constant D (m2 per year): the front of a type moves at 2 sqrt(r D),
//! with r the growth rate. A type grows only where its seeds have reached, so forests creep north after
//! the ice and islands fill slowly. Carbon is moved, not made.

use crate::eco::*;
use crate::env::*;
use crate::flows::{Endpoint, FlowOut};
use crate::grid::Grid;
use crate::module::*;
use crate::stocks::Registry;
use crate::world::WorldError;

const DONOR_MIN: f32 = 0.02; // kgC/m2 of leaf before a tile gives seeds

pub struct Dispersal { env: Env, set: PlantSet, eco: Eco }
impl Dispersal { pub fn new(env: Env, set: PlantSet, eco: Eco) -> Self { Dispersal { env, set, eco } } }

impl Module for Dispersal {
    fn spec(&self) -> ModuleSpec {
        let mut s = ModuleSpec::new("dispersal", Phase::SoilPlants).every(12).reads(&["w_sea"]).writes(&[]);
        for t in &self.set.types { let n = format!("c_leaf_{}", t.id); s.reads.push(n.clone()); s.writes.push(n); }
        s
    }
    fn bind(&mut self, _: &Registry, _: &Grid) -> Result<(), WorldError> { Ok(()) }
    fn step(&mut self, ctx: &StepCtx, view: &View, out: &mut FlowOut) -> Result<(), WorldError> {
        let n = self.env.n();
        let grid = ctx.grid;
        let dt = ctx.dt_months as f64 / 12.0;
        let w_sea = view.stock(self.env.ph.w_sea)?;
        for (p, t) in self.set.types.iter().enumerate() {
            let leaf = view.stock(self.eco.leaf[p])?;
            for i in 0..n {
                let li = leaf[i];
                if li < DONOR_MIN { continue; }
                for (k, j) in grid.nbr[i].iter().enumerate() {
                    if *j < 0 { continue; }
                    let j = *j as usize;
                    if w_sea[j] > 50.0 && self.env.ocean0[j] { continue; }
                    let dist = self.env.p.nbr_km[i][k] as f64 * 1000.0;
                    let lj = leaf[j];
                    if lj >= 0.5 * li { continue; }
                    // diffusion flux over the shared edge; donors give at most one percent a year
                    let frac = (t.seed_d as f64 * dt / (dist * dist)).min(0.01 / 6.0);
                    let amount = frac * (li - lj) as f64 * grid.area_km2[i] as f64;
                    if amount > 0.0 { out.push(Endpoint::Stock(self.eco.leaf[p]), Endpoint::Stock(self.eco.leaf[p]), i, j, amount); }
                }
            }
        }
        Ok(())
    }
}
