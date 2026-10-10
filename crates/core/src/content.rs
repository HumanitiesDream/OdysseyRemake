//! Content as data. Every list of kinds is a JSON file the engine loads at start.
//!
//! Rules, checked on load:
//! 1. Every item has an `id` (unique in its file) and a `source`.
//! 2. Every number sits inside a `{ "value": .., "source": ".." }` object. A bare
//!    number is an error. This is how the "source on every value" habit is enforced.
//! 3. References (`plant:x`, `guild:x`, `pool:x`) must point at something that exists.
//!
//! A bad file stops the load with a message that names the file and the path.
//! `verified: false` marks a figure copied from memory. The loader counts them.

use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub const KINDS: [&str; 5] = ["plant_types", "animal_guilds", "rock_types", "disaster_stocks", "soil_production"];

#[derive(Debug, Clone)]
pub struct File {
    pub kind: String,
    pub items: Vec<Value>,
    pub pools: Vec<String>,
}

impl File {
    pub fn ids(&self) -> Vec<&str> { self.items.iter().filter_map(|i| i["id"].as_str()).collect() }
    pub fn item(&self, id: &str) -> Option<&Value> { self.items.iter().find(|i| i["id"] == id) }
}

#[derive(Debug, Default)]
pub struct Content {
    pub files: BTreeMap<String, File>,
    raw: BTreeMap<String, Value>,
    /// Sourced values flagged `verified: false`, as "file:path".
    pub unverified: Vec<String>,
}

impl Content {
    pub fn get(&self, kind: &str) -> Option<&File> { self.files.get(kind) }
    /// The whole parsed file, for top-level fields such as constants.
    pub fn raw(&self, kind: &str) -> Option<&Value> { self.raw.get(kind) }

    /// Load every file in `dir`. Fails on the first problem with a message naming file and path.
    pub fn load_dir(dir: &Path) -> Result<Content, String> {
        let mut c = Content::default();
        for kind in KINDS {
            let path = dir.join(format!("{kind}.json"));
            let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: cannot read: {e}", path.display()))?;
            c.add_text(kind, &text, &path.display().to_string())?;
        }
        c.check_refs()?;
        Ok(c)
    }

    pub fn add_text(&mut self, kind: &str, text: &str, label: &str) -> Result<(), String> {
        let v: Value = serde_json::from_str(text).map_err(|e| format!("{label}: not valid JSON: {e}"))?;
        if v["kind"] != kind { return Err(format!("{label}: kind is {}, expected \"{kind}\"", v["kind"])); }
        if !v["version"].is_u64() { return Err(format!("{label}: missing integer \"version\"")); }
        let items = v["items"].as_array().ok_or_else(|| format!("{label}: \"items\" must be a list"))?.clone();
        let mut seen = BTreeSet::new();
        for (k, item) in items.iter().enumerate() {
            let id = item["id"].as_str().filter(|s| !s.is_empty()).ok_or_else(|| format!("{label}: items[{k}] has no id"))?;
            if !seen.insert(id.to_string()) { return Err(format!("{label}: duplicate id \"{id}\"")); }
            match item["source"].as_str() { Some(s) if !s.trim().is_empty() => {}, _ => return Err(format!("{label}: item \"{id}\" has no source")) }
            check_numbers(item, &format!("items[{id}]"), label, &mut self.unverified)?;
        }
        let mut pools = Vec::new();
        if let Some(arr) = v["pools"].as_array() {
            for (k, p) in arr.iter().enumerate() {
                let id = p["id"].as_str().ok_or_else(|| format!("{label}: pools[{k}] has no id"))?;
                if p["source"].as_str().map_or(true, |s| s.trim().is_empty()) { return Err(format!("{label}: pool \"{id}\" has no source")); }
                pools.push(id.to_string());
            }
        }
        if let Some(c) = v.get("constants") { check_numbers(c, "constants", label, &mut self.unverified)?; }
        self.raw.insert(kind.into(), v.clone());
        self.files.insert(kind.into(), File { kind: kind.into(), items, pools });
        Ok(())
    }

    /// Cross-file reference check for animal guilds.
    pub fn check_refs(&self) -> Result<(), String> {
        let (Some(g), Some(p)) = (self.files.get("animal_guilds"), self.files.get("plant_types")) else { return Ok(()) };
        let guilds: BTreeSet<&str> = g.ids().into_iter().collect();
        let plants: BTreeSet<&str> = p.ids().into_iter().collect();
        let pools: BTreeSet<&str> = g.pools.iter().map(|s| s.as_str()).collect();
        for item in &g.items {
            let me = item["id"].as_str().unwrap_or("?");
            for r in item["eats"].as_array().map(|a| a.as_slice()).unwrap_or(&[]) {
                let r = r.as_str().ok_or_else(|| format!("animal_guilds: \"{me}\" has a non-text entry in eats"))?;
                let (head, tail) = r.split_once(':').ok_or_else(|| format!("animal_guilds: \"{me}\" eats \"{r}\", expected plant:, guild: or pool:"))?;
                let ok = match head { "plant" => plants.contains(tail), "guild" => guilds.contains(tail), "pool" => pools.contains(tail), _ => false };
                if !ok { return Err(format!("animal_guilds: \"{me}\" eats \"{r}\", which does not exist")); }
            }
        }
        Ok(())
    }
}

fn check_numbers(v: &Value, path: &str, label: &str, unverified: &mut Vec<String>) -> Result<(), String> {
    match v {
        Value::Number(_) => Err(format!("{label}: {path} is a number with no source. Wrap it as {{\"value\": .., \"source\": \"..\"}}")),
        Value::Object(m) if m.contains_key("value") => {
            match m.get("source").and_then(|s| s.as_str()) { Some(s) if !s.trim().is_empty() => {}, _ => return Err(format!("{label}: {path} has a value but no source")) }
            if !(m["value"].is_number() || m["value"].is_string()) { return Err(format!("{label}: {path}.value must be a number or text")); }
            if m.get("verified").and_then(|b| b.as_bool()) == Some(false) { unverified.push(format!("{label}:{path}")); }
            Ok(())
        }
        Value::Object(m) => { for (k, x) in m { check_numbers(x, &format!("{path}.{k}"), label, unverified)?; } Ok(()) }
        Value::Array(a) => { for (k, x) in a.iter().enumerate() { check_numbers(x, &format!("{path}[{k}]"), label, unverified)?; } Ok(()) }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn dir() -> std::path::PathBuf { Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data") }

    #[test]
    fn shipped_files_load_and_cross_check() {
        let c = Content::load_dir(&dir()).unwrap();
        assert_eq!(c.get("plant_types").unwrap().items.len(), 10);
        assert_eq!(c.get("animal_guilds").unwrap().items.len(), 14);
        assert!(c.get("rock_types").unwrap().item("sc").is_some());
        // the plant parameters taken from LPJmL are verified; the ones we chose ourselves are flagged
        assert!(c.unverified.iter().all(|u| u.contains("plant_types") || u.contains("animal_guilds")), "only our own assumptions are unverified");
        assert!(!c.unverified.is_empty());
    }

    #[test]
    fn bare_number_is_refused_with_its_path() {
        let mut c = Content::default();
        let e = c.add_text("plant_types", r#"{"kind":"plant_types","version":1,"items":[{"id":"a","source":"s","rate":0.5}]}"#, "f.json").unwrap_err();
        assert!(e.contains("f.json") && e.contains("items[a].rate"), "{e}");
    }

    #[test]
    fn missing_source_and_duplicates_are_refused() {
        let mut c = Content::default();
        assert!(c.add_text("plant_types", r#"{"kind":"plant_types","version":1,"items":[{"id":"a"}]}"#, "f").unwrap_err().contains("no source"));
        assert!(c.add_text("plant_types", r#"{"kind":"plant_types","version":1,"items":[{"id":"a","source":"s"},{"id":"a","source":"s"}]}"#, "f").unwrap_err().contains("duplicate"));
        assert!(c.add_text("plant_types", r#"{"kind":"plant_types","version":1,"items":[{"id":"a","source":"s","x":{"value":1}}]}"#, "f").unwrap_err().contains("no source"));
        assert!(c.add_text("rock_types", r#"{"kind":"plant_types","version":1,"items":[]}"#, "f").unwrap_err().contains("kind"));
        assert!(c.add_text("rock_types", "{oops", "f").unwrap_err().contains("not valid JSON"));
    }

    #[test]
    fn dangling_reference_is_refused() {
        let mut c = Content::default();
        c.add_text("plant_types", r#"{"kind":"plant_types","version":1,"items":[{"id":"grass","source":"s"}]}"#, "p").unwrap();
        c.add_text("animal_guilds", r#"{"kind":"animal_guilds","version":1,"pools":[],"items":[{"id":"cow","source":"s","eats":["plant:grass"]},{"id":"wolf","source":"s","eats":["guild:cat"]}]}"#, "a").unwrap();
        assert!(c.check_refs().unwrap_err().contains("guild:cat"));
    }
}
