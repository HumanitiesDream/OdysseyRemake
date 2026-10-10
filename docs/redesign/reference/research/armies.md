# Armies: size, logistics, equipment and battle (early states to 1750)

Companion to `chiefdoms-war.md`. That file covers tribal war, raid parties, chiefdom levies, war seasons for farmers,
fortified villages and Neolithic weapons. This file starts where it stops: early states, empires, medieval and
early modern armies. It adds army size as a share of population, logistics limits, equipment with dates and inputs,
battle casualties, and a model spec.

Tags: **[O]** = I opened the source this session (paper, reference page or a page quoting the work).
**[M]** = from memory, not checked this session; use as a plausible default. A search snippet was never used.
"[O, computed]" = I did the arithmetic on opened numbers.

Access notes: Wikipedia worked for many pages but several returned "cache-only" (Military history of the Mongol
Empire, Hittite military, French Royal Army, East Roman army). Perseus and Wikisource (Thucydides) were blocked.
So Athens 431 BCE, Mongol 1206 strength, Hittite figures and Louis XIV's army are [M].

---

## 0. Summary for the model (one screen)

| Item | Default for Odyssey | Basis |
|---|---|---|
| Fighting-age men | ~24% of population (males 15-49) | as in chiefdoms-war.md [M] |
| Early state standing force | 0.2-0.5% of population | Sargon 5,400 "ate bread daily before him" [O]; [M] ratio |
| Bureaucratic empire, standing army | ~1-2% of population | Byzantium 565: 379,300 of 19.5 M = 1.9% [O, computed]; Han 400-700k [O] of ~57 M [M] = ~1% |
| Citizen militia city-state, full call-up | 10-20% of population (40-80% of men) | Rome 225 BCE: 770,000 arms-bearing [O] of ~4 M [O] = 19% [computed] |
| Nomad confederation | 10-20% of population (most men ride) | [M] Mongols ~95-105k from ~1 M |
| Feudal kingdom field army | 0.2-0.5% of population | Hastings 7-12k each side [O]; England ~1.5-2 M [M] |
| Early modern European state | 1-3% (Prussia 3.2% in 1740) | Prussia 80,000 of 2.5 M [O, computed] |
| Largest field army before ~1650 | rarely > 50,000 | Military revolution page [O]; Manning [O] |
| Largest field army 1650-1750 | ~80,000-110,000 with depots | Malplaquet 1709 [O] |
| March rate | 15-25 km/day; 30+ forced; ~5 km/h | Engels, US Army history [O] |
| Food | ~1 kg grain per man per day; horse ~7 kg barley; mule ~2.25 kg + grazing | Devereaux, citing Roth, Engels [O] |
| Days of food carried | ~3-10 days on the body or pack animals | Engels via Hunt [O]; Roman sarcina ~10 days [O] |
| Casualties, battle | winner ~5%, loser ~15% (hoplites); loser up to 50%+ when pursued by cavalry | Krentz [O]; Mohács [O] |
| Quality multiplier | veteran 9 : average 3 : levy 1 (Sabin) — the sim should soften this | Sabin via Manning review [O] |

---

## 1. Army size as a share of population

### 1.1 Terms

- **Military participation ratio (MPR).** Share of adult men under arms (Andreski) [M]. I also give share of all
  people, because the sim tracks total population. With 24% fighting-age men, 1% of population = ~4% of men.
- **Paper vs field.** Paper strength (registers, total arms-bearing men) is often 3-10x the field army at one
  battle. Ming registers listed ~3 million hereditary soldiers; actual strength was ~845,000, and some provinces
  in 1587 had 2% of nominal [O]. Spain in 1625 had 300,000 regulars and 500,000 militia on paper [O].
- **Claimed vs documented.** Delbrück's method (Sachkritik) checks claimed numbers against demography, economics and
  march columns; he concluded ancient chroniclers inflated numbers and that the winner usually had more troops [O].
  Manning (2019 blog of a published historian) argues there is no reliable evidence of 100,000+ combatants at one
  battle before about the Napoleonic Wars; Maurice's *Strategikon* (c. 600 CE) says actual Roman armies rarely
  reached 20,000 [O].

**Rule for the sim:** treat literary numbers for foreign or defeated armies as 3-10x too high. Use records of pay and
food (lists, registers) as the real bound.

### 1.2 Early states (c. 3000-1600 BCE)

| Case | Number | Share | Basis |
|---|---|---|---|
| Sargon of Akkad (c. 2300 BCE) | 5,400 men "ate bread daily before him" (standing core) | Akkad ~1 M people [M] → 0.5% | [O] |
| Old Kingdom Egypt | levies raised by nomarchs and joined under pharaoh; Weni (6th Dyn.) reorganised the army and first used Nubian troops | levy only [O]; size "tens of thousands" [M] | [O] |
| Shang, Fu Hao (c. 1200 BCE) | led up to 13,000 men | Shang core ~1-4 M [M] → <1% | [O] |
| Sumerian city-state | a few hundred to a few thousand per city (Stele of the Vultures phalanx) | 3-10% of a city's people at most [M] | [M] |

### 1.3 Bronze Age empires (1600-1200 BCE)

| Case | Number | Basis |
|---|---|---|
| Kadesh, 1274 BCE, Egypt | ~16,000 infantry, 2,000 chariots (2 crew each); four divisions; plus Ne'arin and Sherden | [O] |
| Kadesh, Hittites | 2,500 chariots (3 crew), 15,000-40,000 infantry (many not engaged) | [O] |
| Egyptian claims are "bombastic"; broad facts likely right | | [O] |
| New Kingdom Egypt total | "over 100,000" under Ramesses II; corps of 4,000+ infantry in 20 companies of 200-250 | [O] |
| Egypt population | ~3-4 M [M] → total ~3%, field ~0.7% | [computed] |

New Kingdom Egypt moved from levies to a professional army [O]. Chariot forces were elite and expensive (see 3.4).

### 1.4 Iron Age and classical (1200 BCE - 500 CE)

| Case | Claimed / documented | Share and notes | Basis |
|---|---|---|---|
| Assyria, Qarqar 853 BCE | Shalmaneser III claims 20,000 inf, 12,000 cav, 1,200 chariots; elsewhere "120,000" | royal inscriptions "notoriously unreliable"; Ahab's 2,000 chariots may be 200 | [O] |
| Qarqar coalition | 53-63k inf, 4,000 chariots, 2,000 cav, 1,000 camels (claim) | first recorded camel troops | [O] |
| Xerxes 480 BCE | Herodotus: 1.7 M infantry, ~2.5 M fighting men | modern 100,000-500,000; Maurice: water along route for ~200,000 men and 70,000 animals | [O] |
| Gaugamela 331 BCE | Alexander ~47,000 (Engels 47,144) | Persians claimed 1 M; Delbrück ~91k, Engels/Green ≤100k | [O] |
| Athens 431 BCE | 13,000 field hoplites, 16,000 garrison, 1,200 cav, 1,600 archers, 300 triremes (Thuc. 2.13) | ~250-300k people → ~10% under arms | [M] |
| Rome 225 BCE | arms-bearing Romans and allies: >700,000 foot, 70,000 horse (Polybius 2.24) | ~4 M people [O] → 19% of all people registered; >100,000 actually under arms incl. reserves | [O] |
| Rome legions | 8 legions (40,000) in 338; 16 (80,000) in 290; 18 (90,000) in 272 BCE | | [O] |
| Warring States | Changping 260 BCE: Qin 550k vs Zhao 450k; "every able-bodied man over 15" conscripted | numbers from Sima Qian ~150 years later; treat as 3-10x high | [O] |
| Han | conscription at 23 (20 after 155 BCE), 2 years' service; standing 400-700k; 119 BCE: 100k cav, 200k inf, 140k supply horses | military-eligible 13-15 M ≈ 30% of people [O]; ~1% standing | [O] |
| Han crossbows | 537,707 in one arsenal (13 BCE) | shows scale of state arms production | [O] |
| Maurya | Megasthenes: 600k inf, 30k cav, 8,000 chariots, 9,000 elephants | 15-30 M people [O] → 2-4%; likely inflated | [O] |

### 1.5 Medieval (500-1450)

| Case | Number | Share / notes | Basis |
|---|---|---|---|
| Byzantium (Treadgold) | 565: 379,300 (field + guards 150,300); 641: 129,000; 773: 80,000; 840: 154,600; 1025: 110,000; 1081: 20,000 | 565: 1.9% of 19.5 M; 641: 1.2% of 10.5 M [computed]. Whittow argues much smaller | [O] |
| Byzantine field army | typically 15,000-25,000; Belisarius 533: <16,000 | | [O] |
| Abbasid | Abu Muslim's revolt 747: ~10,000; al-Mu'tasim (833-842) builds first standing army of Turkish slave soldiers | sizes [M] (Kennedy: tens of thousands paid) | [O]/[M] |
| Western Xia 1030s | all men 15-60 levied = 150,000; 370,000 claimed by 1038 | nomad-style levy | [O] |
| Song 1081 | five armies, 380,000+ against Xia | total standing ~1.2 M in 1040s on ~100 M [M] → ~1% | [O]/[M] |
| Mongols, Khwarazm 1219 | medieval claims 600-800k; modern 75,000-200,000; Secret History 100-135k | | [O] |
| Mongols 1206 | ~95 units of 1,000 (~95-105k) from ~1 M people | ~10% of people, most adult men | [M] |
| Mongol horses | 3-4 per soldier; 60 arrows per quiver | | [O] |
| Hastings 1066 | Normans 7-12k; English 5-13k (most say 7-8k); chroniclers said 14k-150k | Norman mix ~50% inf, 25% cav, 25% archers | [O] |
| Agincourt 1415 | English ~9,000 (Curry), ~5/6 archers; French 12,000 (Curry) to 14-15k (others) | | [O] |
| England, Towton 1461 | chronicle numbers would mean ~600,000 of <3 M people, "almost every adult fighting man"; real Wars of the Roses battles 10-15k at most | | [O] |
| Mali 14th c. | standing army 100,000 with 10,000 cavalry (chroniclers) | horses imported | [O] |

### 1.6 Early modern (1450-1750)

| Case | Number | Share / notes | Basis |
|---|---|---|---|
| Ottoman Janissaries | <1,000 (1400); 7,841 (1484); 13,599 (1574); 37,627 (1609) | firearms from early-mid 1400s; timariot cavalry ~40,000 was the main force | [O] |
| Mohács 1526 | Ottomans ~45,000 regular + 10-20k irregular, up to 160 guns; Hungarians 25-28k, 53 guns | | [O] |
| Ottoman guns | 1,014 guns cast 1522-1526 at the Tophane | | [O] |
| Mughal | Ain-i-Akbari: ~4.4 M "zamindari" men (<0.5 M cavalry); Roy: 911,400 (1647); elephants 5,000-7,000 under Akbar | 4.4 M of ~110-150 M [M] → ~3% paper; ~0.7% real | [O] |
| Ming | 3 M on registers, ~845,000 actual; 1600 campaign >200,000; 1550: 60,000 routed by Altan Khan | ~0.6% of ~150 M [M] | [O] |
| Songhai, Tondibi 1591 | Songhai 9,700-30,000 inf and 12,500-18,000 cav (some say 80,000); Moroccans ~4,000 with arquebuses and 6-8 guns | Moroccans crossed the Sahara with 8,000 camels | [O] |
| Sekigahara 1600 | East 75,000-89,000; West 82,000-120,000; ~2 hours of fighting | | [O] |
| Spain 1625 | 300,000 regulars, 500,000 militia (paper) | | [O] |
| Field armies | <50,000 for most of 1500-1650; then depot supply allowed >100,000 in the 18th c. | | [O] |
| Malplaquet 1709 | Allies 86-110k, 100 guns; French 75-90k, 80 guns | | [O] |
| Prussia | 1713: 40,000; 1740: 80,000 of 2.5 M (3.2%); 1786: 200,000 | canton system: 3 months' drill a year | [O] |
| Europe 1000-1900 | military share of population rose for ~900 years, peaked ~1900 (Adams 2024, from Sorokin) | | [O] |

### 1.7 Americas, Africa, Polynesia

| Case | Number | Basis |
|---|---|---|
| Aztec | claims of 200,000 warriors + 100,000 porters (Coixtlahuacan), 400,000 (Tututepec 1506); march 19-32 km/day | [O] (claims) |
| Aztec unit | xiquipilli of 8,000; porters carried food; realistic field armies tens of thousands | [M] Hassig |
| Inca | 1 in 50 men over 25 served by mit'a; officers professional; armies of 100,000, up to 200,000 under Huayna Capac; decimal units 10-10,000; road storehouses (qullqa) | [O] |
| Mali / Songhai | see above | [O] |
| Zulu (1879, after the sim period) | Isandlwana ~20-25k warriors in age regiments | [O]; kingdom ~250-300k [M] → ~8% |
| Hawaii 1795 | ~12,000 men (see chiefdoms-war.md) | [O] earlier |
| Maori, Hingakaka 1807 | ~16,000 warriors claimed; potatoes let war parties campaign up to a year (Belich) | [O] |

### 1.8 Defaults by society type

Share of **all people** under arms. "Standing" = paid all year. "Max levy" = everything one war can raise.
"Field" = one army in one place.

| Society type (era) | Standing | Max levy, one war | Largest field army | Mix |
|---|---|---|---|---|
| Tribe / chiefdom | see chiefdoms-war.md | 5-20% | 0.3-12k | all levy |
| Early state (palace) | 0.2-0.5% | 2-4% | 5-15k | retinue + corvée levy |
| Bronze Age chariot state | 0.5-1% | 2-3% | 15-30k (2-3k chariots) | chariot elite + levy infantry |
| Citizen city-state (polis, Republic) | ~0 (militia) | 8-15% (30-60% of men) | 10-50k | citizen heavy infantry |
| Conscript bureaucratic state (Qin, early Han) | 1% | 3-5% | 30-100k (real) | conscript infantry, crossbows |
| Professional empire (Late Rome, Byzantium, Song, Abbasid) | 1-2% | 2-3% | 15-50k | paid regulars + allies |
| Feudal kingdom | 0.1-0.3% (household knights) | 0.5-1% | 5-15k | knights + retinues + militia |
| Steppe confederation | ~5% | 10-20% | 20-100k | all horse archers |
| Gunpowder empire (Ottoman, Mughal, Ming) | 0.5-1% | 1-3% (paper more) | 40-100k | provincial cavalry + gun infantry + artillery |
| Early modern European (1650-1750) | 1-3% | 3-4% | 30-100k | musket-bayonet infantry, cavalry, guns |

---

## 2. Logistics limits

### 2.1 Numbers

| Item | Value | Basis |
|---|---|---|
| Man, food | ~1 kg grain a day (0.85-1.36 kg); ~3,000 kcal; Polybius' Roman ration ≥0.83 kg | Devereaux citing Roth, Engels [O]; Roth ~3,000 kcal [O] |
| Man, water | ~2-3 litres a day | [O] |
| Horse | ~7 kg barley a day (Roman cavalry); 8 gallons (~30 L) water; ~10 lb feed in Engels | [O] |
| Mule | carries ~130 kg; eats 2.25 kg barley + grazing | [O] |
| Two-horse wagon | ~1,000 kg | [O] |
| Soldier's own pack | ~55 kg incl. ~12 kg food (~10 days) for a Roman | [O] |
| Alexander's army | ~65,000 people and 15,000 animals needed ~1,000 t a day; could carry ~4 days | Engels via Hunt [O] |
| Servants | ~1 per 4 soldiers (Roman, speculative) | Roth via BMCR [O] |
| March | ~15 mi (24 km)/day for Alexander, best 19 mi (30 km); Romans 15-20 mi in ~7 h; Frederick II ~15 mi; Napoleonic 10-12 mi ordinary | [O] |
| Speed in column | 4.8 km/h open, 4 km/h rough; a pass four men wide took Alexander's army 7.5 h to cross | Engels [O] |
| Aztec | 19-32 km/day with porters | [O] |
| Trireme | 80-100 km a day; 170 rowers, ~200 crew | [O] |

### 2.2 Why armies stayed small

1. **The wagon equation.** Pack animals eat their own load. Self-carried supply lasts ~3-10 days, and animal trains
   at best a few weeks before they consume what they carry [O, Devereaux].
2. **Foraging.** A moving army forages a band ~20 miles (30 km) wide with cavalry, half that without; at ~10 mi a
   day it sweeps ~200 sq mi a day. It can take only ~5-15% of local food and must keep moving [O]. Up to a third of
   a medieval army might be out foraging [O].
3. **Water.** Maurice's water count capped Xerxes at ~200,000 men with 70,000 animals [O].
4. **Columns.** A 50,000-man army on one road is ~20-30 km long; the tail leaves camp as the head arrives [M].
   Narrow passes limit throughput [O, Engels].
5. **Seasons.** Campaign season from about the spring equinox; food doubles after harvest; fodder halves by late
   autumn, so armies go into winter quarters [O, Devereaux].
6. **Money and depots.** Armies over ~50,000 appeared in Europe only after 1650 with magazine depots and supply
   lines [O]. Sea or river supply was the other exception (Han, Rome, Ottoman Danube) [M].

### 2.3 Rules of thumb for the sim

- Daily need **D = N_men × 1 kg + N_horses × 7 kg + N_mules × 2.5 kg** (halve fodder in growing season if grazing).
- Local supply per cell = stored food × 0.1 (what can be seized while passing). An army can **sit** in a cell for
  `S_local_within_radius / D` days, then must move or starve.
- Foraging radius: 15 km (infantry) or 30 km (cavalry-rich).
- Carried supply: 5 days (no train), 15 days (pack train), 30 days (wagon train, road), unlimited along a
  river/sea/depot line.
- Field army hard cap by logistics tech: **10k** (early state, no roads), **25k** (Bronze Age, donkeys),
  **50k** (Iron Age empire, roads, standing army), **20k** (feudal), **60k** (gunpowder empire),
  **100k** (depots, after 1650). Larger forces must split into several armies on different routes.
- Season: no campaigning in winter quarters (temperate) or in the rainy season (tropics, monsoon). Steppe armies
  campaign in autumn and winter when horses are fat and rivers frozen [M].

---

## 3. Equipment by era and what it needs

Dates are first appearance → common use. Near East unless noted.

### 3.1 Missile weapons

| Item | First | Common | Needs | Basis |
|---|---|---|---|---|
| Self bow | Palaeolithic | everywhere | wood; easy to make | chiefdoms-war.md |
| Sling | Neolithic | everywhere; Inca main weapon (sling, bola) | wool/fibre; years of practice | Inca [O]; rest [M] |
| Atlatl (spear-thrower) | Palaeolithic | Mesoamerica to 1520 (Aztec) | wood | [O] |
| Composite bow (horn, wood, sinew) | Bronze Age Anatolia/Mesopotamia | Egypt from ~1600 BCE (Hyksos); steppe Scythian type for riders | horn, sinew, glue; a week of work plus months drying; weak in humid climates | [O] |
| Crossbow | China, bronze locks ~650 BCE; Greek gastraphetes late 5th c. BCE | China Warring States-Han (mass produced); Europe again from ~947 CE, common by 12th c.; steel prods 14th c. | bronze/iron triggers, workshops; little training | [O] |
| Firearms | gunpowder formula 808; fire lance 1132; hand cannon 1287-88; dated gun 1298 (China); Europe 1320s | Janissaries 1400s; arquebus ~1450s-1500 [M]; Japan 1543; Mughal field guns 1526 | saltpetre, sulphur, charcoal, iron/bronze casting; little training per man | [O] |
| Flintlock and socket bayonet | flintlock ~1610 [M]; socket bayonet 1680s-90s | Europe by ~1700 ends the pike | iron, state arsenals | pike page [O] |

### 3.2 Melee weapons

| Item | When | Needs | Basis |
|---|---|---|---|
| Stone mace | 9500-8500 BCE (status object) | stone | chiefdoms-war.md |
| Copper/bronze axe, dagger, spear | Chalcolithic-EBA | copper, tin | [M] |
| Dagger-axe (ge) | Shang China, Fu Hao tomb (130 bronzes) | bronze | [O] |
| Sickle sword (khopesh) | depicted ~2500 BCE (Stele of the Vultures); common in New Kingdom; gone ~1300 BCE | bronze | [O] |
| Bronze sword (long) | ~1600 BCE Aegean/Europe [M] | bronze, skill | [M] |
| Iron weapons | Anatolia c. 1300-1200 BCE; common in Near East by 1000 BCE; Central Europe ~800 BCE; China known 9th c. BCE, mass use Warring States; Africa maybe 2000-1200 BCE | iron ore, charcoal, smiths | [O] |
| Macedonian sarissa (pike) | Philip II ~350 BCE | long ash shafts, drill | [O] |
| Medieval pike | Scots, Flemish 13th-14th c.; Swiss 15th c.; ends ~1700 | drill, cheap | [O] |
| Halberd | 14th-16th c. Europe | iron | [M] |
| Macuahuitl (obsidian-edged club) | Mesoamerica | obsidian, wood | [O] |
| Iron spear + hide shield (assegai) | Southern Africa | iron | [O] |

### 3.3 Armour and shields

| Item | When | Needs | Basis |
|---|---|---|---|
| None / hide / quilted cloth | always; Aztec quilted cotton (ichcahuipilli); Inca quilted cotton tunics | fibre | [O] |
| Linen (linothorax) | Greek/Egypt [M] | flax | [M] |
| Bronze plate (Dendra panoply) | ~1540 BCE (Mycenaean); a 2024 test showed it wearable through 11 h of simulated combat | much bronze, elite | [O] |
| Scale/lamellar | Neo-Assyrian 911-609 BCE (earliest clear); China by 5th c. BCE; steppe, Byzantium, Japan | bronze, iron, leather or bone plates, laced | [O] |
| Mail | Celtic/Etruscan, 4th-3rd c. BCE; Roman lorica hamata; used to 18th c. in places | iron wire; very labour-heavy, expensive | [O] |
| Coat of plates → full plate | late 13th c. → full harness ~1420; 15-25 kg; declines early 17th c. | steel, skilled armourers (Milan, S. Germany); fitted suits for the rich | [O] |
| Shields | wicker, hide, wood everywhere; bronze-faced hoplite aspis; Roman scutum [M] | wood, hide | [M] |

### 3.4 Animals and vehicles

| Item | When / where | Needs | Basis |
|---|---|---|---|
| Horse riding | first likely riders Yamnaya 3000-2500 BCE (5 skeletons, debated) | horses | [O] |
| Spoked-wheel chariot | Sintashta ~2000 BCE; Near East 17th-16th c. BCE; Egypt ~1650 BCE (Hyksos); Greece ~1600 BCE; China ~1250 BCE (late Shang); Britain to 84 CE | 2-4 horses, wheelwrights, bronze, a trained crew of 2-4 | [O] |
| Cavalry (mass) | Neo-Assyria (first large-scale), ~9th c. BCE; Qarqar 12,000 claimed; China from 4th c. BCE | horses, pasture, riders trained from youth | [O] |
| Stirrups | Indian toe loop late 2nd c. BCE; single Chinese mounting stirrup 302 CE; pair 322 CE; common in China by 477; Avars bring them to Europe late 6th-7th c. | iron; effect on shock combat debated | [O] |
| Camels | Qarqar 853 BCE (1,000); Thymbra ~547 BCE scares Lydian horses | arid grazing | [O] |
| War elephants | India from ~6th c. BCE (perhaps earlier); Maurya 9,000 (claim); Gaugamela 331 BCE; Hellenistic, Carthage; Mughal 5,000-7,000; obsolete vs guns after ~1500; Thailand to 1893 | wild forest herds, years of training; one elephant valued at 500 horses (Abul Fazl) | [O] |

### 3.5 Siege, forts, ships

| Item | When | Needs | Basis |
|---|---|---|---|
| City walls (mudbrick, rammed earth) | Near East and China from chiefdom stage | labour | chiefdoms-war.md |
| Rams, siege towers, mining | Assyria, 9th-7th c. BCE; multi-year sieges (Arpad 740-738) | engineers, timber | [O] |
| Non-torsion catapult | Syracuse 399 BCE (Motya 397) | wood, bronze | [O] |
| Torsion catapult | Greeks, mid-4th c. BCE (Athens 338-326) | sinew/hair springs | [O] |
| Traction trebuchet | China 4th-3rd c. BCE; Mediterranean 6th-7th c. CE; 40-250 crew | rope, timber, many men | [O] |
| Counterweight trebuchet | 12th c. (al-Tarsusi c. 1187); Mongols take it to China 13th c.; ~10 crew; gone by mid-15th c. | timber, ironwork, engineers | [O] |
| Cannon / bombard | Europe 1320s; common by 1350; Ottoman foundry 1,014 guns 1522-26 | bronze or iron casting, saltpetre | [O] |
| Trace italienne (bastion fort) | early 16th c. | huge cost; makes sieges long | [O] |
| Galleys | penteconter → trireme (late 8th c. BCE, combat by 525 BCE) → quinquereme | timber, many paid rowers | [O] |
| Gun-armed sailing ships | Europe 16th c. → [M] | timber, guns | [M] |

---

## 4. Battle outcomes and casualties

### 4.1 Data

| Battle | Winner losses | Loser losses | Basis |
|---|---|---|---|
| Greek hoplite battles (472-371 BCE, ~20 cases) | avg ~5% (range 1.7-16.7%) | avg ~14% (mostly 10-20%); ratio 1:2.9; Greeks rarely pursued far | Krentz [O] |
| Gaugamela 331 BCE | 1,100-1,500 of ~47,000 (~3%) | 40,000-90,000 (claim) | [O] |
| Hastings 1066 | ~2,000 Normans (~20%) | ~4,000 English (~50%) | [O] estimate |
| Agincourt 1415 | 112 recorded, up to 600 of ~9,000 (1-7%) | ~6,000 of 12-15k (~40-50%), 90-120 great lords | [O] |
| Mohács 1526 | small | 14,000-20,000 of 25-28k (>50%), king killed, 2,000 prisoners killed | [O] |
| Sekigahara 1600 | 4,000-10,000 | 8,000-35,000; ~23,000 West troops defected | [O] |
| Malplaquet 1709 | Allies "won" but lost 21-25k of ~100k (~22%) | French 11-14k of ~80k (~15%) | [O] |

Pattern:
- In close combat most killing happens **after one side breaks**. The loser's losses depend on pursuit: few with
  infantry-only winners (hoplites), very high when the winner has cavalry or the loser is trapped (Mohács, Cannae
  [M]).
- Gunpowder battles from ~1650 kill more evenly: firefights hurt both sides, even the winner (Malplaquet).
- Defections and disengagement decide many battles (Sekigahara) [O].
- Delbrück: the winner usually had more men [O]. Quality still matters a lot; Sabin's ancient-battle model rates
  veterans 9x and average troops 3x a levy [O, via a critical review].

### 4.2 Lanchester and its limits

- **Linear law:** each fighter engages one opponent at a time; losses equal on both sides; the bigger side wins by the
  difference. Fits melee [O].
- **Square law:** aimed fire; strength ∝ N². Fits fire combat [O]. Practitioners often use an exponent ~1.5 [O].
- **Limits:** Willard's test on battles 1618-1905 gave the wrong sign for the exponent and concluded Lanchester
  should not be used for large engagements [O, Dupuy Institute]. Battles end by morale collapse, not attrition to
  zero. Frontage, not total numbers, limits how many fight at once in melee [M].

**Design reading:** use Lanchester only for the **contact phase** strength ratio, then let morale decide the break,
and apply a separate pursuit step.

---

## 5. Model spec for Odyssey

### 5.1 Troop types by era

| Era (local tech) | Types available |
|---|---|
| Neolithic / Chalcolithic | Warband (club, bow, spear) — see chiefdoms-war.md |
| Early Bronze (state) | Levy spearmen, Archers (self bow), Retinue (bronze axe/spear, shield) |
| Middle-Late Bronze | + Chariots (needs horses + composite bow), Composite archers, Bronze-armoured elite |
| Early Iron | + Cavalry (riding), Heavy infantry (iron, bronze/scale armour), Camel troops (arid), Siege train (rams) |
| Classical | + Phalanx/pike or legion-type infantry (drill), Crossbowmen (China first), Catapults, War elephants (India-tied), Triremes |
| Medieval | + Mailed heavy cavalry with stirrups, Horse archers (steppe), Crossbowmen (everywhere), Trebuchets, Lamellar/mail elites |
| Late medieval | + Plate men-at-arms, Pike blocks, Longbow/crossbow masses, Bombards |
| Early modern | + Arquebusiers/musketeers, Field artillery, Pike-and-shot; after ~1700 Musket-bayonet line infantry |
| Americas (no horse/iron until contact) | Levy, Atlatl/sling troops, Elite orders (macuahuitl, quilted armour), Porters |

### 5.2 Levy formula

```
men       = pop × 0.24                                     # males 15-49
standing  = pop × s[type] × wealth_factor                  # paid troops, always available
levy      = Σ_counties men_c × m[type] × loyalty_c × season_c
field_cap = min(standing + levy, logistics_cap[tech])      # extra must form a second army
```

| Type | s (standing share of pop) | m (share of men called in a big war) |
|---|---|---|
| Early state | 0.003 | 0.10 |
| Bronze Age chariot state | 0.007 | 0.10 |
| Citizen city-state | 0.0 | 0.40 (citizens only) |
| Conscript bureaucratic | 0.01 | 0.15 |
| Professional empire | 0.015 | 0.05 |
| Feudal | 0.002 | 0.03 (vassals 40 days) |
| Steppe | 0.05 | 0.60 |
| Gunpowder empire | 0.007 | 0.05 |
| Early modern European | 0.02 | 0.05 (militia) |

Mods (stack with chiefdoms-war.md): harvest season ×0.5 for levies, famine ×0.7, a lost war in last 10 years ×0.8,
high taxes and loyal elites ×1.2. `wealth_factor` = treasury can pay: stop recruiting standing troops when pay >
~50-70% of state revenue [M].

**Check values.** Rome 225 BCE (4 M): m 0.40 × citizens + allies gives ~380k max; field cap 50k → several armies, as
Polybius shows [O]. Prussia 1740 (2.5 M): s 0.03 → 75k vs 80k actual [O]. Byzantium 565 (19.5 M): s 0.019 →
~370k vs 379k [O]. England 1415 (~2.5 M [M]): field ~9k [O] = 0.36%.

### 5.3 Equipment tiers (gated)

Each unit has a tier 0-4 for **weapon** and **armour**. Tier sets the quality multiplier (5.4).

| Tier | Weapon | Armour | Gate (all must hold) |
|---|---|---|---|
| 0 | stone, wood, self bow, sling | none / hide | none |
| 1 | copper/bronze spear, axe, dagger | quilted, leather, wicker shield | copper source in trade reach; bronze needs tin too |
| 2 | bronze sword, composite bow | bronze helmet, scale for elites | bronze working + workshops; composite bow needs horn, sinew, dry climate |
| 3 | iron weapons, crossbow (where invented or traded) | mail, lamellar | iron smelting + charcoal (forest); state arsenal for mass crossbows |
| 4 | steel, firearms | plate, munition armour | saltpetre source/trade, casting foundry, treasury |

Special units:
- **Chariot:** horses ≥ 2 per chariot + composite bow + wheelwright tech + bronze. Upkeep ~5x an infantryman [M].
- **Cavalry:** horse pasture (steppe or grassland counties) or horse imports (Mali needed imports) [O]. Steppe
  polities get horse archers at tier 2 with no metal requirement beyond arrowheads.
- **Elephants:** tropical forest with wild elephants within reach (South and Southeast Asia, North Africa until
  antiquity); 1 unit = 100 elephants; useless vs gunpowder artillery after 1500.
- **Camels:** arid counties; scares horses (enemy cavalry -25% vs camels) [O pattern, Thymbra].
- **Drilled infantry (phalanx, legion, pike, line):** needs standing army or yearly drill (citizen state or canton
  system); grants quality +1.
- **Siege train:** rams (Iron Age), catapults (classical), trebuchets (medieval), bombards (1320s+); each step
  divides siege time.

### 5.4 Battle resolution

1. **Strength per side.** `S = Σ_units n × q_tier × q_train × matchup × terrain`, where
   `q_tier = 1, 1.3, 1.7, 2.2, 3` for tiers 0-4 (weapon and armour averaged) [M, tuned];
   `q_train = levy 1, trained 1.7, veteran 3` (a softened Sabin 1:3:9, which is too steep for a strategy game) [O
   for Sabin].
   Matchups: cavalry ×1.5 on open ground, ×0.5 in forest/marsh/hills; pikes or bayonets ×1.5 vs cavalry;
   missiles ×1.3 when the enemy attacks uphill or across a river.
2. **Frontage cap.** Only the first `W` men fight in melee per round, `W` set by terrain (open: all; pass: 2,000).
   This is what stops huge levies winning by numbers alone.
3. **Win chance.** `P(A wins) = S_A^α / (S_A^α + S_D^α)`, α = 1 (melee era, linear law), 1.5 (missile-heavy or
   gunpowder after 1450). Leader skill adds ±10-25% to S; surprise ×1.5; walls ×2-4 (see sieges).
4. **Losses before the break.** Both sides lose `k × S_enemy / S_own` of men: k = 3% (pre-gun), 8% (1650+ line
   battles).
5. **Rout and pursuit.** The loser breaks. Extra loser losses = 5% + 30% × (winner cavalry share) × terrain_open
   + 20% if trapped (river behind, encircled). Cap total loser deaths at ~60%. Add 5-20% of the loser as captured.
   Hoplite check: infantry-only winner on mixed ground → ~5% vs ~15% [O].
6. **Morale.** Each unit has a break threshold of losses: levy 10%, trained 20%, veteran 30%. If more than half the
   army breaks, the side loses. Allied contingents may defect with chance tied to loyalty (Sekigahara) [O pattern].
7. **After.** Wounded return 50% within a season. Winner may pursue into the next county; loser retreats toward
   supply.
8. **Calibration.** Over a century, war deaths for states should come to ~0.1-0.3% of population per war year
   (lower than tribal rates in chiefdoms-war.md) [M].

### 5.5 Showing armies on the map

| Element | Look |
|---|---|
| Unit marker | banner in polity colour with the **headcount** (e.g. "12k"); era icon: spear (levy), bow, chariot wheel, horse head, elephant, pike, musket |
| Type mix | thin ring around the marker split by share: infantry (grey), missile (green), cavalry/chariot (brown), elephants/camels (gold), guns (black) |
| Quality | marker rim: none (levy), silver (trained), gold (veteran/elite) |
| Equipment tier | small pips 0-4 under the count |
| Supply | bar under marker: days of food left; turns red at < 3 days |
| Path | line along road/river; dashed off-road; forced march shown with double arrow |
| Size cap warning | marker flashes when above the logistics cap: attrition grows |
| Battle | crossed weapons for one turn; counts tick down; the loser marker turns pale and retreats |
| Sieges | ring around the city with a siege-engine icon and days left |
| Hover | breakdown: by type, by county of origin, standing vs levy, leader, morale |

Merge markers in one county; split when the force exceeds the logistics cap or the paths diverge. Hide numbers of
enemy armies outside scouting range (show "?k").

---

## Sources (opened this session)

- Krentz, P. 1985. Casualties in hoplite battles. GRBS 26:1. https://grbs.library.duke.edu/article/viewFile/5321/5325
- Engels, D. 1978, Alexander the Great and the Logistics of the Macedonian Army, via Hunt et al. textbook: https://digfir-published.macmillanusa.com/hunt5e/hunt5e_ch04_45.html ; via Big Board Gaming: https://bigboardgaming.com/a-true-meeting-engagement/
- US Army Heritage, Marching rates bibliography: https://www.armyheritage.org/wp-content/uploads/ref-bibs/subjects/Marching/Marching%20Rates.pdf
- Devereaux, B. 2022. Logistics, how did they do it, Parts I and II (citing Roth, Engels, Erdkamp, Lynn, Landers): https://acoup.blog/2022/07/15/collections-logistics-how-did-they-do-it-part-i-the-problem/ ; https://acoup.blog/2022/07/29/collections-logistics-how-did-they-do-it-part-ii-foraging/
- Roth, J. 1999, Logistics of the Roman Army at War, review: https://bmcr.brynmawr.edu/?p=21462
- Dupuy Institute, The Lanchester equations and historical warfare (Willard, Fain): https://dupuyinstitute.org/2018/05/02/the-lanchester-equations-and-historical-warfare
- Manning, S. The largest armies in world history: https://www.bookandsword.com/?p=3876 ; review of Sabin's Lost Battles: https://www.bookandsword.com/2013/08/31/some-thoughts-on-sabins-lost-battles/
- Adams, J. J. 2024. The rise and fall of armies. Macroeconomic Dynamics 29: https://www.cambridge.org/core/product/1F35E8911B298CC016A8AA15D55A6647/core-reader
- Polybius 2.24 (LacusCurtius): https://penelope.uchicago.edu/Thayer/E/Roman/Texts/Polybius/2*.html
- Rome's legions and census: https://www.historiascripta.org/classical-antiquity/romes-path-to-supremacy-legions-citizenship-and-conquest/
- Scheidel (ed.), Rome and China, review: https://bmcr.brynmawr.edu/?p=36641
- Black, J., China as a military power to 1700 (UK gov.): https://www.gov.uk/government/publications/china-as-a-military-power-to-1700/china-as-a-military-power-to-1700
- Armies, size of (encyclopedia entry on English medieval armies): https://slovaronline.com/browse/ab02e089-af40-3a26-b292-31188bdd01ce/armies-size-of
- New World Encyclopedia, Battle of Mohács: https://www.newworldencyclopedia.org/entry/Battle_of_Mohacs
- Bostan, I., Ottoman gun foundry: https://epa.oszk.hu/00800/00861/00025/2003-2-02.html
- Science News, Yamnaya riders (Trautmann et al. 2023): https://www.sciencenews.org/article/yamnaya-worlds-earliest-horseback-riders
- De Re Militari, review of May, The Mongol Art of War: https://www.deremilitari.org/REVIEWS/May_MongolArtWar.htm
- Wikipedia: https://en.wikipedia.org/wiki/Battle_of_Kadesh ; https://en.wikipedia.org/wiki/Military_of_the_Mongol_Empire ; https://en.wikipedia.org/wiki/Battle_of_Changping ; https://en.wikipedia.org/wiki/Second_Persian_invasion_of_Greece ; https://en.wikipedia.org/wiki/Military_revolution ; https://en.wikipedia.org/wiki/Aztec_warfare ; https://en.wikipedia.org/wiki/Song%E2%80%93Xia_wars ; https://en.wikipedia.org/wiki/Army_of_the_Mughal_Empire ; https://en.wikipedia.org/wiki/Battle_of_Qarqar ; https://en.wikipedia.org/wiki/Military_of_the_Han_dynasty ; https://en.wikipedia.org/wiki/Byzantine_army ; https://en.wikipedia.org/wiki/Military_of_the_Ming_dynasty ; https://en.wikipedia.org/wiki/Stirrup ; https://en.wikipedia.org/wiki/Chariot ; https://en.wikipedia.org/wiki/Crossbow ; https://en.wikipedia.org/wiki/Trebuchet ; https://en.wikipedia.org/wiki/Mail_(armour) ; https://en.wikipedia.org/wiki/War_elephant ; https://en.wikipedia.org/wiki/Plate_armour ; https://en.wikipedia.org/wiki/Gunpowder_warfare ; https://en.wikipedia.org/wiki/History_of_gunpowder ; https://en.wikipedia.org/wiki/Battle_of_Sekigahara ; https://en.wikipedia.org/wiki/Battle_of_Tondibi ; https://en.wikipedia.org/wiki/Inca_army ; https://en.wikipedia.org/wiki/Battle_of_Isandlwana ; https://en.wikipedia.org/wiki/Mali_Empire ; https://en.wikipedia.org/wiki/Sargon_of_Akkad ; https://en.wikipedia.org/wiki/Fu_Hao ; https://en.wikipedia.org/wiki/Weni_the_Elder ; https://en.wikipedia.org/wiki/Military_of_ancient_Egypt ; https://en.wikipedia.org/wiki/Maurya_Empire ; https://en.wikipedia.org/wiki/Battle_of_Hastings ; https://en.wikipedia.org/wiki/Battle_of_Malplaquet ; https://en.wikipedia.org/wiki/Battle_of_Agincourt ; https://en.wikipedia.org/wiki/Lanchester%27s_laws ; https://en.wikipedia.org/wiki/Abbasid_Caliphate ; https://en.wikipedia.org/wiki/Battle_of_Gaugamela ; https://en.wikipedia.org/wiki/Neo-Assyrian_Empire ; https://en.wikipedia.org/wiki/Prussian_Army ; https://en.wikipedia.org/wiki/Janissary ; https://en.wikipedia.org/wiki/Hans_Delbr%C3%BCck ; https://en.wikipedia.org/wiki/Musket_Wars ; https://en.wikipedia.org/wiki/Mongol_invasion_of_the_Khwarazmian_Empire ; https://en.wikipedia.org/wiki/Composite_bow ; https://en.wikipedia.org/wiki/Lamellar_armour ; https://en.wikipedia.org/wiki/Dendra_panoply ; https://en.wikipedia.org/wiki/Camel_cavalry ; https://en.wikipedia.org/wiki/Pike_(weapon) ; https://en.wikipedia.org/wiki/Iron_Age ; https://en.wikipedia.org/wiki/Catapult ; https://en.wikipedia.org/wiki/Trireme ; https://en.wikipedia.org/wiki/Khopesh

Not reachable or not useful: Wikisource/Perseus Thucydides 2.13 (blocked); Wikipedia pages Hittite military, East
Roman army, French Royal Army, Military history of the Mongol Empire, Gallic invasion of 225 BC (cache-only);
warhistory.org Ottoman page (403); UNT Arab conquests PDF (no numbers). Hassig, Kennedy, Treadgold's book, Lynn,
van Creveld and Agoston were not opened directly; where their numbers appear they come through the pages above or
are [M].
