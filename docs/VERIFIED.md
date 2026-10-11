# What was measured

Measured numbers from this build. Each line says pass, fail or guess. Nothing here is a target the model was tuned to hit unless it says so.

## Long run: 10,000 years, two seeds

Setup: grid n48 (about 150 km tiles), seeds 1 and 2, 3,000-year spin-up with CO2 held at 350 ppm, then 10,000 free years. Data: `docs/runs/n48_seed1.csv` and `n48_seed2.csv`. Each run took about 2.5 hours.

Passed:
- Both runs finish. No module is skipped. The books close for water, heat, carbon, nitrogen, phosphorus, sediment and crust (residual about 1e-12 of the total or smaller).
- The two seeds end in the same place: mean temperature 286.16 K, CO2 about 325 ppm, vegetation 472 PgC, soil 2,320 PgC, animals 2.2 PgC.
- Same seed gives the same result with or without threads. A January save resumes to the same result (tests/determinism.rs).

Did not pass cleanly:
- The world is not at rest after 10,000 years. CO2 falls from 350 to 325 ppm. Vegetation falls from 513 to 472 PgC. The ice sheets lose 3.3 m of sea-level equivalent, and sea level rises about 3 m. All three are still moving at the end, slowly.
- About 315 small violations per run (negative organic nitrogen or phosphorus of 1e-6 to 1e-4 kg/m2 on a cluster of tiles). They do not grow fast, and no module stopped.
- A first pair of runs failed. After about 8,300 years the water and soil modules stopped every month and the world collapsed to 81 ppm CO2. The cause was a lake spill that could overdraw a tile. It is fixed, and the results above are from the fixed code.
- The runs do not include a real glacial cycle. Reaching the Last Glacial Maximum needs the 125,000-year start the plan suggests. I did not run it.

## Short checks (tests that pass)

| Check | Result |
|---|---|
| Base climate stays put, 30 years | drift under 0.1 K |
| Whole model stays near today's climate | mean departure 0.00 K, worst tile 3.3 K |
| CO2 doubling, 350 to 700 ppm, 300 years | +2.53 K (band allowed 1.5 to 4.5, set by the chosen feedback) |
| Glacial CO2 (190 ppm) and orbit, today's ice | -1.96 K. Tierney et al. 2020 give -6.1 K with ice sheets and dust. This run is only part of that |
| Carbon pulse of 100 GtC | 57% still in air after 100 years |
| CO2 with no forcing | 350.0 to 350.0 ppm in 60 years |
| Crust under load | sag 864 m against 834 m by local isostasy. A forebulge forms. 37% of the sag remains after 3,000 years |
| River sediment | 23.2 Gt/yr moved. Fitted to 19 Gt/yr, so this is a calibration, not a test |
| Land height | -0.003 m in 50 years. True by construction, because uplift is inverted from the same year |
| Earthquakes per year, model vs record | M6+ 113 vs 119.5. M7+ 9.6 vs 13.5. M8+ 0.47 vs 0.86. The record also sets each tile's supply, so this is partly circular |
| Eruptions in 6,000 years | VEI 6 or larger: 32 |
| ENSO | standard deviation 0.85 to 1.0 K, period about 4 years. The noise strength is set to give about 0.9 K |
| Storms | 88 born a year, 38 landfalls (n24). The birth rate is set to give about 80 |
| AMOC | steady at today's state. 0.10 Sv slows it. 0.25 Sv switches it off, and it stays off when the water stops |
| Fire, n48 | 345 Mha/yr after the last constant change (GFED4: 348). The constant was set for this. The top 1% of fires cover only 5 to 6% of the burned area, which is far less heavy-tailed than real fires |

## Ecology checks

- Herbivore biomass is highest at 200 to 1,000 mm of rain, and lower in desert and in wet forest. Pass.
- Taking the herbivores away raises the area burned by 10% (3.51 to 3.85 million km2 a year). Pass in direction.
- Starving the grazers of grass cuts their carbon by half in 3 years. Pass.
- Predator biomass against prey biomass: log-log slope 1.83. The literature (Hatton et al. 2015) gives about 0.75. **Fail.** The test only asserts a positive slope.
- Savanna is thin. Annual mean grass leaf carbon is about 0.007 kgC/m2, against about 0.05 in real grassland. Trees take over above 400 mm of rain. **Weak.**
- Forest carbon is below the literature (tropical forest about 9 against 15 to 19 kgC/m2 in earlier runs). Total vegetation (472 to 517 PgC) is inside the usual 450 to 650 PgC range.

## Figures I checked on the web

- Tierney et al. 2020: LGM global cooling 6.1 K (95% range 5.7 to 6.5). Confirmed.
- Lambeck et al. 2014: LGM sea level 134 m below today. Confirmed.
- River sediment to the sea before dams, about 19 Gt/yr, attributed to Milliman and Farnsworth 2011. Confirmed through a secondary source. I did not read the book.
- Outgoing longwave A = 203.3 W/m2 and B = 2.09 W/m2/K. Confirmed (a University of Washington handout).
- Not confirmed: the diffusion constant 0.649 in the stage-1 demo (the model uses 0.55). The two-letter GLiM rock codes in `data/rock_types.json` (the 16 classes are confirmed).
- LPJmL plant traits are marked verified in `data/plant_types.json`. I took them from the LPJmL parameter file named in their source field. That file is not in the repo.

## Guesses

Every value with `verified: false` in `data/animal_guilds.json` (66) and `data/plant_types.json` (85) is my assumption. The tuned constants are listed in `docs/DECISIONS.md`.

## Independent review

An independent review agent read the code. Its findings are in `docs/DECISIONS.md` under "What baked data still drives", "Calibrations and tuned constants" and "Known gaps found in review". The main one: temperature and rain still follow today's observed maps. Tests that say "stays near the base" pass partly for that reason.
