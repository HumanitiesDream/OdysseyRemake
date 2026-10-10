# Odyssey redesign plan

10 Oct 2026

## Goal

The first goal is a living, dynamic world, before any people exist. The world must run on its own and stay believable for thousands of years without scripts, caps or hand fixes. People come later, as one more species inside it.

The core rule: everything feeds into everything. No system only reads the world. Each one takes from shared stocks and gives back to them, so a change anywhere spreads everywhere.

A living world means:

- **Flows that balance.** Energy, water, carbon and nutrients move between air, sea, soil, plants and animals, and none of them appears or vanishes.
- **Feedback, not rules.** Forests make rain and hold soil. Grazers shape grassland. Fire, drought and disease thin what grew too fast. Collapse and recovery come out of these links.
- **Memory.** Soil, forests, herds, ice and lakes build up and run down over decades and centuries. Yesterday shapes today.
- **One clock.** Every process runs on the same real time. No time warp that knocks slow and fast processes out of step.
- **Grown, not played back.** Climate and seasons come from the physics where we can afford it. Baked Earth data sets the starting land and checks the results, but does not drive the world.

The test: leave the world alone for 10,000 years from the same start, twice with different seeds. Both runs should stay alive, differ in their details, and still match real Earth in the large.

## What we keep

- **The look.** The globe, map modes, palette and UI style carry over.
- **Baked Earth data** as starting state and tests: heightmaps (n160, n480), rock types, soils, today's climate, floods, hazards, paleo snapshots.
- **Real physics that already works:** the orbit (Berger 1978), the 4,000-year mantle relaxation, snow melt by degree-days, the whale and seal dynamics in the sea.
- **Rust to WASM, in an Electron app.**
- **Tools:** the fingerprint tool for proving speed-ups exact, the check scripts as the model for new tests, `world_node.js` for headless runs.
- **The `research/` notes** and the habit of a source behind every number.
- **Set aside, not ported:** tribes, polities, courts, economy and the people side of disease and culture. Their research notes stay for later.

## Architecture

The world is a set of shared stocks on one grid. Systems never own the world's state. They only move amounts between stocks, and every move is booked on both sides, so nothing appears or vanishes.

**Shared stocks per tile:** water (air, soil, snow, ice, rivers, lakes, sea), heat, carbon (air, plants, soil, sea), nutrients (nitrogen, phosphorus), sediment, plant biomass by type, animal biomass by group, and later people, herds, fields and buildings.

**What feeds what.** Each row takes from and gives to the columns marked.

| System | Climate | Water | Soil | Plants | Animals | Sea | People |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Climate |  | rain, snow, melt | weathering | growth, stress | heat, cold | currents, ice | harvests, health |
| Water | humidity, ice cover |  | erosion, salt | drought, flood | drinking, habitat | river inflow | wells, floods |
| Soil | dust, carbon | holding, runoff |  | nutrients | burrows, grazing | silt | fertility |
| Plants | rain recycling, albedo, CO2 | transpiration | litter, roots |  | food, cover |  | food, fuel, timber |
| Fire | smoke, CO2 | runoff after burns | ash, erosion | clears, renews | kills, opens land |  | burns fields |
| Animals | methane | trampling | dung, trampling | grazing, seeds | prey, disease |  | meat, hides, disease |
| Sea | heat, moisture |  |  |  | fish, shore life |  | fish, salt, routes |
| People | land clearing, fire | dams, canals | ploughing, salt | clearing, farming | hunting, herding | fishing |  |

Every empty cell is a coupling to justify leaving out, not a default.

**One clock.** All systems step on the same real month. Slow systems (ice sheets, soil) may update yearly but book their flows monthly.

**Order within a month:** climate, then water, then soil and plants, then animals, then people. Each reads last month's stocks and writes this month's, so the order doesn't bias the result.

### Modules

Each system is a module that declares which stocks it reads and which flows it writes. Nothing else in a module is visible to the others. This makes the sim faster as well as cleaner:

1. **Parallel by default.** The scheduler sees which modules touch which stocks. Modules that share no writes run at the same time on separate threads (WASM threads on shared memory). Stocks are double-buffered: everyone reads last month's copy and writes this month's flows.
2. **Each module at its own pace and grid.** Climate monthly on 5 to 10 degrees, plants monthly on tiles, geology yearly, crust bending yearly on a coarse grid. The scheduler runs a module only when its step is due.
3. **Sleeping regions.** A module skips tiles where nothing it handles can change: plants under ice sheets, fire on wet tundra in winter, river cutting on flat ice-covered ground. A tile wakes when a stock it depends on crosses a threshold.
4. **Swappable detail.** Each module can have a cheap and a full version behind the same interface: a degree-day snowmelt or an energy-balance one, a simple or a full food web. Tests run the full one. A slow machine or the 16 km map can pick the cheap one.
5. **Data laid out for speed.** Stocks are stored as one array per stock, not one record per tile, so a module only loads what it uses.
6. **Tested alone.** Each module runs in a test harness with fake inputs, so a bug or a slowdown is found in that module, not in a whole-world run.

The limit: modules must share one memory and pass flows as arrays, not messages. If the coupling becomes copying, it costs more than it saves.

### Data decisions

1. **One grid family.** Keep today's geodesic hex grid. The coarse grids are coarser levels of the same grid, so each coarse cell is an exact set of tiles. Flows move between grids by summing and splitting, and nothing is lost to interpolation. The 2-degree cells and the 6-degree sea cells go.
2. **Resolution-independent rules.** Every rate is per km2, per km or per year, never per tile. A module run at 48 km and at 16 km must give the same world in the large, which is a standing test.
3. **One array per stock** (structure of arrays), `f32` per tile. World budgets sum in `f64`, so rounding cannot hide a leak.
4. **Height bands** baked once per tile from the 1 km heightmap: 8 to 16 bands, stored as area shares.
5. **Content files** in a plain text format (TOML or JSON), one per kind of thing (plant types, animal guilds, rock types, disaster stocks), each value with its source. Loaded and checked at start. A bad file stops the game with a clear message, not a crash mid-run.
6. **Saves** quantised with rounding, compressed, and sparse where a stock is mostly zero (ice, fire, sediment by grain size).

## Geology

The rebuild makes the ground a live stock: ice, water and sediment load it, and erosion, rebound, volcanoes and quakes reshape it every year. People are out of scope until this section passes its tests. The v0.49 people code (tribes, polities, courts, economy) is set aside, not ported.

### What is wrong today

1. The land never changes shape. Height is `today's heightmap - ice sag` (`climate.rs:1041`). Erosion uses a RUSLE formula that moves soil only, never the surface.
2. Rivers are drawn on that fixed surface. Deltas are tiles raised to sea level plus 3 m, so no river can build a plain and wander across it.
3. Quakes and volcanoes read baked hazard maps and a list of real eruptions. They leave marks but never lift a coast or build land.
4. Rock type is a fixed label. It sets resources and soil but never weathers into soil at a rate.
5. The ice load comes from a fixed ice shape per glacial stage, not from ice that grew. Each tile sinks on its own, with the forebulge as a side term.
6. Coasts are a flood-fill against sea level, plus special cases for barrier islands and lost tiles.
7. Sediment is not conserved. Silt appears on floodplains without being taken from the uplands.
8. Geology hands numbers to climate, soil and water and takes nothing back.

The one part that is real physics is the 4,000-year mantle relaxation. It carries over.

### The rule

Every geological change is a move between stocks, booked on both sides. Height is never set, only changed by a flow. Baked Earth data is the starting state and the test, never the driver.

### State: what every tile holds

Height is not stored. It is worked out each year from the stocks below.

| Stock | Unit | Raised by | Lowered by |
| --- | --- | --- | --- |
| Bedrock top | m | volcanic build-up, tectonic uplift field, rebound | weathering into regolith, glacial and river cutting, subsidence under load |
| Crust deflection | m | load (ice, water, sediment) with delay | unloading with delay |
| Regolith and soil | m, by grain size | weathering of bedrock, deposition | erosion, creep, landslides |
| Stored sediment | m, by grain size | river, wind, glacier and wave deposition | re-erosion |
| Ice | m of ice | snow that survives summer, inflow | melt, calving, outflow |
| Water on land | m | rain, melt, inflow | evaporation, infiltration, outflow |
| Groundwater | m | infiltration | seepage to rivers, wells later |
| Frozen ground | depth, m | cold years | warm years |
| Rock type | class | fixed, from GLiM | never, but buried by sediment |
| Strain on faults | m of slip owed | plate motion rate | earthquakes |
| Magma supply | km3 owed | supply rate at arcs and hot spots | eruptions |

Surface height = `bedrock + regolith + sediment + ice - deflection`. Sea level is one global number from the water budget. A tile is sea where the surface lies below it.

### Ice sheets grown from climate

Ice is a stock that the climate builds and the ice moves. No ice shape is ever read from a file.

1. **Mass balance.** Each year, snow that survives summer adds ice. Melt is worked out from summer warmth (a positive-degree-day sum to start, an energy balance later). Refreezing in the snowpack keeps some meltwater.
2. **Flow.** Ice spreads under its own weight with the shallow-ice approximation, the standard in large ice-sheet models. Thick ice on a slope flows fastest.
3. **Sliding and streams.** Where the bed is wet and soft (sediment, not granite), ice slides and fast ice streams form. Rock type and stored sediment set the bed.
4. **Calving and floating.** Ice whose base sits below sea level floats once thin enough. Floating ice forms shelves that melt from below in warm water and break off as icebergs. Icebergs carry their ice and their rock debris out to sea as booked stocks.
5. **Ice erodes.** Sliding ice grinds bedrock and moves the debris to its edge, where it builds moraines and outwash plains. Fjords and lake basins come out of this.
6. **Lakes against the ice.** Meltwater pools in the dip in front of the ice. When an ice dam floats or breaks, the lake drains as a flood that cuts its path, as Lake Agassiz and Missoula did.

Coupling: ice lowers the ground (crust), cools and dries the air above it, brightens the surface, stores water out of the sea, and feeds rivers with its melt and silt.

### The crust: loading, bending, rebound, sea level

The crust model is ELRA, "elastic lithosphere, relaxing asthenosphere". Le Meur and Huybrechts (1996) found it the best of the simple Earth models for ice-sheet work, and it is still widely used because it is cheap.

1. **Load.** Each year, load per tile = ice + water (sea and lakes) + sediment, as weight. A rising sea presses the sea floor. A big delta sinks under itself.
2. **Bending.** The rigid plate spreads each load over a few hundred km. The target deflection is the load map convolved with one fixed bending shape. That shape sinks the ground under the load, less beside it, and lifts a low forebulge further out. No side terms.
3. **Delay.** The mantle flows slowly, so the real deflection moves toward the target with a relaxation time of a few thousand years. v0.49 uses 4,000 years. Keep that as the first value and tune it against the uplift tests.
4. **Rebound.** When the ice goes, the target shrinks and the ground rises for millennia. Old beaches lift above the sea. Lakes in the dips drain. River mouths move.
5. **Sea level.** Global sea level comes from the water budget: ocean water left after ice, lakes and groundwater take their share, spread over the live ocean area. More ice means a lower sea, with no history file.
6. **Feedback.** Ice that sinks its own bed sits lower and warmer, so its edges melt faster. Ice resting on a bed below sea level can float off and collapse.

Known limit: Bueler et al. (2007) found real differences in bed height and ice thickness between ELRA and a full self-gravitating Earth model. A later version can add the sea-level equation (self-gravity, so sea level is not flat). ELRA is the right first step.

### Landscape: rivers, slopes, sediment, lakes, deltas

The model follows FastScape (Braun and Willett, GFZ). Its algorithms are implicit and run in time proportional to the number of tiles, so they can run every year on the whole map.

1. **Flow routing on the live surface.** Each year, water routes downhill over the current surface, including ice and sediment. Pits fill into lakes, and lakes spill at their lowest edge. River paths are never stored. They come out of the surface.
2. **River cutting.** Rivers cut bedrock by the stream power law: faster with more water and steeper slope, slower in hard rock. Rock type sets the hardness.
3. **Carrying and dropping.** Rivers carry sediment up to a capacity set by flow and slope. Above capacity they drop it, on floodplains, in lakes and at the coast. Every grain dropped was picked up somewhere.
4. **Slopes.** Regolith creeps downhill, faster on steep slopes and where soil is thick. Steep, wet slopes fail as landslides above a threshold, more often after fire, quakes or heavy rain.
5. **Plants hold the ground.** Root cover lowers erosion and creep. A burnt or bare slope sheds sediment into the river below. This is the main link from living things to the land.
6. **Lakes fill.** Lakes trap sediment and shrink over centuries. A lake that fills becomes a plain.
7. **Deltas and avulsion.** A river builds its bed and banks where it slows. When its channel sits above the plain, a flood breaks out and the river takes a new, lower course. Deltas grow lobe by lobe this way. No special-case delta code.

Sediment comes in grain sizes (at least gravel, sand, mud). Gravel stays near the hills, sand builds bars and dunes, mud reaches the sea and the floodplains.

### Rock to soil: weathering and its carbon

Soil is made from rock, not assigned.

1. **Soil production.** Bedrock turns into regolith at a rate that falls as soil thickens, by the soil production function of Heimsath et al. (1997): rate = P0 × e^(-k × soil depth). Field values: P0 of 0.05 to 2.0 mm a year, k of 2 to 4 per metre. Warmth, water and rock type set P0 per tile.
2. **Thin soils on steep slopes, thick in basins.** Production and creep together balance out, so soil depth comes from the landscape, not from a map.
3. **Chemical weathering draws down CO2.** Silicate weathering takes carbon from the air and sends it to the sea as dissolved carbonate. It is faster where it is warm and wet. This is the slow thermostat that links rock to climate.
4. **Nutrients come from rock.** Weathering releases phosphorus and other rock nutrients into the soil. Old, leached soils run out, young volcanic and glacial soils are rich. Plants read this stock.
5. **Loess.** Wind-blown glacial silt builds thick, fertile soils downwind of ice sheets (see Wind below).

### Coasts, wind, volcanoes, earthquakes, frozen ground

**Coasts.** Waves cut cliffs at a rate set by wave energy and rock hardness. Longshore drift moves sand along the shore, so spits, bars and barrier islands grow out of it. Reefs grow upward in warm, clear, shallow water and drown if the sea rises faster. Rising and falling seas move all of this with them.

**Wind.** Wind lifts dust and sand from dry, bare ground. Sand moves as dunes. Dust falls downwind as loess, and some reaches the sea as iron for plankton. Plants and wet ground stop it.

**Volcanoes.** Each volcanic arc and hot spot owes magma at a steady supply rate, taken from today's real volcanoes. Eruptions draw the debt down at random, with size set by how much is owed. An eruption builds its cone, buries land in lava and ash, sends sulphur to the air (cooling) and CO2 (slow warming), and gives young rich soil. The list of real past eruptions becomes a test, not a script.

**Earthquakes.** Each fault owes slip at its real long-term rate. A quake releases it, raises or drops the land beside the fault, can send a tsunami, and sets off landslides on steep wet slopes. Baked hazard maps become a test.

**Tectonic uplift.** Plates move a few cm a year, so the map's shape hardly changes in 10,000 years. Uplift still matters to erosion. Keep one uplift-rate field from today's tectonics, so mountains keep rising as rivers cut them.

**Frozen ground.** Permafrost forms in cold years and thaws in warm ones. Frozen ground blocks infiltration and holds carbon. Thawing slopes slump.

**Karst.** Limestone dissolves slowly under wet, CO2-rich soil, making caves, sinkholes and underground rivers. Low priority.

### Grids, cost and order each year

Each process runs on the coarsest grid that still shows it, then hands its flows down to the tiles.

| Process | Grid | How often |
| --- | --- | --- |
| Crust bending and rebound | coarse, about 50 to 100 km | yearly |
| Ice flow | coarse, about 50 km | yearly, more often in fast collapse |
| Ice mass balance | tiles | monthly melt and snow, booked yearly |
| Flow routing, river cutting, sediment | tiles | yearly, floods monthly |
| Slopes, weathering, soil | tiles | yearly |
| Coasts and wind | coast and dry tiles only | yearly |
| Volcanoes and quakes | per source | checked monthly |
| Sea level | one number | yearly |

Order each year: climate gives snow, melt, rain and wind. Then ice mass balance and flow, then load and crust, then sea level, then routing, cutting and deposition, then slopes and soil, then coasts and wind. Each step reads the state at the start of the year and books its flows. At the end, every stock is checked: ice, water and sediment in must equal out plus change, to rounding.

Cost target: the whole geology year under 50 ms at 48 km. FastScape-style routing is linear in tile count, and the coarse grids are small. On the 16 km map, run geology yearly in a worker while the month steps run.

### More geology gaps

**Critical, settle before any other geology code:**

1. **Starting state.** Today's heightmap already holds today's erosion and today's half-finished rebound. Starting at the glacial maximum on it is wrong from year one. Spin the crust and ice back from today to the start date, or start from a reconstructed ice-age Earth.
2. **River resolution.** A 48 km tile holds whole valleys, so the stream power law sees no real slope or channel width. Use a sub-grid rule (channel slope and width from the tile's relief) or a finer river grid.
3. **Erodibility data.** Every rock type needs a published hardness and weathering rate, or the landscape is tuned by hand again.
4. **The sea floor as a sink.** Sediment reaching the sea goes to shelves, sinking deltas, and slides into deep water. Otherwise the sediment budget leaks at every coast.

**Important:**

5. Groundwater that flows: springs, oases, wetlands, dry-season river flow.
6. Closed basins turn salty and leave salt flats and evaporites.
7. Resources that move: erosion exposes ore, rivers gather placer gold and tin, bogs grow iron.
8. Ocean heat at depth, so ice shelves melt from below.
9. Relative sea level per tile from the crust model: Scandinavia rising, the Nile delta sinking.
10. Tides shape estuaries, mudflats and salt marsh.
11. Rendering a moving world: mesh, shading and rivers rebuilt from the live state.
12. Saves sized for bedrock, deflection, sediment by grain size and ice per tile.

### Rivers that meander

Today rivers do not meander. Paths are fixed down-slope routes on the fixed heightmap. The only course change is `avulsions_year` (`land.rs:511`): at most one river in the world per 25 years, after a big flood, on a 2% dice roll. No bends move, no oxbow lakes form, no banks erode.

At 48 km, meanders are smaller than a tile. Each river reach keeps a sub-tile state: how sinuous it is, how fast its bends migrate, and when a loop cuts off. Those feed floodplain wetlands, oxbow lakes and silt. A river jumps course when its bed sits above the plain and a flood breaks the bank.

### Height bands inside each tile

Today each tile has one mean height and one peak. Mountain glaciers (`more.rs`, `mg_build`) take a snowline from the baked normal climate and set the glacier as a share of the tile from how far that one peak rises above it. Ice never flows down valleys, the terminus height is not represented, ice thickness does not raise the surface, and mountain glaciers and ice sheets are two systems that never meet.

Fix: each tile stores the share of its area in 8 to 16 height bands, built once from the 1 km real heightmap. Climate per band is the tile's weather plus a lapse rate of about 6.5 C per km, using this year's weather. Snowline, glaciers, treeline, alpine meadow and frost are worked out per band and summed. Ice builds in the bands above the snowline and flows down into lower bands and down-valley into the next tile, so tongues reach below the snowline as real ones do. Mountain glaciers and ice sheets become one ice model.

### Tests the geology must pass

No people code until all pass on two seeds.

1. **Budgets close.** Ice, water and sediment balance every year to rounding, over 10,000 years.
2. **Stable at rest.** Started from today with today's climate, the land holds still for 1,000 years: no runaway cutting, no flat-out plains, no spikes.
3. **Ice ages come out right.** Driven by the orbital climate alone, ice sheets grow over North America and Scandinavia and not where they never were. Sea level at the last glacial maximum lands near the real lowstand of about 120 to 130 m below today. Check against Lambeck et al. (2014) before fixing a number.
4. **Rebound matches.** After the melt, Hudson Bay and the Gulf of Bothnia still rise today at about 1 cm a year. Memory figure, check against GPS studies before use.
5. **Rivers move sediment like real ones.** Sediment loads of big rivers sit within a factor of two of measured loads. Data source to choose (river sediment databases).
6. **Lakes and floods appear on their own.** Lakes form against the melting ice and drain as floods, without being placed.
7. **Soil depth is believable.** Thin on steep and young land, thick in basins and on loess, against global soil depth maps.
8. **Volcanoes and quakes match the record in the large.** Eruption and quake counts by size over 10,000 years sit near the real record, without the record driving them.
9. **Determinism.** Same seed, same result, proven with the fingerprint tool.

### Sources

- [Soil production function](https://en.wikipedia.org/wiki/Soil_production_function), formula and field values from Heimsath et al. (1997), [Nature 388:358](https://doi.org/10.1038/41056).
- [Zwinger et al. (2020), GMD 13:1155](https://gmd.copernicus.org/articles/13/1155/2020/), on ELRA (Le Meur and Huybrechts 1996) and its limits (Bueler et al. 2007).
- [FastScape, GFZ](https://gfz.de/en/section/earth-surface-process-modelling/projects/current-projects/fastscape-landscape-evolution-model-development), implicit linear-time landscape algorithms.
- [Lambeck et al. (2014), PNAS 111:15296](https://sealevel.nasa.gov/publications/204/sea-level-and-global-ice-volumes-from-the-last-glacial-maximum-to-the-holocene), sea level since the last glacial maximum (numbers to be read from the paper).

The lowstand and Hudson Bay figures are from memory and must be checked before they become test targets.

## Disasters

Every disaster becomes a stock that the world fills and something releases. Randomness stays only where nature is random: lightning, and the exact day a loaded system lets go. Meteors are dropped, since nothing in the world can predict them.

| Stock | Fills from | Empties in |
| --- | --- | --- |
| Fault strain | plate motion at real slip rates | earthquakes |
| Magma | supply at depth | eruptions |
| Fuel | plant growth and dead matter | fires |
| Sea heat | sunlight | tropical storms |
| Lake water behind ice or landslides | melt and rain | dam bursts |
| Snowpack | snowfall | avalanches and spring floods |
| Slope water and soil | rain, weathering | landslides |
| Insects and pathogens | stressed or crowded hosts | outbreaks, then burnout and immunity |

Each release changes the stocks around it: burnt land floods more, quakes drop slides into rivers that dam and burst, storms cool the sea behind them, eruptions cool the world.

### Every kind, today and in the rebuild

"Not read" means the trigger code was not checked this session.

| Disaster | How it happens now | What is wrong | Living version |
| --- | --- | --- | --- |
| Tropical cyclones | Poisson dice per basin at fixed monthly rates (`storms.rs:193`) | Pure RNG: a hot sea makes no more storms | From warm sea, moist air and low wind shear. Storms cool the sea behind them |
| Windstorms, Mediterranean lows | Poisson at fixed rates, a little more in cold periods | RNG with a climate nudge | From the pole-to-equator temperature contrast, which ice ages steepen |
| Extreme rain | Poisson at a fixed rate | RNG | From slow lows and moist air over warm seas |
| Tornadoes, derechos, blizzards, ice storms | Fixed zones plus weather checks (not read in full) | Fixed zone maps | From the month's heat, moisture and wind contrast |
| Drought, heat wave, cold snap, frost | Labels on climate anomalies (`veg.rs:836`) | Anomalies come from baked or emulated climate | Droughts persist through dry soil and less rain recycled by plants |
| Climate modes (monsoon, ENSO, AMO, AMOC, solar, Bond, D-O, Heinrich) | Scripted cycles and emulators | Not caused by the world | Come out of the ocean and ice model, or respond to it |
| Dust storms | Dry bare ground (not read in full) | Partly real | Dust from dry bare soil, booked as sediment, settling as loess and feeding the sea |
| River floods | Flow against the river's normal (`hydro.rs`) | Real, on fixed beds | Live beds silt up and raise flood risk. Floods drop silt and can move a river |
| Flash floods | Not read | Unknown | Heavy rain on steep, bare or burnt slopes |
| Ice jams | A river check (`haz2.rs:356`) | Partly real | From thaw timing on north-flowing rivers |
| Ice-lake bursts, megafloods | Called when a lake is set to break | Lakes do not fill or fail by physics | Lakes fill against the ice and burst when the water lifts the dam |
| Landslide-dam bursts | After a landslide (`haz2.rs:214`) | Partly real | The slide blocks a valley, the lake fills, the dam overtops |
| Avulsions | 1 per 25 years worldwide, 2% dice | Throttled and dice-driven | Bed above the plain plus a flood that breaks the bank |
| Glacier surges | Not read in full | Unknown | Water building up under the ice |
| Earthquakes | Fixed yearly chance per tile from a baked map, size by dice (`quake.rs:100`) | Pure RNG, no memory | Fault strain builds and releases. Quakes lift or drop land. A broken fault goes quiet and loads its neighbour |
| Tsunamis | After a quake or rockfall | Follows RNG quakes | Unchanged, from real quakes and slope collapses |
| Volcanoes, ash | Real eruption list plus random ones | Script plus RNG | Magma fills each chamber. Eruption size follows time since the last one |
| Landslides, avalanches | Rain far above normal, fire scars, a dice roll (`quake.rs:78`) | Mostly real | Fail when slope, soil depth and water pass a stability limit. Snowpack layers for avalanches |
| Limnic eruptions | Rare event (`haz2.rs:194`) | RNG | CO2 builds in deep, still volcanic lakes and escapes when the lake overturns |
| Sinkholes | Not read | Unknown | Limestone dissolves over centuries and the roof falls in |
| Ice-shelf collapse, calving, icebergs | Shelves react to warmth. Calving a fixed share plus random big events | Ice comes from a fixed shape | From live ice thickness and the warmth of water under the ice |
| Coastal change, barrier breaks, storm surge | Special cases against sea level | Not a process | Waves, drift and storms move sand. Surge from the storm and the coast's shape |
| Coral bleaching | Hot sea (`sea.rs:431`) | Real trigger | Dead reefs stop growing and lose their wave protection |
| Fishery crashes | A sea check | Partly real | From fish stocks, plankton and upwelling |
| Bark beetles | Two dry years in a row (`bio.rs`) | Real trigger | Beetles as a stock in stressed forest. Dead trees become fuel |
| Defoliators | Fixed lat/lon boxes plus weather | Fixed boxes | Insect populations in their host forests, wherever they are |
| Locusts | Rain in fixed zone boxes | Fixed boxes | Swarms where good rain follows drought on bare desert edges |
| Herd die-offs, animal plague | Crowding plus a 2% dice roll | Partly real | Disease spreads with density and contact and leaves immunity |
| Crop blight, pests | Not read in full | Unknown | People era, set aside for now |
| Meteors | Fixed rate, 5 times the real one | Random by nature | Dropped |

### Fire

Today fires are held back by rules that have nothing to do with fuel or weather (`veg.rs`, `run_fires`):

1. The size is decided at ignition. Each fire gets an area budget from spread rate times burning days, then spends it. Fuel and weather can stop it sooner, never let it grow further.
2. The front keeps at most 97% of what is left at each tile, so fires die on a shrinking series even in dry, unbroken fuel.
3. Grass fires have a hard cap of 2,500 km2, up to 15,000 km2 in record fire weather (`veg.rs:610`). A typical size from Archibald et al. 2013 became a ceiling.
4. Burning time is capped at 40 days, plus 50 in deep drought.
5. A whole fire, weeks of burning included, runs inside one month's step. It cannot smoulder into the next month.
6. A limit of 40,000 tile visits per fire.
7. A floor of about 6 km2, so small fires do not exist.

What already works: a fire cuts the tile's fuel by 85% and kills trees, so a burnt patch blocks fire for years.

The rebuild: a fire is a live front that carries over from month to month. It spreads while the next tile has dry, connected fuel and the weather allows, and stops at rivers, wet ground, bare land, rain or a wind shift. No budget, caps or per-tile loss. Old burns act as firebreaks, and long gaps build fuel for big fires. Peat smoulders for months and can survive winter underground. The heavy-tailed size pattern (Malamud et al. 1998) becomes a test.

## Ecology

The world with no people. The plants and animals must hold themselves up through feedback, with no target maps.

### What is wrong today

Read from `veg.rs`, `eco.rs`, `fauna.rs` and `bgc.rs`.

1. **Plants chase a target.** Each tile has a "what the normal climate supports" value for trees, grass, wetland and permafrost (`p_tree` and others, `veg.rs:414`). Cover moves toward it with a lag. Growth comes from the Miami model (Lieth 1975), heat and rain only: no CO2, light, nutrients or competition.
2. **Three lumps of plants:** tree, grass, shrub, plus wetland and peat. No plant types, and no succession beyond a stand age.
3. **Animals do not eat the plants.** Herbivore capacity comes from a rainfall regression (Fritz & Duncan 1994). Grazers eat from a separate forage pool per 200 km cell and never take grass out of the plant stock. Herds cannot keep the mammoth steppe open, elephants cannot open forest, and overgrazing cannot happen.
4. **Two grids.** Plants on tiles, animals on 2-degree cells of about 200 km.
5. **Animals are capacity shares,** "how full the land is" from 0 to 1, with growth toward a ceiling.
6. **Five guilds** for all land animals (megaherbivores, grazers, browsers, small game, predators), plus fur predators, voles and weasels, crocodiles and an insect index.
7. **Fish are capacity only.** Nothing eats them.
8. **Nutrients barely limit.** Soil nitrogen exists but does not cap growth. Dung, carcasses and dead plants are not booked back to the soil.
9. **Carbon is checked, not flowed.** `bgc.rs` compares land carbon to its own recent past, not a flow from air to leaf to soil to air.
10. **Spread is partly real.** Trees spread about 300 m a year. Animals migrate only on the coarse cells and cannot colonise new land.

### Predation today

One real predator-prey link, wrapped in shortcuts (`eco.rs:373`):

- One predator guild (wolves, big cats, bears) eats grazers, browsers and small game each month, with a saturating kill. Megafauna count a tenth. Predators hunt toward prey-rich cells.
- Predator capacity is a formula from prey, `0.08 x prey^0.73` (Carbone & Gittleman 2002, Hatton et al. 2015), so predators cannot overshoot. They cannot lose more than 15% a month, which softens crashes.
- The kill is capped at 5% of prey a month.
- The herbivore ceiling is padded for "normal" predation, so the two halves are tuned to each other, not interacting.
- Nothing eats another predator. No scavengers: kills and starved animals vanish. Fish, birds and insects feed no one, except the vole-weasel and hare-lynx side models. Crocodiles get a capacity from prey but take nothing out.

### The rebuild

The same rule as geology: everything is a stock, nothing has a target. Each fix copies a tested method from Earth-system models.

1. **Plants grow.** Use the method of the LPJ dynamic vegetation model (Sitch et al. 2003). About ten plant types per tile, each with leaf, wood and root stocks: tropical evergreen, tropical dry, temperate broadleaf, temperate needle, boreal needle, boreal summergreen, shrub, C3 grass, C4 grass, moss and lichen. Each month: photosynthesis from light, temperature, water and CO2, capped by nutrients, minus respiration. Trees shade grass, grass wins in dry soil, cold, drought and fire kill by each type's limits. `p_tree` and the other targets are deleted. Today's biome map becomes a test.
2. **Succession.** Comes from point 1. After fire or wind-throw, pioneers (grass, shrub, birch, pine) grow first and shade-tolerant types take over. Old growth forms only where nothing disturbs it.
3. **Herbivores eat the plant stock, on the same tiles.** Grazers take grass leaves, browsers take shrub and tree leaves and seedlings, megaherbivores take both and push over trees. Intake follows a Holling type II curve. What is eaten is gone from the plants. Dense herds keep grass short and trees out, elephants and mammoths open forest, too many animals strip the land, starve, and leave bare soil that erodes into the geology. The Fritz & Duncan curve becomes a test.
4. **One grid.** Animals live on the plant tiles. They move each month toward better food and away from crowding, snow and predators. Long migrations come from following the green season, with a speed limit per guild of about 1,000 to 1,500 km a year. The 2-degree cells go.
5. **Populations.** Each guild per tile is a biomass with births, deaths and starvation. Rates scale with body size by metabolic theory (rates as mass to the minus a quarter, Brown et al. 2004), so slow megafauna and fast rodents come from one rule. This is close to the Madingley general ecosystem model (Harfoot et al. 2014), simplified to guilds.
6. **Finer guilds,** by diet and body size: megaherbivores, large and medium grazers, large and medium browsers, small herbivores, insectivores, birds, small, medium and large carnivores, scavengers, insects. Species names become labels for who fills a guild in each region.
7. **The sea and fresh water as a size spectrum:** plankton, small fish, large fish, seals and seabirds, each eating the size below. Plankton grows on light and nutrients from upwelling and rivers. Salmon carry sea nutrients upriver. Rivers and lakes get the same chain, smaller.
8. **Nutrients limit and recycle.** Three soil organic pools (fast, slow, very slow, as in the CENTURY model, Parton et al.), plus mineral nitrogen and phosphorus. Litter, dung and carcasses go to the fast pool. Decomposition speeds up when warm and moist and stalls when cold, wet or frozen, which builds peat and permafrost carbon. Nitrogen comes from fixing plants and lightning and leaves by leaching and fire. Phosphorus comes from rock weathering.
9. **Carbon booked.** Every flow counted: air to leaf, leaf to animal or litter, litter to soil, soil to air, land to river to sea. Air CO2 is the sum, and it feeds back on growth and climate.
10. **Spread and colonisation.** Seeds spread from neighbouring tiles at each type's real rate. A type grows only where its seeds have reached, so forests creep north after the ice and islands fill slowly. Animals cross land, and swim or raft to islands at low odds by guild. Land bridges let species through, rising seas strand them.

The method references above are from memory and must be checked before use.

### The food web

Each consumer guild eats from the guilds below it with a capped intake curve, and only what it eats feeds its births. No capacity formula and no kill caps.

| Eater | Eats | Eaten by |
| --- | --- | --- |
| Grazers | grass | medium and large carnivores |
| Browsers | shrub and tree leaves, seedlings | medium and large carnivores |
| Megaherbivores | grass, browse, trees pushed over | large carnivores, young only |
| Small herbivores | seeds, grass, roots | small and medium carnivores, birds of prey |
| Insects | leaves, wood, dung, litter | insectivores, birds, fish |
| Insectivores and birds | insects, seeds | small carnivores, birds of prey |
| Small carnivores (fox, weasel) | small herbivores, birds, insects | large carnivores |
| Medium carnivores (wolf, leopard) | grazers, browsers, small carnivores | large carnivores |
| Large carnivores (lion, bear) | big prey, medium carnivores, fish | none, as adults |
| Scavengers and decomposers | carcasses, dung, litter | (to the soil) |
| Fish | plankton, insects, smaller fish | bears, otters, seals, birds, crocodiles, bigger fish |

Rules:

- Body size sets who can kill whom. Adults of big species are rarely taken, their young are.
- Big carnivores kill and compete with smaller ones (wolves suppress coyotes).
- Prey has refuges: deep forest, cliffs, water, herds.
- Everything that dies goes to scavengers, then to the soil.

Overshoot, crashes, predators changing where prey graze, and the Carbone & Gittleman predator ratio then come out of the numbers. The ratio becomes a test.

### Who lives where at the start

The rules say how guilds behave, not which species fill them where. The start needs range data:

- **Megafauna ranges** from the PHYLACINE dataset (past ranges of large mammals, including extinct ones).
- **Today's ranges** of the living species that fill each guild.
- **Pollen records** for which plant types grew where during the last glacial cycle.

Each guild in each region gets its species labels and starting amounts from these. Spin-up then lets them settle. The same data becomes a test: run the world forward and the ranges should end near today's.

### Coastal and shallow-sea life

Mangroves, salt marsh, kelp forests, seagrass and coral reefs become their own stocks on coast and shallow tiles. They grow by warmth, light, salt and tide, trap sediment, damp waves, shelter young fish, and feed sea grazers such as turtles, dugongs and the Steller's sea cow. A rising sea drowns them or pushes them inland. They are the link between the land, the sea and the coast's geology.

### Animals move things

The food web has eating. Living ecosystems also need moving:

1. **Seeds.** Animals carry seeds in guts and fur. Big animals spread big-fruited trees, so losing them changes forests. Seed spread per plant type has a wind part and an animal part.
2. **Nutrients.** Salmon carry sea nutrients upriver, seabirds drop guano on islands and coasts, herds move nutrients from where they graze to where they rest. Booked as flows.
3. **Pollination.** Insect-pollinated plant types grow less where insect stocks crash.

### Seasonal migrations

Birds, whales and fish move between hemispheres and seas on a yearly route, not only land herds following grass. Each migratory guild has breeding and feeding grounds chosen each year by the state (food, ice, warmth), and it pulls food from both.

### Ecology tests, order and cost

**Tests:**

1. From bare ground after the ice, with no people, today's biomes, plant biomass and soil carbon form on their own, checked against real maps.
2. Herbivore biomass against rainfall falls near Fritz & Duncan. Predator against prey falls near Carbone & Gittleman. Burned area sits near the GFED satellite totals.
3. Remove herds from a test world and the steppe closes into scrub and forest. Put them back and it opens again.
4. Carbon, nitrogen and water balance every year over 10,000 years.
5. Predator-prey cycles appear without being written in.

**Order,** each step passing its test before the next:

1. Height bands and glaciers.
2. Plant types and growth.
3. Soil and nutrients.
4. Herbivores eating plants on one grid.
5. Predators, scavengers, disease and movement.
6. Fish and the sea.
7. Full carbon booking and colonisation.

**Cost.** Far more work per tile than today. At 48 km (about 256,000 tiles), monthly plants and animals fit in a worker thread. At 16 km, the plant step may need to run every 2 to 3 months, or animal movement on a coarser 3 x 3 tile grid. Build at 48 km first and measure.

This is the biggest lift in the rebuild. Point 3, herbivores eating plants, is what is most wrong today and what would make the world feel alive.

## Climate and atmosphere

Every system above reads the climate, so it is built first, before geology and ecology are switched on.

### What is wrong today

Read from the headers of `climate.rs`, `cycles.rs`, `hydro.rs` and `sea.rs`.

1. **Climate on land is played back.** An emulator fitted to 72 climate-model snapshots (Beyer, Krapp & Manica 2020) gives each tile its temperature and rain for the current point in the ice-age cycle. Ice, plants, dust and the sea in this world cannot change it.
2. **Ice volume is one number.** A three-state conceptual model (Paillard 1998), driven by June sunshine at 65 N, sets global ice volume, which then sets a pre-shaped ice cover. Ice does not grow from snow.
3. **The glacial sea surface is a map** (MARGO 2009), not a sea that cools.
4. **The Green Sahara switches on a sunshine threshold,** with plant feedback written in as abruptness, not computed.
5. **Short cycles are scripted.** ENSO is a damped oscillator, the PDO and AMO are red noise, solar minima and Bond events are random draws, and Dansgaard-Oeschger warmings and Heinrich events are events, not outcomes of ice and ocean.
6. **Sea ice comes from a freezing proxy** baked from today's sea temperatures.

The orbit (Berger 1978) is real physics and stays.

### The fix: keep the emulator, add the feedbacks

Odyssey is a semi-simulation, not a climate supercomputer. The climate stays close to today's: the Beyer emulator, today's observed climate, the orbit and the short cycles carry over. What changes is that the living world can push back on it, through a few cheap terms added on top of the emulator's answer:

1. **Ice.** The emulator reads ice volume from the live ice sheets, not the Paillard model. Ice cover cools and dries the tiles around it.
2. **Plants.** Forest, grass or bare ground changes how much sun a tile reflects and how much rain it recycles. The Green Sahara threshold reads the live plant cover, so the feedback is computed, not written in.
3. **CO2 and methane.** The booked carbon stocks set the emulator's CO2 input.
4. **Dust and volcanic sulphate** dim the sun regionally for as long as they stay in the air.
5. **Meltwater.** A pulse of fresh water from collapsing ice can trigger the cold events the cycles already script, through a small overturning box. Scripted cycles stay as the background.
6. **Clouds.** One simple published rule for how cloud cover shifts with warming, held fixed and listed as a known risk. It is the most uncertain feedback in real climate science.

Live physics on a coarse grid (the CLIMBER-2 class of model, Petoukhov et al. 2000) is the upgrade path if these feedbacks look wrong in testing, not the starting point.

The Beyer snapshots and the MARGO sea surface stay as tests: with the feedbacks on, the climate at those dates should stay near them.

### Weather and storms from the state

The climate model gives monthly means. Weather is the spread around them, and it must depend on the state, not on fixed dice.

1. **A weather generator per tile.** Each month draws its own daily-scale spread (warm and cold spells, wet and dry runs) from statistics that shift with the state: a dry soil makes heat waves hotter, a warm sea makes rain heavier. The base statistics come from today's observed weather.
2. **Tropical cyclones from a genesis index.** Each month, each warm sea cell gets a storm chance from sea warmth, moisture, spin and wind shear, using the genesis potential index of Emanuel and Nolan (2004). A storm takes heat from the sea and leaves a cool wake.
3. **Mid-latitude storms from the temperature contrast** between pole and equator, along tracks that move with it.
4. **Severe local storms** (tornadoes, derechos, hail) from the month's heat, moisture and wind contrast.
5. **Droughts persist** through dry soil, less plant transpiration and less rain recycling.

Randomness stays in the exact timing and place of each storm. How many and how strong come from the state.

### Ocean and overturning

1. **A mixed layer per sea cell** with its own heat budget: sunshine in, heat out to air and ice, heat moved by currents and mixing. Sea surface temperature comes from this, not a map.
2. **Currents from wind.** Surface currents follow today's patterns, steered by the live winds. Coastal upwelling comes from alongshore wind (Ekman transport), so upwelling and its fisheries move with the winds.
3. **Deep ocean boxes** store heat and carbon for centuries. They slow the climate's response and keep its memory.
4. **The overturning (AMOC) as a few-box model** in the style of Stommel (1961): dense, salty, cold North Atlantic water sinks and draws warm water north. Fresh water from melting ice or more rain makes the surface lighter and can stall it. It then has two stable states, and a pulse of meltwater or icebergs can flip it.
5. **Abrupt events come out of it.** Heinrich events are iceberg armadas from a collapsing ice sheet stalling the overturning. Dansgaard-Oeschger warmings are the overturning restarting. The bipolar seesaw is heat piling up in the south while the north freezes. None are scripted.
6. **ENSO from the Pacific's own heat.** A recharge oscillator (Jin 1997): two variables, Pacific warm-pool heat and east-west temperature contrast, that swing on their own every 2 to 7 years. The rain and temperature patterns it brings stay from today's observations.
7. **Sea level** from the water budget, as in the geology section.

The current sea food web (`sea.rs`, on 2-degree cells, with a fixed 10% step up the chain) is replaced by the size spectrum in the ecology section, on the ocean grid. Its whale and seal dynamics are the best part and carry over.

### Sea ice

Sea ice becomes a stock of thickness per sea cell. It grows when the water loses heat to cold air, melts from above in sun and from below in warm water, and snow on top insulates it, as in the simple thermodynamic model of Semtner (1976). It drifts with the wind and the currents and piles up against coasts. Ice brightens the surface, cuts heat loss from the sea, blocks evaporation, and is where seals and polar bears live. The baked freezing proxy goes.

### The water cycle on land

One booked water stock per tile, linking climate, plants, rivers and the ground. Pieces exist today (snow, a 120 mm soil bucket, river flow), but they are separate.

1. **Snowpack per height band,** built by snowfall and melted by warmth (degree-day to start, today's 4.5 mm per degree per day carries over), with sun and rain-on-snow later.
2. **Soil water in layers.** Two or three layers instead of one 120 mm bucket for every soil. Depth and holding capacity come from the live soil in the geology section. This also fixes the monsoon highlands' runoff, which runs about twice the real share today.
3. **Evaporation through plants.** Water leaves through leaves, at a rate plants control by their type and stress, plus bare soil and open water. Plants and climate share this flow.
4. **Groundwater** fills from deep drainage and feeds springs, wetlands and rivers in the dry season.
5. **Lakes and wetlands** fill, evaporate and spill. Closed lakes turn salty.
6. **Runoff goes to the live river routing** in the geology section.

Every drop is booked: rain and snow in, evaporation and runoff out, change in snow, soil, groundwater and lakes, closing every month.

**Lakes as water bodies.** Each lake has a temperature, an ice cover, and layers that form in summer and turn over in spring and autumn. These set lake fish, oxygen at depth, the gas build-up behind limnic eruptions, and the local climate around big lakes (lake-effect snow, milder shores).

### Air chemistry and forcing from space

**CO2 and methane as stocks.** The air holds one well-mixed amount of each. Plants, soil, peat, permafrost, the sea, volcanoes and rock weathering add and remove carbon, all booked. Ice-age CO2 swings of about 80 to 100 ppm should come mostly from the sea storing more carbon in cold, ice-covered, iron-fed water, so the ocean needs surface and deep carbon boxes, the solubility pump (cold water holds more) and the biological pump (sinking plankton). Methane comes from wetlands and thawing ground and breaks down in about a decade. Both feed the energy balance.

**Dust and volcanic sulphate as stocks.** Dust lifted from dry bare ground dims the sun, falls as loess and feeds the sea with iron. Eruptions put sulphate high in the air, which cools for a few years.

**Forcing from space.** The orbit (Berger 1978) stays as real physics. The sun's output varies within its real range, with grand minima drawn at the real rate (Usoskin et al. 2007), since the sun is outside the world. Bond events are no longer separate draws: they are solar dips that the overturning amplifies, or not.

The ice-volume model (Paillard 1998) is dropped. Ice volume comes from the live ice sheets. Its output becomes a test.

### Climate tests

1. **Today.** Started from today with today's orbit and CO2, the live climate stays near today's observed climate for 1,000 years: no drift, no runaway ice or desert.
2. **The last glacial maximum.** With that orbit and the CO2 the world produces, land temperature and rain land near the Beyer snapshots, and sea surface near MARGO.
3. **A whole glacial cycle.** Run from the last interglacial to today: ice grows over North America and Scandinavia, CO2 falls and rises by about 80 to 100 ppm, the Green Sahara comes and goes at roughly the right times.
4. **Abrupt events.** Dansgaard-Oeschger warmings and Heinrich events happen on their own in ice-age conditions, and not in the Holocene.
5. **ENSO.** Swings every 2 to 7 years, with today's rain patterns in El Nino and La Nina years.
6. **Budgets close.** Energy, water and carbon balance every year.

The ice-age figures above are from memory and must be checked before they become targets.

### Speed budget

Target per real month of game time, at 48 km in a worker thread:

| Part | Grid | Budget |
| --- | --- | --- |
| Climate physics | 5 to 10 degrees | about 5 ms |
| Weather, water cycle, tile climate | tiles | about 20 ms |
| Plants, soil, fire | tiles | about 30 ms |
| Animals and the food web | tiles | about 20 ms |
| Ocean, sea ice, sea life | ocean grid | about 10 ms |
| Geology | tiles and coarse grids, yearly | about 5 ms a month on average |
| **Total** |  | **about 90 ms a month** |

That runs one game year in about 1 second at 48 km. These are targets to measure against, not measurements. The 16 km map is the main play scale (it runs well on the owner's machine) and has about 9 times the tiles, so its budget is about 9 times larger, spread over threads by the modules. 48 km stays the scale for development and the long test runs, where speed of iteration matters most. Every module must work at both scales without retuning.

### Start date and time span

Start at the last interglacial, about 125,000 years ago, and spin up there. The climate and ice are then near today's state, so the start is easy to check, and a whole glacial cycle follows, ending in today. Starting at the glacial maximum instead needs a reconstructed ice-age world, which is the hardest start to get right. A world must also be able to start from today, for testing.

## Content as data

Today plant kinds, animal guilds, goods, skills and buildings are constants in the code with fixed counts. Tribe skills already fill all 64 bits of their mask.

In the rebuild, every list of kinds is a data file the engine loads at start:

- plant types with their limits and rates
- animal guilds with body size, diet and rates
- rock types with hardness and weathering rates
- disaster stocks with their fill and release rules
- later: crops, herds, goods, buildings, skills

No fixed counts in the code. Adding a plant type or a rock class is a data edit. Each value in a data file carries its source, so the research habit is built in.

## Save format and versioning

Today saves are a v10 format plus tail blocks found by marker bits, written by hand on both sides. Every new field risks breaking old saves.

The rebuild:

1. **Tagged sections.** Each system writes its own named, versioned section. A reader skips a section it does not know and fills a missing one with defaults.
2. **Generated, not hand-written.** Save and load code comes from one description of each stock, so the two sides cannot drift apart.
3. **Sized for live stocks.** Bedrock, deflection, sediment by grain size, ice, soil pools, plant types and animal guilds per tile add up. Store them compressed, quantised with rounding, and only where non-zero.
4. **A save is the whole state.** Load and continue gives the same result as never stopping, proved by the fingerprint tool.

## People and courts

Deferred until the living world passes its long-run tests. When people come in, they enter as one more species in the same food web and the same stocks: they eat, burn, clear, herd and build from the same plants, animals, water and soil as everything else. Courts and characters come after that, reading the same world. The v0.50 design doc holds the earlier plans.

## App, UI and engine bindings

1. **Generated bindings.** The JS side of the WASM interface comes from one definition. No hand-kept pointer lists like today's `PTR_FNS`, which were incomplete.
2. **Rendering a moving world.** The globe mesh, shading and rivers rebuild from the live surface, river routing and ice. Changes are pushed as small dirty regions, not whole-map uploads.
3. **Map modes from stocks.** Every stock gets a view: bedrock change, sediment, ice thickness, soil depth, each plant type, each animal guild, nutrients, carbon, sea temperature, sea ice.
4. **Budget views.** A panel shows the world's water, carbon, sediment and energy budgets, so a leak is visible.
5. **The engine in a worker** with a fixed message protocol, so the UI never stalls.

**Events from stocks.** The news log only reports what the stocks did: a fire front crossing a region, a herd crash, a flood peak, a lake bursting, an ice shelf breaking. Events are readouts of the world, never separate dice. Each event links to the map mode that shows its cause.

## Testing, determinism and performance

1. **Every system ships with its tests,** listed in its section above. A system is not done until they pass on two seeds.
2. **Budget checks every year** in debug builds: energy, water, carbon, nitrogen and sediment in must equal out plus change.
3. **Determinism.** Same seed, same result, on every machine. The fingerprint tool proves it and proves speed-ups exact.
4. **Run on every build.** Fast checks on every build, the long runs (1,000 and 10,000 years) nightly.
5. **No panics in the world.** Checked access in the step code. A bad state is logged and that entity is skipped for the month, instead of `panic=abort` killing the run.
6. **Units in the types.** Months, years, kg, m and km2 are distinct types, so the old mix of years and months cannot compile.
7. **Speed measured, not guessed,** against the budget in the climate section.

### Data to test against

One list of the real datasets the tests need. Each must be found, licensed and baked before its test can run.

| What | Dataset | Used by |
| --- | --- | --- |
| Past climate | Beyer, Krapp & Manica 2020 snapshots; MARGO 2009 sea surface | Climate |
| Today's climate | the observed monthly climate already baked | Climate base, tests |
| Sea level and ice | Lambeck et al. 2014 | Geology, ice |
| Uplift today | GPS uplift rates (source to choose) | Crust |
| River sediment | a river sediment-load database (to choose) | Landscape |
| Soil depth | a global soil depth map (to choose) | Weathering |
| Biomes and plant biomass | today's land cover and biomass maps (to choose) | Plants |
| Burned area | GFED satellite totals | Fire |
| Herbivores and predators | Fritz & Duncan 1994; Carbone & Gittleman 2002 | Food web |
| Species ranges | PHYLACINE; today's range maps; pollen records | Starting state |
| Quakes and eruptions | today's baked hazard map and eruption record | Disasters |

"To choose" means the dataset is not picked yet.

## Migration from v0.49 and old saves

Old saves do not carry over. The state they hold (fixed terrain, plant targets, capacity shares, people) does not map onto live stocks. v0.49.1 stays installable and playable side by side. The new version uses its own save folder.

## Stages and order of work

The world comes first. Each stage must pass its own long run before the next one starts.

1. **Physical world.** Land, sea, air, ice and water, with conserved water and energy and one clock.
2. **Living land and sea.** Plants, soil and plankton on carbon and nutrient cycles. Fire, drought and erosion as feedbacks.
3. **Animals.** Grazers, predators, fish and disease living on those stocks, with migration and boom and bust.
4. **Long-run proof.** 10,000-year runs on two seeds, checked against real Earth. No people until this passes.
5. **People as a species.** Hunter-gatherers eat from and change the same stocks as every other animal.
6. **Settling, farming and states.** Then courts, culture and the later eras, all reading the same living world.

## Risks and open questions

1. **The climate is the hardest part.** The emulator plus feedback terms may not let ice and plants move the climate enough, or may let them run away. Mitigation: the Beyer snapshots as tests from day one, and coarse live physics as the upgrade path.
2. **Speed.** Live climate, plants, animals and geology together may not fit in about 90 ms a month. Mitigation: coarse grids, slower steps for slow systems, measure early.
3. **River resolution at 48 km.** The stream power law may not work on tiles that size without a sub-grid rule.
4. **The starting state.** Today's map holds today's erosion and rebound. The spin-up method is not chosen.
5. **Data for parameters.** Rock hardness, plant traits and animal rates need published values. Gaps mean hand-tuning.
6. **Stability over 10,000 years.** Coupled systems can drift or blow up. Budget checks and long runs catch it, but fixing it can take time.
7. **Scope.** This is a rewrite of the whole engine. People are years away. The order in Stages must hold, or the old problem comes back.
8. **Memory figures.** Several numbers in this doc are from memory and marked. Each must be checked before use.
