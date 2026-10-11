//! The engine behind the desktop app. It talks in lines of JSON on standard input and output.
//!
//! In:  {"cmd":"load","n":24}            load a saved, spun-up world (data/states/n24.spun)
//!      {"cmd":"run","years":50,"maps_every":10}   run; stats every year, maps every `maps_every` years and at the end
//!      {"cmd":"stop"}                   stop a run after the current year
//!      {"cmd":"maps"}                   send the maps now
//!      {"cmd":"quit"}
//! Out: {"event":"ready",...} {"event":"stats",...} {"event":"maps",...} {"event":"done"} {"event":"error","message":"..."}

use odyssey_core::build::*;
use odyssey_core::sim::Parallel;
use odyssey_core::{save, view};
use std::io::{BufRead, Write};
use std::sync::mpsc;

fn say(s: &str) { let out = std::io::stdout(); let mut o = out.lock(); let _ = writeln!(o, "{s}"); let _ = o.flush(); }
fn err(m: &str) { say(&format!("{{\"event\":\"error\",\"message\":{:?}}}", m)); }

fn field(line: &str, key: &str) -> Option<String> {
    let k = format!("\"{key}\"");
    let i = line.find(&k)? + k.len();
    let rest = line[i..].trim_start().strip_prefix(':')?.trim_start();
    let end = rest.find(|c| c == ',' || c == '}').unwrap_or(rest.len());
    Some(rest[..end].trim().trim_matches('"').to_string())
}

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    std::thread::spawn(move || { for l in std::io::stdin().lock().lines().map_while(Result::ok) { if tx.send(l).is_err() { break; } } });
    let mut world: Option<(Built, Vec<usize>)> = None;
    let mut layer_name = String::from("temp");
    let mut queue: std::collections::VecDeque<String> = Default::default();
    loop {
        let line = match queue.pop_front() { Some(l) => l, None => match rx.recv() { Ok(l) => l, Err(_) => return } };
        let cmd = field(&line, "cmd").unwrap_or_default();
        match cmd.as_str() {
            "quit" => return,
            "load" => {
                let n: u32 = field(&line, "n").and_then(|v| v.parse().ok()).unwrap_or(24);
                let seed: Option<u64> = field(&line, "seed").and_then(|v| v.parse().ok());
                let path = data_dir().join("states").join(format!("n{n}.spun"));
                let mut s = Setup::today(n);
                if let Some(sd) = seed { s.seed = sd; }
                match build(&s) {
                    Err(e) => err(&format!("cannot build n{n}: {e}")),
                    Ok(mut b) => {
                        b.sim.parallel = Parallel::Auto(1);
                        match std::fs::read(&path).map_err(|e| e.to_string()).and_then(|bytes| save::read_into(&mut b.sim, &bytes).map(|_| ()).map_err(|e| e.to_string())) {
                            Err(e) => err(&format!("cannot load {}: {e}", path.display())),
                            Ok(()) => {
                                if let Some(sd) = seed { b.sim.world.seed = sd; }
                                let pix = Vec::new();
                                let view_n: u32 = field(&line, "view").and_then(|v| v.parse().ok()).unwrap_or(n);
                                say(&format!("{{\"event\":\"ready\",\"n\":{n},\"view\":{view_n},\"cells\":{},\"stats\":{}}}", b.sim.world.grid.len(), view::stats_json(&b)));
                                if view_n != n { say(&view::parents_event(n, view_n)); }
                                say(&view::sea_event(&b));
                                if let Some(l) = view::layer_event(&b, &layer_name) { say(&l); }
                                world = Some((b, pix));
                            }
                        }
                    }
                }
            }
            "layer" => {
                if let Some(nm) = field(&line, "name") { layer_name = nm; }
                match &world { Some((b, _)) => { if let Some(l) = view::layer_event(b, &layer_name) { say(&l); } } None => err("load a world first") }
            }
            "tile" => {
                let id: usize = field(&line, "id").and_then(|v| v.parse().ok()).unwrap_or(0);
                match &world { Some((b, _)) if id < b.sim.world.grid.len() => say(&view::tile_json(b, id)), _ => err("no such tile") }
            }
            "maps" => match &world { Some((b, pix)) => say(&format!("{{\"event\":\"maps\",\"maps\":{}}}", view::maps_json(b, pix))), None => err("load a world first") },
            "run" => {
                let Some((b, pix)) = world.as_mut() else { err("load a world first"); continue };
                let years: u64 = field(&line, "years").and_then(|v| v.parse().ok()).unwrap_or(10);
                let every: u64 = field(&line, "maps_every").and_then(|v| v.parse().ok()).unwrap_or(10).max(1);
                'run: for y in 1..=years {
                    b.sim.run_years(1);
                    say(&format!("{{\"event\":\"stats\",\"stats\":{}}}", view::stats_json(b)));
                    if y % every == 0 { if let Some(l) = view::layer_event(b, &layer_name) { say(&l); } }
                    while let Ok(l) = rx.try_recv() {
                        match field(&l, "cmd").unwrap_or_default().as_str() { "stop" => break 'run, "quit" => return, _ => queue.push_back(l) }
                    }
                }
                if let Some(l) = view::layer_event(b, &layer_name) { say(&l); }
                say("{\"event\":\"done\"}");
            }
            "stop" => {}
            "" => {}
            other => err(&format!("unknown command {other}")),
        }
    }
}
