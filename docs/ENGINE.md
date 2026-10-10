# Engine, stage 1: groundwork

This is step 1 of the build order in `redesign/HANDOFF.md`. It holds no climate, geology or ecology yet. It holds the frame those systems plug into, plus tests that prove the frame.

## Layout

- `crates/core/src/grid.rs`: the Goldberg hex grid (10n²+2 cells), real cell areas, and coarse levels with exact sum-up and split-down.
- `stocks.rs`: the stock registry. One array per stock, `f32` density per km² per tile. Totals sum in `f64` as density × area.
- `flows.rs`: the only way to change a stock. A flow moves an amount between two endpoints. Endpoints are a stock on a tile, an outside account (sun, space), or a transit junction.
- `world.rs`: validates and applies flows, keeps the books, runs the audit.
- `module.rs`, `sim.rs`: the module interface and the scheduler.
- `content.rs` and `data/*.json`: content files with a source on every value.
- `save.rs`: tagged, versioned, compressed sections.
- `units.rs`, `rng.rs`: typed units, one clock, deterministic streams.
- `demo.rs`: three placeholder modules (energy balance, heat mixing, water). They exist to exercise the frame. They are not the climate model.
- `bin/world.rs`: headless runner. `cargo run --release --bin world -- --n 20 --years 100`.

## Rules the code enforces

1. Modules return flows. They cannot edit a stock. The world divides each amount by tile area on both ends, so nothing appears or vanishes.
2. A flow may only move matter inside one group (water, heat, carbon, nitrogen, phosphorus, sediment). Mixed groups are refused.
3. A module declares its reads and writes. An undeclared read or write drops that module's output for the month and logs it.
4. Flows between grids go through a transit junction. Per group, what goes in must come out, or the output is refused.
5. No clamps. A stock driven below zero is counted as a violation. Rounding-sized overdraws are kept as debt on the books.
6. `f32` storage carries its own rounding forward, so a 10,000-year run does not drift.
7. A module that errors or panics is skipped for the month. The run goes on.
8. Same seed, same fingerprint, with or without threads.

## Tests

`cargo test --release` runs 47 tests in about 2 seconds. They cover grid geometry, coarse-grid conservation, flow validation, scheduler periods, thread determinism, content checks, save round trips, and whole-world runs on the demo modules.

`cargo test --release --test demo_world -- --ignored --nocapture` runs 10,000 years on two seeds (about 70 seconds). Measured on the demo world at n10: water and heat residuals stay under 5e-12, with zero violations.

## Measured, not promised

- n160 has 256,002 cells and n480 has 2,304,002. Tests check the first.
- The three demo modules take about 100 ms a month at n160 on one thread and about 57 ms with threads. Most of that is `exp` and trig per tile in placeholder code. It says nothing about the real budget.
- The demo climate settles near 294 K. The albedo, clouds and heat transport are placeholders.
- Resolution check: an all-sea world at n12 and n24 agrees within 1.5 K after 40 years.

## Where this differs from the plan

- **Parallel rule.** The plan says modules that share no writes run together. Here flows are additive deltas, so write conflicts cannot happen and every due module runs at once. The real risk left is two modules each drawing most of one stock. That shows up as a violation. Nothing prevents it yet.
- **No second buffer.** Modules read the live arrays during the step. The world applies flows only after all modules finish. The effect is the same as double buffering, with less memory.
- **Phases order the apply step only.** Every module reads the start-of-month state, as the plan says.
- **Explicit diffusion fails at tile size.** An explicit heat step at 48 km is unstable by roughly a factor of 100. The demo mixing module runs on a coarse grid with an implicit solve. The real climate module will need the same.
- **Threads are spawned per month.** A thread pool comes when real modules have enough work to justify it.

## Open items from the handoff

These still block system code and need the owner's call or data:

1. Starting state and spin-up method (the plan suggests the last interglacial).
2. River resolution rule for 48 km tiles.
3. Datasets marked "to choose": uplift rates, river sediment loads, soil depth, land cover and biomass.
4. Every figure marked "from memory". The content loader counts them (`Content::unverified`). Today that is the four Heimsath soil production figures, and the GLiM class list and North energy-balance constants are flagged in their sources.

Not done in this stage: baked Earth data loading, the WASM build, generated JS bindings, the Electron app and globe. The v0.49 source stays the reference for the renderer.
