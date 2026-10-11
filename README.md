# Odyssey rebuild

A living-world simulator, rebuilt from the ground up. The world comes first, people later.

Everything is a stock on one grid. Systems move booked amounts between stocks. No targets, no caps, no scripts.
The plan is in `docs/redesign/`.

## What runs today

Rust engine in `crates/core`. Modules, each with declared reads and writes:

- Climate: orbit, CO2, albedo (snow, ice, sea ice, plants), ocean heat storage, ENSO, weather noise, AMOC, cyclones.
- Water and ice: rain, snow, melt, rivers and lakes, ice flow, sea level, crust rebound.
- Land shape: erosion, deposition, weathering, quakes and volcanoes.
- Carbon, nitrogen, phosphorus: air, sea, soil, plants.
- Life: ten plant types, soil, fire, seed spread, twelve land animal guilds, plankton and two fish classes.

Not built yet: people, glacial erosion, wind and coasts. See `docs/DECISIONS.md`.

## Documents

- `docs/ENGINE.md`: the engine rules.
- `docs/DECISIONS.md`: choices I made and why.
- `docs/VERIFIED.md`: what was measured, what passed, what is a guess.
- `docs/redesign/`: the original plan and handoff.

## Build and test

    cargo test --release          # about 3 minutes
    cargo run --release --bin proof -- --n 48 --seed 1 --spin 3000 --years 10000 --every 100 --out run.csv

`proof` spins the world up, then runs it and writes one line per interval. Grid sizes: n24 (about 300 km tiles), n48 (150 km), n80 (90 km), n160 (45 km, the standard size). Baked Earth data for n24 and n48 is in `data/baked`. Others come from `bake` (see `tools/prep_rasters.py`).

## The Windows app

`app/` is an Electron window around the Rust engine (`crates/core/src/bin/engine.rs`). It loads a spun-up world, runs it for the years you choose, and draws maps and charts as it goes.

To get the exe: open the repository on GitHub, go to Actions, pick the "Windows app" run for your branch (or press "Run workflow"), and download the `Odyssey-windows` artifact. It holds an installer and a portable exe. Nothing else needs installing.

To run it from source on any computer with Rust and Node: `cargo build --release --bin engine`, then `cd app && npm install && npm start`.
