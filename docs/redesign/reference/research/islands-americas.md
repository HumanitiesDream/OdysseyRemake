# Islands and the Americas: when people first got there

Research for Odyssey's human dispersal (`tribes.rs`: `tr_first_wave`, `tr_beringia`, `tr_frontier`, `S_RAFT`, `S_BOAT`).
Written 2026-10-05.

Tags: **[O]** = I opened a source that says this (listed under Sources). **[M]** = from memory, not checked this session. Treat [M] numbers as good to about ±20%.
Dates: **ka** = thousand calibrated years before present. BCE/CE as usual. 1 ka ≈ 1,000 years before 1950.

---

## 0. Why the sim gets this wrong (what the code does now)

- The grid is 2-degree cells (`CW=180, CH=90`). One cell is about 222 km north–south.
- `tr_frontier` lets a people cross **1 sea cell** with `S_RAFT` and **3 sea cells** with `S_BOAT`. One sea cell can hide a real gap of 0–400 km. Three sea cells can be 600+ km. That is Lapita range, not Ice Age range.
- In the fast pre-dispersal (`tr.pre > 0`) a people does not need to know of the land first. So any island within 3 sea cells is fair game in the Ice Age. That is how Fiji, Samoa and East Polynesia fill up during the LGM.
- `tr_flee_sea` calls `tr_frontier(..., sea_min = 3)`. This lets even raft-less peoples hop 3 sea cells when the sea rises.
- `tr_beringia` runs only in 40% of worlds (`world >= 0.4` returns). Nothing seeds anyone south of the ice sheets.

Fix in one line: **measure the gap in km, gate each remote island group by a date, and seed one people south of the ice.** Details in sections 2 and 4.

---

## 1. Island table

"Crossing" = the shortest open-water hop needed at the sea level of that time (not today's). LGM sea level was about −120 to −130 m [M]. At ~50–65 ka it was about −75 to −85 m [O, Bird 2019].
"Tech" uses the tiers defined in section 2 (T0 to T6).
Boxes are rough lat/lon rectangles [M]; on a 2-degree grid, treat a box as "any land cell whose centre falls inside, or within 1 cell of it". Negative lon = west.

### 1a. Sahul, Near Oceania, Wallacea, East and South Asia

| Place | Box (lat; lon) | First people | Crossing at the time | Tech |
|---|---|---|---|---|
| Wallacea: Sulawesi | −6 to 2; 118.5 to 125.5 | Cave art ≥51.2 ka (Leang Karampuang) [O] | Several hops from Borneo; each < ~70 km and intervisible on the northern route [O Bird 2019; widths M] | T1 |
| Wallacea: Lesser Sundas (Lombok–Timor), incl. Flores, Alor | −11 to −8; 115.8 to 127.5 | Timor: Jerimalai 42 ka, deep-sea fishing, oldest fish hooks [O]; Laili ~44 ka [M] | Lombok Strait ~20–35 km [M]; rest short hops | T1 |
| Wallacea: Moluccas | −9 to 3; 124.5 to 135 | ~45–50 ka [M] | Short hops, intervisible [O Bird 2019] | T1 |
| Sahul: Australia | −44 to −10; 112 to 154 | Madjedbebe 65 ka (Clarkson 2017); critics say ~50 ka [O]. "At least 50 ka" [O Bird 2019] | Northern route: last hops to New Guinea, all with land in sight; southern route: Timor/Roti to Sahul banks ~90–100 km, target **not** visible from the coast, only from hills [O Bird 2019] | T1 |
| Sahul: New Guinea | −11 to 0; 130 to 151 | 60–40 ka (same land as Australia) [O Near/Remote Oceania page] | Joined to Australia at low sea | T1 |
| Tasmania | −43.7 to −39.5; 143.8 to 148.5 | ~35 ka (Warreen Cave ~34 ka BP) [O] | Walked over the Bassian land bridge [O]. Cut off ~14–12 ka [O; the page's era label is unclear] | T0 |
| Bismarcks (New Britain, New Ireland) | −7 to −1; 146 to 153 | ~45–40 ka (Buang Merabak, Matenkupkum) [M]; 60–40 ka for the region [O] | New Guinea to New Britain ~30–60 km, intervisible [M] | T1 |
| Admiralties (Manus) | −3.2 to −1.8; 146 to 148 | ~21–20 ka (Pamwak) [M] | 200–300 km voyage with a **60–90 km blind stretch**; the only Pleistocene island settled beyond one-way intervisibility [O] | T1+ |
| Solomons (Buka to Makira) | −11 to −5; 154 to 162.5 | Kilu Cave, Buka: ~29–32 ka [O] | ≥60 km open sea from Nissan to Buka [O] | T1 |
| Philippines: Palawan | 8.3 to 12.5; 117 to 120.5 | Tabon Cave ~47 ka [M] | Nearly joined to Borneo at LGM; Balabac gap a few km [M] | T0–T1 |
| Philippines: Luzon | 12.5 to 18.7; 119.5 to 124.5 | Callao Cave 66.7 ± 1 ka (probably *H. luzonensis*, not *sapiens*) [O]; *sapiens* ~40 ka+ [M] | Always an island; gaps ~20–60 km from Sulawesi/Borneo side [M] | T1 |
| Taiwan | 21.9 to 25.3; 120 to 122 | Zuozhen 30–20 ka; Changbin 15–5 ka [O] | Land bridge to Fujian at low sea [O] | T0 |
| Ryukyus (Yonaguni to Okinawa) | 24 to 30; 122.9 to 131.5 | Yamashita-cho, Okinawa ~32 ka; Shiraho, Ishigaki ~20 ka+ [O]; Ryukyus reached ~35 ka [O] | Taiwan to Yonaguni: replica dugout paddled 225 km in 45 h, island below the horizon most of the way, across the Kuroshio. Reed and bamboo rafts **failed** [O Kaifu 2025] | T1+ (dugout) |
| Japan (Honshu, Kyushu, Shikoku) | 30 to 46; 129 to 146 | 38–35 ka [O] | Korea Strait never fully dry; ~20–40 km at LGM [M]. Hokkaido joined to Sakhalin/Asia [M] | T1 |
| Andamans | 10.5 to 13.7; 92.2 to 93.1 | Genetic: after ~26 ka; archaeology only 1st millennium BCE [O] | Short hops via Coco/Preparis islands at low sea, maybe a near land bridge [O "possible"; widths M] | T1 |
| Sri Lanka | 5.9 to 9.9; 79.6 to 81.9 | Fa-Hien Lena ~48 ka (bows), ~34 ka bones [O] | Land bridge to India at low sea [O] | T0 |

### 1b. Indian Ocean and Atlantic

| Place | Box | First people | Crossing | Tech |
|---|---|---|---|---|
| Madagascar | −25.7 to −11.9; 43.2 to 50.5 | Cut bones ~10.5 ka and forager visits ~4 ka claimed, disputed [O]. Continuous settlement from ~500 CE (Andavakoera 490 CE); Austronesians by early CE, maybe 500–200 BCE [O] | Mozambique Channel ≥ ~400 km [M]; Austronesian voyage from Indonesia thousands of km [M] | T4 |
| Comoros | −12.9 to −11.3; 43.2 to 45.3 | 8th–10th c. CE (Dembeni); Austronesians no later than 8th c. [O] | ~300 km from Africa [M] | T4 |
| Mascarenes (Mauritius, Réunion, Rodrigues) | −21.5 to −19.6; 55.2 to 63.6 | **Never before Europeans** (16th–17th c.) [O] | ~900 km from Madagascar [O] | none (lock) |
| Socotra | 12.1 to 12.8; 52 to 54.6 | Trade visits from 2nd c. BCE in texts [O]; settled earlier, maybe Bronze/Iron Age [M] | ~240 km from Cape Guardafui, less via Abd al-Kuri [M] | T3 |
| Canaries | 27.6 to 29.5; −18.2 to −13.3 | First millennium BCE, Berber-like North Africans; Tenerife site 6th c. BCE [O] | ~100 km to Fuerteventura [O] | T3 |
| Cape Verde | 14.8 to 17.2; −25.4 to −22.6 | **Uninhabited** until Portuguese 1456–1462 [O] | 570–850 km [O] | none (lock) |
| Madeira | 32.4 to 33.1; −17.3 to −16.2 | Portuguese 1419–20; mice hint at Norse visits ~900–1036 CE [M] | ~520 km from Morocco [M] | none (lock) |
| Azores | 36.9 to 39.8; −31.3 to −24.8 | Portuguese 1427/1432; lake sediments suggest people and livestock from ~700 CE (likely Norse), not lasting [O] | ~1,400 km from Iberia [M] | none (lock) |
| Britain | 49.9 to 58.7; −6.4 to 1.8 | Re-occupied after the ice ~14.7 ka [M] | Land bridge (Doggerland) until ~7050 BCE [O] | T0 |
| Ireland | 51.4 to 55.4; −10.5 to −5.4 | Butchered bear bone ~10,500 BCE; Mesolithic Mount Sandel ~7900 BCE [O/M] | No land bridge [O]; North Channel ~20–30 km [M] | T2 |
| Faroes | 61.4 to 62.4; −7.7 to −6.2 | Pre-Norse people ~500 CE (barley, sheep DNA); Norse early 9th c. [O] | ~300 km from Shetland [M] | T6 |
| Iceland | 63.3 to 66.6; −24.6 to −13.5 | Norse 871–874 CE (tephra); maybe Irish papar before [O] | ~450 km from the Faroes [O] | T6 |
| Greenland | 59.7 to 83.7; −73.5 to −11.3 | Paleo-Inuit ~2500 BCE (Independence I, Saqqaq); Dorset; Thule by ~1300 CE [O]; Norse 985 CE [M] | Nares Strait ~30 km, often iced over [M]. Norse: ~300 km from Iceland [M] | T2 (Arctic) / T6 |
| Falklands | −52.5 to −51; −61.4 to −57.7 | No people at European contact [M]. Possible brief visits 1,070–620 years ago (charcoal, butchered seal bones) [O] | ~450 km from Patagonia [M] | none (lock) |

### 1c. Mediterranean

| Place | Box | First people | Crossing | Tech |
|---|---|---|---|---|
| Cyprus | 34.5 to 35.7; 32.2 to 34.6 | Hunter-gatherers ~11,000–10,000 BCE (Akrotiri Aetokremnos); farmers 8800–8600 BCE [O] | ~60–70 km from Anatolia, intervisible from hills [M] | T2 |
| Crete | 34.8 to 35.7; 23.5 to 26.3 | Plakias stone tools ~130 ka claimed, debated [M]; farmers at Knossos ~7000 BCE [M] | Via Kythera/Antikythera, hops < ~40 km [M] | T2–T3 |
| Sardinia + Corsica | 38.8 to 43.1; 8.1 to 9.9 | Joined as one island at LGM, a narrow strait from Tuscany [O]. Upper Paleolithic (Corbeddu) contested; Mesolithic Su Coloru ~9–10 ka [O site; date M]; Neolithic 6th millennium BCE [O] | ~10–30 km via Elba at LGM [M] | T1–T2 |
| Malta | 35.8 to 36.1; 14.2 to 14.6 | Mesolithic ~6500 BCE (Għar Dalam) [O]; Neolithic ~5900 BCE [M] | ~100 km from Sicily, longest known Mediterranean hunter-gatherer crossing [O] | T2 |
| Balearics | 38.6 to 40.1; 1.2 to 4.4 | 3rd millennium BCE (Mallorca), not earlier [O Ramis & Alcover] | ~80–90 km Spain to Ibiza [M] | T3 |

### 1d. The Americas' islands

| Place | Box | First people | Crossing | Tech |
|---|---|---|---|---|
| Channel Islands, California (Santarosae) | 32.8 to 34.1; −120.5 to −118.3 | Arlington Springs ~13 ka [O] | ~6–10 km at the time [M]; needs a boat [O] | T1 |
| Aleutians | 51.2 to 55; 172.4 E to −163 | East (Unalaska) ~9 ka, Adak ~6 ka, west/Near Is. ~2.5 ka [O] | Gaps up to ~100 km in the west (Buldir) [M] | T2 (skin boat) |
| Trinidad | 10 to 10.9; −61.9 to −60.9 | ~8 ka (Banwari Trace) [M] | Joined to Venezuela until early Holocene [M] | T0 |
| Greater Antilles (Cuba, Hispaniola, Puerto Rico, Jamaica) | 17.6 to 23.3; −85 to −65.2 | ~5.8 ka (Cuba, Hispaniola, Puerto Rico first) [O Napolitano 2019]. Jamaica ~600 CE [M] | Yucatán–Cuba ~200 km [M]; the study argues a direct voyage from South America [O] | T3 |
| Lesser Antilles | 11.9 to 18.6; −65.3 to −59.4 | After the Greater Antilles, north first then south [O]; farmers (Saladoid) ~500 BCE [M] | Chain is mostly intervisible; Grenada–Trinidad ~130 km [M] | T3 |
| Bahamas | 20.9 to 27.3; −79.3 to −72.7 | ~700–800 CE [M] | Short hops from Cuba/Hispaniola [M] | T3 |
| Galápagos | −1.5 to 0.7; −92 to −89.2 | **Never** before 1535; Heyerdahl's finds are post-Spanish [O] | 926 km [O] | none (lock) |

### 1e. Remote Oceania

Near Oceania (to the Solomons) was settled in the Pleistocene. **Nothing beyond it was settled until Lapita, ~1200 BCE onward** [O].

| Place | Box | First people | Crossing | Tech |
|---|---|---|---|---|
| Marianas | 13.2 to 20.6; 144.6 to 146.1 | 1500–1400 BCE, from the Philippines; maybe the longest voyage of its time [O] | ~2,000+ km direct from the Philippines [M] | T4 |
| Palau, Yap | 6.8 to 9.7; 134 to 138.3 | ~1000 BCE [O] | ~800 km from Mindanao/Moluccas [M] | T4 |
| Eastern Carolines (Pohnpei, Chuuk, Kosrae), Marshalls, Kiribati | −2.7 to 14.7; 137.5 to 176.9 | ~200 BCE, from Island Melanesia [O]; Marshalls ~2 ka [M] | Atoll hops 200–700 km [M] | T4 |
| Santa Cruz / Reefs | −11.8 to −10; 165.7 to 166.9 | ~3.0–3.2 ka (Lapita) [M] | ~350–400 km from the Solomons, beyond sight [M] | T4 |
| Vanuatu | −20.3 to −13; 166.5 to 170.3 | ~1000 BCE (Teouma) [O] | Short hops from Santa Cruz [M] | T4 |
| New Caledonia | −22.8 to −19.5; 163.5 to 168.2 | ~3.0 ka (Lapita) [M] | ~230 km from Vanuatu [M] | T4 |
| Fiji | −19.3 to −15.7; 176.8 to −178.4 | ~1000–900 BCE (Lapita) [O] | Northern Vanuatu to Fiji >800 km [O] | T4 |
| Tonga | −22.4 to −15.5; −176.3 to −173.7 | 826 BCE ±8 (2838 BP), Nukuleka [O] | ~300 km from Fiji's Lau group [M] | T4 |
| Samoa | −14.6 to −13.4; −172.8 to −168.1 | ~800 BCE [O Wilmshurst 2011] | ~850 km from Fiji, less from Tonga's north [M] | T4 |
| **The long pause** | — | ~1,800 years with no move east of Samoa [O] | — | — |
| Society Is. | −17.9 to −16.2; −154.7 to −148.1 | 1025–1120 CE [O] | ~1,600+ km from Samoa via the Cooks [M] | T5 |
| Cook Is. | −22 to −8.9; −166 to −157.3 | 1190–1290 CE [O] | ~1,000 km hops [M] | T5 |
| Marquesas | −10.6 to −7.8; −140.9 to −138.5 | 1200–1277 CE [O] | ~1,400 km from the Societies [M] | T5 |
| Hawaii | 18.9 to 22.3 (to 28.4 NW); −160.3 to −154.8 | 1219–1266 CE [O] | ~3,500–4,400 km from the Marquesas/Societies [M] | T5 |
| Rapa Nui | −27.2 to −27.05; −109.5 to −109.2 | 1200–1253 CE [O] | ~2,600 km from Mangareva [M] | T5 |
| New Zealand | −47.3 to −34.4; 166.4 to 178.6 | 1230–1282 CE [O] | ~3,000 km from the Cooks; Tahiti–NZ 3,603 km was sailed in 1965 [O] | T5 |
| Chatham Is. | −44.4 to −43.6; −177 to −176.1 | ~1450 CE, from New Zealand; a canoe dated 1440–1470 [O] | ~800 km from NZ [M] | T5 |
| Kerguelen, Crozet, Heard, South Georgia, Bouvet, St Helena, Ascension, Tristan, Bermuda, Juan Fernández, Lord Howe | (each tiny) | **Never** settled before Europeans [M] | Very far | none (lock) |
| Norfolk, Pitcairn/Henderson, Auckland Is., Kermadecs | (each tiny) | Polynesians came ~1200–1400 CE, then left; empty at contact [M] | — | T5, then abandon |

---

## 2. How far could people cross, by era

### What the evidence says

| Era | Craft | Longest proven hop | Target in sight? | Key cases |
|---|---|---|---|---|
| Pleistocene, 65–40 ka | Rafts or simple canoes; planned voyages, not drift. ~1,300 founders needed for Sahul [O Bird 2019] | ~60–100 km | Yes on the main route. Southern route ~90–100 km seen only from hills [O] | Wallacea, Sahul, Buka ≥60 km [O] |
| Late Pleistocene, 35–20 ka, W. Pacific | Dugout canoe (rafts too slow for the Kuroshio) [O] | Taiwan–Yonaguni, 225 km paddled in 45 h [O]; Manus 200–300 km voyage, 60–90 km blind [O] | Mostly no | Ryukyus ~35 ka, Manus ~20 ka |
| Early Holocene, 12–8 ka | Dugouts, hide boats | ~100 km | Usually yes | Cyprus, Malta 100 km [O], Ireland, Channel Is. |
| Neolithic / Bronze Age, 8–3 ka | Plank boats, big canoes, paddled | ~100–200 km, longer by chains | Mostly | Crete, Balearics, Canaries 100 km [O], Caribbean 5.8 ka [O] |
| Austronesian / Lapita, 3.5–2.5 ka | Sailing outrigger canoes [O] | Vanuatu–Fiji >800 km [O]; Philippines–Marianas ~2,000 km [M] | No | Remote Oceania from ~1200 BCE [O] |
| Polynesian, 1000–1450 CE | Double-hulled voyaging canoes, 15–30 m, up to 80–100 people; star paths, swells, birds [O] | ~2,600–4,000 km | No | East Polynesia in about a century [O] |
| Medieval North Atlantic, 500–1000 CE | Curraghs, Norse ships | ~450 km (Faroes–Iceland) [O] | No | Faroes, Iceland, Greenland |

### Rules for the sim (recommended)

Measure the gap as **great-circle km from the last land cell to the next land cell**, not a count of sea cells. On 2-degree cells, take centre-to-centre km minus ~1 cell (~200 km) as the real coast-to-coast gap; better still, use a precomputed coastline gap table for the island boxes in section 1.

| Tier | Sim skill (suggest) | Available from | Max gap | Must see target? |
|---|---|---|---|---|
| T0 | none | always | 0 km (land bridges, ice only) | — |
| T1 | `S_RAFT` (Pleistocene craft) | ~65 ka | 100 km | Yes (gap ≤ ~100 km and target island high or big) |
| T1+ | `S_RAFT` + coast-adapted, W. Pacific only | ~35 ka | 230 km | No, but only along the Ryukyu/Manus kind of chain |
| T2 | `S_BOAT` (skin boats, dugouts) | ~12 ka | 120 km | Yes, or a chain of sights |
| T3 | `S_BOAT` + farming/plank boats | ~8 ka | 200 km | No for ≤ 120 km; yes beyond |
| T4 | Austronesian outrigger sail (new skill) | ~3.5 ka, from Taiwan/Philippines only | 900 km (2,000 km for a rare long shot like the Marianas) | No |
| T5 | Polynesian double canoe (new skill) | ~1000 CE, from Samoa/Tonga only | 4,000 km | No |
| T6 | North Atlantic ship (new skill) | ~500 CE (Irish), ~800 CE (Norse) | 500 km | No |

Plain "intervisible" rule: two islands can see each other when `3.57 * (sqrt(h1) + sqrt(h2)) km >= gap`, with h in metres of the highest point [M]. A 1,000 m peak is seen from ~113 km at sea level.

**Hard date locks (the simple, safe fix).** Whatever the tech, a box cannot be settled before its date:
- Remote Oceania (Santa Cruz, Vanuatu, New Caledonia, Fiji, Tonga, Samoa, Micronesia): not before **1500 BCE** (Marianas) / **1100 BCE** (Lapita boxes).
- East Polynesia (Society, Cook, Marquesas, Austral, Gambier): not before **1000 CE**. Hawaii, Rapa Nui, New Zealand: not before **1200 CE**. Chatham: not before **1400 CE**.
- Madagascar and Comoros: not before **~0 CE** (soft), **500 CE** for lasting settlement.
- Mascarenes, Cape Verde, Madeira, Azores, Galápagos, Falklands, Kerguelen and the other "never" islands: **locked for the whole preindustrial sim**.
- In the fast pre-dispersal: cap sea crossing to **1 cell (≤ ~100 km real gap)** and apply all locks. Also cap `tr_flee_sea`'s `sea_min = 3` to the people's own tier.

---

## 3. The peopling of the Americas

### Sites (south of the ice unless noted)

| Site | Where (lat, lon) | Date | How sure |
|---|---|---|---|
| Chiquihuite Cave, Zacatecas, Mexico | ~24.6, −102.4 [M] | ~26 ka (46 radiocarbon + 6 luminescence dates) [O] | Contested: stones may be natural [O] |
| **White Sands footprints, New Mexico** | 32.8, −106.3 [M] | **23–21 ka**: seed dates (2021), then pollen and OSL (2023) [O] | Widely accepted now |
| Gault, Texas | 30.9, −97.7 [M] | 16–20 ka, pre-Clovis points (OSL) [O] | Fairly accepted |
| Meadowcroft, Pennsylvania | 40.3, −80.5 [M] | ≥16 ka [O] | Debated |
| **Cooper's Ferry, Idaho** | 45.6, −116.4 [M] | **16.56–15.28 ka** [O] | Accepted; read as a Pacific-coast entry up the Columbia [O] |
| Buttermilk Creek, Texas | 30.8, −97.7 [M] | 15.5 ka [O] | Accepted |
| Paisley Caves, Oregon | 42.7, −120.5 [M] | Human DNA in coprolites 14.3 ka [O] | Accepted |
| **Monte Verde II, Chile** | −41.5, −73.2 [O] | **14.5–14.8 ka** [O]. MV-I 18.5 ka claimed, disputed even by its digger [O] | II accepted |
| Arlington Springs, Channel Is. | 34.0, −120.1 [M] | ~13 ka [O] | Accepted |
| Clovis | across North America | 13.0–12.6 ka [O] | Accepted |

### Routes

- **Pacific coast ("kelp highway").** Southeast Alaska islands were ice-free by **~17 ka** (boulder exposure dating) [O Lesnek 2018]. Coast of Alaska and BC open ~16 ka [O]. This is the route most papers now favour for the first peoples [O Clark 2022; O Cooper's Ferry].
- **Ice-free corridor** (between the Cordilleran and Laurentide sheets, ~50–60 N, −125 to −110). Physically open only **~13.8 ka** (64 beryllium dates) [O Clark 2022]. Plants and animals to live on only **~12.6 ka** [O Pedersen 2016]. So it **cannot** carry the first people south. It can carry later people (and bison) after ~13 ka.
- **White Sands at 23–21 ka** means someone was south of the ice **before or during the LGM**. They likely came along the coast before the ice closed it (before ~26–24 ka), or around it by boat [M].

### Genes

- Ancestral Native Americans split from Siberians and East Asians **~23 ka** [O Moreno-Mayar 2018]. Gene flow with Asia ended ~25 ka [O Cambridge 2018]. They formed from East Asian (~65%) and Ancient North Eurasian (~35%) ancestry, 25–20 ka [O].
- **Ancient Beringians** split off **~22–18 ka** (~20 ka). They stayed in Alaska until ~11.5 ka (Upward Sun River) [O].
- **Northern and Southern Native Americans** split **17.5–14.6 ka, south of eastern Beringia**, i.e. after people got past the ice [O]. The southern branch then spread fast: near-identical genomes 2 ka and thousands of km apart (Spirit Cave 10.7 ka, Lagoa Santa 10.4 ka) [O].
- A ghost population "UPopA" split ~24.7 ka (30–22 ka) [O]. Some Amazonians carry ~3% Australasian-like "Population Y" ancestry [O]. The White Sands people may be an early group that left few or no descendants [M].
- **Beringian standstill.** The founders were isolated for a few thousand years (~25 to ~16 ka) in or near Beringia before the push south [M; Llamas 2016 says ~2.4–9 ka of isolation, then a fast coastal spread ~16 ka — not checked: I could not open the paper this session].

### Rule for the sim: seed the first Americans

1. **Always** create the Beringian people when the start is earlier than 11 ka (remove the 60% skip in `tr_beringia`, or make it 100% for starts < 11 ka). The genetics require a Beringian group from ~25 to ~11.5 ka.
2. **Seed one "First American" people south of the ice** at the end of the pre-dispersal, when the start date is **≤ 16 ka** (or always, if the owner wants them from the LGM):
   - Where: the best unoccupied, ice-free land cell in **lat 32–46 N, lon −124 to −104**, within ~4 cells of the Pacific coast (between White Sands, Paisley and Cooper's Ferry). Fallback box: lat 25–35 N, lon −115 to −95.
   - Size: ~300–500 people, like `tr_beringia` (`n` 170–400).
   - Line: a child lineage of the Beringian people with `crossed = true`, so it gets its own branch and colour. Founder year: **~23 ka** (before the coast closed), shown as "Proto-American".
   - Skills: the Beringian set plus `S_RAFT` (coastal), cold adapt ~0.4.
3. **Block** any overland spread from Beringia south across the ice before **13.8 ka**. Allow a **coastal hop** (boats, along the Pacific edge, 1 sea cell) from **16 ka**. Allow the inland corridor (lat 50–60 N, lon −125 to −110) only from **13 ka**.
4. **Spread targets** to check against: southern Chile (−41.5, −73.2) reached by **14.5 ka**; most of both continents by **13 ka**.
5. If the start is **later than ~14.5 ka**, the pre-dispersal should fill both continents (thin) rather than seed one people; if later than **13 ka**, fill fully (Clovis world).
6. No people anywhere in the Americas before **~25 ka** (no Chiquihuite-style 26–33 ka seeds; it is contested).

---

## Sources (opened this session)

- Bird et al. 2019, "Early human settlement of Sahul was not an accident", *Scientific Reports*: https://pmc.ncbi.nlm.nih.gov/articles/PMC6579762
- USGS 2023, White Sands footprints confirmed: https://www.usgs.gov/news/national-news-release/study-confirms-age-oldest-fossil-human-footprints-north-america
- Wilmshurst et al. 2011, East Polynesia colonization, *PNAS*: https://pmc.ncbi.nlm.nih.gov/articles/PMC3033267
- Moreno-Mayar et al. 2018, early dispersal within the Americas (PDF): https://escholarship.org/content/qt03v4b6mr/qt03v4b6mr.pdf
- Clark et al. 2022, ice-free corridor opening (summary): https://popular-archaeology.com/article/ice-free-corridor-opening-and-peopling-of-the-americas/
- Pedersen et al. 2016, corridor viability (Cambridge news): https://www.cam.ac.uk/research/news/textbook-story-of-how-humans-populated-america-is-biologically-unviable-study-finds
- Moreno-Mayar et al. 2018, Upward Sun River genome (Cambridge news): https://www.cam.ac.uk/research/news/direct-genetic-evidence-of-founding-population-reveals-story-of-first-native-americans
- Lesnek et al. 2018, coastal route deglaciation (Univ. at Buffalo): https://www.buffalo.edu/news/releases/2018/05/044.html
- Davis et al. 2019, Cooper's Ferry: https://popular-archaeology.com/article/archaeological-site-reveals-humans-arrived-in-the-americas-more-than-16000-years-ago/
- Williams et al. 2018, Gault (Texas State Univ.): https://news.txst.edu/about/news-archive/press-releases/2018/july/gault071318.html
- Paisley Caves: https://www.heritagedaily.com/2012/07/the-clovis-first-theory-is-put-to-rest-at-paisley-caves/47187
- Settlement of the Americas (overview): https://en.wikipedia.org/wiki/Settlement_of_the_Americas
- Monte Verde: https://en.wikipedia.org/wiki/Monte_Verde
- Chiquihuite Cave: https://en.wikipedia.org/wiki/Chiquihuite_Cave
- Arlington Springs Man: https://en.wikipedia.org/wiki/Arlington_Springs_Man
- Kaifu et al. 2025, Taiwan–Yonaguni dugout voyage: https://popular-archaeology.com/article/ancient-canoe-replica-tests-paleolithic-migration-theory/ and project page https://www.kahaku.go.jp/research/activities/special/koukai/en/
- Oktaviana et al. 2024, Sulawesi art 51.2 ka (Griffith Univ.): https://news.griffith.edu.au/2024/07/04/cave-painting-in-indonesia-is-the-oldest-known-picture-story/
- Mijares et al., Callao Cave 67 ka (ANU): https://openresearch-repository.anu.edu.au/items/800be923-823e-4c49-9dc1-5ccc672eb5a1
- O'Connor et al. 2011, Jerimalai pelagic fishing (ANU): https://researchportalplus.anu.edu.au/en/publications/pelagic-fishing-at-42000-years-before-the-present-and-the-maritim/
- Madjedbebe: https://en.wikipedia.org/wiki/Madjedbebe
- Near and Remote Oceania (Manus blind crossing): https://en.wikipedia.org/wiki/Near_Oceania_and_Remote_Oceania
- Kilu Cave: https://en.wikipedia.org/wiki/Kilu_Cave
- Burley et al. 2012, Tonga U/Th (Sci.News summary): https://www.sci.news/archaeology/article00715.html
- Lapita culture: https://en.wikipedia.org/wiki/Lapita_culture
- Polynesian navigation (World History Encyclopedia): https://www.worldhistory.org/article/1586/polynesian-navigation--settlement-of-the-pacific/
- Anderson, Lapita voyaging review (abstract): https://scholarspace.manoa.hawaii.edu/items/6b6735de-90ff-44a1-8942-425ee5b4c174/full
- Mariana Islands prehistory: https://en.wikipedia.org/wiki/Prehistory_of_the_Mariana_Islands
- History of Micronesia: https://en.wikipedia.org/wiki/History_of_Micronesia
- Moriori (Chathams): https://en.wikipedia.org/wiki/Moriori
- Prehistory of Japan: https://en.wikipedia.org/wiki/Prehistory_of_Japan
- Prehistory of Taiwan: https://en.wikipedia.org/wiki/Prehistory_of_Taiwan
- Andamanese peoples: https://en.wikipedia.org/wiki/Andamanese_peoples
- Prehistory of Sri Lanka: https://en.wikipedia.org/wiki/Prehistory_of_Sri_Lanka
- Aboriginal Tasmanians: https://en.wikipedia.org/wiki/Aboriginal_Tasmanians
- Settlement of Madagascar: https://en.wikipedia.org/wiki/Settlement_of_Madagascar
- Comoros: https://en.wikipedia.org/wiki/Comoros
- Mascarene Islands: https://en.wikipedia.org/wiki/Mascarene_Islands
- History of Socotra: https://en.wikipedia.org/wiki/History_of_Socotra
- Guanches (Canaries): https://en.wikipedia.org/wiki/Guanches
- Cape Verde: https://en.wikipedia.org/wiki/Cape_Verde
- Raposeiro et al. 2021, Azores (Eos): https://eos.org/articles/sediments-suggest-vikings-may-have-been-the-first-to-settle-the-azores
- Faroe Islands: https://en.wikipedia.org/wiki/Faroe_Islands
- Settlement of Iceland: https://en.wikipedia.org/wiki/Settlement_of_Iceland
- Paleo-Eskimo (Greenland): https://en.wikipedia.org/wiki/Paleo-Eskimo
- Prehistoric Cyprus: https://en.wikipedia.org/wiki/Prehistoric_Cyprus
- Pre-Nuragic Sardinia: https://en.wikipedia.org/wiki/Pre-Nuragic_Sardinia
- Prehistory of Malta: https://en.wikipedia.org/wiki/Prehistory_of_Malta
- Ramis & Alcover, earliest Mallorca (abstract): https://cambridge.org/core/journals/proceedings-of-the-prehistoric-society/article/abs/revisiting-the-earliest-human-presence-in-mallorca-western-mediterranean/34BF441F2A5489474C0F15ED88AF70B9
- Prehistoric Ireland: https://en.wikipedia.org/wiki/Prehistoric_Ireland
- Napolitano et al. 2019, Caribbean colonization (Univ. of Oregon; Futurity): https://news.uoregon.edu/content/tracing-journeys-caribbeans-first-people and https://www.futurity.org/first-caribbean-islanders-2249382-2/
- Aleutian Islands: https://en.wikipedia.org/wiki/Aleutian_Islands
- Galápagos Islands: https://en.wikipedia.org/wiki/Galápagos_Islands
- Hamley et al. 2021, Falklands (Archaeology magazine): https://www.archaeology.org/news/10110-211028-falkland-islands-wolf

Not read: Llamas et al. 2016 (*Science Advances*, Beringian standstill and coastal speed): PMC showed a CAPTCHA and the Europe PMC mirror was rate-limited. Science.org and PNAS.org pages returned 403, so I used open summaries for those papers.
