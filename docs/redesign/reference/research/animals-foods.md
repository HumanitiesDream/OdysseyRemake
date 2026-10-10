# Animals and foods: wild-animal sanity check, and what people ate

Research notes, 5 Oct 2026. Part A reviews engine/src/types.rs, hooks.rs (`wild_ancestors`), tools/bake_types.py and
app/renderer/climate-ui.js (the names). Part B gives diet numbers for foragers and early farmers.

Tags: **[O]** = I opened the paper, its abstract, or a reliable summary of it this session. **[M]** = from memory, not
checked this session. Treat [M] numbers as good to about +-20%.
Climate keys as in the engine: **tc** = coldest-month mean (C), **tw** = warmest-month mean (C), **p** = yearly rain (mm).
Dates are calibrated years ago (ka = thousand years ago) unless marked BCE.

Pages I could not read: PubMed (reCAPTCHA wall) for Cordain et al. 2000 and the PMC copy of Pontzer et al. 2012
(I read the PLOS ONE copy instead). The Europe PMC REST API was refused by the proxy (HTTP 429). Cordain's numbers below are therefore [M].

---

## Part A. Wild animals

### A.0 How the engine handles animals (what I read)

- There are no species. There are **14 climate types** (types.rs:18-23; tools/bake_types.py:32-33): mega_cold, mega_warm,
  graze_cold, graze_temp, graze_warm, browse_cold, browse_warm, small_cold, small_warm, pred_cold, pred_warm, fur_cold,
  fur_warm, croc.
- Each type's limits are the 3rd to 97th percentile of today's climate over the **PHYLACINE present-natural** ranges of
  its species, all realms pooled (bake_types.py:91-98). A species joins the "cold" type if its range's mean tc is
  below a cut-off (bake_types.py:82-88).
- **Realm is used only for the names in the inspector.** It does not affect where a type may live
  (types.rs:32-34; realm assigned in bake_types.py:55-61 and climate-ui.js:196-203). Spread follows land adjacency
  only (types.rs:146-195).
- The inspector names a type by realm (climate-ui.js:179-193). Where the realm has no name for a type, it shows a
  generic name such as "cold-country giants", "cold grazers" or "fur animals" (climate-ui.js:194).

Baked limits (decoded from app/data/eco-types.bin):

| k | type | tc lo..hi | tw lo..hi | p lo..hi | mean kg |
|---|---|---|---|---|---|
| 0 | mega_cold | -34.6..8.5 | 5.9..26.5 | 204..1388 | 3505 |
| 1 | mega_warm | -17.1..25.7 | 14.8..34.7 | 46..2680 | 2664 |
| 2 | graze_cold | -33.9..5.2 | 1.2..26.8 | 112..1330 | 240 |
| 3 | graze_temp | -32.7..21.4 | 7.7..34.2 | 110..1980 | 106 |
| 4 | graze_warm | 6.9..25.6 | 18.4..35.1 | 33..2594 | 121 |
| 5 | browse_cold | -34.5..10.3 | 2.7..28.2 | 152..1526 | 120 |
| 6 | browse_warm | -8.3..26.2 | 16.2..32.2 | 234..3076 | 51 |
| 7 | small_cold | -35.8..7.6 | 1.8..28.0 | 143..1484 | 1.9 |
| 8 | small_warm | -0.8..26.2 | 17.0..33.2 | 167..3157 | 1.8 |
| 9 | pred_cold | -55.3..12.0 | -24.2..28.5 | 63..1567 | 203 |
| 10 | pred_warm | -11.6..25.6 | 11.7..35.3 | 46..2672 | 95 |
| 11 | fur_cold | -34.7..16.5 | 2.5..30.7 | 126..1738 | 5.1 |
| 12 | fur_warm | -6.8..26.2 | 15.5..35.2 | 46..3028 | 4.4 |
| 13 | croc | 14..40 (code uses 5.5) | 22..40 | 250..6000 | 300 |

### A.1 Why cold, furry animals show up in Africa (the owner's report)

The effective limits in types.rs:90-109 make the cold types too tolerant of warmth:

| What | Code | Effect |
|---|---|---|
| Cold types get no lower tc limit and only 35% of the rain floor | types.rs:97, 100-101 | Fine for the Arctic. |
| mega_cold gets +4 C extra room on the warm side | types.rs:94 (`wx = 4`) | mega_cold is fully suited up to **tc 12.5, tw 30.5**, and fades out only at tc 16.5, tw 33.5. |
| **Riparian bonus applies to every type, the cold ones too** | types.rs:89, 106-108 (`hx = wx + 8*rip`) | On a desert river the warm limits rise by up to **8 C**. mega_cold then tolerates **tc 20.5 and tw 38.5**: woolly mammoths can follow the Nile, the Tigris-Euphrates and the Indus. The comment at 87-88 is meant for lions, boar and gazelle, not cold types. |
| Warm-side limits come from today's pooled ranges | bake_types.py:94-96 | pred_cold (wolves, bears) is suited to tc 12 (fade 16). fur_cold to tc 16.5 (fade 20.5). browse_cold to 10.3. These cover the **Ethiopian, East African and South African highlands**, the Andes and Patagonia. |
| No realm barrier on spread | types.rs:146-195 | Once a type reaches Sinai or the Levant in a glacial, it can walk on south along the Nile (riparian bonus) to the highlands. |
| Generic fallback names | climate-ui.js:194 | In realm 1 (Africa), realm 2 (Indomalaya), realm 4 (Neotropics) and realm 5 (Australasia), the cold types show as "cold-country giants", "cold grazers", "cold-forest deer", "hares" and "wolves and bears". fur_cold shows as "jackals" in Africa. |

Real ranges [M unless tagged]: sub-Saharan Africa had **no** woolly mammoth, woolly rhino, reindeer, musk ox, saiga,
arctic fox, moose, wolf (the African golden wolf lives only in North and East Africa), brown bear, lynx, sable or
marten at any time in the last 130 ka. Cold-adapted fauna reached the Iberian Peninsula south to Padul, Granada,
**37.0 N** (mammoth, 42-31 ka). In the LGM it was held to the north of Iberia [O, Álvarez-Lao & García 2011]. North
Africa had its own elephants (*Elephas iolensis*), and no woolly mammoth [M].

### A.2 List of types and names to fix

Each row: what is wrong, where it is, and a fixed envelope. "Holocene" means after ~11.7 ka.

| # | Type / name | Problem | Code | Fixed envelope (tc, tw, p, latitude, realm) | Source |
|---|---|---|---|---|---|
| 1 | mega_cold "woolly mammoths, woolly rhinos" (realm 0), "woolly mammoths, mastodons" (realm 3) | Warm-side limits too wide (+4 C, then +8 C riparian). Can reach the Nile, the Levant coast and the African and Andean highlands. | types.rs:94, 106-108; climate-ui.js:180 | **Mammoth:** tc <= 0 (fade 0 to +4), tw 8-20 (fade 20 to 23), p 150-600 (fade to 900), no riparian bonus, lat >= 37 N in Europe, >= 40 N in Asia and North America, realms 0 and 3 only. **Woolly rhino:** same, but realm 0 only (never in North America), tw <= 19. Mainland mammoths gone by ~9.7 ka (Siberia); a last mainland pocket north of 69 N after 12 ka; Wrangel Island to ~3.7 ka. Woolly rhino gone ~14 ka. | Padul 37 N [O, Álvarez-Lao & García 2011]; Siberia, NE China, 69 N, 9.7 ka, Wrangel 7.7-3.7 ka [O, northern Asia mammoth chronology abstract]; rhino absent from N America, 14 ka [M, Stuart & Lister 2012] |
| 2 | graze_cold "reindeer, musk oxen, saiga" (0), "caribou, musk oxen" (3) | No cold limit is right. Warm side tc 5.2 (fade 9.2), tw 26.8 (fade 29.8), plus riparian +8: reaches Lesotho, the Ethiopian plateau and the Nile. | types.rs:97, 106-108; climate-ui.js:182 | **Reindeer/caribou:** tc <= -5 (fade to 0), tw <= 17 (fade to 20) in the Holocene; in a glacial, tw <= 18. Lat >= 42 N in a glacial (N Spain, S France), >= 55 N in Holocene Europe (they left the Baltic 14.2-11.3 ka), >= 45 N in Holocene North America. Realms 0 and 3. **Musk ox:** tc <= -15, tw <= 12, p 100-400, lat >= 45 N in a glacial, >= 65 N in the Holocene. Gone from Eurasia except Taymyr (~3 ka). **Saiga:** tc -25 to -5, tw 18-28, p 100-400, lat 40-55 N in the Holocene; in the LGM it ranged from England to Yukon. Realm 0, plus 3 in Beringia only. | Baltic reindeer 14,180-11,280 cal BP [O, eastern Baltic study, Lund repository]; musk ox, Taymyr 3 ka [O, Royal BC Museum]; Iberia [O]; rest [M] |
| 3 | browse_cold "moose, red deer" (0), "moose, elk" (3) | tc to 10.3 (fade 14.3). Reaches the African highlands under the generic name "cold-forest deer". | types.rs:97; climate-ui.js:185 | **Moose:** tc <= -5, tw 12-20 (heat stress above ~20), p >= 350, lat 42-70 N, realms 0 and 3. **Red deer / elk:** these are temperate, so they belong in browse_warm. They reach North Africa (the Barbary stag, ~36 N) but not south of the Sahara. | [M] |
| 4 | small_cold "hares, marmots" | tc to 7.6 (fade 11.6), riparian bonus. Mild. | types.rs:97 | tc <= 0, tw <= 22, realms 0 and 3. Mountain hare and marmots also live in the Alps and Pyrenees; elevation shows up as a cold cell anyway. | [M] |
| 5 | pred_cold "wolves, brown bears, cave lions" (0), "wolves, grizzlies, short-faced bears" (3) | tc to 12 (fade 16). Shows as "wolves and bears" in Africa, the Neotropics and Sahul. | types.rs:97; climate-ui.js:189 | **Wolf:** lat >= 12 N (Arabia, India), realms 0, 2 (India north of ~12 N) and 3, plus North and East Africa north of 10 N only as golden wolf; never sub-Saharan Africa, South America or Sahul. **Brown bear:** lat >= 30 N (Atlas ~32 N, Mexico ~25 N historically). **Cave lion:** realms 0 and 3 (Beringia), gone ~14 ka. **Short-faced bear:** gone ~13 ka. | [M] |
| 6 | fur_cold "lynx, foxes, sable, martens" (0); "jackals" (1) | tc to 16.5 (fade 20.5). The widest cold type, suited across most of the African and Andean uplands. Arctic fox is the clearest wrong case. | types.rs:97; climate-ui.js:190 | **Arctic fox:** tw <= 12 (Holocene); in a glacial, south to ~45-50 N in central Europe (Belgium, Germany, SW Russia). The southern populations died out instead of moving north. **Sable, lynx, marten:** tc <= 0, tw <= 22, lat >= 40 N. Realm 1 should show no fur_cold. | Arctic fox [O, Dalén et al. 2007]; rest [M] |
| 7 | mega_warm "straight-tusked elephants, hippos" (0) | Cold limit tc -17.1, then -4 megafauna bonus, then -8 edge, so it fades only at **tc -29**. Hippos and forest elephants would sit in the Kazakh steppe and Siberia. The name never changes with time. | types.rs:94, 107; climate-ui.js:181 | **Hippo:** tc >= +2 (no long freeze), needs permanent water; lat <= 33 N in the Holocene (Nile, Levant rivers). In the Eemian it reached 52-54 N (Britain, Rhine); none in Europe in the last glacial. **Straight-tusked elephant:** interglacial only, tc >= -2, gone from Europe by ~30-40 ka. **Holocene realm-0 name:** "hippos, Syrian elephants" south of 37 N only. The pooled -17 C comes from mammoth-steppe-edge species such as *Elasmotherium*; split them out or limit realm 0 to tc >= -5. | [M] |
| 8 | mega_warm "mastodons, giant ground sloths" (3); "giant sloths, gomphotheres, toxodons" (4); "diprotodons, giant kangaroos" (5) | Names stay after the real extinctions (America ~13-11 ka, Sahul ~45-40 ka) unless the model hunts them out. Extinct Sahul megafauna can be "refilled" by Asian elephants crossing the Wallace line (see #12). | climate-ui.js:181; types.rs:192 | Keep the envelopes. Add an era switch: after the type is `gone`, or after its real extinction date, show the Holocene fauna (realm 3: "bison, elk"; realm 4: "tapirs"; realm 5: none). | [M] |
| 9 | graze_warm "bison, horses, camels" (3) | p up to 2594 (fade to 4670), tc 6.9-25.6: puts "camels" in the wet Gulf Coast, the Yucatán and Central American forest (realm 3 runs south to 13 N). | bake_types.py:57-58; climate-ui.js:184 | ***Camelops*:** p <= 800, tw 20-32, lat 15-60 N, realm 3, gone ~13 ka. **American horses:** p <= 900, gone ~11-13 ka. In wet cells of realm 3 show "deer, peccaries, tapirs". | [M] |
| 10 | graze_warm "horses, llamas, toxodons" (4); graze_temp "guanacos" (4) | "Llamas" are domestic. Wild camelids are the guanaco and the vicuña. *Hippidion* horses died out by ~11 ka. | climate-ui.js:183-184 | **Guanaco:** p 100-1000, tw 8-24, lat 8-55 S, realm 4. **Vicuña:** 3,200-4,800 m (tw 5-12). Neither lives in Amazon rainforest (p > 1500). | [M] |
| 11 | graze_temp "wild horses, antelope" (1) | No wild horses in sub-Saharan Africa. | climate-ui.js:183 | Show "zebras, wildebeest" in realm 1. | [M] |
| 12 | All realm-5 names: "kangaroos", "wallabies", "diprotodons", "thylacines", "marsupial lions", "quolls", "tree kangaroos" | **No realm barrier.** Bali (cell centred 115 E) and Lombok (117 E) sit in adjacent 2-degree cells, as do Borneo and Sulawesi, so Asian types walk across the Wallace line and Sahul types walk out. The Lombok Strait was never dry. | types.rs:155-160 (8-neighbour spread), 177-185 (long jump) | Block spread across the Wallace line (between ~115.5 and 116.5 E, south of 0) and across the Makassar Strait (~118 E). Sahul types: realm 5 only. Asian types: never east of the Wallace line, except people's animals (pig, dog ~3.5 ka). **Tree kangaroos:** p >= 1500, lat > -18, New Guinea and NE Queensland only. Elsewhere show "wallabies, possums". **Thylacine:** gone from the mainland ~3.2 ka, Tasmania 1936. | [M] |
| 13 | Long jump crosses the sea | The random long jump takes any source within **4 cells (~880 km)** with no check that land lies between. It can seed Madagascar (2-cell Mozambique Channel) with lions, zebras and wildebeest; Japan from Korea; Crete, Cyprus, Sardinia and Corsica from the mainland; the Greater Antilles from Yucatán; Sri Lanka; Tasmania at high sea. | types.rs:177-185 | Require a land path (or a strait under ~1 cell for flying or swimming types) for the long jump, or limit it to the type's own land mass (types_islands already labels components, types.rs:121-142). Madagascar's real large fauna: pygmy hippos, giant lemurs, elephant birds, fossa, Nile crocodile; no felids, no bovids, no equids. | [M] |
| 14 | `gone` cleared by any re-entry | An extinct type is marked back as present once pres > 0.3 (types.rs:192), even if the animals came from another realm. | types.rs:164-192 | Keep `gone` set when the source of the re-entry is in another realm. | code |
| 15 | WA_AUROCHS uses graze_temp presence (realm 0) | graze_temp reaches tc -32.7, so "aurochs" reaches Yakutia. | hooks.rs:165 | **Aurochs:** tc >= -15, tw >= 15, lat <= 60 N, p 300-1200. Range: Europe, North Africa north of the Sahara, the Near East, Central Asia, India (*namadicus*) and north China. | [M] |
| 16 | WA_AUROCHS for Africa can never fire | `r == 1 && la > 20` is impossible, because realm 1 is defined as la < 20 (bake_types.py:59). It is harmless only because the Maghreb and Egypt are realm 0. | hooks.rs:167 | Delete it, or change it to `(r == 0 \|\| r == 1) && la > 15 && lo < 35 && p > 150`. | code |
| 17 | WA_HORSE in realm 3 | North American horses died out ~11-13 ka, so there was no wild horse there to tame in the Holocene. | hooks.rs:175 | Realm 0 only after 11 ka. **Holocene wild horse:** steppe and Iberia, tc -25 to +5, p 200-600, lat 38-58 N. | [M] |
| 18 | WA_WOLF in all of realm 2 | Lets wolves into the Sundaland rainforest. | hooks.rs:178 | Add p < 1500 and lat > 10 N. | [M] |
| 19 | WA_CAMEL box | Includes Egypt and Sudan (realm 1 and 0, lon 30-35). The wild dromedary lived in Arabia. The wild Bactrian lived in the Gobi, Mongolia and NW China. | hooks.rs:181 | Dromedary: lat 15-32 N, lon 35-60 E. Bactrian: lat 38-48 N, lon 85-110 E, tc -20 to -5. | [M] |

Fine as they are: croc (types.rs:98-114, 218-230, already clamped by a hand-made range), WA_WHEAT, WA_MILLET,
WA_SHEEPGOAT, WA_LLAMA, WA_FOWL, WA_BUFFALO, WA_DONKEY. The New Guinea realm fix at types.rs:58 is correct.

### A.3 Suggested code shape

1. Add a realm mask per type: `TYPE_REALMS: [u8; NT]`, a bit per realm where the type may live. For example
   mega_cold, graze_cold, browse_cold, small_cold, pred_cold and fur_cold get `{0, 3}`. Multiply `suit` by it in
   types_climate (types.rs:115).
2. Give the cold types a hard warm side: `tc_hi`, `tw_hi` from A.2, with no `wx` and no riparian `hx` (types.rs:94, 106).
3. Block spread and long jumps across the Wallace line and over open sea wider than 1 cell (types.rs:155-185).
4. Make the names depend on the date and on the cell's climate (climate-ui.js:179-197). Remove the generic fallback
   for cold types in realms 1, 2, 4 and 5, so a wrong type cannot hide behind a vague name.

---

## Part B. Foods

### B.1 Energy and protein needs (for code)

| Quantity | Value | Tag / source |
|---|---|---|
| Hadza men, total energy use | ~2,600 kcal/day; physical activity level (PAL) 2.26 | [O] Pontzer et al. 2012 |
| Hadza women | ~1,900 kcal/day; PAL 1.78 | [O] Pontzer et al. 2012 |
| Energy use vs Westerners | Same after body size is controlled for | [O] Pontzer et al. 2012 |
| Hadza share of energy from wild food | > 95% (tubers, berries, game, baobab, honey) | [O] Pontzer et al. 2012 |
| Mean need per head, all ages, preindustrial population | **2,000-2,200 kcal/day** (children pull the mean down) | [M] FAO/WHO/UNU 2004 |
| Extra for pregnancy / breastfeeding | +300 / +500 kcal/day | [M] |
| Weight-stable floor for an active adult | ~1,800 kcal/day. At ~1,600 kcal/day adults lose ~25% of body mass in 6 months (Minnesota experiment). | [M] Keys et al. 1950 |
| Protein need | ~0.8 g/kg/day, so **~45-55 g/day** per adult | [M] |
| Protein ceiling | ~35-40% of energy. Above this, people get "rabbit starvation" (lean meat without fat). So meat-heavy diets need fat. | [M] Speth 1989; Cordain et al. 2000 |
| Fat need in meat-heavy diets | >= 20-30% of energy. Marrow and grease are prized in late winter and spring. | [M] Speth |

**Model rule:** `need = 2100 kcal × head`. `protein_ok = clamp(protein_g / 50, 0, 1)`. Lean-meat energy above 35% of
the total counts for only half.

### B.2 Forager diets

| Measure | Value | Tag / source |
|---|---|---|
| Ethnographic Atlas foragers (229 societies) with > 50% of energy from animal food | ~73% of societies | [M] Cordain et al. 2000 |
| Foragers with > 50% from plants | ~14% of societies | [M] Cordain et al. 2000 |
| Macronutrients (Cordain's estimate) | protein 19-35%, carbohydrate 22-40%, fat 28-58% of energy | [M] Cordain et al. 2000 (the fat range is debated) |
| Tropical foragers, measured (11 groups, 15 studies) | animal food 23-64% by weight (inter-quartile range); plants 35-70%; honey up to 7% (4.5-9.8% in 6 groups) | [O] Lieberman et al. 2023 |
| Tropical groups that fit "Paleo" macronutrient ranges | 2 of 11 | [O] Lieberman et al. 2023 |
| Gathering share vs climate | Gathering is ~50-60% of food in the tropics, ~25-35% in temperate land, < 10% in the Arctic | [M] Marlowe 2005; Binford 2001 (also the basis of the comment at hooks.rs:91-93) |
| Animal share and latitude | Rises with latitude (p < 0.001) | [O] Pontzer & Wood 2021 (via HRAF summary) |
| Inuit / Arctic | 90-99% of energy from animals, mostly fat | [M] |
| NW Coast | ~50-70% from fish (salmon) | [M] |

**Staple plant foods by region** (all [M] unless tagged):

| Region / time | Staples |
|---|---|
| Levant, Natufian, 15-11.7 ka | Wild barley, einkorn and oat; bread at Shubayqa 1, 14.4 ka [O]; acorns, almonds, pistachio, lentils; gazelle |
| Europe, Mesolithic | Hazelnuts (roasted in bulk), acorns, water chestnut, roots, berries; red deer, boar, fish, seal on coasts |
| Japan, Jomon | Acorns (leached), chestnut, walnut, buckeye; salmon; shellfish |
| California | Acorns ~50% of energy in many groups (leached meal) |
| Kalahari (!Kung) | Mongongo nut, a large share of plant energy; tubers |
| East Africa (Hadza) | Tubers (fallback), baobab, berries, honey, meat [O for the food list] |
| Australia, arid | Grass and acacia seeds ground on millstones; yams, bush tomato |
| Amazon | Palm fruit, Brazil nut, fish; later manioc |
| Andes / coast | Coastal anchovy and shellfish; tubers inland |
| Great Basin | Pine nuts, grass seeds |

### B.3 Early farmer and herder diets

| Measure | Value | Tag |
|---|---|---|
| Cereal or tuber share of energy, early farmers | **60-80%** (wheat/barley; rice; millet; maize later 70-80%) | [M] |
| Meat share of energy in farming villages | 5-15% (higher in herding) | [M] |
| Pulses | 5-15% of energy; they supply the lysine that cereals lack | [M] |
| Pastoralists (East Africa, steppe) | Milk 30-60% of energy in the wet season; blood and meat; grain bought | [M] |
| Bread | Before farming: Shubayqa 1 (NE Jordan), Natufian, 14.4 ka; wild barley, einkorn, oat; ≥ 4,000 years before farming | [O] Arranz-Otaegui et al. 2018 (UCL) |
| Porridge / gruel | Raqefet Cave beer-gruel, wheat and barley, 13.7-11.7 ka | [O] Liu et al. 2018 (Stanford) |

### B.4 Dairy and lactase persistence

| Fact | Value | Tag / source |
|---|---|---|
| Milk residues in pottery, earliest | NW Anatolia, 7th millennium BCE (~7000-6500 BCE) | [M] Evershed et al. 2008 |
| Green Sahara dairying | Takarkori, Libya, ~5200 BCE | [M] Dunne et al. 2012 |
| Cheese making, N Europe | Sieves with milk fat, Kuyavia (Poland), 6th millennium BCE (~5200-4800 BCE). Cheese lowers lactose, so people without lactase persistence (LP) could use milk. | [O] Salque et al. 2013 (Bristol) |
| European milk use, large survey | ~7,000 fat residues from 550+ sites over 9,000 years | [O] Evershed et al. 2022 |
| What drove LP | Milk use alone explains it no better than a flat selection since the Neolithic. People without LP suffered more in **famine and disease**. LP is only weakly tied to milk drinking today. | [O] Evershed et al. 2022 |
| LP allele common only late | LP still rare (< 10%) in Europe around 3000 BCE; it rose mainly after ~2000-1000 BCE | [M] Burger et al. 2020; Evershed et al. 2022 |
| -13910*T age | ~2,000-20,000 years (best guess ~7,500-9,000) | [O] Tishkoff et al. 2007 |
| African C-14010 age | ~2,700-6,800 years (East Africa) | [O] Tishkoff et al. 2007 |
| Selection coefficient | 0.035-0.097 a generation, among the highest known in humans | [O] Tishkoff et al. 2007 |
| LP today | N Europe > 90%; S Europe and Arab pastoralists ~50%; Tutsi ~90%; Fulani ~50%; West African farmers 5-20%; China ~1% | [O] Tishkoff et al. 2007 |
| Steppe and Mongolia | Ruminant dairying from ~3000 BCE with almost no LP | [M] Wilkin et al. 2020 |

**Model rule:** dairy energy reaches people as fermented milk, cheese or butter from the start of herding
(about 70% of the milk's energy is kept). Fresh milk energy for adults × LP share. Let the LP share rise with
famine years among milk users (s ≈ 0.02-0.05 a generation).

### B.5 Fermentation, cooking, storage

| Item | Earliest | Tag / source |
|---|---|---|
| Beer (cereal, malted) | Raqefet Cave, Israel, ~13 ka (Natufian) | [O] Liu et al. 2018 |
| Mixed fermented drink (rice, honey, fruit) | Jiahu, China, ~7000-6600 BCE | [M] McGovern et al. 2004 |
| Grape wine | Gadachrili Gora and Shulaveris Gora, Georgia, 6000-5000 BCE; jars of > 300 L | [O] McGovern et al. 2017 (Sci.News) |
| Wine, Zagros | Hajji Firuz, Iran, 5400-5000 BCE | [O] (same) |
| Cheese | Poland, ~5200 BCE (see B.4) | [O] |
| Cooking with fire | Habitual by ~400 ka; possibly ~1 Ma (Wonderwerk) | [M] |
| Pottery for cooking | China, Xianrendong ~20 ka; Japan (Jomon) ~16 ka; Sahara ~9.5 ka; Near East only ~7000 BCE (the early Neolithic there had none) | [M] |
| Methods | Roasting, earth ovens, stone boiling in skins or baskets, then pots (stews, porridge); parching and grinding of seeds; leaching acorns and toxic tubers | [M] |
| Cooking gain | Cooking raises the net energy from starch and meat by roughly 10-30% | [M] Carmody & Wrangham |
| Granaries | Dhra', Jordan, ~11.3 ka (PPNA), before domestic grain | [M] Kuijt & Finlayson 2009 |
| Storage losses | Grain in pits or jars loses ~10-30% a year (pests, mould) | [M] |
| Preservation | Drying and smoking (meat, fish), freezing (Arctic caches), fat-and-meat mixes (pemmican), salting (later), fermenting fish, nut storage | [M] |

### B.6 Famine foods

| Food | Where | Notes [M] |
|---|---|---|
| Inner bark (phloem) of pine | Scandinavia, Sami, N America | Bark bread; a few hundred kcal/day at most; much fibre |
| Acorns, beech mast, horse chestnut | Temperate Eurasia, Japan, California | Need leaching; a normal food for some, a famine food for farmers |
| Lichen (Iceland moss), seaweed | North Atlantic | Low energy |
| Fern rhizomes, wild roots | Widespread | Some are toxic |
| Wild grass seeds, weed seeds | Steppe, Sahel | Hard work to gather |
| Cycad, bitter cassava, wild yams | Tropics | Toxic without processing |
| Blood from live cattle, slaughter of breeding stock | Herders | Eats tomorrow's herd |

This matches WILD_FOOD_TRAITS row 3 in hooks.rs:34 (keeps 0.6, work 0.9). Suggestion: famine food keeps people alive
but gives **no protein credit and ~50% micronutrient credit**.

### B.7 Deficiencies

| Deficiency | Who gets it | Onset and effect | Tag |
|---|---|---|---|
| Scurvy (vitamin C) | Long winters on stored food without fresh plants or raw organ meat; sieges; sailors | Signs after 1-3 months without vitamin C; death after ~3-6 months. Arctic foragers avoid it with raw liver, skin and fish. | [M] |
| Pellagra (niacin) | Near-maize-only diets of the poor in Europe, the US and Africa, 18th to mid-20th century | Millions ill, hundreds of thousands dead | [O] SAJS review (authors not checked) |
| Nixtamalisation (lime cooking of maize) | Mesoamerica | Frees niacin. **But** the SAJS review argues it was not the main reason pre-Columbian America lacked pellagra. A mixed diet (beans, squash, chili) was. | [O] SAJS review |
| Iodine (goitre, cretinism) | Inland mountains and glaciated land far from the sea: Alps, Himalaya, Andes, the Great Lakes, central Africa | Very common in the Swiss Alps until iodised salt (1922). Cretinism lowers mental capacity and work. Infertility and stillbirth rise. | [O] Environment & Society (Swiss study); rest [M] |
| Iron (anaemia) | Farmers on cereals, high parasite load, weaning stress | Porotic hyperostosis and cribra orbitalia become more common with farming | [M] Larsen |
| Vitamin D (rickets) | High latitude, dark winters, cereal diets, children kept indoors | Bent bones; narrow pelvis (death in childbirth) | [M] |
| Protein / lysine | Cereal-only diets without pulses or animals | Stunting; kwashiorkor in weaned children | [M] |
| Dental caries | Starchy farming diets | Foragers ~1-2% of teeth; farmers ~5-10% (maize higher) | [M] Turner 1979 |

**Model rule:** a "diet diversity" index 0-1 = 1 − largest single staple's share of energy. Below ~0.25 (more than 75%
from one crop), add deficiency disease: maize-only gives pellagra, polished rice gives beriberi (historic only), and a
winter with no fresh food for over 3 months gives scurvy. Iodine lack sets a flat penalty on inland mountain and
glaciated-till cells far (> 300 km) from the sea, until salt trade.

### B.8 Food, stature, fertility and death

| Effect | Size | Tag / source |
|---|---|---|
| Stature fell as agriculture took over | In most studies, worldwide (Europe, Africa, Middle East, Asia, the Americas). Causes: seasonal hunger, single crops lacking nutrients, crop failure, inequality, dense settlement, infection. | [O] Mummert et al. 2011 |
| Size of the fall | ~3-10 cm in adult men (e.g. Europe ~170 to ~163 cm; the Levant ~ -5 cm) | [M] |
| Health markers | 19 of 21 societies got worse health indicators with agriculture | [O] Cohen & Armelagos 1984, cited in Mummert et al. 2011 |
| Forager fertility | Total fertility rate (TFR) ~5-6; birth spacing 3-4 years (long breastfeeding, low energy surplus) | [M] |
| Farmer fertility | TFR ~6-8; birth spacing 2.5-3 years. This is the "Neolithic demographic transition": +~0.5-1 births per woman early on. | [M] Bocquet-Appel 2011 |
| Energy balance and ovulation | Negative energy balance and heavy work suppress ovulation. Births fall in hungry seasons. | [M] Ellison 2001 |
| Famine | Births in a famine year fall ~20-50%. Deaths roughly double to triple in a bad famine year (the young and old first; also from disease). | [M] |
| Child mortality | ~40-50% die before 15 in both foragers and early farmers | [M] Volk & Atkinson 2013 |

**Suggested code (per group, per year):**

```
adequacy  = clamp(kcal_available / (2100 * pop), 0, 1.3)
protein_q = clamp(protein_g_per_head / 50, 0, 1)
diversity = 1 - max_staple_share                          // 0..1
quality   = 0.5*min(adequacy,1) + 0.25*protein_q + 0.25*smooth(0.1, 0.4, diversity)

fertility_mult = 0.6 + 0.5*smooth(0.7, 1.05, adequacy)    // sedentism adds +0.1..0.15 on top
death_mult     = 1 + 2.0*smooth(1.0, 0.6, adequacy)       // famine: up to x3
               + 0.3*(1 - quality) + deficiency_penalties
stature_cm     = base_cm - 8*(1 - quality_mean_childhood) // ~0..8 cm range
```

---

## Sources (opened this session)

- Álvarez-Lao, D.J. & García, N. 2011. Geographical distribution of Pleistocene cold-adapted large mammal faunas in the Iberian Peninsula. https://docta.ucm.es/entities/publication/6a077c53-cb0e-4fe4-a278-1415ba9080d0
- Northern Asia mammoth chronology (abstract; author likely Kuzmin [M]) (on woolly mammoth chronology in northern Asia). https://repository.geologyscience.ru/handle/123456789/36628
- Eastern Baltic reindeer study (authors not checked; Ukkonen et al. [M]) (14,180-11,280 cal BP). https://lup.lub.lu.se/record/408429
- Dalén, L. et al. 2007. Ancient DNA reveals lack of postglacial habitat tracking in the arctic fox. PNAS. https://www.zin.ru/labs/theriology/staff/sablin/references/dalen_et_al._2007.pdf
- Royal BC Museum, Muskox in British Columbia (Keddie). https://royalbcmuseum.bc.ca/assets/Muskox-Keddie-05-06-2016.pdf
- Lieberman, D. et al. 2023. Comparing measured dietary variation ... to the Paleo Diet. Am J Clin Nutr. https://nme.fas.harvard.edu/sites/projects.iq.harvard.edu/files/carmodylab/files/lieberman_et_al_2023_am_j_clin_nutr.pdf
- Pontzer, H. et al. 2012. Hunter-gatherer energetics and human obesity. PLOS ONE. https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0040503
- HRAF summary of Pontzer & Wood 2021 (Ann Rev Nutr 41:363). https://hraf.yale.edu/hypotheses/4776
- Evershed, R.P. et al. 2022. Dairying, diseases and the evolution of lactase persistence in Europe. Nature. https://www.nature.com/articles/s41586-022-05010-7
- Tishkoff, S. et al. 2007. Convergent adaptation of human lactase persistence in Africa and Europe. Nat Genet. https://pmc.ncbi.nlm.nih.gov/articles/PMC2672153
- University of Bristol 2012, on Salque et al. 2013 (cheese, Kuyavia). https://www.bristol.ac.uk/news/2012/9008.html
- Sci.News 2017, on McGovern et al. 2017 PNAS (Georgian wine). https://www.sci.news/archaeology/worlds-oldest-wine-georgia-05427.html
- UCL 2018, on Arranz-Otaegui et al. 2018 PNAS (bread, Shubayqa 1). https://www.ucl.ac.uk/news/2018/jul/bread-predates-agriculture-4000-years
- Stanford 2018, on Liu et al. 2018 (Raqefet beer). https://news.stanford.edu/stories/2018/09/crafting-beer-lead-cereal-cultivation
- Mummert, A. et al. 2011. Stature and robusticity during the agricultural transition. Econ Hum Biol 9:284. https://ideas.repec.org/a/eee/ehbiol/v9y2011i3p284-301.html
- Aetiological doctrines and prevalence of pellagra: 18th century to middle 20th century. S Afr J Sci. https://sajs.co.za/article/view/4597
- Hypothyroidism in Switzerland (Environment & Society Portal). https://www.environmentandsociety.org/node/6152

Not opened (cited from memory): Cordain et al. 2000 (AJCN 71:682; PubMed blocked), Marlowe 2005, Binford 2001,
Speth 1989, Evershed et al. 2008, Dunne et al. 2012, Burger et al. 2020, Wilkin et al. 2020, McGovern et al. 2004,
Kuijt & Finlayson 2009, Stuart & Lister 2012, Bocquet-Appel 2011, Ellison 2001, Keys et al. 1950, Turner 1979,
Volk & Atkinson 2013, FAO/WHO/UNU 2004.
