# Decisions

Choices I made on my own while building Odyssey. The handoff left them open, or the plan did not reach the detail. Each one says what I chose and why. Where a number is a guess, it says so.

## Starting state

The handoff suggested a start at the last interglacial, about 125,000 years ago. I did not do that for the first build.

- Today's bed already holds today's erosion and rebound. Today's ice already presses on that bed.
- The world starts as today: today's bed, today's ice as base load, today's temperature as the base of the climate.
- Land is seeded with a small amount of every plant type the climate allows. The soil pools start from a simple guess by climate.
- A spin-up holds CO2 at 350 ppm (the `nudge` module) while the land fills with carbon. The carbon the nudge supplies is booked in its own account. The ledger resets when the spin-up ends.

The 125 ka start needs a long run from a saved spin-up state. The `proof` program does that: it can run the spin-up once, save it, and start later runs from it.

## River resolution

A 48 km tile is far wider than a river valley. A stream power law with a plain tile slope would erode too little. The rule I used:

- Rivers follow a priority-flood tree on the live surface, with lakes that fill and spill.
- Stream power runs along that tree, implicit in time, with a slope that takes the drop to the next tile down the tree. It does not use the tile-average slope.
- A hillslope term carries soil off each tile by diffusion. This is the sub-grid part.
- The erodibility K is fitted once, on the first full year of real runoff, so that total erosion equals 19 Gt/yr (about 19 Gt/yr of pre-dam river sediment reaches the sea, attributed to Milliman and Farnsworth 2011 by a secondary source; I did not read the book).
- Uplift is then inverted from today's landscape so that today's land stands still. This is a steady-state assumption. It is wrong for young mountain belts and right on average.

## Grid size

Tile width is about the square root of 510 million km2 divided by (10 n^2 + 2). That gives n24 about 300 km, n48 about 150 km, n80 about 90 km and n160 about 45 km.

- n160 is the standard size (the "48 km" of the plan). It is too slow for 10,000-year runs on this machine: about 20 s per simulated year, so about 55 hours for one run.
- The long runs use n48 (150 km tiles): about 1.8 s per simulated year. Tests use n24.
- The 16 km level (n480) is removed. It had no use at these speeds.
- Results at 150 km tiles are coarse. Rivers, coasts and small lakes are blurred. Climate, carbon and ecology totals do not need finer tiles, and that is what the long run checks.

## Datasets

I picked these myself. The user said to use my best guess.

| Use | Dataset |
|---|---|
| Land and sea bed | ETOPO1 (NOAA) |
| Temperature and rain on land | WorldClim 2.1 |
| Sea surface temperature | ERSST v5 |
| Air temperature over ice and oceans | NCEP/NCAR reanalysis |
| Rain over the sea | GPCP |
| Ice thickness | ETOPO1 ice surface minus bed |
| Earthquakes | USGS ComCat, M5.5 and above, 1900 to 2024 |
| Eruptions | NOAA NCEI significant eruptions database |

Targets that are single numbers (global erosion, forest carbon by biome, storm counts, ENSO size) come from papers. I could check some of them on the web. Those I could not check are marked in `docs/VERIFIED.md`. No gridded dataset for uplift, river loads, soil depth, land cover or biomass was downloaded. The plan's "to choose" list stays open for those. The tests use global and biome-level numbers instead.

## Simplifications that change results

- Nitrogen and phosphorus are lumped. Each tile has one organic N stock and one organic P stock, moved with carbon by fixed C:N and C:P ratios. Plant uptake takes from a mineral pool. There is no separate leaf, wood and root N.
- Shrub is a derived type. Its traits are a mix of grass and tree values.
- The heat capacity class of a tile (land or ocean) is static. A tile that floods keeps its land heat capacity until `ocean0` changes.
- Climate has no clouds, no water vapour feedback and no ocean circulation, other than the single AMOC box. The restoring term B stands in for them. This is a known risk.
- Fire calibration depends on grid size. The lightning constant was tuned at n24.
- Marine phosphorus is a closed box. No rivers add to it. No burial removes from it.
- Fish are two classes (small and large). There is no size spectrum.
- Land animals are twelve guilds. Their rates follow metabolic theory. The coefficients are tuned, not measured, and flagged as unverified.

## Not built

- Glacial erosion and moraines. Ice sheets do not scour the bed or carry rock.
- Wind erosion and loess.
- Waves and coasts.
- Meanders and river avulsion (below one tile).
- Cyclone damage to plants, storm surge.
- Marine nutrients other than phosphorus. No iron limit, no oxygen.
- People.

## What the long run checks

See `docs/VERIFIED.md`.
