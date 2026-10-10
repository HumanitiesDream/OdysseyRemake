//! Shared pieces of the living land: plant types read from the content files, the stock catalogue,
//! and the pure functions both the plant and water modules use (canopy light, phenology, transpiration).
//! Both modules recompute these from the same start-of-month stocks, so they always agree.

use crate::content::Content;
use crate::stocks::{BoundaryId, Group, Registry, Storage, StockId};
use serde_json::Value;

pub const NPT: usize = 10;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Phen { Evergreen, Summergreen, Raingreen, Any }

#[derive(Clone, Debug)]
pub struct PlantType {
    pub id: String,
    pub phen: Phen,
    pub c4: bool,
    pub sla: f32,        // m2 per kgC
    pub longevity: f32,  // leaf, yr
    pub k_ext: f32,
    pub t_min: f32, pub t_max: f32, pub t_opt_lo: f32, pub t_opt_hi: f32,
    pub cold_min: f32, pub cold_max: f32,
    pub turn_leaf: f32, pub turn_wood: f32, pub turn_root: f32,
    pub mort_max: f32,
    pub albedo_leaf: f32,
    pub height: f32,
    pub lai_max: f32,
    pub al: f32, pub aw: f32, pub ar: f32,
    pub seed_d: f32,
    pub flam: f32,
    pub woody: bool,
    pub aprec_min: f32,
}

impl PlantType { pub fn aprec_min_mm(&self) -> f32 { self.aprec_min } }

#[derive(Clone, Debug)]
pub struct PlantSet {
    pub types: Vec<PlantType>,
    /// type indices from tallest to shortest
    pub order: Vec<usize>,
}

fn num(it: &Value, key: &str) -> Result<f32, String> {
    it[key]["value"].as_f64().map(|x| x as f32).ok_or_else(|| format!("plant type {}: no number for {key}", it["id"]))
}

impl PlantSet {
    pub fn from_content(c: &Content) -> Result<PlantSet, String> {
        let f = c.get("plant_types").ok_or("no plant_types content")?;
        let mut types = Vec::new();
        for it in &f.items {
            let phen = match it["phenology"].as_str() { Some("evergreen") => Phen::Evergreen, Some("summergreen") => Phen::Summergreen, Some("raingreen") => Phen::Raingreen, _ => Phen::Any };
            let id = it["id"].as_str().unwrap_or("?").to_string();
            let aw = num(it, "alloc_wood")?;
            types.push(PlantType {
                id, phen, c4: it["path"] == "C4", sla: num(it, "sla_m2_per_kgC")?, longevity: num(it, "leaf_longevity_yr")?, k_ext: num(it, "light_extinction")?,
                t_min: num(it, "t_photosynthesis_min_c")?, t_max: num(it, "t_photosynthesis_max_c")?, t_opt_lo: num(it, "t_optimum_low_c")?, t_opt_hi: num(it, "t_optimum_high_c")?,
                cold_min: num(it, "coldest_month_min_c")?, cold_max: num(it, "coldest_month_max_c")?,
                turn_leaf: num(it, "turnover_leaf_yr")?, turn_wood: num(it, "turnover_sapwood_yr")?, turn_root: num(it, "turnover_root_yr")?,
                mort_max: num(it, "mort_max_per_yr")?, albedo_leaf: num(it, "albedo_leaf")?, height: num(it, "height_m")?, lai_max: num(it, "lai_max")?,
                al: num(it, "alloc_leaf")?, aw, ar: num(it, "alloc_root")?, seed_d: num(it, "seed_diffusion_m2_per_yr")?, flam: num(it, "flammability")?, woody: aw > 0.0, aprec_min: num(it, "aprec_min_mm")?,
            });
        }
        if types.len() != NPT { return Err(format!("expected {NPT} plant types, found {}", types.len())); }
        let mut order: Vec<usize> = (0..types.len()).collect();
        order.sort_by(|&a, &b| types[b].height.partial_cmp(&types[a].height).unwrap().then(a.cmp(&b)));
        Ok(PlantSet { types, order })
    }
}

/// Ids of the ecology stocks and accounts.
#[derive(Clone, Debug)]
pub struct Eco {
    pub leaf: Vec<StockId>, pub wood: Vec<StockId>, pub root: Vec<StockId>,
    pub litter: StockId, pub sfast: StockId, pub sslow: StockId, pub spass: StockId,
    pub n_min: StockId, pub n_org: StockId, pub p_min: StockId, pub p_org: StockId, pub p_occ: StockId,
    pub b_nfix: BoundaryId, pub b_nloss: BoundaryId, pub b_rockp: BoundaryId, pub b_ploss: BoundaryId,
}

impl Eco {
    pub fn register(reg: &mut Registry, set: &PlantSet) -> Eco {
        let c = |reg: &mut Registry, n: String| reg.add_stock(&n, "kgC/m2", Group::Carbon, Storage::Exact);
        let mut leaf = vec![]; let mut wood = vec![]; let mut root = vec![];
        for t in &set.types {
            leaf.push(c(reg, format!("c_leaf_{}", t.id)));
            wood.push(c(reg, format!("c_wood_{}", t.id)));
            root.push(c(reg, format!("c_root_{}", t.id)));
        }
        let litter = c(reg, "c_litter".into()); let sfast = c(reg, "c_sfast".into()); let sslow = c(reg, "c_sslow".into()); let spass = c(reg, "c_spass".into());
        let n_min = reg.add_stock("n_min", "kgN/m2", Group::Nitrogen, Storage::Exact);
        let n_org = reg.add_stock("n_org", "kgN/m2", Group::Nitrogen, Storage::Exact);
        let p_min = reg.add_stock("p_min", "kgP/m2", Group::Phosphorus, Storage::Exact);
        let p_org = reg.add_stock("p_org", "kgP/m2", Group::Phosphorus, Storage::Exact);
        let p_occ = reg.add_stock("p_occ", "kgP/m2", Group::Phosphorus, Storage::Exact);
        Eco {
            leaf, wood, root, litter, sfast, sslow, spass, n_min, n_org, p_min, p_org, p_occ,
            b_nfix: reg.add_boundary("nfix", Group::Nitrogen), b_nloss: reg.add_boundary("nloss", Group::Nitrogen),
            b_rockp: reg.add_boundary("rockp", Group::Phosphorus), b_ploss: reg.add_boundary("ploss", Group::Phosphorus),
        }
    }
    pub fn names_read(&self, reg: &Registry) -> Vec<String> {
        let mut v = vec![];
        for s in self.leaf.iter().chain(self.wood.iter()).chain(self.root.iter()) { v.push(reg.def(*s).name.clone()); }
        v
    }
}

/// Soil carbon to nitrogen and phosphorus ratios by mass for each pool: litter, fast, slow, passive.
pub const CN: [f32; 4] = [45.0, 10.0, 12.0, 9.0];
pub const CP: [f32; 4] = [900.0, 150.0, 200.0, 150.0];
/// Plant tissue ratios: leaf, wood, root (LPJmL CTON_LEAF 30, CTON_ROOT 30, CTON_SAP 330; wood blends sapwood and heartwood).
pub const CN_LEAF: f32 = 30.0; pub const CN_WOOD: f32 = 330.0; pub const CN_ROOT: f32 = 30.0;
pub const CP_LEAF: f32 = 450.0; pub const CP_WOOD: f32 = 5000.0; pub const CP_ROOT: f32 = 450.0;

/// Plant cover from leaf carbon: 1 - exp(-0.5 LAI), LAI capped at 7. A year-round figure with no leaf-off.
pub fn cover_of(sla: &[f32], leaf: &[f32]) -> f32 {
    let lai: f32 = sla.iter().zip(leaf.iter()).map(|(s, l)| s * l).sum::<f32>().min(7.0);
    1.0 - (-0.5 * lai).exp()
}

pub fn temp_factor(t: &PlantType, tc: f32) -> f32 {
    if tc <= t.t_min || tc >= t.t_max { 0.0 }
    else if tc < t.t_opt_lo { (tc - t.t_min) / (t.t_opt_lo - t.t_min) }
    else if tc > t.t_opt_hi { (t.t_max - tc) / (t.t_max - t.t_opt_hi) }
    else { 1.0 }
}

/// Share of the leaves that are out. Summergreen follows warmth, raingreen follows soil water, the rest always.
pub fn phenology(t: &PlantType, tc: f32, w_ratio: f32) -> f32 {
    match t.phen {
        Phen::Evergreen | Phen::Any => 1.0,
        Phen::Summergreen => ((tc - 3.0) / 7.0).clamp(0.0, 1.0),
        Phen::Raingreen => (w_ratio / 0.3).clamp(0.0, 1.0),
    }
}

pub struct Canopy { pub lai: [f32; NPT], pub light_above: [f32; NPT], pub fpar: [f32; NPT], pub fpar_tot: f32 }

/// Layered canopy: taller types see the sky first. `leaf` is leaf carbon per type, kgC/m2.
pub fn canopy(set: &PlantSet, leaf: &[f32; NPT], tc: f32, w_ratio: f32) -> Canopy {
    let mut c = Canopy { lai: [0.0; NPT], light_above: [1.0; NPT], fpar: [0.0; NPT], fpar_tot: 0.0 };
    let mut light = 1.0f32;
    for &p in &set.order {
        let t = &set.types[p];
        let lai = (leaf[p] * t.sla).min(t.lai_max * 1.5) * phenology(t, tc, w_ratio);
        c.lai[p] = lai;
        c.light_above[p] = light;
        let absorbed = 1.0 - (-t.k_ext * lai).exp();
        c.fpar[p] = light * absorbed;
        light *= 1.0 - absorbed;
    }
    c.fpar_tot = 1.0 - light;
    c
}

/// Transpiration (mm this month) and the stress ratio supply/demand, from the canopy cover,
/// potential evaporation, and soil water. The roots can pull up to 60 percent of the soil water a month.
pub fn transpiration(pet_mm: f32, fpar_tot: f32, w_land: f32) -> (f32, f32) {
    let demand = pet_mm * fpar_tot * 0.9;
    if demand <= 1e-6 { return (0.0, 1.0); }
    let supply = 0.6 * w_land;
    let t = demand.min(supply);
    (t, t / demand)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set() -> PlantSet {
        let c = crate::content::Content::load_dir(&crate::build::data_dir()).unwrap();
        PlantSet::from_content(&c).unwrap()
    }

    #[test]
    fn ten_types_tallest_first() {
        let s = set();
        assert_eq!(s.types.len(), NPT);
        assert_eq!(s.types[s.order[0]].id, "tropical_evergreen");
        assert!(s.types[*s.order.last().unwrap()].height < 0.5);
    }

    #[test]
    fn temperature_response_is_a_plateau() {
        let s = set();
        let t = s.types.iter().find(|t| t.id == "temperate_broadleaf").unwrap();
        assert_eq!(temp_factor(t, 25.0), 1.0);
        assert_eq!(temp_factor(t, -5.0), 0.0);
        assert!(temp_factor(t, 12.0) > 0.0 && temp_factor(t, 12.0) < 1.0);
    }

    #[test]
    fn tall_canopy_shades_the_grass() {
        let s = set();
        let mut leaf = [0f32; NPT];
        let forest = s.types.iter().position(|t| t.id == "tropical_evergreen").unwrap();
        let grass = s.types.iter().position(|t| t.id == "c4_grass").unwrap();
        leaf[grass] = 0.2;
        let open = canopy(&s, &leaf, 25.0, 1.0);
        leaf[forest] = 0.4;
        let shaded = canopy(&s, &leaf, 25.0, 1.0);
        assert!(shaded.fpar[grass] < 0.5 * open.fpar[grass], "{} vs {}", shaded.fpar[grass], open.fpar[grass]);
        assert!(shaded.fpar_tot > open.fpar_tot);
    }

    #[test]
    fn dry_soil_stresses_the_canopy() {
        let (t_wet, s_wet) = transpiration(100.0, 0.8, 200.0);
        let (t_dry, s_dry) = transpiration(100.0, 0.8, 20.0);
        assert!(s_wet > 0.9 && s_dry < 0.2 && t_wet > t_dry);
    }
}
