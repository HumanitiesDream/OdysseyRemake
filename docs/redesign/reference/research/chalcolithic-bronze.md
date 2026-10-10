# Chalcolithic and Bronze Age: metals, cities, states, war (c. 5000-1000 BCE)

5 Oct 2026. Companion to `neolithic-plan.md` and `chiefdoms-war.md` (same folder). Those files already cover chiefdom
triggers, tiers 1-3 (simple chiefdom, complex chiefdom, proto-state), levies, counties and rulers. This file picks up
where they stop: metal, cities, writing, states, the Bronze Age economy and war, and a model spec.

Tags follow `chiefdoms-war.md`: **[O]** = opened this session; **[M]** = from memory of a work I am confident exists,
not re-checked this session. Everything below is **[M]** unless marked. Numbers are rounded defaults for a simulation,
not settled facts. Where scholars disagree, I give a range and a default.

---

## 0. Summary for the model (one screen)

| Item | Default | Basis |
|---|---|---|
| Native copper hammered | from ~8500 BCE (Çayönü); Great Lakes from ~5000 BCE or earlier | Yener 2000; Pompeani et al. 2021 |
| First firm smelting | ~5000 BCE Belovode (Serbia); ~5000-4500 BCE Iran and Anatolia | Radivojević et al. 2010 |
| Independent metal origins | SW Asia/Balkans; Andes (~1500 BCE gold, ~1000 BCE copper); maybe China; West Africa disputed | Roberts et al. 2009; Burger & Gordon 1998 |
| Spread of metallurgy | ~0.5-1.5 km/yr overland (Balkans 5000 BCE to Britain/Ireland 2500 BCE) | Roberts et al. 2009 |
| Arsenical copper | common 4000-2000 BCE; 1-5% As; harder than pure copper | Lechtman 1996; Thornton 2009 |
| Tin bronze onset | regular from ~3000 BCE (Near East), ~2200 BCE (Europe), ~1900 BCE (China), ~1100 BCE (SE Asia) | Pernicka 1998; Higham & Higham 2009 |
| Standard alloy | ~10% tin | Uluburun cargo 10 t copper : 1 t tin (Pulak 1998) |
| City threshold | ~5,000+ people, ~2+ ha dense core, 3+ settlement tiers, non-farmers 10-20% | Fletcher 1995; Smith 2009 |
| Uruk | ~100 ha (3500 BCE) to ~250 ha and 25,000-50,000 people (3000 BCE) | Nissen 1988; Algaze 2008 |
| State test | 3+ levels of administration above the village (4 settlement tiers) | Wright & Johnson 1975 |
| Writing | tokens ~8000 BCE; bullae ~3500 BCE; proto-cuneiform ~3300-3200 BCE; ~85% accounts | Schmandt-Besserat 1992; Nissen et al. 1993 |
| House-size Gini | Neolithic ~0.3; Old World Bronze Age ~0.5-0.6; New World stays ~0.3-0.4 | Kohler et al. 2017 |
| Polity lifespan | median ~150-250 yr for states (exponential-like) | Arbesman 2011; Taagepera 1978 |
| Big armies | Kadesh 1274 BCE: Egypt ~20,000; Hatti ~40,000 with ~2,500-3,500 chariots | Healy 1993 |
| Wheel and cart | ~3500-3300 BCE, spread across Europe-Mesopotamia within ~300 yr | Anthony 2007 |
| Spoked chariot | ~2000-1800 BCE, Sintashta (south Urals) | Anthony 2007 |

---

## 1. Copper

### 1.1 Native copper (cold-worked, then annealed)

- Native copper is pure metal found in nuggets and sheets. People hammered it like stone. Heating (annealing) at
  ~300-600 °C stops it cracking. No smelting is needed.
- Earliest finds: a copper pendant from Shanidar Cave (Iraq), ~9500 BCE (Solecki 1969). Beads and awls at Çayönü
  (SE Turkey), ~8500-7500 BCE, made from Ergani ore (Yener 2000). Small native copper items appear at Çatalhöyük
  and Aşıklı Höyük in the 8th-7th millennia BCE.
- North America: the "Old Copper Complex" of the Great Lakes worked native copper from Keweenaw and Isle Royale.
  Pompeani et al. (2021, *Quaternary International*) push the start back to ~7000-5000 BCE. Points, knives, awls.
  It never led to smelting. It faded after ~1000 BCE into ornament use (Hopewell trade).
- Game reading: native copper is a **resource bit** on a few cells (Great Lakes, Ergani, Urals, Andes, Zambia
  copperbelt surface, Iran Talmessi). Any settled people with stone skill can use it. It gives ornaments and prestige.
  It does not give better tools. It does not by itself lead to smelting.

### 1.2 Smelting: the knowledge chain

Smelting reduces ore (copper carbonates, oxides, later sulphides) with charcoal at ~800-1,100 °C. Copper melts at
1,083 °C.

The chain the evidence supports:
1. **Hot kilns.** Pottery fired in two-chamber updraft kilns reaches 900-1,050 °C. These are common in the Halaf
   and Vinča worlds by ~5500 BCE.
2. **Coloured minerals.** People already ground malachite and azurite for green and blue pigment and beads.
3. **Crucible smelting.** Ore and charcoal heated in a small crucible or pit with blowpipes. Tiny yields (grams to a
   few hundred grams per charge).
4. **Casting** into open moulds (~4500 BCE), then **two-piece moulds** (~3500 BCE), then **lost-wax** (~4000-3700 BCE,
   Nahal Mishmar hoard; Bar-Adon 1980).
5. **Furnaces with bellows** and **sulphide ore** (chalcopyrite, needs roasting first): routine from the 3rd
   millennium BCE.

Key sites:
- **Belovode** (Vinča culture, Serbia): smelting slag ~5000 BCE, the oldest secure date (Radivojević et al. 2010,
  *Journal of Archaeological Science*). Pločnik nearby gives a tin-bronze foil ~4650 BCE, made by smelting a natural
  copper-tin ore (Radivojević et al. 2013, *Antiquity*). That was a one-off, not a tradition.
- **Rudna Glava** (Serbia) and **Ai Bunar** (Bulgaria): copper mines ~5000-4500 BCE. Ai Bunar shafts reached ~20 m
  (Chernykh 1992, *Ancient Metallurgy in the USSR*).
- **Iran**: Tal-i Iblis crucible slag, ~4500 BCE; Tepe Yahya and Arisman are later centres (Thornton 2009, *Journal
  of World Prehistory*).
- **Anatolia**: Değirmentepe smelting ~5000-4500 BCE; Norşuntepe and Arslantepe later.
- **Southern Levant**: Ghassulian Chalcolithic, ~4500-3800 BCE. Faynan and Timna ores (Hauptmann 2007, *The
  Archaeometallurgy of Copper*).
- **Andes, independent**: hammered gold foil at Waywaka ~1500 BCE (Grossman 1972); copper foil at Mina Perdida,
  ~1400-1100 BCE (Burger & Gordon 1998, *Science*). Smelting by ~1000-500 BCE. Lechtman (1984) stresses the Andean
  focus on colour and status, not tools.
- **China**: a brass fragment at Jiangzhai ~4700 BCE is odd and likely accidental. The first real tradition is in the
  Gansu-Qinghai region (Majiayao bronze knife ~3000 BCE; Qijia ~2200-1600 BCE), probably fed by steppe contact
  (Linduff & Mei 2009).
- **West Africa**: Agadez (Niger) copper smelting is claimed ~2000 BCE but disputed; Akjoujt (Mauritania) ~800 BCE.
  Africa south of the Sahara largely went straight from stone to iron after ~800-500 BCE.
- **Mesoamerica**: metal only after ~600-800 CE (West Mexico), via the sea from South America (Hosler 1994).

**Game reading.** Smelting needs: pottery kiln skill ≥ high, a copper ore bit within ~50 km, and charcoal (forest or
woodland within reach). Independent invention is rare: ~4-6 inventions in all of history. Set the base chance so that,
with all conditions met, it takes ~1,000-3,000 years on average (p ≈ 0.0005 per people per year), then spread does
the rest.

### 1.3 Ore types and major ore regions

| Ore | Minerals | Notes |
|---|---|---|
| Native | Cu metal | rare surface deposits; cold-worked |
| Oxide/carbonate | malachite, azurite, cuprite | near the surface; easy to smelt; used first |
| Arsenical / fahlore | tennantite, tetrahedrite, enargite | gives arsenical copper "naturally" |
| Sulphide | chalcopyrite, bornite, chalcocite | most of the world's copper; roast then smelt; large-scale from ~3000-2000 BCE |

Major Bronze Age copper regions (put as resource bits on cells):
- **Balkans**: Ai Bunar, Rudna Glava, Majdanpek.
- **Anatolia**: Ergani Maden; Black Sea coast (Küre).
- **Caucasus**: Kvemo Bolnisi, Trans-Caucasian arsenical ores.
- **Cyprus** (Troodos): big after ~2000 BCE; the "Alashiya" of the texts; oxhide ingots of ~29 kg.
- **Arabah** (Faynan, Timna).
- **Oman** ("Magan"): major supplier to Mesopotamia ~2500-1800 BCE.
- **Iranian plateau**: Anarak/Talmessi, Veshnoveh.
- **Urals and steppe**: Kargaly (Orenburg), worked very heavily ~1900-1400 BCE (Chernykh 2004); Sintashta region.
- **Alps**: Mitterberg (Austria), with shafts over 100 m, ~1700-800 BCE.
- **Iberia**: Huelva/Rio Tinto, south-east Spain (Los Millares).
- **Ireland and Wales**: Ross Island ~2400-1900 BCE (O'Brien 2004); Great Orme ~1900-1400 BCE.
- **China**: Tonglüshan (Hubei), Zhongtiao Mountains (Shanxi), the middle Yangtze belt.
- **SE Asia**: Phu Lon, Khao Wong Prachan (Thailand); Laos Sepon.
- **Andes**: Atacama (Chile), Bolivian highlands, north Peru.
- **Africa**: Akjoujt; Central African Copperbelt (worked from ~400 CE onward).
- **North America**: Keweenaw (native only).

### 1.4 Arsenical copper

- 1-5% arsenic makes copper harder after cold-working and easier to cast. It was the main "bronze" of 4000-2000 BCE in
  the Near East, Caucasus, Iberia and later the Moche Andes (Lechtman 1996, *Journal of Field Archaeology*).
- Much of it came from mixed ores. Some was deliberate (adding arsenic minerals).
- Arsenic fumes harm smiths. A game can add a small health cost to smiths in arsenical eras. (The lame smith-god
  Hephaestus is often linked to this; that is a popular hypothesis, not proof.)

### 1.5 How fast metallurgy spread

- Roberts, Thornton & Pigott (2009, *Antiquity*) argue for one SW Asian-Balkan origin zone and then spread. Rough
  arrival dates: Carpathian basin ~4500 BCE, Italy and Alps ~3500 BCE, Iberia ~3200 BCE, Britain and Ireland
  ~2500 BCE (with Beaker groups), Scandinavia ~2000 BCE (local production ~1700 BCE), Egypt ~4000 BCE,
  Indus ~3500 BCE, steppe ~4500 BCE (via Balkan trade), Gansu ~3000 BCE, Central Plains China ~2000 BCE,
  SE Asia ~1500-1000 BCE.
- That is ~2,500 years over ~2,500-3,000 km to Atlantic Europe: **~1 km per year**, similar to farming spread.
  It jumped faster along rivers, coasts and steppe routes (~2-5 km/yr) and slower into dense forest.
- Early on, metal objects travelled long before the know-how did. In Europe, imported copper ornaments often precede
  local smelting by 500-1,000 years.

### 1.6 Who controlled metal

- **Chalcolithic Balkans**: metal mostly went into ornaments and show axes for a few graves. In Varna about 3,000
  gold items (~6 kg) sit in ~300 graves, but most gold is in a handful. Grave 43 alone holds ~1.5 kg (Higham et al.
  2007, *Antiquity*; Renfrew 1986). Metal was a prestige good.
- **Smiths were part-time** at first. Village or household production, with some itinerant smiths.
- **States took control of distribution**, not usually of mines. Mesopotamian temples and palaces imported copper
  and tin and handed out tools. Oman copper came through Dilmun (Bahrain) merchants (Potts 1990).
- **Steppe and Alpine mining** was done by local communities, sometimes on a large scale (Kargaly, Mitterberg), and
  traded outward.
- Game reading: a polity that holds an ore cell or a trade choke point gets prestige and treasury. A people with
  metal skill and no ore depends on trade links.

---

## 2. Tin bronze

### 2.1 Why tin mattered

- Bronze with ~8-12% tin is harder than copper, casts better and does not poison the smith.
- Tin ore (cassiterite) is rare. It sits mostly in granite zones and their streams (placer tin). Most copper regions
  have no tin. So bronze makes long-distance trade a necessity, and its loss breaks economies.
- Uluburun shipwreck (~1300 BCE, off Turkey): ~10 t copper and ~1 t tin in ingots, plus glass, ebony, ivory, amber
  (Pulak 1998, *International Journal of Nautical Archaeology*). That gives the 10:1 ratio and the scale of one ship.

### 2.2 Tin sources

| Region | Notes |
|---|---|
| Cornwall and Devon | stream tin; tin ingots from Bronze Age wrecks (Salcombe, Erme); traded to the continent from ~2000 BCE |
| Erzgebirge (Saxony-Bohemia) | likely a source for Únětice central Europe ~2200-1600 BCE |
| Brittany, NW Iberia (Galicia, north Portugal) | Atlantic tin; Iberian tin found in Levant ingots ~1000 BCE |
| Tuscany (Monte Valerio) | small; Etruscan and earlier |
| Anatolia: Kestel / Göltepe (Taurus) | worked ~3000-2000 BCE (Yener & Vandiver 1993, *American Journal of Archaeology*); low grade, disputed scale |
| Central Asia: Afghanistan, Tajikistan (Mušiston), Uzbekistan (Karnab) | Bronze Age mining dated ~2000-1500 BCE (Garner 2013 and Stöllner et al. on Karnab). Likely supplier to Mesopotamia via Iran |
| East Desert Egypt | minor |
| China: Yunnan, south China; Jiangxi | Shang tin and lead may partly come from the south-west (Jin Zhengyao's lead isotope work) |
| SE Asia tin belt: Malay peninsula, Thailand, Burma | world's richest tin belt; used locally from ~1100-1000 BCE |
| Bolivia (Andes) | tin bronze in the south-central Andes from Tiwanaku times (~600 CE), spread by the Inca (Lechtman 1980s) |
| Nigeria (Jos plateau), Zimbabwe | used late, in the Iron Age |

### 2.3 Dates

- Rare early tin bronze: Pločnik ~4650 BCE; a few 4th-millennium items.
- **Regular tin bronze from ~3000-2800 BCE** in Mesopotamia, Anatolia (Troy II hoards, Ur Royal Cemetery ~2600 BCE).
- Egypt mostly arsenical until ~2000 BCE; tin bronze common in the New Kingdom (from ~1550 BCE).
- Indus: mixed copper, arsenical and some tin bronze, ~2600-1900 BCE.
- Europe: tin bronze standard by ~2200-2000 BCE (Early Bronze Age, Únětice, Wessex).
- China: Erlitou (~1900-1500 BCE) casts ritual vessels in piece-moulds; Shang Zhengzhou and Anyang (~1600-1046 BCE)
  scale up hugely. The Simuwu (Houmuwu) ding weighs ~830 kg. Fu Hao's tomb (~1200 BCE) held ~1.6 t of bronze.
  Chinese bronze is often ternary: copper-tin-lead (Bagley 1999 in *Cambridge History of Ancient China*).
- SE Asia: Ban Chiang and Non Nok Tha bronze from ~1100-1000 BCE (Higham & Higham 2009, *Antiquity*).
- Andes: arsenical bronze on the north coast (Moche ~100-800 CE); tin bronze in the south and under the Inca.

### 2.4 Tin trade

- Old Assyrian trade (~1950-1750 BCE): donkey caravans carried tin and woollen textiles from Assur to Kanesh
  (Kültepe) in Anatolia, ~1,000 km, and came back with silver and gold. Archives of ~23,000 tablets. Larsen (2015,
  *Ancient Kanesh*) and Veenhof estimate tens of tonnes of tin over a few generations. Tin roughly doubled in silver
  value between Assur and Kanesh. Donkeys carried ~65-70 kg each.
- Mari (~1780 BCE) texts show tin from the east (Elam) being sold west to Ugarit and Crete.
- Game reading: a people that wants tin bronze needs copper and tin. With no local tin it needs a trade path to a tin
  cell. Price at the end of a route = source price × (1 + 0.1 per 100 km overland, 0.02 per 100 km by sea), with
  raids and tolls on top. When paths break, bronze output falls and the polity loses arms and prestige.

---

## 3. Societies

### 3.1 Chalcolithic societies

| Society | Where, when | What stands out |
|---|---|---|
| **Ubaid** | south Mesopotamia, ~6500-3800 BCE | irrigation farming, temples at Eridu rebuilt ~18 times; towns of 10-20 ha; low visible inequality; wide cultural spread to the Gulf and north Mesopotamia |
| **Varna / Karanovo VI / Gumelniţa** | lower Danube and Black Sea, ~4600-4200 BCE | richest gold of its day; tell villages; strong grave inequality; collapse ~4200 BCE |
| **Cucuteni-Trypillia** | Ukraine-Moldova, ~4800-3000 BCE | "megasites" ~3900-3600 BCE: Talianki ~320 ha, perhaps 10,000-15,000 people, with no clear palace or ruler (Müller et al. 2016, *Trypillia Mega-Sites and European Prehistory*); burnt and abandoned within ~150-200 years |
| **Ghassulian** | southern Levant, ~4500-3800 BCE | copper, arsenical lost-wax maces; shrines (En Gedi) |
| **Maikop** | north Caucasus, ~3700-3000 BCE | rich kurgans with silver, gold, arsenical bronze; link steppe to Near East |
| **Uruk** | Mesopotamia, ~4000-3100 BCE | first cities, writing, cylinder seals, mass-made ration bowls; "Uruk expansion" colonies up the Euphrates (Algaze 1993, *The Uruk World System*) |
| **Naqada I-III** | Egypt, ~4000-3100 BCE | chiefdoms merge into the unified state ~3100 BCE |
| **Harappan antecedents** (Early Harappan) | Indus, ~3300-2600 BCE | towns, then sudden Mature Harappan cities |

### 3.2 The rise of cities

Definitions vary. A workable one: a settlement of **≥ ~5,000 people**, with a dense built core, many non-farming
specialists, and central functions (temple, palace, market) serving a hinterland (Smith 2009, "V. Gordon Childe and
the urban revolution", *Town Planning Review*; Childe 1950).

Thresholds the evidence supports:
- **Size jumps.** Fletcher (1995, *The Limits of Settlement Growth*) argues settlements stall at ~1-2 ha, then ~100 ha,
  then ~100 km², each needing new tools of communication and order. Settlements above ~100 ha appear only with
  writing or strong recording systems (Trypillia is a near-exception, and it failed).
- **Density.** Early Mesopotamian city cores ran ~100-200 people/ha (Postgate 1992, *Early Mesopotamia*). Use 150/ha.
- **Surplus.** Non-farmers in early cities were ~10-20% of the polity's people; in the city itself up to ~30-50%.
  Each farmer must feed ~1.1-1.25 people. Mesopotamian irrigated barley gave ~15-25 times the seed in the late 3rd
  millennium (Postgate 1992), versus ~4-8 on rainfed land. So irrigated alluvium supports cities first.
- **Hinterland.** A city of 30,000 needs ~5,000-10,000 t of grain a year (~250 kg each), from ~150-500 km² of fields,
  plus herds and fish. Barges let it pull from further away. Algaze (2008, *Ancient Mesopotamia at the Dawn of
  Civilization*) stresses cheap river transport as the key Mesopotamian edge.
- **Synoecism.** In the Warka survey, the countryside emptied as Uruk grew: people moved in (Adams 1981, *Heartland of
  Cities*). A city often forms by pulling in nearby villages, not only by births.

City sizes over time (rounded; Chandler 1987; Modelski 2003, *World Cities –3000 to 2000*; Reba, Reitsma & Seto
2016, *Scientific Data*):

| Date | Largest cities | Population |
|---|---|---|
| ~4000 BCE | Tell Brak (~130 ha, dispersed; Ur et al. 2007, *Science*), Eridu, Uruk | ~5,000-15,000 |
| ~3500 BCE | Uruk ~100 ha | ~10,000-20,000 |
| ~3000 BCE | Uruk ~250 ha (Jemdet Nasr / ED I) | ~25,000-50,000 (default 40,000) |
| ~2800 BCE | Uruk walled area ~5.5 km² | up to ~50,000-80,000 |
| ~2500 BCE | Uruk, Lagash-Girsu, Ur, Mohenjo-daro (~250 ha) | 30,000-60,000 |
| ~2000 BCE | Ur, Memphis, Isin | ~50,000-65,000 |
| ~1700 BCE | Babylon, Avaris | ~50,000-60,000 |
| ~1300 BCE | Thebes (Egypt), Memphis, Anyang, Babylon, Hattusa | 50,000-100,000 (Anyang perhaps 50,000-150,000) |
| ~1100 BCE | Thebes, Babylon, Anyang | ~80,000-120,000 |

Rule of thumb: the largest city on Earth grew from ~10,000 (3500 BCE) to ~100,000 (1200 BCE). That is about
**+0.1% a year** for the top city over two millennia, with long stalls.

### 3.3 Temples

- In Mesopotamia the temple came first. The Eridu temple sequence runs from a small shrine (~5000 BCE) to a big
  platform temple by ~3500 BCE. The Uruk Eanna precinct covered ~9 ha by ~3200 BCE.
- Temples held land, herds and workers, stored grain and paid rations. In the Ur III state (~2100-2000 BCE) temple and
  crown estates employed tens of thousands on rations: ~60 litres (sila) of barley a month for a man, ~30-40 for a
  woman, less for children (Gelb 1965, *Journal of Near Eastern Studies*).
- Game reading: a "temple" building in a big village stores surplus, gives legitimacy to the ruler, and lets the
  polity pay specialists. It is the main path to a city in river valleys.

### 3.4 Writing and accounting

1. **Tokens**: small clay shapes for counting goods, from ~8000 BCE across the Near East (Schmandt-Besserat 1992,
   *Before Writing*).
2. **Seals and sealings**: stamp seals from the Neolithic; cylinder seals ~3500 BCE.
3. **Bullae** (clay envelopes with tokens inside and marks outside), ~3500 BCE.
4. **Numerical tablets**, ~3400 BCE (Uruk, Susa, Godin Tepe).
5. **Proto-cuneiform**, ~3300-3200 BCE (Uruk IV). About 5,000 archaic tablets. About 85% are administrative: grain,
   rations, herds, labour; most of the rest are lexical lists for training scribes (Nissen, Damerow & Englund 1993,
   *Archaic Bookkeeping*). Narrative and royal texts come only ~2600-2400 BCE.
6. Egyptian hieroglyphs ~3200 BCE (Abydos U-j labels). Indus script ~2600 BCE (undeciphered). Chinese oracle bones
   ~1250 BCE (Anyang). Mesoamerican writing ~600-400 BCE. The Andes used khipu (knotted cords) instead.
- Writing was invented independently 3-4 times (Mesopotamia, probably Egypt, China, Mesoamerica).
- Game reading: "recording" is a skill. It starts with tokens (any chiefdom with stores), rises with temple size and
  trade, and becomes writing when a polity runs ≥ ~10,000 people through one store system. Writing lets cities grow
  past ~100 ha and polities hold more tiers without breaking.

### 3.5 Early states

- **Wright & Johnson (1975, *American Anthropologist*)**: a state has specialised administration with **three or more
  levels of decision-making above the village**. On the ground, that shows as a four-tier settlement hierarchy
  (capital, towns, large villages, villages). Chiefdoms have one or two levels. Their Susiana case shows the change
  ~3500 BCE.
- **Carneiro (1970, *Science*)**: states form where farmland is hemmed in (circumscribed) by desert, mountain or sea,
  or by dense neighbours (social circumscription). Losers cannot flee, so they become subjects. Examples: Nile, coastal
  Peru valleys, Mesopotamia.
- **Spencer (2010, *PNAS*)**: primary states expand territory beyond a day's round trip from the capital at the same
  time as they develop internal offices. Delegation is the solution to distance. Monte Albán and Moche cases: conquests
  ~75-100 km out, a step change in a few generations. Chiefdoms are capped at about half a day's walk (25-30 km).
- **Flannery (1972)** and **Marcus (1998)**: the "dynamic model": early states expand to a peak soon after forming, then
  break into smaller units, then regroup. Cycles of ~200-500 years.
- **Turchin et al. (2018, *PNAS*, Seshat)**: across 30 regions, measures of polity size, hierarchy, government,
  infrastructure, writing and money rise together on one main axis. That supports a single "complexity" dial with
  sub-parts.
- **Time from first chiefdom to first state**: ~1,000-2,500 years in the primary centres (Mesopotamia ~5000 to ~3500
  BCE; Egypt ~4000 to 3100 BCE; Oaxaca ~1200 BCE to ~300 BCE; China ~3000 to ~1900 BCE).
- Primary states: Mesopotamia (~3500-3200 BCE), Egypt (~3100), Indus (~2600, form debated), China (Erlitou ~1900),
  Peru (Moche/Wari ~200-600 CE), Mesoamerica (Monte Albán ~300 BCE, Teotihuacan). Everyone else got states by
  contact.

### 3.6 State sizes

Polity size has a heavy tail: many small city-states, few large kingdoms. Taagepera (1978, *Social Science
Research*) gives areas (rounded):

| Date | Largest polity | Area (km²) | People |
|---|---|---|---|
| ~3000 BCE | Egypt (Early Dynastic) | ~100,000-150,000 settled Nile land and margins | ~0.9-1 million (Butzer 1976) |
| ~2500 BCE | Old Kingdom Egypt | ~400,000 (incl. desert claims) | ~1.6 million (Butzer 1976) |
| ~2300 BCE | Akkad (Sargon) | ~650,000 | ~1-2 million |
| ~2050 BCE | Ur III | ~250,000 | ~1 million |
| ~1450 BCE | New Kingdom Egypt | ~1 million | ~3 million |
| ~1300 BCE | Hittite empire; Shang | ~0.5 million each | ~1-3 million |

- A Sumerian city-state ~2500 BCE: one city of 10,000-50,000, plus towns and villages; ~1,000-3,000 km²; ~30,000-
  100,000 people. About 15-30 such states at once in Sumer.
- Distribution: take city-state counts roughly power-law, P(size > s) ∝ s^(-1) to s^(-1.5) (Zipf-like; Taagepera).
- Game: a "state" (tier 4) holds ~30,000-300,000 people; a "kingdom" (tier 5) 0.3-3 million; "empire" later.

### 3.7 Palace economies and specialists

- **Redistribution**: Bronze Age palaces (Mari ~1800 BCE, Ugarit, Knossos, Pylos ~1200 BCE) collected staples and
  wool, ran workshops and paid rations. Linear B tablets record ~ hundreds of women textile workers at Knossos and
  Pylos (Killen; Halstead 1992). Much of the economy stayed outside the palace in households and villages.
- **Specialists**: Uruk's "Standard Professions" list names ~120 titles (priests, officials, smiths, potters,
  weavers...). Default shares for a Bronze Age state: farmers and herders ~80-90%; crafts ~5-8%; administrators,
  priests and scribes ~1-3%; soldiers (full-time) ~1-2%; traders ~0.5-1%. Literacy below 1%.
- Wool in Ur III: tens of thousands of sheep on state books; textile workshops at Ur and Lagash with thousands of
  women and children, many war captives (Gelb 1973, "Prisoners of war in early Mesopotamia", *JNES*).

### 3.8 Inequality

- **Kohler et al. (2017, *Nature*)**: Gini of house sizes across 63 sites. Neolithic Old World ~0.3. Old World rises to
  ~0.5-0.6 by the Bronze Age and later. New World stays ~0.3-0.4. They link the gap to **large draft animals**:
  ploughs let a household farm more land, so land and animal wealth piles up (also Bogaard, Fochesato & Bowles 2019,
  *Antiquity*).
- Grave wealth: Varna and Bronze Age princely graves (Leubingen, Ur Royal Cemetery, Mycenae Shaft Graves) show top
  0.1-1% holding much of the visible wealth.
- Kohler & Smith (eds, 2018, *Ten Thousand Years of Inequality*) give site Ginis from ~0.2 to ~0.7.
- Game: wealth Gini drifts toward a target = 0.25 + 0.15·plough + 0.10·irrigation + 0.10·(polity tier ≥ 3) +
  0.05·(metal elite control) − 0.10·(land abundant), capped 0.2-0.7. Move ~1% of the gap per year. Collapses and
  plagues pull it down sharply (Scheidel 2017, *The Great Leveler*).

### 3.9 Slavery and dependency

- Slaves existed in all Bronze Age states, mostly from war captives and debt. In Mesopotamia chattel slaves were a
  small share, probably ~5-10% in cities and less in the countryside. Most labour was free farmers, tenants and
  temple/palace dependents on rations.
- Debt bondage was common. Kings issued debt-cancellation edicts (e.g. Ammi-saduqa, ~1640 BCE).
- Egypt: corvée labour (state service for weeks a year) mattered more than slavery for building.
- Game: war captives taken = 10-30% of the beaten side's non-combatants in a sack. They join the winner as "unfree"
  (a population class with no political weight, high work, lower fertility, rising to freedom at ~2-5% per year).

### 3.10 Kingship and dynasties

- Sumerian King List: kingship "descended from heaven" ~2900 BCE. Real reigns: Mesopotamian kings average ~15-20
  years; Egyptian ~15-20 years. Dynasties (in Egypt, the scholarly dynasties 1-20) average roughly 100-200 years.
- Sacred kingship: kings tie themselves to gods (Naram-Sin deified ~2250 BCE; Egyptian pharaoh as god).
- Royal tombs and sacrifice: Ur Royal Cemetery (~2600 BCE) with dozens of retainers killed (Woolley 1934); Egypt First
  Dynasty retainer burials; Shang royal tombs with hundreds of human sacrifices.
- Game: from tier 4 rulers gain "dynasty", "divine mandate" and "royal cult" traits. These raise legitimacy and lower
  breakaway chances at succession (see `chiefdoms-war.md` §4.1, p by 0.1).

### 3.11 Collapse

- **Polity lifespans**: Arbesman (2011, *Historical Methods*) finds empire lifespans roughly exponential, mean ~220
  years, with many short-lived ones. Taagepera (1978) finds empires reach their peak quickly and decline slowly.
- **Big collapse waves**: ~4200 BCE (Balkan Chalcolithic tells abandoned); ~3100 BCE (Uruk expansion ends);
  ~2200 BCE (the "4.2 ka" drought; Akkad falls, Old Kingdom Egypt breaks up; Weiss et al. 1993, *Science*);
  ~1900-1700 BCE (Indus cities decline); **~1200-1150 BCE Late Bronze Age collapse** (Cline 2014, *1177 B.C.*;
  Knapp & Manning 2016, *American Journal of Archaeology*; Drews 1993).
- 1200 BCE causes, in combination: drought (pollen and isotope records, Kaniewski et al. 2013), earthquakes, raiders
  ("Sea Peoples"), revolts, broken trade in tin and grain, and fragile palace systems. Hattusa, Ugarit, Mycenaean
  palaces, Cyprus towns fell within ~50 years. Writing was lost in Greece for ~400 years.
- Systems theory (Tainter 1988, *The Collapse of Complex Societies*): rising costs of complexity with falling returns
  make states brittle.
- Game: base collapse chance per state ~0.4-0.6% a year (giving median life ~150 years). Multiply by 2-5 in drought,
  by 1.5 when a key trade import (tin, grain) fails, by 1.5 on weak succession. Collapse drops the polity 1-2 tiers,
  shrinks the city by 30-70% over a generation, and can lose writing if no scribal centre survives nearby.

---

## 4. Economy

### 4.1 Plough and the secondary products revolution

- **Sherratt (1981, in *Pattern of the Past*; 1983, *World Archaeology*)**: from ~4000-3000 BCE animals were used for
  more than meat: **traction** (plough, cart), **milk**, **wool**, and **riding**. This changed farming, inequality and
  mobility.
- Later work moves milk earlier: milk lipids in pots from ~6500 BCE in Anatolia (Evershed et al. 2008, *Nature*).
  Traction and wool still fit the 4th millennium. Bone pathologies in cattle feet show draught work from ~4000 BCE.
- **Ard (scratch plough)**: plough marks under barrows ~3600-3500 BCE (e.g. South Street, England; Sarnowo, Poland).
  Mesopotamian plough signs in proto-cuneiform ~3200 BCE; seeder plough later.
- Effect: a man with an ox pair can farm ~3-5 ha versus ~1-2 ha by hoe. Sets up larger holdings, more land
  inequality, and male control of farm work.

### 4.2 Wool

- Wild and early sheep were hairy. Woolly sheep spread ~4000-3000 BCE. Textiles become the main Mesopotamian export
  (Assur to Kanesh). Wool sheep need cooler and drier pasture and big flocks: pushes pastoralism and herding
  specialists.
- Game: woolly sheep is a breed of the sheep herd (see the crop-breeds idea in `neolithic-plan.md`), appearing in
  SW Asia ~4000 BCE and spreading at herd speed. It gives textile goods, a trade good and more value per animal.

### 4.3 Wheel and cart

- Earliest wheeled-vehicle evidence: Bronocice pot picture (Poland) ~3500 BCE; Uruk pictograph ~3300 BCE; Ljubljana
  marsh wooden wheel ~3150 BCE; steppe wagon graves ~3000 BCE (Anthony 2007, *The Horse, the Wheel, and Language*).
- Spread across the whole zone within ~300 years: a "fast" technology.
- Solid wheels, ox-drawn, ~2-3 km/h. Carts carry ~500 kg versus ~70 kg for a donkey. They matter for farms and short
  haul. Long distance still went by donkey, boat or barge.
- Spoked wheels and the **chariot**: Sintashta, ~2000-1800 BCE. Chariot spreads to the Near East ~1800-1600 BCE,
  Egypt ~1600 BCE (Hyksos), China ~1200 BCE (Anyang chariot burials), India by the Vedic period.

### 4.4 Horses and steppe herders

- Botai (Kazakhstan, ~3500 BCE) horses were ridden or milked but are not the modern horse line (Gaunitz et al. 2018,
  *Science*).
- The modern domestic horse (DOM2) comes from the lower Volga-Don, ~2200 BCE, and replaced others across Eurasia by
  ~1500 BCE (Librado et al. 2021, *Nature*).
- Yamnaya (~3300-2600 BCE): wagon-using herders on the steppe; large migrations into Europe ~3000-2500 BCE (Haak et
  al. 2015, *Nature*). Whether they rode much is debated.
- Game: horse domestication is a herd event on the steppe cells; it unlocks "mounted herding" (larger ranges) and,
  with spoked wheels and bronze, the chariot. True mounted cavalry is Iron Age (after ~900 BCE).

### 4.5 Irrigation

- Small canals at Choga Mami (Iraq) ~6000 BCE. Large canal networks in Sumer by ~3000 BCE. Egypt relied on basin
  flooding of the Nile.
- Yields: irrigated barley in Sumer ~15-25 times seed (Postgate 1992), often ~1-2 t/ha in good years versus ~0.5-1 t/ha
  rainfed.
- **Salinisation**: Jacobsen & Adams (1958, *Science*) argue wheat fell from about half of grain in south Mesopotamia
  ~3500 BCE to almost none by ~1700 BCE as fields salted, and yields fell. Their numbers are debated but the trend
  is accepted.
- Game: irrigation is a land improvement. Cost ~ labour days per ha; doubles yield on arid floodplain; adds salinity
  0.2-0.5% per year on poorly drained land (yield × (1 − salinity)); fallow and drainage reverse it slowly. It feeds
  malaria and schistosomiasis per `diseases.md`.

### 4.6 Trade networks and goods

| Good | Source | Reach |
|---|---|---|
| Obsidian | Anatolia (Göllü Dağ, Nemrut), Melos, Lipari | 100s of km; fall-off with distance (Renfrew, Dixon & Cann 1968) |
| Lapis lazuli | Badakhshan (Sar-i Sang, Afghanistan) | to Egypt and Mesopotamia from ~3500 BCE (~3,000-4,000 km); Harappan outpost Shortugai |
| Carnelian | Gujarat (Indus) | to Ur ~2600 BCE |
| Tin | see §2.2 | 1,000-3,000 km |
| Copper | Oman, Cyprus, Anatolia, Arabah | Gulf, Mediterranean |
| Amber | Baltic coast | to Mycenae ~1600 BCE, ~2,000 km |
| Gold, silver | Egypt/Nubia (gold); Anatolia, Laurion (silver) | silver became the Near Eastern unit of value |
| Cedar | Lebanon | Egypt and Mesopotamia |
| Textiles | Mesopotamia | Anatolia |
| Salt | inland springs, coasts | regional |

- Volumes grew a lot: Old Assyrian trade carried tonnes per year; Late Bronze Age ships carried 10-20 t each.
- **Weights and money precursors**: Balance weights appear in Mesopotamia ~3000 BCE. Standard: shekel ~8.3 g; mina =
  60 shekels ~500 g; talent = 60 minas ~30 kg. Silver by weight was the measure of value. Laws of Eshnunna
  (~1800 BCE) price ~300 litres of barley at 1 shekel of silver. Hacksilver (cut silver) was paid by weight.
- In Europe, bronze rings, ribs and axe-ingots made to near-standard weights served as money (Ialongo & Lago 2021,
  *Journal of Archaeological Science*; Rahmstorf on Aegean weights).
- Game: a polity reaches "weights" when it trades with a state or holds ≥2 trade links. Weights lower trade costs ~10%
  and allow a treasury in metal, not just grain.

---

## 5. War

### 5.1 Army sizes

| Date | Event | Size (best guess) |
|---|---|---|
| ~2450 BCE | Lagash vs Umma (Stele of the Vultures) | phalanx of helmeted spearmen; hundreds to a few thousand |
| ~2300 BCE | Sargon of Akkad | claims "5,400 men ate daily before him": a standing core force |
| ~1800 BCE | Mari letters | kings field ~10,000-20,000; coalitions up to ~30,000-40,000 |
| ~1457 BCE | Megiddo (Thutmose III) | Egypt ~10,000-20,000; the Canaanite coalition captured 924 chariots and 2,041 horses (Annals of Thutmose III) |
| ~1274 BCE | Kadesh | Egypt ~20,000 in four divisions of ~5,000 plus chariots; Hittites ~2,500-3,700 chariots and ~37,000-47,000 men in all (Egyptian figures, likely inflated; Healy 1993) |
| ~1250 BCE | Tollense valley (Germany) | perhaps ~2,000-4,000 fighters; >140 individuals found (Jantzen et al. 2011, *Antiquity*) |
| ~1200 BCE | Ugarit | letters mention 150 ships and big chariot forces |

- Mobilisation share: Bronze Age states raised **~1-3% of their population** for a field army; city-states up to ~5%
  for short wars. Compare chiefdoms at ~10-15% of all people (30-50% of men) in `chiefdoms-war.md`. States raise a
  smaller share but far bigger absolute numbers, and can keep them out longer.
- Professional core: royal guards, chariot elite (Mitanni *maryannu*), mercenaries (Sherden in Egypt). Core ~5-20% of
  a field army.

### 5.2 Fortified towns

- City walls: Jericho early; Uruk ~9 km wall ~2900 BCE; Habuba Kabira ~3300 BCE. Most Early Bronze Levant towns
  walled. Mycenae and Tiryns Cyclopean walls ~1350-1250 BCE. Hattusa ~6 km of walls. Shang Zhengzhou rammed-earth wall
  ~7 km, perhaps 10-20 m wide at base.
- Siege works: battering rams and siege towers in Old Babylonian and Mari texts ~1800 BCE; sieges could last months.
- Game: a "walled town" building multiplies defence by 2-4 (defenders need ~1 per 2-3 m of wall to man it fully;
  starving it out takes 3-12 months).

### 5.3 Weapons and armour

| Date | Change |
|---|---|
| ~4000 BCE | copper flat axes, daggers; stone maces still common |
| ~3300 BCE | arsenical swords at Arslantepe (~45-60 cm) |
| ~2600-2400 BCE | socketed axes, spears, copper helmets (Ur); four-wheeled battle wagons drawn by onagers; phalanx |
| ~2300 BCE | composite bow shown on Naram-Sin stele |
| ~1900-1700 BCE | sickle sword (khopesh); long rapiers in the Aegean (~1700-1600 BCE) |
| ~1800-1600 BCE | light spoked chariot + composite bow spreads across the Near East |
| ~1500 BCE | scale armour (bronze scales on leather; Nuzi texts); Dendra bronze panoply ~1450 BCE |
| ~1300-1200 BCE | Naue II cut-and-thrust sword across Europe and the Aegean; greaves, round shields |
| ~1200 BCE onward | iron spreads after the collapse |

- The **sword** is the first purpose-built weapon only for killing people. It marks a warrior class.
- Chariots: two-horse, 2-3 crew (Egyptian 2, Hittite 3). A chariot needs ~2 horses, ~1 driver, 1 archer and support:
  expensive, so held by elites and kings. Chariot armies of 500-3,000 cars.

### 5.4 Professional warriors

- The European Bronze Age had a warrior ideal (graves with swords; Kristiansen & Larsson 2005, *The Rise of Bronze Age
  Society*). Sword finds show heavy use and repair.
- Near Eastern states paid soldiers with land (Hammurabi's *ilkum* service fields) or rations.
- Game: from tier 4 a polity keeps a standing force of ~0.5-1% of its people; it costs ~1.5 rations each per year in
  surplus. Standing troops fight at ×1.5 a levy man; chariots at ×4-6 on open ground, ×1 in rough or wooded land.

### 5.5 Early horse nomads and chariot peoples

- Sintashta (~2100-1800 BCE): fortified round settlements, metal production, chariot burials. Andronovo (~1800-1300
  BCE) spread across the steppe. These groups linked to Indo-Iranian languages (Anthony 2007).
- Chariot-borne elites appear in Mitanni (~1550 BCE), the Kassites in Babylonia, and probably the Aryans of the
  Rigveda. They were warrior aristocracies on top of farmer populations, not mass migrations of whole peoples.
- Game: steppe pastoral peoples with horse + bronze + spoked wheel become "chariot herders". Their war strength per
  man ×2-3 on steppe and open plain. When adjacent to farming states they raid, sell their service, or take over as a
  ruling caste (new ruler culture over old population).

### 5.6 Disease with density (pointer)

- Covered in `diseases.md` / `diseases-full.md`. Two Bronze Age items to wire in: plague (*Yersinia pestis*) in steppe
  and European genomes from ~3000-1000 BCE (Rasmussen et al. 2015, *Cell*; Spyrou et al. 2018, *Nature
  Communications*); and city mortality above rural (an urban penalty of ~+5-10 deaths per 1,000 a year), so cities
  need in-migration to hold size.

---

## 6. Suggested simulation design

### 6.1 Stages

Extend the existing ladder (simple chiefdom = 1, complex chiefdom = 2, proto-state = 3).

| Stage | Trigger (all must hold unless noted) | Unlocks |
|---|---|---|
| **Copper users** (people) | native copper or copper trade within reach; prestige demand (Gini ≥ 0.3) | copper ornaments; prestige; no tool bonus |
| **Smelters** (people) | kiln skill ≥ 0.7; copper ore bit ≤ 50 km; woodland; then invention (p ≈ 0.0005/yr) or contact with smelters (p ≈ 0.02-0.05/yr per link) | copper tools and weapons (tier "copper"); smith specialists |
| **Arsenical** | smelters + arsenical ore or 200 yr of practice | +10% weapon/tool quality |
| **Bronze** | smelters + access to tin (own cell or trade path) | weapon tier "bronze"; quality +25% over copper; tin demand |
| **Tier 4: early state** | proto-state + ≥ 30,000 people + 4 settlement tiers + recording ≥ 0.6 + surplus ≥ 15% of output for 50 years | city, temple, writing, officials, standing force, law and taxes |
| **Tier 5: kingdom** | early state that holds ≥ 3 cities or ≥ 300,000 people | provinces, governors, royal cult, chariot corps |
| **City** (settlement) | ≥ 5,000 people within one cell or 2 km; non-farmers ≥ 15% of the local people; polity tier ≥ 3 | crafts, market, trade node; urban disease penalty |

### 6.2 State variables

Per people (village / cluster):
- `metal_know`: enum {none, native, smelt, arsenical, tin_bronze, lost_wax}.
- `metal_skill`: 0-1, rises with practice (+0.002/yr while smelting), decays (−0.001/yr) when no ore or trade.
- `kiln_skill`: 0-1, from pottery.
- `recording`: 0-1 (tokens 0.2, seals 0.4, bullae 0.5, numerical tablets 0.6, writing 0.8, literate admin 1.0).
- `plough`, `wheel`, `woolly_sheep`, `horse`: bools from herd and tech events.
- `irrigated_frac`: 0-1 of fields; `salinity`: 0-1.
- `specialist_frac`: share not farming; `unfree_frac`: share captives/slaves.
- `metal_stock` (kg copper, kg tin), `metal_demand`.

Per polity:
- `tier` 1-5; `settlement_tiers` (count from the hierarchy); `capital_pop`.
- `surplus_frac` (stored surplus / yearly need), `treasury` (grain and silver).
- `trade_links`: list of (partner, good, cost, safety).
- `standing_force`, `chariots`, `walls` per settlement.
- `legitimacy`, `dynasty_age`, `collapse_risk`.

Per cell: `ore_bits` {native_cu, cu_oxide, cu_sulphide, arsenical, tin, gold, silver, lapis, obsidian, amber},
`charcoal` (woodland stock, falls with smelting), `floodplain`, `aridity`.

### 6.3 Yearly update rules

1. **Metal invention** (people with `kiln_skill ≥ 0.7`, ore within 50 km, no smelt): p = 0.0005 × (1 + pop/1000).
2. **Metal spread**: for each link to a smelting people, p = 0.03 × link_strength × (kiln_skill). Arrival of objects
   first: a non-smelting neighbour gets `native`-level ornaments at once.
3. **Production**: copper output (kg/yr) = smiths × 20-50 kg (village scale) or × 100+ (mine-scale with bellows and
   sulphide roasting after 3000 BCE). Each kg of copper uses ~10-30 kg of charcoal (~50-150 kg of wood); deduct from
   `charcoal`. Bronze = copper + tin at 9:1.
4. **Trade**: each year move goods along links; price grows with distance (§2.4). Links fail with p = 0.02 base, ×3 if
   a polity on the route is at war or collapsing.
5. **Weapons tier**: weapon quality = {stone 1.0, copper 1.15, arsenical 1.25, bronze 1.4}; drops one step if stock of
   metal per soldier < 0.5 kg.
6. **Plough**: if cattle and `metal ≥ smelt` or from ~4000 BCE spread, area farmed per worker ×2; Gini target +0.15.
7. **Irrigation**: on arid floodplain, polity invests labour; `irrigated_frac` += 0.01-0.03/yr while tier ≥ 2;
   yield × (1 + 1.0 × irrigated_frac) × (1 − salinity); salinity += 0.003 × irrigated_frac − 0.002 × fallow.
8. **Recording**: `recording` += 0.001 × (stored surplus in t / 1,000) × (1 + trade_links/5), capped by the next
   level's threshold; writing (0.8) needs ≥ 10,000 people fed through one store, else stalls at 0.6.
9. **City growth**: a city pulls in villagers within 15 km at 0.5-2% of their people a year while food holds; city
   deaths +0.5-1%/yr over rural. Cap city size at hinterland grain / 0.25 t per person; cap at 100 ha (15,000 people)
   without writing.
10. **State formation**: as in §6.1. Each year a proto-state meeting all conditions becomes tier 4 with p = 0.01
    (~70 yr median lag). Spencer's rule: on becoming tier 4 the polity may conquer out to ~3 days' walk (75-100 km).
11. **Succession and collapse**: base break chance 0.5%/yr per state; ×(2-5) drought; ×1.5 trade failure; ×1.5 weak
    heir; −30% with "royal cult". On collapse: tier −1 or −2, city −30-70% over 25 years, Gini −0.1, writing lost
    if no tier ≥ 4 neighbour within 300 km for 100 years.
12. **War**: army = levy (§`chiefdoms-war.md` 4.3) for tiers 1-3; for tier 4-5, field army = 1-3% of population +
    standing force. Strength = men × weapon quality × (chariot bonus on open ground) × wall factor for defenders.
13. **Captives**: sacked settlement loses 10-30% of non-combatants to the winner's `unfree_frac`; freedom rate 3%/yr.
14. **Steppe chariot peoples**: pastoral steppe people with `horse` + `wheel` + bronze → chariot status; raid
    neighbours with p = 0.1/yr; if a neighbour state collapses, take it over with p = 0.2 (new ruling culture).

### 6.4 Calibration checks

Run from ~6000 BCE on a real Earth map and check:
- Smelting in the Balkans/Anatolia/Iran by ~5000-4500 BCE; Britain by ~2500 BCE; China by ~2000 BCE.
- Tin bronze common in Mesopotamia by ~2800 BCE and Europe by ~2100 BCE.
- First city (≥ 10,000) in south Mesopotamia ~3800-3300 BCE; ~20 cities on Earth by 2500 BCE; ~50+ by 1200 BCE.
- First tier 4 state ~3500-3100 BCE (Mesopotamia, Egypt); 4-6 primary-state regions by 1500 BCE; Americas lag by
  ~2,500 years.
- Largest city ~40,000 at 3000 BCE, ~100,000 by 1200 BCE.
- A collapse wave when drought and trade failure hit together (4.2 ka; 1200 BCE).
- Old World Gini ~0.5 by 2000 BCE; New World ~0.35.

---

## Sources

All [M] unless marked. These are real works I am confident exist; page-level details were not re-checked.

- Adams, R. McC. 1981. *Heartland of Cities*. University of Chicago Press.
- Algaze, G. 1993. *The Uruk World System*. University of Chicago Press.
- Algaze, G. 2008. *Ancient Mesopotamia at the Dawn of Civilization*. University of Chicago Press.
- Anthony, D. W. 2007. *The Horse, the Wheel, and Language*. Princeton University Press.
- Arbesman, S. 2011. The life-spans of empires. *Historical Methods* 44(3).
- Bar-Adon, P. 1980. *The Cave of the Treasure*. Israel Exploration Society.
- Bogaard, A., Fochesato, M. & Bowles, S. 2019. The farming-inequality nexus. *Antiquity* 93.
- Burger, R. & Gordon, R. 1998. Early Central Andean metalworking from Mina Perdida, Peru. *Science* 282.
- Butzer, K. 1976. *Early Hydraulic Civilization in Egypt*. University of Chicago Press.
- Carneiro, R. 1970. A theory of the origin of the state. *Science* 169.
- Chandler, T. 1987. *Four Thousand Years of Urban Growth*. St David's University Press.
- Chernykh, E. N. 1992. *Ancient Metallurgy in the USSR*. Cambridge University Press.
- Cline, E. 2014. *1177 B.C.: The Year Civilization Collapsed*. Princeton University Press.
- Drews, R. 1993. *The End of the Bronze Age*. Princeton University Press.
- Evershed, R. et al. 2008. Earliest date for milk use in the Near East and southeastern Europe. *Nature* 455.
- Fletcher, R. 1995. *The Limits of Settlement Growth*. Cambridge University Press.
- Flannery, K. 1972. The cultural evolution of civilizations. *Annual Review of Ecology and Systematics* 3.
- Gaunitz, C. et al. 2018. Ancient genomes revisit the ancestry of domestic and Przewalski's horses. *Science* 360.
- Gelb, I. J. 1965. The ancient Mesopotamian ration system. *JNES* 24.
- Gelb, I. J. 1973. Prisoners of war in early Mesopotamia. *JNES* 32.
- Haak, W. et al. 2015. Massive migration from the steppe was a source for Indo-European languages in Europe. *Nature* 522.
- Hauptmann, A. 2007. *The Archaeometallurgy of Copper*. Springer.
- Healy, M. 1993. *Qadesh 1300 BC*. Osprey.
- Higham, C. & Higham, T. 2009. A new chronological framework for prehistoric Southeast Asia. *Antiquity* 83.
- Higham, T. et al. 2007. New perspectives on the Varna cemetery. *Antiquity* 81.
- Hosler, D. 1994. *The Sounds and Colors of Power*. MIT Press.
- Ialongo, N. & Lago, G. 2021. A small change revolution: weight systems and the emergence of the first Pan-European
  money. *Journal of Archaeological Science* 129.
- Jacobsen, T. & Adams, R. McC. 1958. Salt and silt in ancient Mesopotamian agriculture. *Science* 128.
- Jantzen, D. et al. 2011. A Bronze Age battlefield? Weapons and trauma in the Tollense Valley. *Antiquity* 85.
- Kaniewski, D. et al. 2013. Environmental roots of the Late Bronze Age crisis. *PLoS ONE* 8.
- Knapp, A. B. & Manning, S. 2016. Crisis in context: the end of the Late Bronze Age in the eastern Mediterranean.
  *American Journal of Archaeology* 120.
- Kohler, T. et al. 2017. Greater post-Neolithic wealth disparities in Eurasia than in North America and Mesoamerica.
  *Nature* 551.
- Kohler, T. & Smith, M. E. (eds) 2018. *Ten Thousand Years of Inequality*. University of Arizona Press.
- Kristiansen, K. & Larsson, T. 2005. *The Rise of Bronze Age Society*. Cambridge University Press.
- Larsen, M. T. 2015. *Ancient Kanesh*. Cambridge University Press.
- Lechtman, H. 1984. Andean value systems and the development of prehistoric metallurgy. *Technology and Culture* 25.
- Lechtman, H. 1996. Arsenic bronze: dirty copper or chosen alloy? *Journal of Field Archaeology* 23.
- Librado, P. et al. 2021. The origins and spread of domestic horses from the Western Eurasian steppes. *Nature* 598.
- Linduff, K. & Mei, J. 2009. Metallurgy in ancient eastern Asia. *Journal of World Prehistory* 22.
- Marcus, J. 1998. The peaks and valleys of ancient states. In Feinman & Marcus (eds), *Archaic States*. SAR Press.
- Modelski, G. 2003. *World Cities: –3000 to 2000*. FAROS 2000.
- Müller, J., Rassmann, K. & Videiko, M. (eds) 2016. *Trypillia Mega-Sites and European Prehistory*. Routledge.
- Nissen, H. 1988. *The Early History of the Ancient Near East*. University of Chicago Press.
- Nissen, H., Damerow, P. & Englund, R. 1993. *Archaic Bookkeeping*. University of Chicago Press.
- O'Brien, W. 2004. *Ross Island: Mining, Metal and Society in Early Ireland*. NUI Galway.
- Pompeani, D. et al. 2021. On the timing of the Old Copper Complex in North America. *Quaternary International*.
- Postgate, J. N. 1992. *Early Mesopotamia*. Routledge.
- Potts, D. T. 1990. *The Arabian Gulf in Antiquity*. Oxford University Press.
- Pulak, C. 1998. The Uluburun shipwreck: an overview. *International Journal of Nautical Archaeology* 27.
- Radivojević, M. et al. 2010. On the origins of extractive metallurgy: new evidence from Europe. *Journal of
  Archaeological Science* 37.
- Radivojević, M. et al. 2013. Tainted ores and the rise of tin bronzes in Eurasia, c. 6500 years ago. *Antiquity* 87.
- Rasmussen, S. et al. 2015. Early divergent strains of *Yersinia pestis* in Eurasia 5,000 years ago. *Cell* 163.
- Reba, M., Reitsma, F. & Seto, K. 2016. Spatializing 6,000 years of global urbanization. *Scientific Data* 3.
- Renfrew, C., Dixon, J. & Cann, J. 1968. Further analysis of Near Eastern obsidians. *Proceedings of the Prehistoric
  Society* 34.
- Roberts, B., Thornton, C. & Pigott, V. 2009. Development of metallurgy in Eurasia. *Antiquity* 83.
- Scheidel, W. 2017. *The Great Leveler*. Princeton University Press.
- Schmandt-Besserat, D. 1992. *Before Writing*. University of Texas Press.
- Sherratt, A. 1981. Plough and pastoralism. In Hodder, Isaac & Hammond (eds), *Pattern of the Past*. Cambridge UP.
- Smith, M. E. 2009. V. Gordon Childe and the urban revolution. *Town Planning Review* 80.
- Spencer, C. 2010. Territorial expansion and primary state formation. *PNAS* 107.
- Spyrou, M. et al. 2018. Analysis of 3800-year-old *Yersinia pestis* genomes. *Nature Communications* 9.
- Taagepera, R. 1978. Size and duration of empires: systematics of size. *Social Science Research* 7.
- Tainter, J. 1988. *The Collapse of Complex Societies*. Cambridge University Press.
- Thornton, C. 2009. The emergence of complex metallurgy on the Iranian Plateau. *Journal of World Prehistory* 22.
- Turchin, P. et al. 2018. Quantitative historical analysis uncovers a single dimension of complexity. *PNAS* 115.
- Weiss, H. et al. 1993. The genesis and collapse of third millennium north Mesopotamian civilization. *Science* 261.
- Wright, H. & Johnson, G. 1975. Population, exchange, and early state formation in southwestern Iran. *American
  Anthropologist* 77.
- Yener, K. A. 2000. *The Domestication of Metals*. Brill.
- Yener, K. A. & Vandiver, P. 1993. Tin processing at Göltepe. *American Journal of Archaeology* 97.
