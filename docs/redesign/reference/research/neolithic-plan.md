# Neolithic build plan

5 Oct 2026. Built on the research in neolithic.md and diseases.md (same folder) and on what the engine already has:
the six dials (meso.rs), settled home bases, ways of life, cultures, wild-ancestor bits per 2-degree cell
(hooks.rs `wild_ancestors`), the wild-animal types (types.rs) and the farm model of the historical mode (lib.rs crops).

## The rule that matters most

Farming starts only where three things meet:
1. the wild ancestors live in that cell (a WA bit),
2. a settled or part-settled people already lives on them (dial[0] at least 0.4 and a grain, nut, wetland or
   swamp way of life),
3. the climate has steadied (the melt is mostly over).

Everywhere else, farming arrives by spread: farmers budding into new land at about 1 km a year, faster by sea,
or foragers taking it up after a long contact pause where they are dense and settled.

## Stages a people goes through

| Stage | What it is | How it shows on the map |
|---|---|---|
| Roving (Paleolithic) | bands move through the year | camp dots and trails, the land washed in the people's colour (as now) |
| Settled foragers (Mesolithic) | a home base, seasonal camps, stores | ringed home dot, spokes to camps, a border, midden and cemetery (as now) |
| Tending | cultivating wild stands; can still give it up | the same, plus small dark-green plots by the home |
| Farmers (Neolithic) | domestic crops and herds, villages | a square village mark, a patchwork of fields round it, pens or pasture in a herd colour |
| Chiefdom | a centre over several villages | a larger village mark with a ring, a filled border in the chiefdom's colour |

The labels change too: plain names for roving peoples, a small hut mark for settled ones, a field mark for
farmers, and a crown-like mark and capital name for chiefdoms.

## Build order

1. Data first (no change to results). Fix and extend `wild_ancestors`:
   - New Guinea yams and taro at height (the tc > 18 C test misses Kuk at 1,560 m).
   - New bits: zebu, Ethiopia (teff, enset, finger millet), South India (mungbean, horsegram, small millets),
     African rice, donkey, turkey, South American lowland crops (manioc, squash, arrowroot, peanut).
   - Aurochs in North Africa and India (zebu).
2. Disease burden per tile (place diseases): falciparum and vivax malaria, tsetse, schistosomiasis, hookworm and
   soil worms, from climate with the thresholds in diseases.md. Shown as a map mode. It feeds deaths and work, and
   grows with density, clearing and irrigation. Tsetse blocks cattle and horses.
3. Tending and domestication. A settled people on its wild ancestors tends them. A domestication score rises
   ~1/2,000 a real year (faster, ~1/500, on islands and for small groups). From 0.5 the crop yields more and can be
   carried out of its wild range. At 1 it is domestic. Animals start with herd management and change ~500-1,500
   years later.
4. Farming food. Crops by season, rain regime, frost-free months and day length (each crop carries the latitude it
   is used to and adapts ~1 degree per few centuries). Fields replace forest. Farmers hold far more people per km2.
5. Spread. Farmer peoples bud into suitable land; foragers next to farmers adopt after a pause scaled by their own
   density and settling. Barriers: day length, cold, rain season, tsetse, altitude, dense settled foragers.
6. Crowd and herd diseases. Zoonoses come with herds and dense villages. Crowd diseases need ~250,000-500,000
   connected people and arrive late (measles ~500 BCE). Before that, rare imported epidemics.
7. Chiefs and the proto-state AI (below).
8. Map looks for the stages (above).

## Chiefs and proto-states (the quasi-CK3/EU4 part)

When it wakes: a farming or rich settled people whose inequality dial passes ~0.45, or where several triggers hold
together (storable surplus above a year's need, land full and hemmed in, 1,000-2,000 people within reach of one
centre, frequent lethal raids, a choke point such as a pass, river mouth, salt or copper, and, from ~4000 BCE,
traction animals).

What a polity is: a centre village and its member villages (peoples), a ruler with a few traits, a lineage that
inherits, a treasury (stored surplus), a levy, prestige and legitimacy.

What it can do each year (the AI picks by weighted need, as CK3 and EU4 do):
- Raid or conquer a neighbour (the loser's land is taken when it cannot flee: circumscription).
- Demand tribute or make a village a vassal.
- Marry into another ruling lineage (alliance), or swap gifts.
- Feast and build (raises legitimacy and prestige, spends surplus).
- Split on a weak succession or a revolt by a big vassal.

Scale ladder: village 100-500 people; simple chiefdom 1,000-10,000; complex chiefdom 10,000-100,000; state above
~100,000 with three tiers of settlement and tribute. Expect 2,000-7,000 years from farming to the first state.

## Owner's choices (5 Oct 2026)

1. Rulers. Early on, while little is known, each polity is EU4-like: a ruler name and a few personality traits.
   Toward the Chalcolithic, as proto-countries form, rulers gradually become CK3-like: full personalities with far
   more traits than CK3 has, plus lineages and family ties.
2. Crops. Start with about ten crop groups, each with its wild-ancestor bit and climate limits. As a group is
   developed in a region it can split into local breeds or varieties (for example a hardy northern barley or a
   short-season maize). A breed can make a breakthrough: a new climate limit, more yield, or a new use.
