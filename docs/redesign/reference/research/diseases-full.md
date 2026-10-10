# Every infectious disease that mattered before ~1500 CE

Companion to `odyssey/research/diseases.md` (read that first: it holds the three-class scheme, the 6 place diseases
already in `disease.rs`, measles CCS, plague foci, tsetse and malaria temperature work). This file does not repeat
those tables; it extends them to the full register the owner asked for, with reservoir, region box, climate limits,
human-adaptation date, persistence needs, fatality, immunity and a staged model rule for each agent. 5 Oct 2026.

Tags: **[O]** = I opened the paper, its abstract, a repository copy or a reliable summary of that specific paper this
session. **[M]** = memory, not checked this session; treat as +-20% and as a guess on dates. Dates: BP = before 1950;
BCE/CE as given. Lat/lon boxes are deliberately generous; they are meant as code guards, not biogeography.

---

## 0. How this plugs into the engine

`disease.rs` today has 6 place diseases as pure climate suitabilities. The full register needs three things added:

1. **More place diseases** (section 2): same shape as now, `suit(tile) -> 0..1` from months, rain, water, realm,
   altitude. Cheap. No history.
2. **A reservoir layer** (section 3): per-tile presence of the animal or environmental host (rodent foci, bats, dogs,
   herds, waterfowl, snails, copepods, soil spores). This is what makes stage 0 real.
3. **A per-agent state machine** (section 6): `stage 0 reservoir -> 1 spillover -> 2 human-adapted`, advanced by
   triggers that read population, density, settlement, herds, irrigation and trade links, with a long dwell time so
   nothing human-adapted exists in the first few thousand years of play.

Suggested record per agent:

```
struct Agent {
  stage: u8,            // 0 reservoir, 1 spillover only, 2 human-adapted
  stage_t: f32,         // years in current stage (triggers need dwell time)
  suit: fn(tile)->f32,  // climate/vector suitability, 0..1
  res: ResKind,         // which reservoir map gates stage 0
  needs: Needs,         // ccs, density, herd, water_works, trade
  cfr: [f32; 3],        // child / adult / virgin-soil case fatality
  burden: f32,          // chronic weight on work and births
  immune: ImmKind,      // none / lifelong / partial-by-exposure / genetic
}
```

Rule of thumb for the whole set: **about 25 agents of the ~70 below should be able to reach stage 2 before 1500 CE in
a normal game.** The rest stay at 0 or 1 forever unless the player builds cities, irrigation and long trade.

---

## 1. Malaria: the five human species on one map

All five can share a single "malaria" field with a per-species mask. The temperature thresholds are what separate
them.

| Species | Threshold for sporogony | Degree-days to finish | Extra gate | Pre-1500 range box | Notes |
|---|---|---|---|---|---|
| P. falciparum | mean T >= 16 C (use 18 C for strong transmission) | 111 DD above 16 C | needs the month warm **and** wet (>= 60 mm) for vector breeding | Africa 20 N to 25 S strongest; S and SE Asia; New Guinea; Mediterranean/Near East lowlands in summer; Himalayan foothills reached by 3,000 BP | Gething et al. 2011 [O]; Himalayan aDNA ~3,000 BP (Michel et al. 2024 [O]) |
| P. vivax | mean T >= 14.5 C | 105 DD above 14.5 C | one warm month is enough: liver hypnozoites wait out winter | all temperate and warm Eurasia to ~60-64 N in summer [M]; present in Eurasia by the 4th millennium BCE [O]; rare in West/Central Africa (Duffy) | Gething et al. 2011 [O]; Michel et al. 2024 [O] |
| P. malariae | ~16 C, same as falciparum [M] | [M] | always sympatric with falciparum, never alone | Africa, S Asia, SE Asia, New Guinea, Amazonia (Old World origin) [M] | quartan fever, 72 h cycle; decades-long low-grade infection, so it survives in **smaller** groups than falciparum [M] |
| P. ovale (curtisi and wallikeri, two cryptic species) | ~16 C [M] | [M] | West and Central African forest belt; scattered New Guinea, Philippines [M] | relapsing like vivax (liver stages), so it tolerates a short season | the two lineages are genetically distinct species; genomes published 2017 [O: Rutledge et al., Nature 542, record opened] |
| P. knowlesi | ~16 C [M] | [M] | **macaque mask**: only where M. fascicularis / M. nemestrina / M. leonina live and a Leucosphyrus-group Anopheles flies | SE Asia only: Malay peninsula, Borneo, Sumatra, Indochina, Philippines; box lon 92-128 E, lat 10 S to 24 N | Moyes et al. 2016 [O]: Leucosphyrus complex wants **disturbed** forest (>60% tree cover, not intact); M. fascicularis also uses mosaic, cropland, wetland, urban; M. nemestrina mostly forest. Zoonotic only: no human-to-human chain. Stage caps at 1. |

Vectors by region: the table in `diseases.md` 1.2 stands (Sinka et al. 2012 [O]). Add: **Leucosphyrus/Dirus group**
for knowlesi; those are also the forest-edge falciparum vectors of SE Asia.

**Model rule (malaria).** One climate field per species, all stage 2 from the start (malaria is pre-agricultural), but
intensity multiplied by a land-change factor: `inten = suit * (0.4 + 0.6 * clearing_and_water)` where clearing is
forest cut for farming and standing water from irrigation or rice. Falciparum should roughly triple in a tile that
goes from forest to farmed. Knowlesi: `stage = 1` always, incidence proportional to how much a group forages or farms
inside macaque forest edge.

**Immunity and genetics.**

| Trait | Effect | Where / when | Tag |
|---|---|---|---|
| HbS (sickle) heterozygote | ~90% protection against severe falciparum; homozygote usually died young before medicine | single origin, East-Central Africa, ~7,300 years ago (259 generations, CI 123-395), before the Bantu expansion | [O] Esoh/Shriner-type coalescent estimate, bioRxiv 187419 |
| Duffy-negative FY*O | near-total resistance to vivax | fixed in West and Central Africa; why vivax is nearly absent there | [M] |
| Thalassaemias, G6PD deficiency | partial protection, falciparum | Mediterranean, Near East, S and SE Asia | [M] |
| Acquired immunity | after years of repeated bites, adults get sick but rarely die; children carry the deaths | everywhere with year-round transmission | [M] |

For code: give each population a `malaria_adapt` 0..1 that rises slowly (centuries) where falciparum suitability is
high, cutting adult deaths but costing a little fertility/anaemia. Duffy as a flag on African lineages.

---

## 2. Place diseases to add (vector, water or soil)

Already coded: falciparum, vivax, tsetse, schistosomiasis (lumped), soil worms (lumped), Chagas. Add these.

| Agent / disease | Varieties | Reservoir | Region box before 1500 | Climate limits for code | Vector | Needs to persist | Burden / fatality | Tag |
|---|---|---|---|---|---|---|---|---|
| Yellow fever virus | one virus, several African genotypes | monkeys (forest cycle) | Africa only: lat 15 N to 15 S, lon 17 W to 40 E | transmission 17.8-34.6 C, optimum 29.1 C (Ae. aegypti limits) | Aedes aegypti (towns), forest Aedes/Haemagogus | forest cycle needs monkeys; urban cycle needs a town of a few thousand with water jars | adult CFR 20-50% in virgin-soil outbreaks [M]; survivors immune for life | [O] Mordecai et al. 2017 thermal limits; Bryant et al. 2007 (in diseases.md) |
| Dengue 1-4 | four serotypes | monkeys in SE Asian and West African forest | forest cycle: SE Asia + West Africa; human cycle: large Asian ports only | Ae. aegypti 17.8 / 29.1 / 34.6 C; Ae. albopictus 16.2 / 26.4 / 31.6 C (cooler, more temperate) | Aedes aegypti, Ae. albopictus | human-only cycle needs big connected towns | MRCA of all four serotypes ~1,115 years ago (i.e. ~900 CE); each serotype's jump from monkeys to steady human transmission only ~125-320 years ago. **So before 1500 dengue should be mostly sylvatic/stage 1, with at most rare town outbreaks in Asia.** | [O] Twiddy, Holmes & Rambaut 2003, MBE 20:122; [O] Mordecai 2017 |
| Leishmania (cutaneous) | L. tropica (dry urban, Near East), L. major (rodent, desert fringe), L. aethiopica (hyrax, Ethiopia), L. mexicana/braziliensis (New World forest) | gerbils and other rodents, dogs, hyraxes, forest rodents | Near East, Central Asia, N Africa, Mediterranean, Sahel, Ethiopian highlands; separate New World set in C and S America | sandflies need **> 15 C for several months**; dry to semi-arid; L. major tied to gerbil colonies on desert edge | Phlebotomus (Old World), Lutzomyia (New World) | rodent burrows or dog-keeping villages; mud-walled houses help | disfiguring, rarely fatal; "oriental sore" | [O] EEA/Climate-ADAPT leishmaniasis indicator (15 C rule); rest [M] |
| Leishmania (visceral, kala-azar) | L. donovani (human-to-human, India/E Africa), L. infantum (dog reservoir, Mediterranean, Central Asia) | humans (donovani), dogs (infantum) | Bengal and Ganges plain, Sudan/Ethiopia/Kenya, Mediterranean rim, Iraq, Central Asia | as above | Phlebotomus argentipes (India), P. orientalis (E Africa) | donovani needs dense settled villages; infantum only needs dogs | untreated visceral CFR 75-95% [M]; slow, months | [M] |
| Trypanosoma brucei gambiense | West/Central African sleeping sickness | humans (and pigs) | tsetse belt west of the Rift | tsetse band already coded (16-32 C, woody cover, >500 mm) | Glossina palpalis group (riverine) | riverine forest + people fetching water | chronic, fatal in 1-3 years untreated; near 100% CFR | [M] (tsetse temps [O] in diseases.md) |
| T. b. rhodesiense | East African sleeping sickness | wild game (bushbuck, hartebeest), cattle | Rift Valley and south-east to Zambezi | as above | Glossina morsitans group (savanna) | game or cattle in savanna woodland | acute, fatal in weeks-months | [M] |
| T. congolense, T. vivax, T. brucei brucei | nagana in stock | wild ungulates | tsetse belt | as above | Glossina | — | the real cost: kills cattle, horses, camels; keeps herders out of the belt | [M] |
| Wuchereria bancrofti | lymphatic filariasis | humans only | Africa, Nile, S and SE Asia, Pacific islands, coastal Brazil(?) | needs > ~22 C for larval development; humid; **urban Culex form needs foul standing water, so villages/towns** | Culex quinquefasciatus, Anopheles, Aedes (Pacific: Ae. polynesiensis) | long close residence; takes years of repeated bites | elephantiasis, hydrocele; low deaths, high lost work | [M] |
| Brugia malayi / timori | Asian filariasis | humans, also leaf monkeys (B. malayi zoonotic form) | SE Asia, India, Indonesia, Timor | swamp forest, rice land | Mansonia, Anopheles | swamp/rice | as above | [M] |
| Onchocerca volvulus | river blindness; savanna strain (blinding) and forest strain (milder) | humans only (came from a cattle Onchocerca ancestor [M]) | West and Central African savanna rivers, Nile headwaters, Yemen highlands; Americas only post-1492 | needs **fast, broken, well-oxygenated water** for Simulium larvae: use `river_slope * river_flow` not just river size; warm | Simulium damnosum s.l. (savanna), S. neavei (forest, on crabs) | rapids and people farming the fertile river banks | blindness in 5-10% of adults in hyperendemic villages; whole valleys abandoned | [M] |
| Dracunculus medinensis | guinea worm | humans; copepods in still drinking water | Sahel, Nile, Arabia, Iran, India, Ghana/Volta; box lat 5-25 N across Africa and to the Indus | needs **seasonal ponds and step-wells**: 2-6 dry months with people drinking from standing water; warm | copepods (water fleas), swallowed | stagnant drinking water in the dry season; one village pond is enough | not fatal; a worm emerging over weeks cripples the harvest season; seasonal labour loss is the point | [M] |
| Hookworm | Necator americanus, Ancylostoma duodenale | soil larvae | tropics and warm subtropics both hemispheres; both species in the pre-Columbian Americas | soil T > 15-20 C, moist, shaded, not frozen or desert; already coded as D_WORMS | none (skin penetration) | defecation near dwellings; sedentism and bare feet | anaemia; mainly lost work and child growth | [M] |
| Ascaris lumbricoides (roundworm) | — | eggs in soil/manure | global, including temperate Europe and China | eggs survive cold; need moisture; rare in deserts; use rain > 250 mm and any non-frozen summer | none | sedentism, manure on fields | heavy loads in children stunt growth | [M] |
| Trichuris trichiura (whipworm) | — | eggs in soil | as Ascaris, slightly more moisture-demanding | same | none | same | mild unless heavy | [M] |
| Enterobius vermicularis (pinworm) | — | humans only | global, all climates (including Arctic) | **no climate limit**: direct egg transfer indoors | none | any household; survives in tiny bands - a true heirloom parasite | trivial burden; good flavour text | [M] |
| Strongyloides stercoralis | — | soil, autoinfection | warm moist tropics | soil > 20 C, moist | none | sedentism; persists in one host for decades | occasional fatal hyperinfection | [M] |
| Taenia saginata (beef), T. asiatica (pig liver, E Asia) | — | cattle / pigs as intermediate host | Old World wherever cattle or pigs are kept; human infection predates domestication (came from hyaena/felid tapeworms in Africa, Pliocene-Pleistocene) | none | — | eating raw or rare meat; herds | mild in the human host | [O] Hoberg et al. (Harold W. Manter Lab reprint): taeniids colonised hominids twice in Africa, **before** agriculture and domestication |
| Taenia solium (pork) | — | pigs; humans can host the larvae | Old World pig country; worst where pigs roam on human faeces | none | — | free-ranging pigs + open defecation | **neurocysticercosis**: epilepsy and death; the one serious tapeworm of settled farmers | [M] |
| Echinococcus granulosus | hydatid disease | dogs (definitive), sheep and other stock (cysts) | all herding country, worst on open steppe and sheep uplands | none | — | dog + sheep husbandry; feeding offal to dogs | cysts in liver and lung; slow, sometimes fatal | [M] |
| Diphyllobothrium latum | fish tapeworm | fish, via copepods | cold northern lakes and rivers: Baltic, Finland, Siberia, Alpine lakes, N Japan, Great Lakes | cold fresh water; **fish-eating peoples only** | copepods then fish | raw or lightly cured freshwater fish | B12-deficiency anaemia | [M] |
| Clonorchis sinensis / Opisthorchis | liver flukes | snails then fish; dogs, cats | China, Korea, Japan, Mekong, Siberian Ob | fish ponds, slow rivers, warm; **rice-and-fishpond farming** | snail + freshwater fish | eating raw fish; night soil in ponds | cholangiocarcinoma, liver disease | [M] |
| Fasciola hepatica | liver fluke | sheep and cattle; snails on wet pasture | temperate Europe, Near East, Andes later | wet pasture, snails, 10-25 C | Galba/Lymnaea snails on watercress | grazing wet meadows; eating raw greens | mostly a stock disease | [M] |
| Paragonimus | lung fluke | crabs and crayfish; cats, dogs, tigers | E and SE Asia, W Africa, S America | streams with crabs | crab/crayfish eaten raw | — | chronic cough mistaken for TB | [M] |
| Chagas (already coded) | T. cruzi TcI-TcVI lineages | opossums, armadillos, rodents, bats; domestic guinea pigs and dogs | Americas, 42 N to 46 S | bugs need mean T > ~15 C; thatch and mud walls; to 3,500 m in the Andes | triatomine bugs (T. infestans, Rhodnius, Triatoma) | permanent houses of mud and thatch; stored grain; guinea pigs | chronic heart and gut disease decades later; found in ~9,000-year-old Chinchorro mummies | [M] |
| Schistosoma (already coded; split it) | S. haematobium (urinary, Bulinus), S. mansoni (intestinal, Biomphalaria), S. japonicum (Oncomelania, amphibious), S. intercalatum/guineensis (Central Africa), S. mekongi (Mekong) | snails + humans; japonicum also buffalo, pigs, rodents | haematobium: Africa + Nile + Mesopotamia + Arabia; mansoni: Africa, Arabia, Nile; japonicum: Yangtze, S China, Philippines, Sulawesi; mekongi: lower Mekong | water 15-32 C, best 22-28 C; japonicum snail stops at ~33-34 N; needs standing or slow water people stand in | aquatic snails | **irrigation is the trigger**: canals and paddies multiply snail habitat | S. mansoni arose in East Africa 0.30-0.43 Ma with its snail host; spread to West Africa, reached the New World only with the slave trade (7 closely related New World haplotypes of West African type) | [O] Crellen/Webster-type mansoni phylogeography (UG Space record); rest [M] |

---

## 3. Reservoir zoonoses to add or extend

| Agent | Varieties | Reservoir | Region | Entry to humans | Needs | Fatality | Model rule | Tag |
|---|---|---|---|---|---|---|---|---|
| Yersinia pestis | preLNBA (5,300-4,867 BP), LNBA (5,100-2,457 BP, no flea genes), LNBA+ with **ymt** from ~4,000 BP, then 0.PE branches and the Antiqua/Medievalis/Orientalis pandemic lineages | burrowing rodents: great gerbil, marmots, ground squirrels (Central Asia, Mongolia, W China, Caucasus, Iran); many African rodents | Central Asian steppe and desert foci; Caucasus; Iran; E and S Africa | split from Y. pseudotuberculosis; earliest human cases ~5,300 BP (Latvia, Sweden); **flea-borne bubonic form only after ymt ~4,000 BP**; big epidemics need carts, horses, camels and later ships | rodent focus + trade route + grain stores; pneumonic spread needs crowding and cold indoor months | bubonic 40-70% untreated, pneumonic ~100%; a pandemic kills 30-60% of a connected region | stage 0 in rodent-focus tiles always; stage 1 (sporadic, small clusters) once people live within ~200 km of a focus; stage 2 **epidemic-capable** only after `ymt_year` (4,000 BP) AND a trade link of >1,000 km AND a settlement > ~5,000 people | [O] Slavin & Sebbane 2022, PNAS 119(21) |
| Rabies virus (and older lyssaviruses) | RABV dog/carnivore clades; bat clades (Americas) | dogs, wolves, foxes, jackals, raccoon dogs; vampire and insectivorous bats in the Americas | all Old World except some islands; Americas | the **modern** RABV radiation is young: all lineages MRCA ~749 years (HPD 363-1215), canid lineages within the past ~1,500 years, bat lineages ~180 years; so pre-1500 rabies in the sim should be "a rabies-like lyssavirus", not modern RABV | village dogs; wolf packs near herds | ~100% once symptomatic | stage 1 forever: a low constant spillover rate `k * dogs_per_capita + wolf_pressure`; a few deaths and a big fear/culture effect; never stage 2 | [O] Bourhy et al. / Troupin et al. rabies phylodynamics (PMC3326349) |
| Bacillus anthracis | A and B clades | **spores in soil**, grazing animals | global; worst on alkaline, calcium-rich soils in warm dry grassland: Eurasian steppe, Sahel, E Africa, Near East | prehistoric; herders catch it skinning dead stock | herds grazing soil with old carcasses; drought following rain concentrates it | cutaneous ~20% untreated, inhalational >90% [M] | stage 1, event-driven: a `soil_spore` field rises each time stock die on a tile, decays over decades; a drought year triggers a stock die-off plus a few human deaths | [M] |
| Brucella | B. melitensis (goats, sheep), B. abortus (cattle), B. suis (pigs) | domestic herds | wherever herds are milked: Near East, Mediterranean, steppe, N Africa, S Asia | with domestication (goats/sheep from ~10,500 BP) [M]; medieval B. melitensis genomes from Italy [M] | milk and birthing contact; indoor herding | rarely fatal; months of fever, joint damage, miscarriage | stage 1 tied to `herd_density * milk_use`; permanent background sickness, a flat work penalty in pastoral tiles | [M] |
| Leptospira | many serovars | rats, dogs, cattle, pigs; survives in warm standing water | tropics worldwide, rice country, flood plains | prehistoric | **flooding plus rats**: rice paddies, towns with rats, monsoon floods | 5-15% in severe (Weil's) cases [M] | place disease, event-driven: suitability = warm (> 20 C) + heavy rain month + rodent/stock density; spikes after floods | [M] |
| Coxiella burnetii (Q fever) | — | sheep, goats, cattle; wind-blown dust from birth fluids | dry herding country, Near East, Mediterranean, Central Asia | with herds [M] | herds plus dry dusty wind | low fatality, long fevers | stage 1 with herds; small flat penalty | [M] |
| Rickettsia prowazekii (epidemic typhus) | — | **humans + body louse**; flying squirrels in E North America | cold and cool temperate Eurasia; mountain and highland Americas later | ancient but the epidemics are historical; "when social, economic or political systems are disrupted and expose a large population such as refugees to louse infestation" | body lice: needs **clothing worn continuously**, cold season, crowding, no washing; sieges, armies, famine camps | 10-40% untreated in adults, worse with age | event disease: fires on `crowding * cold_months * (famine or war or siege)`; kills 10-30% of the crowded group, then burns out | [O] CDC EID 14(8) R. prowazekii genotyping paper |
| Rickettsia typhi (murine typhus) | — | rats, rat fleas | ports and granaries worldwide | with grain stores and ships [M] | rats + stored grain | low, ~2% | stage 1 with rats in towns | [M] |
| Orientia tsutsugamushi (scrub typhus) | — | chiggers (larval mites), field rodents | "tsutsugamushi triangle": Japan, Korea, E China, SE Asia, India, N Australia | ancient; recorded in Chinese texts [M] | scrub regrowth and field work | 5-30% untreated [M] | place disease: warm + shrubby regrowth + farming | [M] |
| Borrelia recurrentis (louse-borne relapsing fever) | — | humans + body louse | Eurasia and NE Africa; famine and war country | diverged from the tick-borne B. duttonii line within the last ~8,000 years; Bayesian estimate **4,724-5,630 years ago**; genome reduction done by ~2,000 BP; linked to "gradually increasing levels of sedentism during the development of agriculture, as well as the emergence of densely-occupied mega-settlements" | lice, clothing, crowding, cold | 10-40% untreated in epidemics [M] | stage 2 unlockable only after ~5,000 BP AND clothing AND a settlement > ~2,000; then behaves like typhus | [O] Swali et al. 2024/25 ancient Borrelia preprint (LJMU copy) |
| Borrelia duttonii / hispanica / persica (tick-borne relapsing fever) | several | soft ticks (Ornithodoros) in burrows and mud houses; rodents | Africa (duttonii, E Africa), Iberia and Maghreb (hispanica), Iran-Central Asia (persica) | ancient, pre-agricultural | mud-and-crack houses, rodent burrows, caves | few % | place disease: dry warm + mud housing | [O] same paper for the duttonii relationship |
| Borrelia burgdorferi s.l. (Lyme) | B. burgdorferi, B. afzelii, B. garinii | deer, small mammals, birds; Ixodes ticks | temperate Europe and Eurasian forest, NE North America | ancient (found in the Tyrolean Iceman [M]) | deer and woodland; forest edge | not fatal; arthritis, chronic pain | low-level place disease in temperate forest tiles; good flavour, small burden | [M] |
| Tick-borne encephalitis, Crimean-Congo haemorrhagic fever, Omsk fever | several flavi/nairoviruses | rodents, livestock, ticks | Siberian and European taiga (TBE); steppe and Near East (CCHF) | ancient [M] | forest or steppe with ticks and stock | TBE ~1-2%; CCHF 10-40% | rare-event place disease | [M] |
| Hantaviruses | Puumala, Hantaan, Seoul | voles and field mice | N Eurasia (Puumala), China/Korea (Hantaan), ports (Seoul) | ancient; Chinese texts describe a haemorrhagic fever with renal failure [M] | rodent irruptions in grain and forest mast years | 1-15% | event spikes in rodent-boom years | [M] |
| Monkeypox-like orthopoxviruses, cowpox | — | rodents (African squirrels, voles); cows as an accidental host | Central/West African rainforest; Europe (cowpox in voles) | ancient | forest hunting; dairying | low | stage 1; **cowpox matters as cross-immunity to smallpox** - a nice late twist | [M] |
| Influenza A | H1-H16/N1-N9 in birds; H1N1, H2N2, H3N2 lines in people | **wild ducks, geese, shorebirds**; pigs and domestic ducks as mixing vessels | flyways everywhere; worst where dense people + pigs + domestic waterfowl, i.e. S China rice country | cannot be dated from aDNA; recurrent pandemics are historical. Treat human endemic flu as unavailable before dense poultry/pig farming | domestic ducks + pigs + a large connected population; reintroduced from birds each winter | pandemic 0.5-3% [M], higher in virgin-soil islands | stage 0 in waterfowl tiles always; stage 1 spillover where `ducks * pigs * density` is high; stage 2 only as **repeating pandemic waves**, not steady endemicity, once connected population > ~1 M | [M] (Webster et al. 1992 from memory; I could not open it) |

---

## 4. Crowd diseases: human-only, immunising, need density

These are the ones the owner wants to **evolve gradually and late**. Dates below are the earliest the sim should
allow them, with the trigger they wait on.

| Disease | Agent and origin | Earliest date evidence | CCS (connected people) | Virgin-soil CFR | Endemic pattern | Model trigger | Tag |
|---|---|---|---|---|---|---|---|
| Smallpox | Variola virus; closest relatives **taterapox (gerbils)** and camelpox. VARV split from that pair ~7,700 years ago; TMRCA of sampled VARV ~4,000 ya (3,790-4,351), VARV-only estimate ~4,700 ya; likely origin Africa or the Middle East | ancient DNA in 11 northern European individuals 600-1050 CE (Viking Age), strains distinct from modern variola; earlier Egyptian claims unproven | ~200,000 [M] | 20-40% (variola major); 10-30% in a mixed population | childhood disease; scarring and blindness in survivors; lifelong immunity | spillover possible from gerbil/camel country (N Africa, Arabia, Sahel) after ~7,700 BP; **stage 2 requires** connected pop > 200,000 and 500+ years of continuous contact; before that, each arrival is a burn-out epidemic | [O] Forni et al. 2023, Microbial Genomics (dating); [O] Mühlemann et al. 2020, Science 369:eaaw8977 (Viking aDNA) |
| Measles | Morbillivirus from the rinderpest lineage (cattle) | rinderpest-measles split dated 1174 BCE - 165 CE, mean ~528 BCE | 250,000-500,000 | 20-30% | the classic childhood killer; lifelong immunity | needs **cattle plague present** + a city over ~250,000 in one connected net; nothing before ~1000 BCE under any conditions | [O] Düx et al. 2020, Science 368:1367 (via AAAS summary, in diseases.md) |
| Mumps | Paramyxovirus, likely bat-origin relatives | described by Hippocrates ~400 BCE [M] | ~150,000-300,000 [M] | low: orchitis and deafness, death rare | childhood disease | city > ~150,000 | [M] |
| Rubella | Rubivirus. **Closest known relatives found 2020: ruhugu virus in Ugandan leaf-nosed bats and rustrela virus in German yellow-necked field mice** - so a mammalian zoonotic origin, date unknown | first clearly described 18th century CE [M] | ~100,000-200,000 [M] | trivial in children; **congenital rubella** wrecks births | childhood disease, quiet unless pregnancies hit | allow only in large connected populations; main effect is a small permanent birth-defect rate | [O] Bennett et al. 2020, Nature 586 (USU/NMRC summary) |
| Pertussis (whooping cough) | Bordetella pertussis, from a B. bronchiseptica-like animal ancestor [M] | first epidemics described 16th century CE [M] | ~200,000+ [M] | kills infants, 5-15% of infants in virgin-soil outbreaks [M] | childhood disease | as measles but a bit lower | [M] |
| Diphtheria | Corynebacterium diphtheriae + phage toxin | described by Hippocrates (throat membrane) [M] | ~50,000-100,000 [M] | 20-50% in children untreated [M] | childhood; carriers keep it alive in smaller groups than measles | village-town level; **carriers mean a low CCS**, so it can appear early (Bronze Age towns) | [M] |
| Scarlet fever / strep | Streptococcus pyogenes, emm types; toxin phages | ancient; "scarlet" epidemics from the medieval period [M] | very low: carriers everywhere | 10-30% in epidemic waves of a virulent type [M] | endemic sore throats always; rheumatic heart disease and puerperal fever as chronic drains | available from the start at low level; **epidemic waves** when a new toxin type appears | [M] |
| Polio | Poliovirus 1-3, enterovirus | Egyptian stele of a withered leg, ~1400 BCE [M] | low: faecal-oral, endemic in small groups | <1% paralytic; paralysis in ~0.5% of infections | in dirty, dense settlements almost all children are infected young and most escape paralysis; **paralysis becomes common only when sanitation delays first infection** | stage 2 from early sedentism; a small permanent crippling rate | [M] |
| Hepatitis A | Hepatovirus | ancient, faecal-oral | none | <0.5% in children, worse in adults | universal childhood infection in dirty settlements | background disease with sedentism | [M] |
| Hepatitis E | Orthohepevirus; genotypes 3/4 from **pigs and deer** | ancient [M] | — | 1-4%, but **15-25% in pregnant women** | waterborne outbreaks | event disease: pigs + flooded water supply | [M] |
| Cholera | Vibrio cholerae O1, classical then El Tor biotypes; toxigenic strains carry the CTX phage | **reservoir is the water itself**: brackish estuaries, plankton and copepods, Ganges-Brahmaputra delta. Pandemics only from 1817 CE; El Tor appeared 1935 (Sulawesi), seventh pandemic from 1961 | none (environmental) | 30-60% untreated in severe cases | endemic in the Bay of Bengal delta; elsewhere explosive epidemics along trade | stage 0: environmental suitability field = brackish delta + water > ~20 C + plankton; **stage 1** local "bowel sickness" outbreaks in such deltas any time; **stage 2 pandemic-capable only very late**: require a delta city > ~100,000 plus long-distance shipping, and if the sim stops at 1500 it should normally never fire. This is the model's cleanest example of "evolves only when people are dense" | [O] Frontiers in Microbiology 6:1040 cholera pandemic review; [M] on temperature/salinity (I could not open the Vibrio-ecology paper) |
| Typhoid | Salmonella enterica Typhi | human only | host-specialised serovars "likely evolved in Europe over the last ~4,000 years", coinciding with **pig domestication**; an 800-year-old Paratyphi C genome from Trondheim, Norway | ~1,000s: carriers (gallbladder) keep it alive in small towns | 10-20% untreated | endemic where wells sit near latrines | stage 2 from the first permanent towns with wells, ~5,000 BP; carriers mean no large CCS needed | [O] Zhou et al. 2018, Current Biology 28:2420 (Warwick WRAP record) |
| Paratyphoid | S. Paratyphi A, B, C | Paratyphi C human-only, sister serovars in pigs (Choleraesuis, Typhisuis) | same paper; same ~4,000-year window | as typhoid | ~5-10% | same | same as typhoid, with a pig link: Paratyphi C should appear in pig-keeping Europe | [O] Zhou et al. 2018 |
| Shigella | S. dysenteriae (Shiga toxin), S. flexneri, S. boydii, S. sonnei | human only (evolved repeatedly out of E. coli) [M] | global | ancient [M] | low: tiny infectious dose, so it persists in small groups | 1-10%, far worse in famine and in children | the classic dysentery of armies, sieges and camps | available from early sedentism; spikes with crowding, famine, war | [M] |
| Enteric E. coli, Campylobacter, Giardia, Entamoeba histolytica, Cryptosporidium | many | humans, livestock, water | global | ancient | — | amoebic dysentery and liver abscess can kill; the rest sap children | treat as one "faecal-oral load" scalar driven by density, water quality and livestock | [M] |

---

## 5. Chronic human diseases and heirloom infections

| Disease | Agent and varieties | Reservoir | Region before 1500 | When it entered or adapted | Needs | Fatality / burden | Immunity | Tag |
|---|---|---|---|---|---|---|---|---|
| Tuberculosis | MTBC: human lineages L1-L4, L7, L8 (L8 a new sister clade from the African Great Lakes); animal lineages **M. bovis** (cattle), M. caprae (goats), M. pinnipedii (seals), M. microti (voles); M. canettii outside the complex (~14,000 SNPs away vs ~2,000 within the MTBC) | mostly human; cattle for M. bovis | Old World everywhere; pre-Columbian Peru had **seal-derived** strains | MTBC common ancestor younger than ~6,000 years on most calibrations (so not a long Palaeolithic disease); animal lineages lost TbD1/RD9 after the ancestral split | crowding, indoor smoke, poor food; milk and close herd contact for M. bovis | 50% death over years untreated; a slow steady drain on cities | partial, not sterile | [O] Ngabonziza et al. 2020, Nature Communications (L8 and M. canettii distances); Bos et al. 2014 (seals) in diseases.md |
| Leprosy | Mycobacterium leprae branches 0-5; **M. lepromatosis** (Mexico, and British red squirrels) | humans; red squirrels (medieval England), armadillos (Americas, post-1492) | Old World: India, China, Near East, Europe, Africa | medieval European strains sit on branch 3; a Winchester **squirrel** strain is closer to medieval human strains there than to modern squirrel strains - animal hosts are old | years of close household contact; cool damp climates are not required but Europe's epidemic needed towns | not fatal directly; disfigures and excludes; the social effect is the point (leper houses) | most people are naturally resistant (only ~5% susceptible) [M] | [O] Urban et al. 2024, Current Biology (red squirrel genome) |
| Treponematoses | T. pallidum **pallidum** (syphilis), **pertenue** (yaws), **endemicum** (bejel); pinta is T. carateum (no genome) | humans only | yaws: warm humid tropics worldwide (Africa, SE Asia, Pacific, Amazonia); bejel: dry Sahel/Near East/Central Asia; pinta: tropical Americas; syphilis: see date column | **the modern subspecies had already diverged in pre-Columbian times**; four ~2,000-year-old genomes from Brazil are closest to **T. p. endemicum** (bejel), in a region with no bejel today; early modern Europe (c. 1400-1700 CE) held both syphilis and yaws strains plus an extinct fourth lineage | yaws and bejel: skin and mouth contact among children in a village - **no density needed**; syphilis: sexual networks, so towns | yaws/bejel/pinta disfiguring, rarely fatal; syphilis 10-20% of untreated cases die of late disease, plus stillbirths | cross-immunity between the subspecies: a yaws-infected people resists syphilis | [O] Majander et al. 2024 Nature (Brazil endemicum-like, pre-Columbian divergence, via SUFE summary); [O] Majander et al. 2020, Current Biology 30:3788 (via The Science Breaker) |
| Hepatitis B | genotypes A-J; older relatives in apes | humans | global, including the Americas before contact | oldest genomes **~10,500 BP**; MRCA of all HBV ~20,000-12,000 BP; Eurasia/Americas split ~16-12 ka; an ~9,000-year-old genome from Cuncaicha in the Andes; **Mesolithic strains replaced by a new WENBA lineage at the Neolithic transition ~8-7 ka**, WENBA last seen ~3.3 ka, genotypes A and D appear ~2.5 ka | nothing: mother-to-child and blood contact | 15-25% of chronic carriers die of liver disease decades later | lifelong carriage, partial immunity | [O] Kocher et al. 2021, Science (author PDF) |
| Herpes simplex 1 | — | humans | global | ancient in the species, but **modern Eurasian HSV-1 diversity has an MRCA of only ~4.68 ka (3.87-5.65), globally ~5.29 ka**; ancient genomes 253-1700 CE; authors link the turnover to Bronze Age population growth and new customs (romantic kissing) | nothing: latency means it survives in bands of 20 | trivial deaths; neonatal herpes occasionally fatal | lifelong latent, reactivates | [O] Guellil et al. 2022, Science Advances 8:eabo4435 |
| Herpes 2, varicella-zoster, Epstein-Barr, CMV | — | humans | global | all co-evolved with humans; **latency, so no CCS** - varicella reappears as shingles decades later and reseeds the group | nothing | low | lifelong latent | [M] |
| HTLV-1 | subtypes A (cosmopolitan), B, D, E, F, G (Central Africa), **C (Australo-Melanesian, most divergent)** | humans; STLV-1 in African monkeys and Asian macaques | Central Africa, Japan (pockets), Melanesia/Australia, Andean South America | STLV-1 to human transmission ~**27,300 +- 8,200 years ago**; subtypes A, B, D, E in that frame; F within the last 10,000 years; multiple independent spillovers | breastfeeding and sex; clusters in isolated groups | ~5% of carriers get adult T-cell leukaemia or a paralysing myelopathy after decades | lifelong carriage | [O] Gessain & Cassar et al. 2019, Retrovirology 16 (Springer copy) |
| Trachoma | Chlamydia trachomatis ocular serovars A, B, Ba, C (genital D-K and LGV L1-3 are separate lineages) | humans | dry dusty country with flies and scarce water: Sahel, Egypt, Near East, Central Asia, India, Australia | ancient; the lineages diversified long ago and then mixed recently | **dry season dust + Musca sorbens flies breeding in human faeces + little washing water**; crowded sleeping huts | blindness in 1-5% of adults in bad villages; no deaths | repeated reinfection scars the lid; no protective immunity | [M] (I could not open the Hadfield 2017 Genome Research PDF: 403) |
| Lice, scabies, fleas, bedbugs | Pediculus humanus capitis/corporis, Pthirus pubis, Sarcoptes scabiei | humans | global | body louse needs clothing, so it dates from clothed humans (>70,000 BP) [M] | clothing, bedding, crowding | no deaths by themselves; **body lice are the vector for typhus, relapsing fever and trench fever** | none | [M] |

---

## 6. The staged model, concretely

### 6.1 Three stages and their triggers

```
stage 0  reservoir only
  - the agent exists on the map wherever its host or environment does
  - no human cases; the player may see "sick monkeys", "dead marmots", "rotten water"
  - gate: reservoir map (rodent focus, macaque forest, waterfowl flyway, brackish delta, soil spores)

stage 0 -> 1  SPILLOVER  (fires when people touch the reservoir)
  - trigger:  contact = pop_in_tile * reservoir_suit * exposure_mode
              exposure_mode = hunting, butchering, herding, water-fetching, grain-storing
  - effect:   rare deaths, small clusters, no chain; the agent cannot leave the tile
  - most agents stay here forever (rabies, anthrax, knowlesi, scrub typhus, hantavirus)

stage 1 -> 2  HUMAN ADAPTATION  (slow, needs all of: time, density, numbers)
  - trigger:  stage_t > dwell_years           (centuries; see table)
          AND connected_pop > ccs
          AND settled_fraction > s_min
          AND spillover_events > n_min        (each spillover is a lottery ticket)
          AND calendar_year > earliest_year   (hard floor from aDNA / molecular clocks)
  - effect:   now transmits person to person; spreads along trade ties; becomes endemic
              where connected_pop > ccs and epidemic where it is below
```

### 6.2 Parameters per agent (suggested)

| Agent | earliest_year | ccs | dwell | s_min | Can reach stage 2 before 1500 CE? |
|---|---|---|---|---|---|
| Plague, flea-borne | 4,000 BP | — (epidemic, not endemic) | 200 y | 0.3 | yes, with steppe trade |
| Smallpox | 7,700 BP (spillover), realistic stage 2 ~3,500 BP | 200,000 | 500 y | 0.6 | yes, in a big river-valley civilisation |
| Measles | 1,000 BCE | 250,000-500,000 | 300 y | 0.7 | yes, late and only in the largest nets |
| Mumps, rubella, pertussis | 500 BCE | 150,000-250,000 | 300 y | 0.7 | only in the very largest |
| Influenza (pandemic-capable) | 3,000 BP | 1,000,000 | 300 y | 0.6 | rarely; needs rice, ducks, pigs, cities |
| Cholera (pandemic) | ~1800 CE realistically | 100,000 delta city + shipping | 500 y | 0.8 | **no** in a game ending at 1500; keep it as local delta dysentery |
| Typhoid, paratyphoid | 4,000 BP | ~2,000 (carriers) | 100 y | 0.5 | yes, easily, with wells and latrines |
| Diphtheria, Shigella, polio, hepatitis A | from sedentism | small | 100 y | 0.4 | yes |
| Tuberculosis (human lineages) | 6,000 BP | ~10,000 | 300 y | 0.5 | yes |
| Leprosy | 4,000 BP | ~5,000 | 300 y | 0.4 | yes |
| Syphilis (sexual treponeme) | pre-Columbian split, so allow from ~2,000 BP | ~10,000 | 300 y | 0.6 | yes, in towns |
| Yaws, bejel, pinta | from the start | none | — | 0 | already stage 2 |
| Louse-borne relapsing fever | 5,000 BP | ~2,000 + clothing | 200 y | 0.4 | yes |
| Epidemic typhus | 4,000 BP | crowding event, no ccs | 200 y | 0.4 | yes, as a war/famine disease |
| Dengue (human cycle) | ~900 CE | 500,000 port city | 300 y | 0.8 | only just, in a large Asian port |
| Yellow fever (urban cycle) | 1,000 BP [M] | 10,000 town in Africa | 200 y | 0.6 | yes, in West African towns |
| Rabies, anthrax, brucellosis, Q fever, leptospirosis, hantavirus, scrub typhus, knowlesi, sylvatic dengue and yellow fever, tick fevers | — | — | — | — | **no: capped at stage 1 by design** |

### 6.3 Epidemic versus endemic once at stage 2

- `connected_pop < ccs`: the disease cannot stay. Arrival is an event: a share of the non-immune die
  (use the virgin-soil CFR), survivors are immune for life, the agent then vanishes from that region until
  reimported. This is the Columbian-exchange machine.
- `connected_pop > ccs`: endemic childhood disease. Steady child mortality of a few percent per year of exposure, no
  adult deaths, and the region becomes a **source** that devastates every low-density neighbour it touches.
- Immunity bookkeeping can be a single `immune_frac` per region per agent, decaying as children are born.

### 6.4 Landscape and economy multipliers (apply to any agent)

| Change the player makes | Diseases it raises |
|---|---|
| clear forest for fields | falciparum (puddles, sunlit water), scrub typhus, leishmaniasis (forest edge) |
| irrigate, dig canals, flood paddies | schistosomiasis, malaria, filariasis, leptospirosis, liver flukes, Japanese encephalitis |
| store grain in settlements | plague, murine typhus, hantavirus, leptospirosis |
| keep cattle | TB (M. bovis), brucellosis, anthrax, Taenia saginata, measles's ancestor |
| keep pigs | T. solium (epilepsy), paratyphoid C, hepatitis E, influenza reassortment |
| keep dogs | rabies, hydatid, visceral leishmaniasis |
| build permanent mud-and-thatch houses | Chagas (Americas), tick-borne relapsing fever, trachoma |
| wear clothing continuously, crowd in winter | body lice, typhus, louse-borne relapsing fever |
| dig wells beside latrines | typhoid, dysentery, hepatitis A, polio |
| trade over 1,000+ km | every stage 2 agent; plague above all |

---

## 7. Climate limits table ready for code (new entries only)

| Agent | Rule |
|---|---|
| P. falciparum | months with mean T >= 16 C and rain >= 60 mm; strong above 18 C; cut above ~34 C |
| P. vivax | months with mean T >= 14.5 C and rain >= 30 mm; 1-2 months suffices |
| P. malariae, P. ovale | as falciparum, but ovale restricted to realm 1 (Africa) west of ~30 E plus New Guinea; malariae wherever falciparum is |
| P. knowlesi | lat -10..24, lon 92..128; tree cover 0.3-0.85 (disturbed, not intact); mean T >= 20 C |
| Aedes-borne (yellow fever, dengue) | transmission between 17.8 and 34.6 C, peak 29.1 C (Ae. aegypti); 16.2-31.6 C, peak 26.4 C (Ae. albopictus); needs a frost-free winter |
| Sandflies (leishmaniasis) | mean T > 15 C for 3+ months; semi-arid best (rain 100-800 mm); to ~2,000 m |
| Blackflies (onchocerciasis) | realm 1; river with high slope x flow ("rapids"); mean T > 20 C; within ~15 km of the river |
| Guinea worm | 2-6 consecutive dry months, mean T > 22 C, and a settlement that drinks from standing water |
| Snails (schistosomes) | water 15-32 C part of the year; standing or slow water; S. japonicum only below ~34 N in E Asia |
| Cholera (environmental) | coastal or deltaic tile, water > 20 C, brackish (river mouth), plankton-rich |
| Anthrax | grassland, alkaline soil proxy (dry, calcareous: rain < 800 mm, pH high), herds present |
| Leptospirosis | mean T > 20 C and a month with rain > 200 mm (flood), plus rodents or stock |
| Body-louse diseases | needs cold months (clothing worn) plus crowding: `min(cold_months/4, 1) * crowding` |
| Tick-borne (Lyme, TBE) | temperate forest, 7 or more months above 7 C, humid litter layer |
| Diphyllobothrium | lakes/rivers in tiles with mean T < 10 C and fishing as a food source |

---

## 8. What I could not verify this session

- Many climate thresholds are still [M]: schistosome snail water temperatures, hookworm soil limits, filariasis
  22 C, blackfly and guinea-worm rules, anthrax soil chemistry. These are my own numbers shaped to be codeable, not
  quotations.
- **PubMed/PMC answered with a captcha** on most tries (one PMC page did load), so several dating papers were reached
  only through repository copies or institutional summaries. Pages refused: Europe PMC REST API (429), PMC Spyrou
  2022 plague review (captcha), Hadfield et al. 2017 trachoma genomics (403), UCL copy of Schuenemann et al. 2018
  leprosy diversity (429), nature.com article page for the Brazil treponeme paper (429), the Vibrio climate-ecology
  review (captcha), the Webster 1992 influenza review (login wall).
- No molecular dates found for: mumps, pertussis, diphtheria, Shigella, polio, hepatitis A, trachoma, Ascaris,
  hookworm, filariasis, onchocerciasis, guinea worm. Their dates above are [M] and should be treated as placeholders.
- M. leprae MRCA age and M. bovis's split date were not pinned down; only the branch structure and the squirrel host.
- Influenza's deep history is genuinely unknowable from aDNA; the model rule above is a design choice, not a finding.
- Cholera: I found no evidence of pandemic cholera before 1817 CE. If the owner wants "cholera evolving once people
  are dense", the honest version is that it should stay a **local delta disease** in a game ending at 1500.

---

## 9. Sources opened this session

- Forni D. et al. 2023. Quantifying the evolutionary history of variola virus. *Microbial Genomics* (Univ. Milano-Bicocca copy). https://boa.unimib.it/retrieve/d0f7aed4-182c-4de8-8618-f85fc928bdce/Forni-2023-Microbial%20Genomics-VoR.pdf
- Gessain A., Cassar O. et al. 2019 (HTLV-1 origins review). *Retrovirology* 16. https://link.springer.com/article/10.1186/s12977-019-0503-0
- Guellil M. et al. 2022. Ancient herpes simplex 1 genomes reveal recent viral structure in Eurasia. *Science Advances* 8:eabo4435. https://eprints.whiterose.ac.uk/209184/1/sciadv.abo4435.pdf
- Hoberg E.P. et al. (taeniid tapeworms colonised hominids in Africa before domestication), Harold W. Manter Laboratory reprint. https://digitalcommons.unl.edu/parasitologyfacpubs/805
- Kocher A. et al. 2021. Ten millennia of hepatitis B virus evolution. *Science* 374:182. https://www.stephanschiffels.de/pdfs/Kocher2021-zq.pdf
- Majander K. et al. 2020. Ancient bacterial genomes reveal a high diversity of Treponema pallidum strains in early modern Europe. *Current Biology* 30:3788 (The Science Breaker summary by the authors). https://thesciencebreaker.org/breaks/health-physiology/diversity-matters-syphilis-and-related-diseases-in-historical-europe
- Majander K. et al. 2024/25. Redefining the treponemal history through pre-Columbian genomes from Brazil. *Nature* (university news summary). https://academicnewsletter.sufe.edu.cn/info/250537
- Michel M., Skourtanioti E., Pierini F. et al. 2024. Ancient Plasmodium genomes shed light on the history of human malaria. *Nature* 631:125-133 (CSIC Incipit record). https://www.incipit.csic.es/en/research_outcomes/publication/artigo/ancient-plasmodium-genomes-shed-light-on-the-history-of-human-malaria
- Mordecai E.A. et al. 2017. Detecting the impact of temperature on transmission of Zika, dengue and chikungunya using mechanistic models. (NSF PAR copy.) https://par.nsf.gov/servlets/purl/10057084
- Moyes C.L. et al. 2016. Predicting the geographical distributions of the macaque hosts and mosquito vectors of Plasmodium knowlesi malaria. *Parasites & Vectors* 9. https://ir.unimas.my/id/eprint/14051/
- Mühlemann B. et al. 2020. Diverse variola virus (smallpox) strains were widespread in northern Europe in the Viking Age. *Science* 369:eaaw8977 (sci.news summary). https://www.sci.news/archaeology/viking-age-smallpox-08675.html
- Ngabonziza J.C.S. et al. 2020. A sister lineage of the Mycobacterium tuberculosis complex discovered in the African Great Lakes region. *Nature Communications* 11. https://pure.itg.be/ws/files/12719094/2020ncom2917.pdf
- Prendergast J. (Rickettsia prowazekii genotyping and epidemic typhus). *Emerging Infectious Diseases* 14(8). https://wwwnc.cdc.gov/eid/article/14/8/08-0444
- Rutledge G.G. et al. 2017. Plasmodium malariae and P. ovale genomes provide insights into malaria parasite evolution. *Nature* 542:101 (CDC Stacks record). https://stacks.cdc.gov/view/cdc/44403
- Bennett A.J., Paskey A.C. et al. 2020. Relatives of rubella virus in diverse mammals. *Nature* 586:424 (USU summary). https://news.usuhs.edu/2020/10/researchers-identify-first-known.html
- Schistosoma mansoni origin and diversification (East African origin 0.30-0.43 Ma; New World founder haplotypes), UG Space record. https://ugspace.ug.edu.gh/items/cd574585-3d23-492f-b38a-084f871e8b2f
- Shriner D. / Esoh K.-type coalescent dating of the sickle allele (~7,300 years, single origin). bioRxiv 187419. https://www.biorxiv.org/content/10.1101/187419v1
- Slavin P., Sebbane F. 2022. Emergence and spread of ancestral Yersinia pestis in Late-Neolithic and Bronze-Age Eurasia, ca. 5,000 to 2,500 y B.P. *PNAS* 119(21). https://www.storre.stir.ac.uk/bitstream/1893/34412/1/pnas.2204044119.pdf
- Swali P. et al. 2024/25. Ancient Borrelia genomes document the evolutionary history of louse-borne relapsing fever. (LJMU manuscript copy.) https://researchonline.ljmu.ac.uk/id/eprint/26462/8/Borrelia_manuscript_14May_2024.pdf
- Twiddy S.S., Holmes E.C., Rambaut A. 2003. Inferring the rate and time-scale of dengue virus evolution. *Molecular Biology and Evolution* 20:122. https://academic.oup.com/mbe/article/20/1/122/1016343
- Urban C. et al. 2024. Ancient Mycobacterium leprae genome reveals medieval English red squirrels as animal leprosy host. *Current Biology* (HEAS summary). https://heas.at/allgemein/ancient-mycobacterium-leprae-genome-reveals-medieval-english-red-squirrels-as-animal-leprosy-host
- Vibrio cholerae pandemic biotypes review. *Frontiers in Microbiology* 6:1040. https://www.frontiersin.org/journals/microbiology/articles/10.3389/fmicb.2015.01040/pdf
- Zhou Z. et al. 2018. Pan-genome analysis of ancient and modern Salmonella enterica demonstrates genomic stability of the invasive Para C lineage for millennia. *Current Biology* 28:2420 (Warwick WRAP record). https://wrap.warwick.ac.uk/id/eprint/102459
- A fresh look at the origin of Plasmodium falciparum (review of gorilla-origin dating; estimates from 5,000-50,000 years to 2.5 Ma, all weakly calibrated). https://pmc.ncbi.nlm.nih.gov/articles/PMC3044689
- Troupin C. / Bourhy H. et al., rabies virus phylodynamics (canid lineages within ~1,500 years). https://pmc.ncbi.nlm.nih.gov/articles/PMC3326349
- European Climate-ADAPT indicator: climatic suitability for leishmaniasis transmission (sandflies need > 15 C for several months). https://climate-adapt.eea.europa.eu/en/metadata/indicators/climatic-suitability-for-infectious-disease-transmission-leishmaniasis
