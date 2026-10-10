//! Saves: tagged, versioned sections, written from the stock registry.
//!
//! ```text
//! "ODYS" u16 format  u32 section_count
//! per section: tag[4]  u16 version  u8 codec  u32 stored_len  u32 raw_len  payload
//! ```
//! Sections: `META` (clock, seed, ledger), `STCK` (every stock), `RSID` (the f32 rounding
//! each stock carries, so a resume is exact), `MODS` (module state).
//!
//! - A reader skips a section it does not know, and a stock it does not know.
//! - A stock missing from the file is set to zero and reported.
//! - The stock code is written once, from the registry, so the two sides cannot drift.
//! - `Exact` and `Sparse` stocks round-trip bit for bit. `Quantised` ones do not.
//!   A save with only lossless stocks resumes to the same fingerprint as an unbroken run.

use crate::sim::Sim;
use crate::stocks::Storage;
use crate::world::WorldError;
use flate2::{read::DeflateDecoder, write::DeflateEncoder, Compression};
use std::io::{Read, Write};

const MAGIC: &[u8; 4] = b"ODYS";
pub const FORMAT: u16 = 1;

fn err<T>(s: impl Into<String>) -> Result<T, WorldError> { Err(WorldError::Save(s.into())) }

#[derive(Default)]
struct W(Vec<u8>);
impl W {
    fn u8(&mut self, v: u8) { self.0.push(v) }
    fn u16(&mut self, v: u16) { self.0.extend(v.to_le_bytes()) }
    fn u32(&mut self, v: u32) { self.0.extend(v.to_le_bytes()) }
    fn u64(&mut self, v: u64) { self.0.extend(v.to_le_bytes()) }
    fn i64(&mut self, v: i64) { self.0.extend(v.to_le_bytes()) }
    fn f32(&mut self, v: f32) { self.0.extend(v.to_le_bytes()) }
    fn f64(&mut self, v: f64) { self.0.extend(v.to_le_bytes()) }
    fn bytes(&mut self, b: &[u8]) { self.0.extend(b) }
    fn varint(&mut self, mut v: u32) { loop { let b = (v & 0x7f) as u8; v >>= 7; if v == 0 { self.0.push(b); break; } self.0.push(b | 0x80); } }
}

struct R<'a> { b: &'a [u8], p: usize }
impl<'a> R<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], WorldError> {
        if self.p + n > self.b.len() { return err("save is cut short"); }
        let s = &self.b[self.p..self.p + n]; self.p += n; Ok(s)
    }
    fn u8(&mut self) -> Result<u8, WorldError> { Ok(self.take(1)?[0]) }
    fn u16(&mut self) -> Result<u16, WorldError> { Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap())) }
    fn u32(&mut self) -> Result<u32, WorldError> { Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap())) }
    fn u64(&mut self) -> Result<u64, WorldError> { Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap())) }
    fn i64(&mut self) -> Result<i64, WorldError> { Ok(i64::from_le_bytes(self.take(8)?.try_into().unwrap())) }
    fn f32(&mut self) -> Result<f32, WorldError> { Ok(f32::from_le_bytes(self.take(4)?.try_into().unwrap())) }
    fn f64(&mut self) -> Result<f64, WorldError> { Ok(f64::from_le_bytes(self.take(8)?.try_into().unwrap())) }
    fn varint(&mut self) -> Result<u32, WorldError> {
        let (mut v, mut shift) = (0u32, 0);
        loop {
            let b = self.u8()?;
            if shift > 28 { return err("bad varint"); }
            v |= ((b & 0x7f) as u32) << shift;
            if b & 0x80 == 0 { return Ok(v); }
            shift += 7;
        }
    }
}

fn section(out: &mut W, tag: &[u8; 4], version: u16, payload: &[u8]) {
    let mut enc = DeflateEncoder::new(Vec::new(), Compression::default());
    enc.write_all(payload).expect("write to memory");
    let packed = enc.finish().expect("finish");
    let (codec, body): (u8, &[u8]) = if packed.len() < payload.len() { (1, &packed) } else { (0, payload) };
    out.bytes(tag);
    out.u16(version);
    out.u8(codec);
    out.u32(body.len() as u32);
    out.u32(payload.len() as u32);
    out.bytes(body);
}

pub fn write(sim: &Sim) -> Result<Vec<u8>, WorldError> {
    let w = &sim.world;
    let mut sections: Vec<([u8; 4], u16, Vec<u8>)> = Vec::new();

    let mut m = W::default();
    m.u64(w.clock.month_index); m.i64(w.clock.start_year); m.u64(w.seed); m.u32(w.grid.n);
    for v in w.initial { m.f64(v) }
    for v in w.boundary_net { m.f64(v) }
    m.u64(w.violation_count);
    sections.push((*b"META", 1, m.0));

    let mut s = W::default();
    s.u32(w.reg.stocks.len() as u32);
    for (i, def) in w.reg.stocks.iter().enumerate() {
        s.u16(def.name.len() as u16); s.bytes(def.name.as_bytes());
        let data = &w.cur[i];
        let mut p = W::default();
        match def.storage {
            Storage::Exact => { s.u8(0); for &v in data { p.f32(v) } }
            Storage::Sparse => {
                s.u8(1);
                let nz = data.iter().filter(|&&v| v.to_bits() != 0).count();
                p.u32(nz as u32);
                let mut last = 0usize;
                for (k, &v) in data.iter().enumerate() { if v.to_bits() != 0 { p.varint((k - last) as u32); p.f32(v); last = k; } }
            }
            Storage::Quantised(step) => {
                s.u8(2); s.f32(step);
                for &v in data {
                    let q = (v / step).round();
                    if q.is_nan() || q.abs() > 32767.0 { return err(format!("stock {} value {v} is out of range for step {step}", def.name)); }
                    p.u16((q as i16) as u16);
                }
            }
        }
        s.u32(p.0.len() as u32); s.bytes(&p.0);
    }
    sections.push((*b"STCK", 1, s.0));

    let mut rs = W::default();
    rs.u32(w.reg.stocks.len() as u32);
    for (i, def) in w.reg.stocks.iter().enumerate() {
        rs.u16(def.name.len() as u16); rs.bytes(def.name.as_bytes());
        let mut p = W::default();
        let nz = w.resid[i].iter().filter(|&&v| v.to_bits() != 0).count();
        p.u32(nz as u32);
        let mut last = 0usize;
        for (k, &v) in w.resid[i].iter().enumerate() { if v.to_bits() != 0 { p.varint((k - last) as u32); p.f32(v); last = k; } }
        rs.u32(p.0.len() as u32); rs.bytes(&p.0);
    }
    sections.push((*b"RSID", 1, rs.0));

    let mut md = W::default();
    let with_state: Vec<_> = sim.slots.iter().filter_map(|sl| sl.module.save_state().map(|b| (sl.res.name.clone(), b))).collect();
    md.u32(with_state.len() as u32);
    for (n, b) in &with_state { md.u16(n.len() as u16); md.bytes(n.as_bytes()); md.u32(b.len() as u32); md.bytes(b); }
    sections.push((*b"MODS", 1, md.0));

    let mut out = W::default();
    out.bytes(MAGIC); out.u16(FORMAT); out.u32(sections.len() as u32);
    for (tag, ver, payload) in &sections { section(&mut out, tag, *ver, payload); }
    Ok(out.0)
}

#[derive(Debug, Default, PartialEq)]
pub struct ReadReport {
    pub skipped_sections: Vec<String>,
    pub unknown_stocks: Vec<String>,
    pub defaulted_stocks: Vec<String>,
}

pub fn read_into(sim: &mut Sim, bytes: &[u8]) -> Result<ReadReport, WorldError> {
    let mut r = R { b: bytes, p: 0 };
    if r.take(4)? != MAGIC { return err("not an Odyssey save"); }
    let fmt = r.u16()?;
    if fmt > FORMAT { return err(format!("save format {fmt} is newer than this build ({FORMAT})")); }
    let count = r.u32()?;
    let mut report = ReadReport::default();
    let mut seen_stocks: Vec<bool> = vec![false; sim.world.reg.stocks.len()];
    let mut got_meta = false;

    for _ in 0..count {
        let tag: [u8; 4] = r.take(4)?.try_into().unwrap();
        let version = r.u16()?;
        let codec = r.u8()?;
        let stored = r.u32()? as usize;
        let raw_len = r.u32()? as usize;
        let body = r.take(stored)?;
        let tag_s = String::from_utf8_lossy(&tag).to_string();
        let known = matches!((&tag, version), (b"META", 1) | (b"STCK", 1) | (b"RSID", 1) | (b"MODS", 1));
        if !known { report.skipped_sections.push(format!("{tag_s} v{version}")); continue; }
        let payload: Vec<u8> = match codec {
            0 => body.to_vec(),
            1 => { let mut v = Vec::with_capacity(raw_len); DeflateDecoder::new(body).read_to_end(&mut v).map_err(|e| WorldError::Save(e.to_string()))?; v }
            c => return err(format!("section {tag_s} uses unknown codec {c}")),
        };
        if payload.len() != raw_len { return err(format!("section {tag_s} has the wrong length")); }
        let mut p = R { b: &payload, p: 0 };
        match &tag {
            b"META" => {
                let w = &mut sim.world;
                w.clock.month_index = p.u64()?; w.clock.start_year = p.i64()?; w.seed = p.u64()?;
                let n = p.u32()?;
                if n != w.grid.n { return err(format!("save is for grid n{n}, this world is n{}", w.grid.n)); }
                for v in w.initial.iter_mut() { *v = p.f64()?; }
                for v in w.boundary_net.iter_mut() { *v = p.f64()?; }
                w.violation_count = p.u64()?;
                got_meta = true;
            }
            b"STCK" => {
                let cells = sim.world.grid.len();
                let ns = p.u32()?;
                for _ in 0..ns {
                    let nl = p.u16()? as usize;
                    let name = String::from_utf8_lossy(p.take(nl)?).to_string();
                    let enc = p.u8()?;
                    let step = if enc == 2 { p.f32()? } else { 0.0 };
                    let plen = p.u32()? as usize;
                    let body = p.take(plen)?;
                    let Some(id) = sim.world.reg.stock(&name) else { report.unknown_stocks.push(name); continue };
                    let mut q = R { b: body, p: 0 };
                    let dst = &mut sim.world.cur[id.0 as usize];
                    match enc {
                        0 => { if plen != cells * 4 { return err(format!("stock {name} has the wrong size")); } for v in dst.iter_mut() { *v = q.f32()?; } }
                        1 => {
                            dst.iter_mut().for_each(|v| *v = 0.0);
                            let nz = q.u32()?;
                            let mut at = 0usize;
                            for k in 0..nz { let d = q.varint()? as usize; at = if k == 0 { d } else { at + d }; let v = q.f32()?; if at >= cells { return err(format!("stock {name} has a cell outside the grid")); } dst[at] = v; }
                        }
                        2 => { if plen != cells * 2 { return err(format!("stock {name} has the wrong size")); } for v in dst.iter_mut() { *v = (q.u16()? as i16) as f32 * step; } }
                        e => return err(format!("stock {name} uses unknown encoding {e}")),
                    }
                    seen_stocks[id.0 as usize] = true;
                }
            }
            b"RSID" => {
                let cells = sim.world.grid.len();
                let ns = p.u32()?;
                for _ in 0..ns {
                    let nl = p.u16()? as usize;
                    let name = String::from_utf8_lossy(p.take(nl)?).to_string();
                    let plen = p.u32()? as usize;
                    let body = p.take(plen)?;
                    let Some(id) = sim.world.reg.stock(&name) else { continue };
                    let mut q = R { b: body, p: 0 };
                    let dst = &mut sim.world.resid[id.0 as usize];
                    dst.iter_mut().for_each(|v| *v = 0.0);
                    let nz = q.u32()?;
                    let mut at = 0usize;
                    for k in 0..nz { let d = q.varint()? as usize; at = if k == 0 { d } else { at + d }; let v = q.f32()?; if at >= cells { return err(format!("rounding of {name} has a cell outside the grid")); } dst[at] = v; }
                }
            }
            b"MODS" => {
                let nm = p.u32()?;
                for _ in 0..nm {
                    let nl = p.u16()? as usize;
                    let name = String::from_utf8_lossy(p.take(nl)?).to_string();
                    let bl = p.u32()? as usize;
                    let b = p.take(bl)?;
                    if let Some(slot) = sim.slots.iter_mut().find(|s| s.res.name == name) { slot.module.load_state(b)?; }
                }
            }
            _ => unreachable!(),
        }
    }
    if !got_meta { return err("save has no META section"); }
    for (i, seen) in seen_stocks.iter().enumerate() {
        if !seen { sim.world.cur[i].iter_mut().for_each(|v| *v = 0.0); sim.world.resid[i].iter_mut().for_each(|v| *v = 0.0); report.defaulted_stocks.push(sim.world.reg.stocks[i].name.clone()); }
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;
    use crate::stocks::{Group, Registry, StockId};
    use crate::world::World;
    use std::sync::Arc;

    fn sim_with(storage: [Storage; 3]) -> Sim {
        let mut reg = Registry::new();
        reg.add_stock("dense", "mm", Group::Water, storage[0]);
        reg.add_stock("ice", "mm", Group::Water, storage[1]);
        reg.add_stock("shown", "mm", Group::Water, storage[2]);
        let mut w = World::new(Arc::new(Grid::new(10)), reg, 42, -125000);
        for (i, v) in w.stock_mut(StockId(0)).iter_mut().enumerate() { *v = (i as f32 * 0.37).sin() * 50.0 + 60.0; }
        for i in (0..1002).step_by(97) { w.stock_mut(StockId(1))[i] = i as f32 * 0.5; }
        for (i, v) in w.stock_mut(StockId(2)).iter_mut().enumerate() { *v = (i % 100) as f32 * 0.25; }
        w.reset_ledger();
        w.clock.month_index = 1234;
        Sim::new(w)
    }
    fn lossless() -> Sim { sim_with([Storage::Exact, Storage::Sparse, Storage::Exact]) }

    #[test]
    fn lossless_round_trip_is_bit_exact() {
        let a = lossless();
        let bytes = write(&a).unwrap();
        let mut b = sim_with([Storage::Exact, Storage::Sparse, Storage::Exact]);
        b.world.cur.iter_mut().for_each(|s| s.iter_mut().for_each(|v| *v = -1.0));
        let rep = read_into(&mut b, &bytes).unwrap();
        assert_eq!(rep, ReadReport::default());
        assert_eq!(a.fingerprint(), b.fingerprint());
        assert_eq!(b.world.clock.month_index, 1234);
        assert_eq!(b.world.clock.start_year, -125000);
    }

    #[test]
    fn sparse_is_smaller_than_exact_for_mostly_zero() {
        let sparse = write(&sim_with([Storage::Exact, Storage::Sparse, Storage::Exact])).unwrap().len();
        // compare the same data with the sparse stock stored dense and not compressed away
        let mut s = sim_with([Storage::Exact, Storage::Exact, Storage::Exact]);
        s.world.stock_mut(StockId(1)).iter_mut().enumerate().for_each(|(i, v)| *v = (i as f32).sqrt() * 3.1);
        let dense = write(&s).unwrap().len();
        assert!(sparse < dense, "{sparse} vs {dense}");
    }

    #[test]
    fn quantised_error_is_within_half_a_step() {
        let a = sim_with([Storage::Exact, Storage::Sparse, Storage::Quantised(0.1)]);
        let bytes = write(&a).unwrap();
        let mut b = sim_with([Storage::Exact, Storage::Sparse, Storage::Quantised(0.1)]);
        read_into(&mut b, &bytes).unwrap();
        for (x, y) in a.world.stock(StockId(2)).iter().zip(b.world.stock(StockId(2))) { assert!((x - y).abs() <= 0.05 + 1e-5); }
        assert_ne!(a.fingerprint(), b.fingerprint(), "lossy stocks change the fingerprint, as documented");
    }

    #[test]
    fn quantised_out_of_range_is_an_error_not_a_wrap() {
        let mut a = sim_with([Storage::Exact, Storage::Sparse, Storage::Quantised(0.001)]);
        a.world.stock_mut(StockId(2))[0] = 1.0e6;
        assert!(write(&a).is_err());
    }

    #[test]
    fn unknown_sections_and_stocks_are_skipped_missing_ones_default() {
        let full = lossless();
        let bytes = write(&full).unwrap();
        // reader knows fewer stocks: "ice" is unknown to it
        let mut reg = Registry::new();
        reg.add_stock("dense", "mm", Group::Water, Storage::Exact);
        reg.add_stock("shown", "mm", Group::Water, Storage::Exact);
        reg.add_stock("fresh", "mm", Group::Water, Storage::Exact);
        let mut w = World::new(Arc::new(Grid::new(10)), reg, 1, 0);
        w.stock_mut(StockId(2)).iter_mut().for_each(|v| *v = 9.0);
        let mut other = Sim::new(w);
        let rep = read_into(&mut other, &bytes).unwrap();
        assert_eq!(rep.unknown_stocks, vec!["ice"]);
        assert_eq!(rep.defaulted_stocks, vec!["fresh"]);
        assert!(other.world.stock(StockId(2)).iter().all(|&v| v == 0.0));
        assert_eq!(other.world.stock(StockId(0)), full.world.stock(StockId(0)));
    }

    #[test]
    fn an_extra_section_from_a_newer_writer_is_skipped() {
        let a = lossless();
        let bytes = write(&a).unwrap();
        let mut out = W::default();
        out.bytes(MAGIC); out.u16(FORMAT);
        let count_at = out.0.len();
        out.u32(0);
        let mut body = R { b: &bytes, p: 10 };
        let n = u32::from_le_bytes(bytes[6..10].try_into().unwrap());
        let mut kept = 0u32;
        for _ in 0..n { let start = body.p; body.take(4).unwrap(); body.u16().unwrap(); body.u8().unwrap(); let st = body.u32().unwrap() as usize; body.u32().unwrap(); body.take(st).unwrap(); out.bytes(&bytes[start..body.p]); kept += 1; }
        section(&mut out, b"FUTR", 7, b"from the future");
        section(&mut out, b"META", 99, b"newer meta layout");
        out.0[count_at..count_at + 4].copy_from_slice(&(kept + 2).to_le_bytes());
        let mut b = lossless();
        let rep = read_into(&mut b, &out.0).unwrap();
        assert_eq!(rep.skipped_sections, vec!["FUTR v7", "META v99"]);
        assert_eq!(a.fingerprint(), b.fingerprint());
    }

    #[test]
    fn damaged_saves_are_errors() {
        let a = lossless();
        let bytes = write(&a).unwrap();
        let mut b = lossless();
        assert!(read_into(&mut b, b"nope").is_err());
        assert!(read_into(&mut b, &bytes[..bytes.len() / 2]).is_err());
        let mut wrong_grid = Sim::new(World::new(Arc::new(Grid::new(8)), a.world.reg.clone(), 1, 0));
        assert!(read_into(&mut wrong_grid, &bytes).unwrap_err().to_string().contains("grid"));
    }
}
