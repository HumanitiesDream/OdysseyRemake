# Odyssey: redesign handoff

Read this first. It lets a new chat, account or model pick up the redesign where it stopped.

Written 10 Oct 2026. Last shipped version: v0.49.1 (bug and speed pass, see `reference/v0.49.1-HANDOFF.md`).

## Where things stand

Odyssey is a preindustrial world simulator: a Rust no_std engine compiled to WASM, in an Electron app. It looks good. The owner felt something underneath was broken, and a review this session found what: the world is mostly played back, not grown.

- The land never changes shape. Height is today's heightmap minus ice sag.
- Climate comes from an emulator fitted to climate-model snapshots.
- Plants move toward a target map. Animals do not eat the plants. Two grids that do not match.
- Quakes, cyclones and volcanoes are dice on a map. Fires are capped in size and time.
- Water, carbon, sediment and nutrients are not conserved. Caps and clamps hold things in line.
- People were added on top of the world, not into it.

The decision: rebuild from the ground up. A living, dynamic world first, where everything feeds into everything, and people only after the world passes its long-run tests.

The full design is in `REDESIGN-PLAN.md`. Nothing of the rebuild is coded yet.

## What is in this zip

- `HANDOFF.md`: this file.
- `REDESIGN-PLAN.md`: the whole redesign plan, exported from the live doc.
- `reference/v0.49.1-HANDOFF.md`: the old engine's handoff (layout, build, tests, history).
- `reference/research/`: the research notes from the source. Keep using them.

Live docs (the owner can edit them, they may be newer than this zip):

- Redesign plan: https://claude.ai/code/artifact/562f168f-dd48-4b09-a9df-092a04791d87
- v0.50 design (the old people-era plan, now on hold; it holds the Bronze Age buildings, goods and resources research): https://claude.ai/code/artifact/65e80bb3-d3db-496f-8a08-2c30d2089f46

The v0.49.1 source and setup zips were delivered separately. Work from that source.

## How the owner likes to work

- Plain English, short sentences, active voice, no emojis, no filler. Say what was done, not how hard it was.
- Be blunt and honest. "Be critical, don't BS me." Give opinions.
- Work on your own when they are away. Make sensible calls and say which reading you took.
- Be honest about limits and measured numbers. Mark figures from memory. Review your own work with a separate agent before claiming done.
- Two map sizes only: 48 km (n160) and 16 km (n480). 16 km runs fine on the owner's machine and is the main play scale. 48 km is for development and long test runs.
- The owner likes how the world looks. Keep the look.
- Climate stays a semi-simulation: keep the emulator, add feedbacks. Not a supercomputer.
- Meteors are dropped.

## The core rules of the rebuild

1. Everything is a stock on one grid family. Systems only move amounts between stocks, booked on both sides. Nothing appears or vanishes.
2. No targets, no caps, no scripts. Disasters are stocks that fill and release. Randomness only where nature is random (lightning, exact timing).
3. One clock, units in the types. No time warp.
4. Baked Earth data is the starting state and the test, never the driver.
5. Modules declare what they read and write, run in parallel, each on its own pace and grid.
6. Content is data files with a source on every value.

## Order of work

The plan's stages ("Stages and order of work": physical world, living land and sea, animals, long-run proof, people), broken into build steps. Each step passes its tests before the next starts.

1. Groundwork: grid family, modules and scheduler, units, saves, budget checks, content files.
2. Climate: emulator with feedbacks, weather generator, ocean boxes, sea ice, water cycle.
3. Geology: ice sheets, crust, landscape, weathering, coasts, disaster stocks.
4. Ecology: height bands, plant types, soil, herbivores eating plants, food web, sea, carbon.
5. Long-run proof: 10,000-year runs on two seeds against real data.
6. Only then people, as one more species.

## First things to settle

These block other work. Settle them before writing system code.

1. Starting state. Today's heightmap already holds today's erosion and rebound. Pick the spin-up method. The plan suggests starting at the last interglacial, about 125,000 years ago.
2. River resolution. Decide the sub-grid rule that lets the stream power law work on 48 km tiles.
3. Datasets. Choose the ones marked "to choose" in "Data to test against": uplift rates, river sediment loads, soil depth, land cover and biomass.
4. Check every figure marked "from memory" before it becomes a target.

## Known risks

Listed in full in the plan. The top three: the climate feedbacks may be too weak or run away; speed with all systems live; and scope, since this is a rewrite of the whole engine.
