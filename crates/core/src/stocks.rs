//! Stock definitions. One array per stock (structure of arrays), f32 per tile.
//! A stock holds a density per km2, so its value means the same at 48 km and 16 km.
//! Totals are always summed in f64 as density x area.

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StockId(pub u16);
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundaryId(pub u16);

/// What a stock is made of. A flow may only move amounts inside one group.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Group { Water, Heat, Carbon, Nitrogen, Phosphorus, Sediment }

pub const GROUPS: [Group; 6] = [Group::Water, Group::Heat, Group::Carbon, Group::Nitrogen, Group::Phosphorus, Group::Sediment];

impl Group {
    pub fn idx(self) -> usize { self as usize }
    pub fn name(self) -> &'static str {
        match self { Group::Water => "water", Group::Heat => "heat", Group::Carbon => "carbon", Group::Nitrogen => "nitrogen", Group::Phosphorus => "phosphorus", Group::Sediment => "sediment" }
    }
}

/// How a stock is written to a save.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Storage {
    /// Raw f32. Lossless.
    Exact,
    /// Only non-zero cells. Lossless. For stocks that are mostly zero (ice, fire, sediment).
    Sparse,
    /// Rounded to a multiple of `step`. Lossy, for derived or display stocks only.
    Quantised(f32),
}

#[derive(Clone, Debug)]
pub struct StockDef {
    pub name: String,
    /// Unit of the density, for example "mm" or "MJ/m2".
    pub unit: String,
    pub group: Group,
    pub storage: Storage,
}

#[derive(Clone, Debug)]
pub struct BoundaryDef {
    pub name: String,
    pub group: Group,
}

/// The one description of every stock. Saves, budgets and module wiring all read it.
#[derive(Clone, Debug, Default)]
pub struct Registry {
    pub stocks: Vec<StockDef>,
    pub boundaries: Vec<BoundaryDef>,
}

impl Registry {
    pub fn new() -> Self { Self::default() }

    pub fn add_stock(&mut self, name: &str, unit: &str, group: Group, storage: Storage) -> StockId {
        assert!(self.stock(name).is_none(), "stock {name} is already registered");
        self.stocks.push(StockDef { name: name.into(), unit: unit.into(), group, storage });
        StockId((self.stocks.len() - 1) as u16)
    }

    /// An outside account: sunlight, heat lost to space, volcanic gas. Flows to and from it
    /// are tallied, so the budget still closes: change in world = net flow from outside.
    pub fn add_boundary(&mut self, name: &str, group: Group) -> BoundaryId {
        assert!(self.boundary(name).is_none(), "boundary {name} is already registered");
        self.boundaries.push(BoundaryDef { name: name.into(), group });
        BoundaryId((self.boundaries.len() - 1) as u16)
    }

    pub fn stock(&self, name: &str) -> Option<StockId> { self.stocks.iter().position(|s| s.name == name).map(|i| StockId(i as u16)) }
    pub fn boundary(&self, name: &str) -> Option<BoundaryId> { self.boundaries.iter().position(|s| s.name == name).map(|i| BoundaryId(i as u16)) }
    pub fn def(&self, id: StockId) -> &StockDef { &self.stocks[id.0 as usize] }
    pub fn bdef(&self, id: BoundaryId) -> &BoundaryDef { &self.boundaries[id.0 as usize] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lookup_by_name() {
        let mut r = Registry::new();
        let a = r.add_stock("water_air", "mm", Group::Water, Storage::Exact);
        let b = r.add_boundary("sun", Group::Heat);
        assert_eq!(r.stock("water_air"), Some(a));
        assert_eq!(r.boundary("sun"), Some(b));
        assert_eq!(r.stock("nope"), None);
        assert_eq!(r.def(a).group, Group::Water);
    }
    #[test]
    #[should_panic]
    fn duplicate_names_are_refused() {
        let mut r = Registry::new();
        r.add_stock("x", "mm", Group::Water, Storage::Exact);
        r.add_stock("x", "mm", Group::Water, Storage::Exact);
    }
}
