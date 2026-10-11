//! The engine behind the desktop app. It talks in lines of JSON on standard input and output.
//!
//! In:  {"cmd":"load","n":24,"view":24,"seed":7}      start a story from the spun-up world (data/states/n24.spun)
//!      {"cmd":"load","n":24,"view":24,"file":"x"}    resume a kept story (<saves>/x.owstate)
//!      {"cmd":"run","pace":12}                        run month by month until "stop"; pace is months a second, 0 = flat out
//!      {"cmd":"pace","pace":60}                       change the pace while running
//!      {"cmd":"stop"}                                 stop after the current month
//!      {"cmd":"layer","name":"temp"}                  choose the map layer and send it
//!      {"cmd":"tile","id":123}                        everything about one tile
//!      {"cmd":"save","file":"x"}                      write the world to <saves>/x.owstate
//!      {"cmd":"quit"}
//! Out: ready, parents, sea, layer, stats, tile, saved, done, error.
//! <saves> is the folder in ODYSSEY_SAVES. File names are plain: letters, digits, '-' and '_'.

use odyssey_core::build::*;
use odyssey_core::save;
use odyssey_core::sim::Parallel;
use odyssey_core::view;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::{Duration, Instant};

fn say(s: &str) { let out = std::io::stdout(); let mut o = out.lock(); let _ = writeln!(o, "{s}"); let _ = o.flush(); }
fn err(m: &str) { say(&format!("{{\"event\":\"error\",\"message\":{:?}}}", m)); }

fn field(line: &str, key: &str) -> Option<String> {
    let k = format!("\"{key}\"");
    let i = line.find(&k)? + k.len();
    let rest = line[i..].trim_start().strip_prefix(':')?.trim_start();
    let end = rest.find(|c| c == ',' || c == '}').unwrap_or(rest.len());
    Some(rest[..end].trim().trim_matches('"').to_string())
}

fn save_path(line: &str) -> Result<PathBuf, String> {
    let f = field(line, "file").ok_or("no file name")?;
    if f.is_empty() || f.len() > 80 || !f.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') { return Err(format!("bad file name {f:?}")); }
    let dir = std::env::var_os("ODYSSEY_SAVES").map(PathBuf::from).ok_or("ODYSSEY_SAVES is not set")?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join(format!("{f}.owstate")))
}

struct World { b: Built, layer: String }

fn load(line: &str) -> Result<World, String> {
    let n: u32 = field(line, "n").and_then(|v| v.parse().ok()).unwrap_or(24);
    let view_n: u32 = field(line, "view").and_then(|v| v.parse().ok()).unwrap_or(n);
    let seed: Option<u64> = field(line, "seed").and_then(|v| v.parse().ok());
    let path = if field(line, "file").is_some() { save_path(line)? } else { data_dir().join("states").join(format!("n{n}.spun")) };
    let mut b = build(&Setup::today(n)).map_err(|e| format!("cannot build n{n}: {e}"))?;
    b.sim.parallel = Parallel::Auto(1);
    let bytes = std::fs::read(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    save::read_into(&mut b.sim, &bytes).map_err(|e| format!("cannot load {}: {e}", path.display()))?;
    if let Some(sd) = seed { b.sim.world.seed = sd; }
    let c = &b.sim.world.clock;
    say(&format!("{{\"event\":\"ready\",\"n\":{n},\"view\":{view_n},\"cells\":{},\"start_year\":{},\"months\":{},\"stats\":{}}}",
        b.sim.world.grid.len(), c.start_year, c.month_index, view::stats_json(&b)));
    if view_n != n { say(&view::parents_event(n, view_n)); }
    say(&view::sea_event(&b));
    Ok(World { b, layer: String::from("temp") })
}

/// Commands that work whether or not a run is going. Returns false for ones it does not handle.
fn handle(w: &mut Option<World>, line: &str) -> bool {
    match field(line, "cmd").unwrap_or_default().as_str() {
        "layer" => {
            let Some(w) = w.as_mut() else { err("start a world first"); return true };
            if let Some(nm) = field(line, "name") { w.layer = nm; }
            match view::layer_event(&w.b, &w.layer) { Some(l) => say(&l), None => err(&format!("no layer {}", w.layer)) }
        }
        "tile" => {
            let id: usize = field(line, "id").and_then(|v| v.parse().ok()).unwrap_or(usize::MAX);
            match w { Some(w) if id < w.b.sim.world.grid.len() => say(&view::tile_json(&w.b, id)), _ => err("no such tile") }
        }
        "save" => {
            let Some(w) = w.as_ref() else { err("start a world first"); return true };
            let r = save_path(line).and_then(|p| {
                let bytes = save::write(&w.b.sim).map_err(|e| e.to_string())?;
                let tmp = p.with_extension("tmp");
                std::fs::write(&tmp, &bytes).map_err(|e| e.to_string())?;
                std::fs::rename(&tmp, &p).map_err(|e| e.to_string())?;
                Ok(p)
            });
            match r { Ok(_) => say(&format!("{{\"event\":\"saved\",\"file\":{:?},\"months\":{}}}", field(line, "file").unwrap_or_default(), w.b.sim.world.clock.month_index)), Err(e) => err(&format!("save failed: {e}")) }
        }
        _ => return false,
    }
    true
}

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    std::thread::spawn(move || { for l in std::io::stdin().lock().lines().map_while(Result::ok) { if tx.send(l).is_err() { break; } } });
    let mut world: Option<World> = None;
    loop {
        let Ok(line) = rx.recv() else { return };
        if handle(&mut world, &line) { continue; }
        match field(&line, "cmd").unwrap_or_default().as_str() {
            "quit" => return,
            "load" => { world = None; match load(&line) { Ok(w) => { if let Some(l) = view::layer_event(&w.b, &w.layer) { say(&l); } world = Some(w); } Err(e) => err(&e) } }
            "run" => {
                if world.is_none() { err("start a world first"); continue; }
                let mut pace: f64 = field(&line, "pace").and_then(|v| v.parse().ok()).unwrap_or(12.0);
                let start = Instant::now();
                let mut done_months = 0f64;
                let (mut last_stats, mut last_map) = (Instant::now() - Duration::from_secs(1), Instant::now() - Duration::from_secs(1));
                'run: loop {
                    {
                        let w = world.as_mut().unwrap();
                        w.b.sim.step_month();
                        let now = Instant::now();
                        // the map every month when slow, else about four times a second
                        if pace > 0.0 && pace <= 6.0 || now - last_map >= Duration::from_millis(250) {
                            if let Some(l) = view::layer_event(&w.b, &w.layer) { say(&l); }
                            last_map = now;
                        }
                        if pace > 0.0 && pace <= 24.0 || now - last_stats >= Duration::from_millis(100) {
                            say(&format!("{{\"event\":\"stats\",\"stats\":{}}}", view::stats_json(&w.b)));
                            last_stats = now;
                        }
                    }
                    done_months += 1.0;
                    // hold to the pace, still answering commands while waiting
                    loop {
                        while let Ok(l) = rx.try_recv() {
                            if handle(&mut world, &l) { continue; }
                            match field(&l, "cmd").unwrap_or_default().as_str() {
                                "stop" => break 'run,
                                "quit" => return,
                                "pace" => { pace = field(&l, "pace").and_then(|v| v.parse().ok()).unwrap_or(pace); }
                                "load" | "run" => { err("stop the run first"); }
                                _ => {}
                            }
                        }
                        if pace <= 0.0 { break; }
                        let due = Duration::from_secs_f64(done_months / pace);
                        let el = start.elapsed();
                        if el >= due { break; }
                        std::thread::sleep((due - el).min(Duration::from_millis(30)));
                    }
                    // a pace change restarts the budget so the clock does not race to catch up
                    if pace > 0.0 && start.elapsed().as_secs_f64() > done_months / pace + 0.5 { done_months = start.elapsed().as_secs_f64() * pace; }
                }
                let w = world.as_ref().unwrap();
                say(&format!("{{\"event\":\"stats\",\"stats\":{}}}", view::stats_json(&w.b)));
                if let Some(l) = view::layer_event(&w.b, &w.layer) { say(&l); }
                say("{\"event\":\"done\"}");
            }
            "stop" | "pace" | "" => {}
            other => err(&format!("unknown command {other}")),
        }
    }
}
