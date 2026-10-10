# Resources on the map: stone, salt, metals, luxuries, fibres, animals, fuel

Research notes for putting every resource that Paleolithic-to-preindustrial societies need onto the map by the Neolithic,
so later eras can use them. 5 Oct 2026.

Tags: **[O]** = I opened the page or paper (or a reliable summary of it) this session. **[M]** = from memory, not checked
this session. Treat [M] coordinates as good to about +-1 degree and [M] dates as good to about +-20%.

Boxes are lat/lon in degrees (south and west negative): `lat1..lat2, lon1..lon2`. Points are `(lat, lon)`.
Dates: BCE/CE, or "ka" = thousand years ago.

---

## 0. Summary for the model

**A correction first.** The engine is not quite without geology. It already has:

- `lith[i]` per tile: the GLiM rock class (Hartmann & Moosdorf 2012), baked by tools/bake_soil.py. Classes:
  1 su unconsolidated sediment, 2 vb basic volcanic, 3 ss sandstone/siliciclastic, 4 pb basic plutonic (gabbro),
  5 sm mixed sedimentary, 6 sc carbonate (limestone, chalk), 7 va acid volcanic (rhyolite), 8 mt metamorphic,
  9 pa acid plutonic (granite), 10 vi intermediate volcanic (andesite), 11 wb water, 12 py pyroclastic,
  13 pi intermediate plutonic (diorite), 14 ev evaporite, 15 nd, 16 ig ice. (Seen in the source this session.)
- `VOLCANOES` (52 points) in lib.rs, elevation and relief, rivers, coasts, dryness, wetland and tree cover.
- 16 trade goods per 2-degree cell in tribes.rs `tr_goods_cells` (toolstone, obsidian, grindstone, ochre, shell, ivory,
  salt, fat, fur, bone, wood, fibre, amber, beads, ostrich, dry fish), already built from `lith`, coast and climate.

So most of what follows can be placed from `lith` plus terrain and climate. Only the rare things need fixed point lists.

**Three kinds of placement:**

| Kind | Resources | How |
|---|---|---|
| Everywhere, graded | clay, ochre, fresh water, timber, fibre plants, grindstone, ordinary chert | from `lith`, soil, rivers, climate. No list needed. |
| Common, rule-based | flint/chert (in chalk and limestone), salt (evaporite, dry coasts, salt springs), bog iron (wet lowland north of ~45 N), iron ore, copper (orogenic belts and volcanic arcs), gold (placers below granite and metamorphic uplands), lead-silver, peat, coal | a rule on `lith` + relief + climate, then thinned by a per-cell hash so only some cells hold a deposit. Real points listed below are used to check the rule. |
| Rare, fixed list | obsidian, tin, lapis lazuli, jade/jadeite, turquoise, mercury, amber, Spondylus, money cowrie, frankincense/myrrh, cloves/nutmeg, cinnamon, pepper, murex, cedar, carnelian, bitumen | a fixed point list (section 9). The rule would place them wrongly. |

**What each one unlocks (the chain the later eras need):**
toolstone -> everything; obsidian -> best blades, prestige trade; ground stone -> axes for clearing forest; salt -> herding
and preserved food; clay -> pottery, brick; native copper -> ornaments; copper ore + fire -> smelting (~5000 BCE);
copper + arsenic -> arsenical bronze (5th millennium BCE); copper + tin -> tin bronze (~3000 BCE, needs long trade);
lead + silver -> money metal; iron -> cheap tools everywhere (iron ore is almost everywhere); mercury -> silver
amalgamation (1550s CE); horses, donkeys, camels, llamas -> carrying and range; coal -> fuel when wood runs out.

**Trade rule of thumb:** Renfrew's obsidian fall-off. Within about 250-320 km of a source, ~80% of chipped stone is
obsidian ("supply zone"); beyond that the share falls off nearly exponentially with distance [O]. Prestige goods
(jade axes, Spondylus, lapis, tin, amber) went much further: 500-3,500 km (section 10).

---

## 1. Knappable stone

### 1.1 Flint and chert

- First used: from the earliest tools (Oldowan, ~2.6 Ma) [M]. Mined in shafts in the Neolithic: "flint has been mined
  since the Palaeolithic, but was most common during the Neolithic" [O].
- Enabled: all cutting tools; Neolithic polished flint axes for forest clearing.
- Geology: flint forms as nodules in chalk; chert in limestone and some other sediments [M]. In GLiM: class 6 (sc).
  The engine's `G_TOOLSTONE` already weights lt[6] most, then mixed sedimentary (5), sandstone (3), metamorphic (8).

| Site | Point | Dates | Notes |
|---|---|---|---|
| Spiennes, Belgium | (50.42, 4.00) [M] | 4300-2200 BCE [O] | ~100 ha of mines, shafts over 10 m deep [O] |
| Grime's Graves, England | (52.48, 0.67) [M] | c. 2600-2300 BCE [O] | at least 433 shafts, up to 14 m deep [O] |
| Krzemionki, Poland | (50.97, 21.50) [M] | c. 3900-1600 BCE, peak 2500-2000 BCE [O] | banded flint axes travelled ~300 km, then ~500 km, up to 660 km [O] |
| Cissbury, England | (50.86, -0.38) [M] | Neolithic [O] | |
| Hov, Denmark | (56.9, 8.7) [M] | Neolithic [O] | |
| Rijckholt, Netherlands | (50.80, 5.73) [M] | ~4000 BCE [M] | |
| Grand-Pressigny, France | (46.92, 0.80) [M] | ~2800-2400 BCE; long blades to the Netherlands [M] | |
| Flint Ridge, Ohio | (40.0, -82.3) [M] | Archaic to Hopewell [M] | |
| Knife River, North Dakota | (47.3, -102.0) [M] | Paleoindian onward [M] | |
| Edwards Plateau chert, Texas | 29.5..31.5, -101..-97.5 [M] | Clovis onward [M] | |

**Rule:** toolstone quality = lt[6] (chalk/limestone) high, sandstone/mixed sedimentary medium, quartzite (metamorphic)
low-medium, almost nil on unconsolidated sediment (1). Floodplains and deltas (big river, class 1) have poor stone: people
there import it. This is what drives the stone trade in the Neolithic.

### 1.2 Obsidian

- First used: Kariandusi (Kenya) and other Acheulean sites, ~700 ka, a few objects [O]. Olorgesailie (Kenya) ~295-320 ka:
  toolstone from at least 25-50 km away, plus pigment [O].
- Sea crossings for obsidian: Kozushima (Japan) obsidian carried to Honshu 37-34 ka, "the oldest round-trip voyage" [O];
  Melos obsidian at Franchthi Cave (Greece) from ~13,000 BCE [O] (some say ~11,000 BCE [M]).
- Enabled: the sharpest blades; a prestige stone; the clearest proof of exchange networks (each flow has a chemical
  fingerprint).
- Geology: acid volcanic glass. Young (mostly under ~20 Ma) rhyolite in volcanic arcs, rifts and hotspots [M]. GLiM 7
  (va), sometimes 12 (py). But only a few flows give good, big, clean nodules: that is why a list is better.

| Region | Source | Point | Use and reach |
|---|---|---|---|
| C Anatolia | Göllü Dağ, Nenezi Dağ, Acıgöl | (38.25, 34.55); (38.6, 34.5) [M] | "most important sources in central Anatolia"; to the Levant and Iraqi Kurdistan from ~12,500 BCE [O] |
| E Anatolia | Nemrut Dağ, Bingöl | (38.65, 42.23); (39.3, 41.0) [M] | to the Zagros and Mesopotamia [M] |
| Armenia | Gutansar, Arteni, Hatis | (40.3, 44.7); (40.4, 43.8) [M] | Armenian sources supplied the Fertile Crescent with Cappadocian ones [O] |
| Aegean | Melos (Sta Nychia, Demenegaki), Gyali | (36.73, 24.43); (36.66, 27.13) [M] | Greek mainland from ~13,000 BCE [O] |
| C Mediterranean | Lipari | (38.48, 14.95) [M] | late Neolithic, to Sicily, the southern Po valley and Croatia [O] |
| | Monte Arci (Sardinia) | (39.78, 8.73) [M] | one of four central Med sources [O]; to Corsica, S France [M] |
| | Palmarola | (40.94, 12.86) [M] | [O] named |
| | Pantelleria | (36.78, 11.99) [M] | Malta's obsidian came from here, ~240 km, not from Melos (~966 km) [O] |
| Carpathians | Tokaj (Hungary-Slovakia) | (48.4, 21.4) [M] | Neolithic central Europe [M] |
| E Africa | Rift Valley: Naivasha, Eburru, Sibilo; Ethiopia: Baltu, Fantale | 0..10, 36..41 [M] | from ~700 ka [O]; Kenya and Ethiopia supplied regional people [O] |
| Mesoamerica | Pachuca (green-gold, best), Otumba, Ucareo-Zinapécuaro, Zaragoza, Orizaba | (20.1, -98.7); (19.7, -98.75); (19.9, -100.7) [M] | Teotihuacan had over 100 obsidian workshops; Pachuca reached Maya land [O] |
| Guatemala | El Chayal, Ixtepeque, San Martín Jilotepeque, Tajumulco | (14.6, -90.4); (14.4, -89.7); (14.8, -90.8) [M] | "as far north as the Yucatán" [O] |
| N America | Obsidian Cliff (Yellowstone) | (44.82, -110.73) [M] | 12,000 years of use; Hopewell (200 BCE-400 CE) took it ~1,500 miles to Ohio [O] |
| | Glass Buttes OR, Coso CA, Jemez NM, Medicine Lake CA | (43.6, -120.1); (36.0, -117.8); (35.9, -106.6); (41.6, -121.6) [M] | |
| Andes | Quispisisa (Ayacucho), Alca, Chivay | (-14.3, -74.4); (-15.1, -72.8); (-15.6, -71.6) [M] | three main sources, 100-300 km apart; mean distance to use sites 171 km (Chivay) to 260 km (Quispisisa) [O] |
| Ecuador | Mullumica, Yanaurcu | (-0.2, -78.2) [M] | |
| Japan | Kozushima, Shirataki (Hokkaido), Wada (Nagano), Koshidake (Kyushu) | (34.2, 139.15); (43.9, 143.2); (36.2, 138.2) [M] | Kozushima 37-34 ka by sea [O] |
| Korea/Russia | Paektu-san (Changbai) | (42.0, 128.1) [M] | to Korea, Manchuria, Russian Far East [M] |
| Melanesia | Talasea/Kutau-Bao (New Britain), Lou (Admiralty Is.) | (-5.3, 150.0); (-2.4, 147.3) [M] | New Britain obsidian at Bukit Tengkorak, Borneo, over 3,500 km, ~3000 BP [O]; Lapita to Fiji [M] |
| New Zealand | Mayor Island (Tuhua) | (-37.28, 176.25) [M] | [O] NZ named; Mayor Is. is the main one [M] |
| Easter Island | Orito | (-27.15, -109.4) [M] | |

**Rule:** a fixed list of these ~40 sources. Each source has a quality (0.5-1). Add a weak generic chance on GLiM 7/12
cells near a `VOLCANOES` point, but only for local use.

---

## 2. Ground stone: axes, querns, jade

| Stone | First used | Where | Notes |
|---|---|---|---|
| Greenstone, dolerite, basalt (axes) | Neolithic polished axes ~9000 BCE onward in SW Asia [M]; Australia ground-edge axes much earlier (~45-50 ka) [M] | any basic volcanic (2), basic plutonic (4), metamorphic (8) upland | Mount William (Victoria, Australia, (-37.3, 144.8) [M]): 268 pits; axes traded "several hundred kilometres" [O]. Langdale (England, (54.45, -3.1) [M]) axes all over Britain [M] |
| Querns (sandstone, basalt) | grinding from ~30 ka; querns in Natufian and later [M] | sandstone (3), basalt (2) | the engine's `G_GRINDSTONE` already uses lt 2, 3, 4, 9, 13 |
| Alpine jadeitite | peak quarrying "the centuries around 5000 BC", to ~3000 BCE [O] | Mont Viso (44.67, 7.09) and Mont Beigua (44.43, 8.58) [M], quarries at 2000-2400 m [O] | to the Paris Basin 550 km, Morbihan (Brittany) 800 km [O]; 180+ axes in the British Isles [O] |
| Nephrite, Europe | 7th-5th millennium BCE (Poland Jordanów, Balkans, Switzerland) [O] | Jordanów (50.87, 16.87) [M] | |
| Nephrite, China | Hongshan (~4700-2900 BCE), Liangzhu (~3300-2300 BCE) [M] | Khotan/Hetian river boulders (37.1, 79.9) [M]: "pebbles and boulders in the rivers" [O]; Lake Baikal (Sayan) (52, 101) [M]; Xiuyan, Liaoning (40.3, 123.3) [M] | the "imperial gem" [O] |
| Nephrite, Taiwan | Maritime Jade Road, ~3,000 years, peak 2000 BCE-500 CE [O] | Fengtian, Hualien (23.8, 121.5) [M] | to the Philippines, Vietnam, Thailand, Borneo [M] |
| Jadeite, Japan | "the world's oldest known jadeite-using culture" [O], Jōmon ~5000 BCE [M] | Itoigawa (37.0, 137.85) [M] | traded across Japan and Korea [O] |
| Jadeite, Mesoamerica | Olmec onward (~1500 BCE) [M] | Motagua Valley, Guatemala (15.0, -89.6) [M] | Olmec, Maya, Costa Rica [O] |
| Jadeite, Myanmar | late; major only from ~18th c. CE [M] | Hpakant, Kachin (25.6, 96.3) [M] | ~70% of world high-quality jadeite today [O] |
| Pounamu (NZ nephrite) | Māori, from ~1300 CE [M] | Westland, South Island (-42.7, 171.2) [M] | taonga (treasure) [O] |
| British Columbia nephrite | Coast peoples [M] | Fraser River (50.5, -122) [M] | |

Geology: jadeite forms only at high pressure and low temperature in subduction zones, with serpentinite; known in
"fewer than 12 places worldwide" [O]. Nephrite forms where serpentinite meets dolomite or granite [M]. Both: fixed list.

---

## 3. Salt

- Use: salt licks and seawater since always; salt production becomes vital with farming and herding (cereal diets and
  livestock need added salt; meat and fish can be kept) [M]. Salt "enabled food preservation" and was used as money [O].
- First production: Poiana Slatinei (Lunca, Romania) brine boiling ~6000 BCE [O], (47.1, 26.2) [M]. Solnitsata
  (Provadia, Bulgaria), "oldest salt production centre in continental Europe (5500-4200 BC)", walled; it fed the Balkans,
  and the Varna gold is linked to its wealth [O]. Yorkshire salt pans 3766-3647 BCE [O]. China: Yuncheng salt lake
  (35.0, 111.0) [M]; Zigong brine wells (29.35, 104.77) [M], drilled before the Han, 1,001 m deep by the Shenhai well,
  gas used to boil the brine [O]. Wieliczka (49.98, 20.05) [M]: brine from the Neolithic, rock salt shafts from the
  13th century [O]. Hallstatt (47.56, 13.65) [M]: commercial in the Bronze Age, peak 8th-5th c. BCE [O].
- Sahara: Taghaza, Taoudenni (22.7, -4.0) [M], Bilma (18.7, 13.0) [M]; salt "traded at the market of Timbuktu almost
  weight for weight with gold"; caravans ~1,000 camels, up to 12,000 [O].

| Type | Where | Rule |
|---|---|---|
| Rock salt | evaporite beds (GLiM 14); salt domes; arid basins | lt[14] strong |
| Brine springs | where evaporite beds lie under wetter land (Carpathian foreland, Cheshire, Lorraine, Halle, Sichuan) [M] | lt[14] in the cell or a neighbour, and rain > ~500 mm. The salt comes up in springs. |
| Salt lakes, playas, sabkhas | inland drainage in deserts (Great Salt Lake, Dead Sea, Danakil, Uyuni, Lake Eyre) [O for the settings] | closed basin + dry climate. The engine has lakes and dryness. |
| Sea salt pans | flat coasts and lagoons where evaporation beats rain in summer [O]; Mediterranean "by classical antiquity", Guérande by the 9th c. [O] | coast + flat + warm dry season. Before ~1000 BCE, coastal salt was made by boiling (briquetage), which works on any coast with fuel [M]. |
| Plant ash salt | Philippines (sea-soaked plant matter burned) [O]; also inland Africa, New Guinea [M] | anywhere with poor salt access: low-grade substitute |

Evaporites form where "water input remains below the net rate of evaporation": closed basins in dry lands, rifts (Red
Sea, Dead Sea), restricted seas [O]. The engine's `G_SALT` already uses lt[14], coast and dryness. Add brine springs.

**Value:** high and rising with farming. A herding or grain people far from salt pays heavily. Salt routes are some of the
oldest roads [M].

---

## 4. Clay, ochre, other pigments

| Resource | First used | Where | Rule |
|---|---|---|---|
| Clay | fired figurines ~26 ka (Dolní Věstonice) [M]; pottery ~20 ka in China (Xianrendong) [M] | almost everywhere: alluvium, lake clay, weathered shale | the engine has soil clay %. Good potting clay = clay > 30% or parent "lake clay"/alluvium. Never a limit except on sand, ice and bare rock. |
| Red and yellow ochre | red ochre processing from ~300 ka [O]; Blombos engraved ochre ~75 ka [O]; Australia ~50 ka, Asia ~40 ka [O] | iron-rich weathered rock; widespread, best in the tropics and on basalt and iron formations | engine `G_OCHRE` already. Famous mines: Lion Cavern, Eswatini (~43 ka) [M]; Wilgie Mia, W Australia (-26.9, 117.6) [M] |
| Manganese black | Paleolithic cave art [M] | widespread | treat as ochre |
| Cinnabar red (vermilion) | "in use since the Neolithic" [O]; Çatalhöyük [M]; China Neolithic [M] | mercury sources (section 5.8) | fixed list |
| Kaolin (white clay, porcelain) | China porcelain ~600-900 CE [M] | Jingdezhen (29.3, 117.2) [M]; weathered granite (GLiM 9) in wet warm climate [M] | lt[9] + warm + wet |

---

## 5. Metals

### 5.0 Placement logic (geology in one table)

| Deposit type | Metals | Where it forms | Engine proxy |
|---|---|---|---|
| Porphyry copper | Cu, Mo, Au, Ag | subduction arcs; Pacific Ring of Fire (Andes, W North America, SE Asia), S central Europe, E Turkey, Iran; Chile has the biggest [O] | intermediate volcanic or plutonic (10, 13) or acid plutonic (9) + high relief + within ~800 km of an ocean trench (approx: near `VOLCANOES` or the Pacific rim) |
| VMS (massive sulphide) | Cu, Zn, Pb, Ag, Au | old seafloor: Cyprus and Oman ophiolites, Iberian Pyrite Belt, Kuroko (Japan), Noranda (Canada) [O] | basic volcanic or plutonic (2, 4) in mountains; fixed list for the famous ones |
| Sediment-hosted copper | Cu, Co | Central African Copperbelt (Zambia/Katanga) in Neoproterozoic shales [O]; Kupferschiefer (Germany-Poland) [M] | fixed boxes |
| Orogenic and greenstone gold | Au (Ag) | greenstone belts in old cratons: Abitibi, Yilgarn, Barberton, Kaapvaal, Slave, Wyoming, Scandinavia [O]; also young mountain belts [M] | metamorphic (8) or basic volcanic (2) in old shields; placers downstream |
| Placers | Au, Sn (cassiterite), Pt, gems | river bends, slope breaks, beaches below the source rock [O]; cassiterite: ~80% of mined tin is from placers [O] | river cells downstream of source cells |
| Granite tin | Sn, W | "almost always associated with granite" [O] | acid plutonic (9) is necessary but far from enough: use the fixed list |
| Lead-zinc-silver | Pb, Ag, Zn | galena veins; in limestone (MVT) and in mountain belts [M] | carbonate (6) + mountains, plus fixed list |
| Bog iron | Fe | bogs and swamps where iron-rich groundwater comes up as springs; regrows, "the same bog can be harvested about once each generation" [O] | wetland + lowland + cool climate (Scandinavia, Baltic, Russia, Canada, E US) |
| Laterite and ironstone | Fe | tropical weathering crusts (Africa, India) [M] | warm + wet-dry + old surface |

### 5.1 Copper

- Native copper hammered from ~8700 BCE (pendant, northern Iraq) [O]. Hot working in the Balkans and Carpathians
  6200-5000 BCE [O]. Smelting: Belovode (Serbia) "the world's oldest reliably dated evidence of copper smelting at high
  temperature, from 5,000 BCE" [O]. Sequence: cold working -> annealing -> smelting -> casting [O].
- Americas: Old Copper Complex, Lake Superior: ~6500-1580 BCE (maybe earlier); native copper heated, annealed and
  hammered, never smelted; quarries on Isle Royale, Keweenaw, Brule River [O]. Andes smelted copper by ~1000 BCE [M].
- Africa: copper "more precious and prestigious than either gold or silver" in Sub-Saharan Africa from ~2000 BCE to the
  1800s [O].
- Enabled: ornaments, then tools and weapons, then the whole bronze economy.

| Region | Source | Point or box | Dates |
|---|---|---|---|
| Balkans | Rudnik / Belovode (Serbia) | (44.1, 20.5) [M] | 5000 BCE smelting [O] |
| | Rudna Glava (Serbia), Ai Bunar (Bulgaria) | (44.3, 22.1); (42.5, 25.6) [M] | ~5000-4500 BCE mines [M] |
| | Sredna Gora (Bulgaria) | (42.5, 25.5) [M] | the Varna copper came from a Sredna Gora mine near Stara Zagora [O] |
| Levant | Timna, Faynan (Wadi Arabah) | (29.78, 34.98); (30.63, 35.49) [M] | Timna from the 5th-6th millennium BCE; big Iron Age smelting 11th-9th c. BCE [O] |
| Cyprus | Troodos ophiolite | 34.8..35.2, 32.7..33.4 [M] | Late Bronze Age major source [M]; VMS [O] |
| Oman | Magan (Semail ophiolite) | 22.5..25, 55.5..58.5 [M] | copper (and diorite) to Mesopotamia from ~2300 BCE in texts; Umm an-Nar 2600-2000 BCE [O] |
| Anatolia | Ergani (Diyarbakır), Black Sea Pontides | (38.27, 39.75) [M] | Çayönü native copper ~8000 BCE [M] |
| Iran | Anarak/Talmessi, Veshnaveh, Deh Hosein | (33.3, 53.7) [M] | 5th millennium smelting, arsenical [O for arsenical start on the plateau] |
| Caucasus | Lesser Caucasus | 39.5..41.5, 43..46.5 [M] | |
| Alps | Mitterberg (Austria) | (47.4, 13.1) [M] | Bronze Age major [M] |
| Iberia | Rio Tinto / Iberian Pyrite Belt | 37.2..38.0, -8.5..-6.0 [M] | from ~3000 BCE: copper, silver, gold [O] |
| Ireland/Britain | Ross Island (Kerry), Great Orme (Wales) | (52.05, -9.53); (53.33, -3.85) [M] | ~2400 BCE; ~1800 BCE [M] |
| Urals | Kargaly (Orenburg) | (51.9, 55.2) [M] | Bronze Age, huge [M] |
| C Asia | Mushiston (Tajikistan, polymetallic Cu-Sn) | (39.3, 68.0) [M] | "natural" bronze to Andronovo and BMAC [O] |
| China | Tonglüshan (Hubei), Zhongtiao Shan (Shanxi) | (30.1, 115.0); (35.2, 111.5) [M] | Shang-Zhou [M] |
| N America | Keweenaw, Isle Royale | 46.5..48.2, -89.5..-88 [M] | 6500 BCE+ [O] |
| Andes | Chilean and Peruvian porphyry belt | -32..-5, -76..-68 [M] | Chile has "the greatest concentration" of large porphyries [O] |
| C Africa | Copperbelt (Zambia, Katanga) | -13.5..-10.5, 25..29 [M] | "worked this ore for ages" (1895 report) [O]; from ~4th c. CE [M] |
| W Africa | Takedda / Azelik (Niger), Akjoujt (Mauritania) | (17.4, 6.2); (19.75, -14.4) [M] | Akjoujt ~800 BCE [M] |

### 5.2 Arsenic (arsenical bronze)

- Arsenical bronze is the first alloy: Iranian plateau 5th millennium BCE; Mesopotamia 4th-2nd millennium; Gansu-Qinghai;
  N Italy 2800-2200 BCE; Peru and Ecuador (Sicán 900-1350 CE) [O].
- Made from copper ores that carry arsenic (tennantite, enargite, arsenopyrite, fahlore) [O]. So in the sim: a fraction
  of copper deposits (polymetallic, in arcs and in Iran/Caucasus/Andes) give arsenical copper with no extra resource.
- Replaced by tin bronze because tin bronze casts as strong without work-hardening and is more predictable [O].

### 5.3 Tin (rare: fixed list)

- Tin bronze from about 3000 BCE (~12.5% tin) [O]. Tin is the great bottleneck: tin sources are few and far from the
  copper and the cities.

| Source | Point or box | Dates of use | Notes |
|---|---|---|---|
| Kestel / Göltepe (Taurus, Turkey) | (37.55, 34.55) [M] | 3250-1800 BCE [O] | gave two thirds of the Uluburun wreck's tin (~1300 BCE) [O] |
| Mushiston (Zeravshan, Tajikistan) | (39.3, 68.0) [M] | from ~2000 BCE [O]; Late Bronze Age | one third of Uluburun's tin [O]; the news report says "Uzbekistan" [O], the paper puts it in Tajikistan [O] |
| Karnab (Uzbekistan) | (39.7, 65.6) [M] | ~2000 BCE [M] | |
| Afghanistan, Hindu Kush (Herat, Badakhshan) | 34..37, 62..71 [M] | probable Late Bronze Age source [O] | along the old lapis route [O] |
| Iran (Deh Hosein) | (33.9, 49.3) [M] | 2nd millennium BCE [M] | the article names it for copper [O] |
| Erzgebirge / Ore Mountains (Saxony-Bohemia) | 50.3..50.8, 12.3..14.0 [M] | possibly from ~2500 BCE; know-how spread from there to Brittany and Cornwall ~2000 BCE [O] | |
| Cornwall and Devon | 50.0..50.7, -5.8..-3.8 [M] | Bronze Age; tin from here in E Mediterranean ingots 13th-12th c. BCE [O] | dominant in Europe from the 3rd c. CE [O] |
| Brittany | 47.5..48.5, -4.5..-2.0 [M] | big after 50s BCE [O] | |
| NW Iberia (Galicia, N Portugal) | 41..43.5, -9..-6 [M] | Bronze Age, Roman [O] | |
| Tuscany (Monte Valerio, Etruria) | (43.0, 10.6) [M] | ~800 BCE, minor [O] | |
| Yunnan (Gejiu) | (23.4, 103.2) [M] | from ~700 BCE; main Chinese tin by the Han [O] | |
| Yellow River small deposits | 33..37, 110..115 [M] | Erlitou-Shang [O] | |
| SE Asian tin belt (Malay Peninsula, S Thailand, Bangka-Belitung) | 1..14, 98..107 [M] | "richest deposits" in the world [O]; Ban Chiang bronze ~1000 BCE [M] | |
| Jos Plateau, Nigeria | 9..10.5, 8.5..9.5 [M] | precolonial tin smelting [M] | Africa: "ancient exploitation debated" [O] |
| Zimbabwe/S Africa (Rooiberg) | (-24.8, 27.7) [M] | tin mined and traded 11th-15th c. CE [O] | |
| Bolivia (Potosí-Oruro belt) | -21..-16, -68..-65 [M] | tin bronze from ~1000 CE [O] | |
| Zacatecas, Mexico | (22.8, -102.6) [M] | "only known exploitable source" in ancient North America [O] | |

### 5.4 Gold

- Oldest worked gold hoard: Varna necropolis (Bulgaria) 4569-4340 BCE, ~3,000 objects, ~6 kg; more than all other gold of
  that millennium together [O]. Sakdrisi (Georgia) may be an early mine [O].
- Mostly panned from rivers (placers) first, then hard rock [O]. Egypt-Nubia: both [O]. Romans: hydraulic mining at
  Las Médulas, Spain [O]. India: Kolar from at least the 2nd-3rd c. CE [O]. Inca: rivers and surface deposits [O].
- Enabled: prestige and display, later money.

| Region | Point or box | Notes |
|---|---|---|
| Balkans (Thrace) | 41..43.5, 22..27 [M] | Varna [O] |
| Caucasus (Sakdrisi, Colchis) | (41.4, 44.4) [M] | [O] named |
| Egypt Eastern Desert and Nubia | 18..26, 32..36 [M] | [O] |
| Iberia (Las Médulas, NW) | (42.46, -6.77) [M] | [O] |
| West Africa: Bambuk, Bure | (13.5, -11.5); (11.8, -8.8) [M] | drove trans-Saharan trade [O] |
| Akan / Lobi (Ghana, Côte d'Ivoire, Burkina) | 5..11, -4..-0.5 [M] | Birimian greenstone [M] |
| Zimbabwe plateau | -21..-17, 28..33 [M] | Great Zimbabwe gold trade via Sofala, ~1000-1500 CE [M] |
| Altai, Siberia | 49..53, 83..90 [M] | Scythian gold [M] |
| Kolar, India | (12.95, 78.27) [O for site, M for coords] | |
| Colombia (Muisca, Quimbaya), Ecuador, Peru | -15..8, -79..-70 [M] | [O] for Inca |
| Barberton, Abitibi, Yilgarn | (-25.8, 31.0); (48.5, -79); (-30.8, 121.5) [M] | greenstone gold; preindustrial use limited [O for belts] |
| Japan (Sado), Korea | (38.0, 138.3) [M] | |

**Rule:** gold chance on metamorphic (8) and basic volcanic (2) in old shields and in young mountains; its placers in the
river cells downstream (gold is heavy: placers concentrate where slopes break) [O].

### 5.5 Silver and lead

- Lead worked from ~7000 BCE in the Middle East [O] (Çatalhöyük beads [M]). Silver separated from lead (cupellation)
  in Asia Minor and the Aegean in the 4th millennium BCE; silver mining "dates back to 3,000 BC in Anatolia" [O].
- Silver comes mostly from lead ore (galena), so the two are one resource [O].

| Source | Point | Dates |
|---|---|---|
| Laurion (Attica) | (37.72, 24.05) [M] | from ~3200 BCE; systematic 6th c. BCE; peak 5th c. BCE (Athens' fleet) [O] |
| Taurus (Bolkardağ, Turkey) | (37.4, 34.6) [M] | 3rd millennium BCE [M] |
| Siphnos (Aegean) | (36.98, 24.7) [M] | Early Bronze Age [M] |
| Iberian Pyrite Belt, Cartagena, Sierra Morena | (37.6, -6.6); (37.6, -0.9) [M] | Tartessos, Phoenicians, Rome [O] |
| Sardinia (Iglesiente) | (39.3, 8.5) [M] | |
| Mendip, Derbyshire (Britain) | (51.27, -2.7); (53.2, -1.7) [M] | Roman lead [M] |
| Erzgebirge (Freiberg), Harz, Kutná Hora | (50.9, 13.3); (51.8, 10.4); (49.95, 15.27) [M] | medieval; the 15th c. liquation boom [O] |
| Potosí (Cerro Rico) | (-19.62, -65.75) [O] | found 1545; "80% of the world's silver" 16th-18th c. [O]; 4,782 m [O] |
| Zacatecas, Guanajuato (Mexico) | (22.8, -102.6); (21.0, -101.3) [M] | from 1546 [O] |
| Iwami Ginzan (Japan) | (35.1, 132.4) [M] | 16th c. [M] |
| Panjshir (Afghanistan), Anatolia, Iran | (35.3, 69.5) [M] | medieval Islamic silver [M] |

Bolivia, Peru and Mexico made ~85% of world silver 1500-1800 [O].

### 5.6 Iron

- Meteoric iron used "for millennia" before smelting [O]. Steel probably in use in Central Anatolia in the 3rd millennium
  BCE [O]. Wide smelting: India 1800-1200 BCE implements [O]; Levant bloomery ~930 BCE (Tell Hammeh) [O]; Europe from the
  late 11th c. BCE, probably from the Caucasus, spreading north over ~500 years [O]; China by the 9th c. BCE [O].
  Sub-Saharan Africa: Lejja (Nigeria) claims 2631-2458 BCE (disputed) [O]; Nok from ~1000 BCE [O]; Termit (Niger) ~1500-800
  BCE [M].
- Bog iron: Scandinavia from ~800-500 BCE; dominant in Norse lands 500-1300 CE; L'Anse aux Meadows ~1021 CE; colonial
  New England and New Jersey [O]. It regrows in a generation [O].
- Enabled: cheap tools and weapons. Iron ore is everywhere, so iron frees societies from the long trade routes bronze
  needs [M].

**Rule:** almost every cell has some iron. Grade it: banded iron and laterite (old shields, tropics), bog iron (wet
lowlands, cool, GLiM 1 and peat), ironstone in sediments (3, 5). Iron needs fuel: ~charcoal from forest. Famous ore
fields (for checking): Elba (42.78, 10.4), Noricum/Erzberg (47.52, 14.9), Forest of Dean (51.8, -2.55), Meroë (16.9,
33.7), Hunan-Hebei (China), Dhar/Delhi (India) [all M].

### 5.7 Zinc, brass

Brass by cementation in Anatolia and Rome (1st c. BCE) [M]; metal zinc in India (Zawar, Rajasthan (24.35, 73.7)) by ~9th
c. CE and in China later [M]. Low priority. Comes with lead-zinc deposits.

### 5.8 Mercury (cinnabar)

- Cinnabar as pigment "since the Neolithic"; mercury in Egyptian tombs ~1500 BCE [O].
- Patio process (1550s) used mercury to amalgamate silver; it made mercury a strategic metal [O]. Almadén, Monte Amiata and
  Idrija "dominated mercury production" from Almadén's opening ~2,500 years ago to the late 19th c. [O].
  Huancavelica (Peru): found 1563, >100,000 t over three centuries [O].

| Source | Point |
|---|---|
| Almadén, Spain | (38.77, -4.84) [M] |
| Idrija, Slovenia | (46.0, 14.03) [M] |
| Monte Amiata, Italy | (42.88, 11.62) [M] |
| Huancavelica, Peru | (-12.79, -74.97) [M] |
| Wanshan, Guizhou, China | (27.5, 109.2) [M] |
| New Almaden, California | (37.18, -121.82) [M] |
| Sizma (Konya), Turkey | (38.0, 32.4) [M] |

### 5.9 Bitumen (asphalt)

- Hafting glue on flint at Umm el Tlel (Syria) ~71 ka; Neanderthal bitumen-ochre glue at Le Moustier ~60-35 ka [O].
  Waterproofing and glue from at least the 5th millennium BCE [O]; Babylon walls, Egyptian embalming [O].
- Sources: Dead Sea (31.5, 35.5), Hit on the Euphrates (33.64, 42.83), Pitch Lake Trinidad (10.23, -61.63, the largest),
  La Brea (34.06, -118.36), Athabasca (57, -111.5) [O for names, M for points]; also Khuzestan/Susiana (32, 49) and
  Mosul (36.3, 43.1) [M], Baku (40.4, 49.9) [M].
- Rule: seeps over oil basins: a fixed list; plus weak chance on sedimentary (3, 5, 6) cells in known oil basins (Zagros,
  Caspian, Gulf coast of Mexico, Maracaibo, California) [M].

---

## 6. Gems, luxury stones, shells, amber

| Good | First use | Sources | Reach and value |
|---|---|---|---|
| Lapis lazuli | oldest artefacts ~7570 BCE at Bhirrana (Indus area) [O]; mining in Badakhshan by the 7th millennium BCE [O] | Sar-i Sang, Kokcha valley (36.21, 70.79) [O]: "the main source in the ancient world" [O]. Minor: west of Lake Baikal (51.6, 103.6), Chilean Andes (Ovalle, (-30.6, -70.6)) used by the Inca [O for names, M for points]; Pamir (37.4, 72.6) [M] | Mesopotamia from ~4900-4000 BCE (late Ubaid) [O]; Egypt Naqada 3300-3100 BCE [O]; to Mycenae, the Caucasus, even Mauritania [O]. Harappan Shortugai colony near the mines ~2000 BCE [O]. ~2,500-4,000 km. |
| Carnelian and agate | etched Indus beads, 3rd millennium BCE [O] | Gujarat (Ratanpur near Khambhat (21.7, 73.2)) [M]; Deccan basalt gravels; Iran; Sri Lanka (Ratnapura) [O names] | Indus to Mesopotamia by the Gulf [O]; to SE Asia and sub-Saharan Africa later [O]. Proxy: basic volcanic (2) in the tropics + rivers. |
| Turquoise | Egypt mined Sinai from at least the First Dynasty (~3000 BCE); Serabit el-Khadim and Wadi Maghareh [O] | Sinai (28.9, 33.45); Nishapur, Iran (36.5, 58.8); Cerrillos, New Mexico (35.45, -106.1) [O names, M points]; Hubei, China; Kyzylkum (Uzbekistan) [M] | forms in arid climates where water seeps through oxidised copper porphyries [O]: so rule = copper deposit + dry climate. To Mesoamerica from the US Southwest [M]. |
| Amber | Baltic amber to Mycenae and Egypt (Tutankhamun) from at least the 16th c. BCE [O]; Paleolithic and Mesolithic local use [M] | Baltic and North Sea coasts (Sambia peninsula (54.9, 20.0), Jutland) [O coasts; M points]; Sicily (simetite), Burma [O for Sicily]; Dominican Republic, Chiapas [M] | Roman route Baltic -> Aquileia [O]; Baltic replaced Sicilian amber in Iberia ~1000 BCE [O]. ~1,500-3,000 km. The engine already has `G_AMBER`. |
| Spondylus (Europe) | main Neolithic use ~5350-4200 BCE [O] | Aegean and Adriatic coasts, 35..42, 19..27 [M] | LBK and Lengyel bracelets and buckles; Varna [O]; to the Rhine and Paris Basin, ~1,500-2,000 km [M] |
| Spondylus (Andes) | from ~2500 BCE (Valdivia) [M] | Ecuador coast, -3..1.5, -81..-80 [M] (S. crassisquama) | "the driving factor of trade within the Central Andes"; reached Mexico; built a merchant class [O] |
| Money cowrie | China Shang as money and grave goods (~1200 BCE) [M]; "over three thousand years" in China [O] | Maldives (4, 73.5), Sri Lanka, Malabar [O] | India until ~1830; West Africa: at least 30 billion cowries to the Bight of Benin 1500-1875 [O]. |
| Other shell money | Olivella in California for 9,000+ years; dentalium (Alaska-California); wampum (clams, E coast) [O] | dentalium: Vancouver Island (49.5, -126) [M] | |
| Ostrich eggshell beads | ~50 ka [engine note] | dry open Africa and Asia | engine has `G_OSTRICH` |
| Ivory | Paleolithic mammoth ivory [M]; elephant and hippo later | elephant ranges; walrus in the Arctic | engine has `G_IVORY` |
| Pearls | Gulf pearls from ~5000 BCE (Ubaid) [M] | Gulf (26, 51), Gulf of Mannar (9, 79), Panama/Venezuela [M] | warm shallow coast, fixed boxes |
| Jade | see section 2 | | |

---

## 7. Plants: timber, fibres, dyes, incense, spices

### 7.1 Timber

| Wood | Where | Use | Notes |
|---|---|---|---|
| Cedar of Lebanon | Lebanon, W Syria, S-central Turkey (Taurus), Cyprus (Troodos); thickest at 1,300-3,000 m [O] | ships and temples; Phoenician trade to Egypt, Greece, Mesopotamia [O]; Gilgamesh's Cedar Forest [O] | heavy logging, Cyprus cedars once 40 m tall [O]. Box 34..38, 32..37 above 1,300 m [M] |
| Oak | temperate Europe, Near East, E Asia, E North America [M] | ships, houses, charcoal, tannin, acorns | the engine's veg already has broadleaf forest |
| Pine, fir, spruce (masts, pitch, tar) | boreal and mountain forest [M] | masts, tar for ship sealing | Baltic tar and masts 1500-1800 CE [M] |
| Teak | India, Myanmar (nearly half the world's natural teak), Thailand, Laos, Indonesia (Java) [O] | ships; rot proof; 1,250-1,650 mm rain with a 3-5 month dry season is best [O] | |
| Sal, sandalwood, ebony, mahogany | S Asia; Timor and S India (sandalwood); W/C Africa, Sri Lanka (ebony); Caribbean and C America (mahogany) [M] | luxury and ships | fixed boxes later |
| Bamboo | S and E Asia | everything | rain > ~1,000 mm, warm [M] |

**Rule:** timber = tree cover by forest type (the veg model has it). Special woods = fixed boxes with climate gate. Wood
regrows; heavy use for fuel and smelting removes it (a deforestation feedback is worth having).

### 7.2 Fibres

| Fibre | First use | Where | Climate |
|---|---|---|---|
| Wild flax | spun, dyed, knotted wild flax fibres at Dzudzuana Cave (Georgia) ~30 ka [O] | | |
| Flax (linen) | domestic in the Fertile Crescent (Tell Ramad); cloth at Çatalhöyük ~9,000 years ago; in Switzerland/Germany within 5,000 years; China and India ≥5,000 years ago [O] | temperate [O] | cool to warm, moist |
| Hemp | wild fibre "nine to fifty thousand years ago"; Yangshao pottery cord prints 5th millennium BCE; Japan seeds ~8000 BCE; Neolithic across northern latitudes Europe-China [O] | rope, sails, cloth, paper [O] | temperate |
| Cotton | G. arboreum near the Indus before 6000 BCE (Mehrgarh); G. herbaceum maybe eastern Sudan ~5000 BCE; G. hirsutum Mexico 3400-2300 BCE (Tehuacán bolls maybe 5500 BCE); G. barbadense Peru, Ancón ~4200 BCE [O] | | long frost-free season, sunny, 500-1,000 mm rain or irrigation [O] |
| Wool (sheep) | woolly sheep maybe from ~6000 BCE (Iran); woven wool 4000-3000 BCE; into Europe early 4th millennium BCE [O] | | |
| Camelid wool | Andes: camelid fibre by ~5000 BCE; woven wool ~2500 BCE [O] | | |
| Silk | silkworm figure at Nancun (Hebei) 5400-5500 years ago; Yangshao signs [O]; wild silk in the Indus 2450-2000 BCE [O] | mulberry; Byzantium 6th c. CE; ~2,500 worms per pound [O] | warm temperate with mulberry |
| Bast and palm (nettle, lime bast, raffia, agave, yucca, tapa) | Paleolithic onward [M] | everywhere | engine `G_FIBRE` covers wild fibre |

### 7.3 Dyes

| Dye | First use | Source region | Value |
|---|---|---|---|
| Ochre, charcoal | Paleolithic | everywhere | |
| Indigo | oldest indigo-dyed cloth, Huaca Prieta (Peru) ~6,000 years ago [O] | Indigofera (India, the main centre), woad (Europe), Persicaria (E Asia), anil (Americas) [O] | "blue gold" [O] |
| Tyrian purple (murex) | Minoan 20th-18th c. BCE; Phoenician from ~1200 BCE (maybe 1570) [O] | E Mediterranean coasts and Atlantic Morocco (Mogador) [O] | "fetched its weight in silver"; 12,000 snails = 1.4 g [O]. Americas: Plicopurpura on Pacific and Caribbean coasts [O] |
| Cochineal | pre-Columbian [M] | on Opuntia cactus; Oaxaca, Tlaxcala, Puebla; also Peru [O] | after 1500 New Spain's "second-most-valuable export next to silver" [O] |
| Madder (red) | Indus ~3000 BCE, Egypt [M] | W and C Asia, Europe [M] | |
| Saffron, kermes, safflower, henna, turmeric | Bronze Age [M] | Mediterranean, Near East, S Asia [M] | |

### 7.4 Incense and spices (fixed boxes; all tropical or dry-tropical)

| Good | Source | Box [M] | First trade | Notes |
|---|---|---|---|---|
| Frankincense | Dhofar (Oman), Hadramawt (Yemen), Somalia [O] | 16..19, 52..55; 9..12, 47..51 | Egypt's Punt trade (~2500-1500 BCE) [M]; land route peak 3rd c. BCE-2nd c. CE [O] | "value exceeded that of gold" [O] |
| Myrrh | Yemen, Somalia, Ethiopia [O] | 9..16, 42..50 | same | |
| Cinnamon | Sri Lanka [O]; cassia S China [M] | 6..9.8, 79.7..81.9 | | |
| Black pepper | Malabar coast, Kerala [O] | 8..13, 74.5..77 | pepper in Ramesses II's mummy (~1213 BCE) [M] | |
| Cloves | North Moluccas [O] (Ternate, Tidore) | (0.8, 127.4) | cloves in Syria ~1700 BCE [O] (Terqa [M]) | |
| Nutmeg, mace | Banda Islands [O] | (-4.53, 129.9) | | |
| Ginger, cardamom, turmeric | South Asia [O] | 8..22, 73..90 | | |
| Austronesian sea routes | | | by 1500 BCE [O] | the spice routes |
| Cacao (Americas) | Upper Amazon (Ecuador, ~5,300 BP) and Mesoamerica [M] | -5..0, -79..-76; 14..18, -94..-88 | Maya and Aztec money [M] |
| Tobacco, coca | Andes, Amazon (coca ~8,000 BP) [M] | | |

---

## 8. Animals as resources (carrying and riding)

| Animal | Domestic from | Where | Use |
|---|---|---|---|
| Dog | Paleolithic (>15 ka) [M] | Eurasia | hauling sleds and travois [M] |
| Donkey | ~5000 BCE in the Horn of Africa and Red Sea hills, from Nubian and Somali wild ass; Ma'adi (Egypt) 4th millennium BCE; SW Asia by the late 4th millennium [O] | NE Africa | pack animal; "vital in the development of long-distance trade across Egypt" [O] |
| Horse | Botai (Kazakhstan) managed horses ~3700-3500 BCE, but not the modern line; DOM2 domesticated ~2200 BCE in the Volga-Don steppe; chariots at Sintashta ~2000 BCE [O] | wild horses on steppe and open woodland | riding, chariots, war, range |
| Dromedary | "southern Arabian Peninsula around 4000-3000 BCE" (the page; many scholars say ~1000 BCE [M]); common in the Near East by the 10th-9th c. BCE; Egypt with Cambyses 525 BCE [O] | Arabia | caravans: "up to 50 miles per day for a week, up to a month without water" [O]; made trans-Saharan trade possible after the Islamic conquest [O] |
| Bactrian camel | ~3000-2500 BCE, Iran/Central Asia [M] | C Asia | Silk Road |
| Llama, alpaca | ~4500-3500 BCE, Junín puna [M] | Andes | llamas carry 25-30% of body weight 8-13 km a day [O] |
| Cattle (ox) | ~8500 BCE (taurine), ~6000 BCE (zebu) [M; see neolithic.md] | | plough and cart from ~4000-3500 BCE (secondary products) [M] |
| Yak, reindeer, elephant, water buffalo | yak ~2500 BCE Tibet; reindeer ~1st millennium CE; working elephant Indus? [M] | | |

Horses and pack animals are already wild animals in fauna.rs/types.rs; neolithic-plan.md adds donkey bits. Here they matter
as carrying power: they raise the distance a good travels per year and so flatten the trade fall-off.

---

## 9. Fuel and water

| Resource | Notes | Rule |
|---|---|---|
| Wood and charcoal | the fuel of everything until ~1700 CE [M]; smelting 1 kg of iron took very roughly 10-20 kg charcoal [M] | tree cover; deforestation with use |
| Dung | the fuel of treeless steppe, plateau, Andes and Tibet [M] | herds + few trees |
| Peat | covers ~4.23 M km2, ~2.84% of land; mostly high latitudes (W Siberia, Hudson Bay, Mackenzie) and tropical Indonesia; grows ~1 mm a year; fuel in Scotland and Ireland since Roman times; Dutch kilns [O] | wetland + cool climate (or tropical swamp), flat, post-glacial |
| Coal | Neanderthal lignite in France ~70 ka; black coal at Ostrava-Petřkovice ~25-23 ka; China surface coal mining from ~3490 BCE; Roman Britain by the 2nd c. CE; English "sea coal" from ~1215 CE; most British coalfields worked by the 13th c. [O]; Song China's iron on coal ~1000 CE [O] | carboniferous basins: fixed boxes (below). GLiM 3/5 alone is too broad |
| Fresh water | rivers, lakes, springs, wells | engine has rivers and lakes; add springs at the foot of limestone (6) uplands and at the edge of volcanic plateaus, and oases in deserts [M] |

Coal basins (all [M]): Ruhr (51.5, 7.2); Northumberland-Durham (55, -1.6); S Wales (51.7, -3.4); Silesia (50.2, 19.0);
Donbas (48.0, 38.0); Shanxi-Shaanxi (37.5, 111.5); Fushun (41.9, 123.9); Appalachian (37..41, -82..-78); Kuznetsk
(54, 87); Karoo-Witbank (-26, 29.3); Bengal-Jharia (23.75, 86.4); Newcastle NSW (-32.9, 151.7).

---

## 10. Trade distance and value

### 10.1 Distance-decay

Renfrew, Dixon and Cann (1960s) on Near Eastern obsidian [O]:

- **Supply zone:** within ~240-320 km of a source, ~80% of chipped stone at early Neolithic villages is obsidian [O].
- **Contact zone:** beyond ~300 km the share falls off as "a nearly perfect exponential function of distance" [O].
- **Interaction zone:** the area where 30% or more of the obsidian comes from one source [O].
- Caveat: "no ethnographic basis" for the 300 km / 80% / 30% thresholds; they are empirical [O].

A model: share(d) = 0.8 for d < D0, else 0.8 * exp(-(d - D0) / L). With D0 ~ 300 km and an e-folding L ~ 100-150 km
fitted to the Near East data the share is ~1% by ~800-900 km [M for L; the shape is [O]]. A down-the-line chain of
gifts gives this curve [M].

### 10.2 How far goods went

| Good | Typical reach | Long reach | Source tags |
|---|---|---|---|
| Ordinary chert | 10-50 km | 100+ km | Olorgesailie 25-50 km at ~300 ka [O] |
| Obsidian (village use) | supply zone 300 km | 900 km (Anatolia to the Levant, S Iraq) [M]; 1,500 miles Yellowstone -> Ohio [O]; 3,500 km New Britain -> Borneo [O] | |
| Andean obsidian | 170-260 km mean | Quispisisa to the coast [O] | |
| Flint axes (Krzemionki) | 300 km | 660 km [O] | |
| Alpine jade axes | 550 km | 800 km (Brittany), British Isles ~1,500 km [O for 550/800; M for 1,500] | |
| Mount William greenstone | "several hundred km" [O] | | |
| Mushiston copper-tin | <500 km [O] | | |
| Tin (Bronze Age) | 1,000-3,000 km (Kestel and Mushiston to Uluburun) [O sources; M distances] | | |
| Lapis | Badakhshan -> Ur ~2,400 km, -> Egypt ~4,000 km [M distances, O for presence] | | |
| Spondylus, Neolithic Europe | 1,000-2,000 km [M] | | |
| Baltic amber | 1,500-3,000 km [M distances, O for Mycenae and Egypt] | | |
| Salt | 100-300 km by land in the Neolithic [M]; Taghaza-Timbuktu ~800 km by camel [M] | | |

### 10.3 Value (relative scale for the sim)

| Value | Goods |
|---|---|
| Staple (bulk, cheap per kg, carried short) | clay, ordinary stone, wood, salt near its source, ochre |
| Valuable (carried hundreds of km) | obsidian, flint axes, salt far from source, copper, bitumen, dried fish, fibres, cloth |
| Prestige (carried 1,000+ km, small, often "weight for weight" with gold) | gold, silver, tin, lapis, jade, turquoise, carnelian, amber, Spondylus, murex purple (weight in silver [O]), frankincense (above gold [O]), cloves, nutmeg, pepper, cinnamon, silk |

Salt is special: cheap at the source, but "almost weight for weight with gold" at Timbuktu [O]. In the sim value should be
price at source x a cost per km that falls with carrying power (feet < donkey < boat < camel caravan < ship).

---

## 11. Rare-resource point list for the code

One line per entry: resource, lat, lon, quality (0-1), first year it can be found or used (BCE negative). All points [M]
unless the row above says [O]. Quality is a judgement.

```
# obsidian
obs 38.25 34.55 1.0 -12500   Gollu Dag / Acigol (C Anatolia)
obs 38.65 42.23 0.9 -10000   Nemrut Dag
obs 39.30 41.00 0.7 -10000   Bingol
obs 40.30 44.70 0.8 -10000   Armenia (Gutansar/Arteni)
obs 36.73 24.43 0.9 -13000   Melos
obs 36.66 27.13 0.4 -6000    Gyali
obs 38.48 14.95 0.9 -6000    Lipari
obs 39.78  8.73 0.8 -6000    Monte Arci
obs 40.94 12.86 0.5 -6000    Palmarola
obs 36.78 11.99 0.6 -6000    Pantelleria
obs 48.40 21.40 0.6 -20000   Tokaj/Carpathian
obs  0.70 36.40 0.9 -300000  Naivasha/Eburru
obs  8.80 39.90 0.8 -300000  Fantale/Ethiopia rift
obs 20.10 -98.70 1.0 -10000  Pachuca
obs 19.70 -98.75 0.8 -10000  Otumba
obs 19.90 -100.70 0.8 -10000 Ucareo
obs 14.60 -90.40 0.9 -10000  El Chayal
obs 14.40 -89.70 0.8 -10000  Ixtepeque
obs 44.82 -110.73 0.9 -11000 Obsidian Cliff
obs 43.60 -120.10 0.7 -11000 Glass Buttes
obs 35.90 -106.60 0.7 -11000 Jemez
obs -14.30 -74.40 0.9 -10000 Quispisisa
obs -15.10 -72.80 0.8 -10000 Alca
obs -15.60 -71.60 0.8 -10000 Chivay
obs 34.20 139.15 0.9 -36000  Kozushima
obs 43.90 143.20 0.9 -30000  Shirataki
obs 36.20 138.20 0.8 -30000  Wada
obs 42.00 128.10 0.8 -30000  Paektu
obs -5.30 150.00 1.0 -20000  Talasea/Kutau-Bao
obs -2.40 147.30 0.8 -10000  Lou Island
obs -37.28 176.25 0.9 1300   Mayor Island (people arrive ~1250-1300 CE)
# tin
tin 37.55 34.55 0.6 -3250    Kestel
tin 39.30 68.00 0.6 -2000    Mushiston
tin 39.70 65.60 0.5 -2000    Karnab
tin 34.50 62.50 0.4 -2000    Herat area
tin 50.55 13.00 0.8 -2500    Erzgebirge
tin 50.30 -5.10 0.9 -2000    Cornwall
tin 48.00 -3.50 0.5 -2000    Brittany
tin 42.20 -7.80 0.7 -2000    Galicia/N Portugal
tin 43.00 10.60 0.3 -800     Tuscany
tin 23.40 103.20 0.9 -700    Gejiu, Yunnan
tin 35.00 112.50 0.3 -2000   Yellow River small deposits
tin  4.50 101.00 1.0 -1500   Malay tin belt (Kinta)
tin  8.00 98.30 0.9 -1500    Phuket/S Thailand
tin -2.10 106.00 0.9 -500    Bangka
tin  9.70 8.90 0.7 -500      Jos Plateau
tin -24.80 27.70 0.5 1000    Rooiberg
tin -18.00 -66.50 0.9 1000   Oruro-Potosi belt
tin 22.80 -102.60 0.4 1000   Zacatecas
# lapis, turquoise, jade, carnelian
lap 36.21 70.79 1.0 -7000    Sar-i Sang
lap 51.60 103.60 0.4 -3000   Baikal (Slyudyanka)
lap -30.60 -70.60 0.5 1000   Chile (Ovalle)
tur 28.90 33.45 0.8 -3000    Sinai
tur 36.50 58.80 1.0 -3000    Nishapur
tur 35.45 -106.10 0.8 500    Cerrillos
tur 41.50 64.00 0.5 -3000    Kyzylkum
jad 44.67 7.09 0.9 -5300     Mont Viso jadeitite
jad 44.43 8.58 0.7 -5300     Mont Beigua
jad 50.87 16.87 0.4 -6000    Jordanow nephrite
jad 37.10 79.90 1.0 -3000    Khotan nephrite
jad 52.00 101.00 0.7 -6000   Sayan/Baikal nephrite
jad 40.30 123.30 0.8 -5000   Xiuyan
jad 23.80 121.50 0.9 -3000   Fengtian, Taiwan
jad 37.00 137.85 0.9 -5000   Itoigawa jadeite
jad 15.00 -89.60 1.0 -1500   Motagua jadeite
jad 25.60 96.30 1.0 1700     Hpakant jadeite (late)
jad -42.70 171.20 0.9 1300   Pounamu
jad 50.50 -122.00 0.6 -2000  Fraser River nephrite
car 21.70 73.20 1.0 -3000    Ratanpur/Khambhat
# mercury, bitumen
hg 38.77 -4.84 1.0 -500      Almaden
hg 46.00 14.03 0.8 1490      Idrija
hg 42.88 11.62 0.6 -500      Monte Amiata
hg -12.79 -74.97 0.9 1563    Huancavelica (cinnabar used earlier)
hg 27.50 109.20 0.9 -1000    Wanshan
bit 31.50 35.50 0.8 -6000    Dead Sea
bit 33.64 42.83 1.0 -6000    Hit
bit 32.00 49.00 0.8 -6000    Khuzestan
bit 40.40 49.90 0.7 -3000    Baku
bit 10.23 -61.63 1.0 -3000   Pitch Lake
bit 34.06 -118.36 0.7 -8000  La Brea
bit 57.00 -111.50 0.5 -5000  Athabasca
# shells, amber
spo 38.50 23.50 0.9 -5500    Aegean Spondylus
spo 42.00 17.00 0.6 -5500    Adriatic Spondylus
spo -1.50 -80.70 1.0 -3000   Ecuador Spondylus
cow  4.00 73.50 1.0 -1500    Maldives cowrie
amb 54.90 20.00 1.0 -10000   Sambia amber
amb 55.50 8.30 0.7 -10000    Jutland amber
amb 37.40 14.80 0.5 -3000    Sicily amber (simetite)
amb 26.30 96.80 0.5 -1000    Burma amber
mur 33.27 35.20 1.0 -1800    Tyre/Sidon murex
mur 35.20 25.50 0.7 -1900    Crete murex
mur 31.50 -9.77 0.5 -700     Mogador murex
# incense and spices
inc 17.50 54.00 1.0 -2500    Dhofar frankincense
inc 15.50 48.50 0.9 -2500    Hadramawt
inc 10.50 49.00 0.9 -2500    Somalia
cin  7.50 80.70 1.0 -1500    Sri Lanka cinnamon
pep 10.50 76.20 1.0 -1500    Malabar pepper
clo  0.80 127.40 1.0 -1700   Ternate/Tidore cloves
nut -4.53 129.90 1.0 -1000   Banda nutmeg
```

---

## 12. Rules for the common resources (in engine terms)

Cell richness r in 0..1, from `lt[k]` (share of the cell's sample tiles in GLiM class k), relief R (0..1, high = rugged),
climate and a per-cell hash h in 0..1 (so deposits are patchy, not a smooth wash). "Arc" = within ~5 degrees of a
`VOLCANOES` point or of an ocean trench (Pacific rim, Sunda, Aegean, Caribbean, Kurile-Japan).

| Resource | Rule (sketch) | Then thin by |
|---|---|---|
| Flint / chert | lt6 + 0.6 lt5 + 0.3 lt3 + 0.3 lt8 (as now) | none; quality = lt6 |
| Grindstone, axe stone | lt2 + lt4 + lt8 + lt3 (axe stone best from 2, 4, 8) | none |
| Clay | soil clay % and parent alluvium / lake clay | none |
| Ochre | as now (warmth + basalt + metamorphic) | none |
| Salt | 2 lt14 + brine springs (lt14 within one cell and rain > 500 mm) + closed dry basins + coast x evaporation | none |
| Native copper | lt2 x old shield (Lake Superior type) or VMS ophiolite list | h < 0.15 |
| Copper ore | (lt10 + lt13 + 0.5 lt9 + 0.5 lt2 + 0.5 lt8) x R x (arc ? 1.5 : 0.7) | h < 0.25 |
| Arsenical copper | copper cells in arcs and in 25..45 N, 35..75 E (Anatolia-Iran-Caucasus) or the Andes | share 0.5 of copper |
| Gold | (lt8 + lt2 + 0.5 lt9) x (old shield or R > 0.5); placers on river cells downstream within ~200 km | h < 0.15 |
| Lead-silver | (lt6 + lt8 + 0.5 lt10) x R | h < 0.10 |
| Iron ore | 0.3 everywhere + laterite (warm, wet-dry, old surface) + ironstone (lt3, lt5) + bog iron (wet lowland, mean temp < 8 C, lt1) | none |
| Bog iron | wetland share x cool x lowland; regrows in ~25-30 years [O: "once each generation"] | none |
| Peat | wetland x (cool or tropical swamp) x flat, growing ~1 mm/yr after the ice left [O] | none |
| Coal | fixed boxes (section 9) x lt3/lt5 | none |
| Timber kinds | veg forest types; cedar, teak, special woods by box + climate | none |
| Fresh water | rivers, lakes; springs at lt6 upland edges; oases | none |

Check the rule against the real points in sections 5-6: most real copper and gold sites should land in cells the rule
scores in the top ~20%. Where they do not (Timna, Oman, Cyprus, the Copperbelt), add them as fixed points.

---

## 13. Gaps and doubts

- Many coordinates are [M]. The most important ones to check: the tin list, the obsidian list, Mont Viso/Beigua,
  Fengtian, Motagua, Huancavelica.
- Lejja (Nigeria) iron at 2631-2458 BCE is disputed; most writers put West African iron at ~1000-800 BCE [M].
- Dromedary dates differ: the page opened says 4000-3000 BCE; much of the literature says ~1200-1000 BCE [M].
- Mušiston: the ScienceDaily report says Uzbekistan; it is in Tajikistan [O for the paper's view; M for the map].
- I did not open sources on: clay/pottery dates, Spondylus distances, cowrie in China, pearls, cacao, coca, yak, llama
  domestication date, Hallstatt's earliest date, horse/donkey carrying loads, charcoal ratios, coal basin outlines. All [M].
- No global, open, gridded mineral-occurrence map was opened. The USGS Mineral Resources Data System (MRDS) and the USGS
  global porphyry copper assessment would let one test the rules in section 12 and is worth fetching before coding.
- Australia, Pacific islands and sub-Saharan Africa outside the gold and copper belts are thin here.
- The distance-decay e-folding length L is my estimate, not Renfrew's number.

---

## Sources (opened this session)

- Tin sources and trade in ancient times: https://en.wikipedia.org/wiki/Tin_sources_and_trade_in_ancient_times
- Berger et al. 2023, Mushiston tin, Frontiers in Earth Science: https://public-pages-files-2025.frontiersin.org/journals/earth-science/articles/10.3389/feart.2023.1224873/text
- Uluburun tin sources (Powell et al. 2022 report): https://www.sciencedaily.com/releases/2022/11/221130151458.htm
- Tin: https://en.wikipedia.org/wiki/Tin
- Obsidian: https://en.wikipedia.org/wiki/Obsidian
- Obsidian use in Mesoamerica: https://en.wikipedia.org/wiki/Obsidian_use_in_Mesoamerica
- Renfrew zones (Tripcevich, MapAspects): https://mapaspects.org/book/export/html/1901/index.html
- Renfrew, Dixon and Cann (EBSCO research starter): https://ebsco.com/research-starters/history/renfrew-dixon-and-cann-reconstruct-ancient-near-eastern-trade-routes
- Andean obsidian sources (Tripcevich dissertation): https://mapaspects.org/tripcevich-phd-diss/ch3-archaeology-region/33-regional-patterns-and-major-sources-obsidian/index.html
- Franchthi Cave: https://en.wikipedia.org/wiki/Franchthi_Cave
- Kozushima obsidian (KAKEN 16K03165): https://kaken.nii.ac.jp/grant/KAKENHI-PROJECT-16K03165
- Island SE Asia obsidian, Bukit Tengkorak (JAS 2011): https://faculty.washington.edu/plape/pubs/JAS%202011.pdf
- Obsidian Cliff: https://en.wikipedia.org/wiki/Obsidian_Cliff
- Brooks et al. 2018, Olorgesailie: https://experts.illinois.edu/en/publications/long-distance-stone-transport-and-pigment-use-in-the-earliest-mid/
- Flint mining: https://en.wikipedia.org/wiki/Flint_mining
- Spiennes: https://en.wikipedia.org/wiki/Neolithic_flint_mines_of_Spiennes
- Krzemionki: https://en.wikipedia.org/wiki/Krzemionki
- Grime's Graves: https://en.wikipedia.org/wiki/Grime%27s_Graves
- Mount William axe quarry: https://en.wikipedia.org/wiki/Mount_William_stone_axe_quarry
- Mont Viso quarries, radiocarbon (EJA): https://core-cms.cambridgecore.org/core/journals/european-journal-of-archaeology/article/neolithic-quarries-of-mont-viso-piedmont-italy-initial-radiocarbon-dates/7C4570F8497CCEA531497D1A2B287427
- Jade: https://en.wikipedia.org/wiki/Jade
- Nephrite: https://en.wikipedia.org/wiki/Nephrite
- Jadeite: https://en.wikipedia.org/wiki/Jadeite
- Lapis lazuli: https://en.wikipedia.org/wiki/Lapis_lazuli
- Sar-i Sang: https://en.wikipedia.org/wiki/Sar-i_Sang
- Turquoise: https://en.wikipedia.org/wiki/Turquoise
- Carnelian: https://en.wikipedia.org/wiki/Carnelian
- Spondylus: https://en.wikipedia.org/wiki/Spondylus
- Amber Road: https://en.wikipedia.org/wiki/Amber_Road
- Shell money: https://en.wikipedia.org/wiki/Shell_money
- History of salt: https://en.wikipedia.org/wiki/History_of_salt
- Solnitsata: https://en.wikipedia.org/wiki/Provadia-Solnitsata
- Hallstatt: https://en.wikipedia.org/wiki/Hallstatt
- Wieliczka: https://en.wikipedia.org/wiki/Wieliczka_Salt_Mine
- Zigong: https://en.wikipedia.org/wiki/Zigong
- Salt evaporation pond: https://en.wikipedia.org/wiki/Salt_evaporation_pond
- Evaporite: https://en.wikipedia.org/wiki/Evaporite
- Trans-Saharan trade: https://en.wikipedia.org/wiki/Trans-Saharan_trade
- Ochre: https://en.wikipedia.org/wiki/Ochre
- Copper: https://en.wikipedia.org/wiki/Copper
- Old Copper Complex: https://en.wikipedia.org/wiki/Old_Copper_Complex
- Belovode / Rudnik: https://en.wikipedia.org/wiki/Belovode
- Timna Valley: https://en.wikipedia.org/wiki/Timna_Valley
- Magan: https://en.wikipedia.org/wiki/Magan_(civilization)
- Arsenical bronze: https://en.wikipedia.org/wiki/Arsenical_bronze
- Porphyry copper deposit: https://en.wikipedia.org/wiki/Porphyry_copper_deposit
- Volcanogenic massive sulfide: https://en.wikipedia.org/wiki/Volcanogenic_massive_sulfide_ore_deposit
- Copperbelt: https://en.wikipedia.org/wiki/Copperbelt
- Rio Tinto: https://en.wikipedia.org/wiki/Rio_Tinto_(river)
- Varna Necropolis: https://en.wikipedia.org/wiki/Varna_Necropolis
- Gold mining: https://en.wikipedia.org/wiki/Gold_mining
- Greenstone belt: https://en.wikipedia.org/wiki/Greenstone_belt
- Placer deposit: https://en.wikipedia.org/wiki/Placer_deposit
- Silver mining: https://en.wikipedia.org/wiki/Silver_mining
- Laurium: https://en.wikipedia.org/wiki/Laurium
- Cerro Rico: https://en.wikipedia.org/wiki/Cerro_Rico
- Lead: https://en.wikipedia.org/wiki/Lead
- Mercury: https://en.wikipedia.org/wiki/Mercury_(element)
- Iron Age: https://en.wikipedia.org/wiki/Iron_Age
- Bog iron: https://en.wikipedia.org/wiki/Bog_iron
- Bitumen: https://en.wikipedia.org/wiki/Bitumen
- Spice trade: https://en.wikipedia.org/wiki/Spice_trade
- Incense trade route: https://en.wikipedia.org/wiki/Incense_trade_route
- Tyrian purple: https://en.wikipedia.org/wiki/Tyrian_purple
- Indigo dye: https://en.wikipedia.org/wiki/Indigo_dye
- Cochineal: https://en.wikipedia.org/wiki/Cochineal
- Cotton: https://en.wikipedia.org/wiki/Cotton
- Sericulture: https://en.wikipedia.org/wiki/Sericulture
- Flax: https://en.wikipedia.org/wiki/Flax
- Hemp: https://en.wikipedia.org/wiki/Hemp
- Wool: https://en.wikipedia.org/wiki/Wool
- Cedrus libani: https://en.wikipedia.org/wiki/Cedrus_libani
- Teak: https://en.wikipedia.org/wiki/Teak
- Domestication of the horse: https://en.wikipedia.org/wiki/Domestication_of_the_horse
- Donkey: https://en.wikipedia.org/wiki/Donkey
- Dromedary: https://en.wikipedia.org/wiki/Dromedary
- Llama: https://en.wikipedia.org/wiki/Llama
- Peat: https://en.wikipedia.org/wiki/Peat
- History of coal mining: https://en.wikipedia.org/wiki/History_of_coal_mining
- Engine source read: engine/src/tribes.rs (`tr_goods_cells`), lib.rs (`lith`, `VOLCANOES`), tools/bake_soil.py (`GLIM_NAMES`)
