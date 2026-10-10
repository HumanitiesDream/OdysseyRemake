# Neolithic: where farming began, wild ancestors, spread, and the road to chiefs

Research notes for the Neolithic build (TODO "Neolithic: tameable plants and animals, food control dial ... chiefs").
5 Oct 2026. Fits engine/src/hooks.rs `wild_ancestors()` (WA_* bits on 2-degree cells) and types.rs (climate limits as
coldest month tc, warmest month tw, yearly rain p).

Dates are calibrated. BP = before 1950. BCE = BP minus 1950. Rounded to 50 years.
Tags: **[O]** = I opened the paper, its abstract, or a reliable summary of it this session. **[M]** = from memory, not
checked this session. Treat [M] numbers as good to about +-20%.

Boxes are lat/lon in degrees (south and west negative). They are the core area, not the whole later range.

---

## 0. Summary for the model

- Cultivation of wild plants comes first. Visible domestication follows **1,000-3,000 years later** in most centres
  (Fuller et al. 2014). So the model needs two stages: "tending wild stands" (no genetic change, can be dropped) and
  "domestic crop" (locked in; cannot go back to the wild easily).
- Non-shattering in wheat and barley took **2,000-2,500 years** to become fixed (Fuller et al. 2014). Rice took longer:
  ~8.7% non-shattering at 9000-8400 BP, ~58% still shattering at 8000-7700 BP, mostly domestic by 6300 BP (Zheng et al. 2016).
- Settled villages often come **before** domestic crops (Natufian, Shangshan, Ñanchoc, Late Archaic ENA). In New Guinea and
  West Africa, cultivation came before full settling.
- Only 9-11 independent centres. Everywhere else, farming arrives by spread. The engine should let a centre start only
  where its wild ancestors live **and** a settled or semi-settled people already uses them (meso.rs wild grain etc.).
- Existing WA bits cover most centres. Missing: Ethiopia (teff, enset), South India (mung, horsegram, small millets),
  Mesoamerican turkey, donkey, zebu (separate from taurine), guinea pig. Suggested new bits are in section 2.4.

---

## 1. Independent centres

### 1.1 Data table

| # | Centre | Box (lat, lon) | First cultivation | First morphological domestication | Main crops | Main animals | Climate and soil | Sources |
|---|---|---|---|---|---|---|---|---|
| 1 | Fertile Crescent (core: upper Tigris-Euphrates, Zagros, S Levant) | 30-39 N, 34-48 E; core 36-39 N, 37-44 E | Natufian wild-cereal use ~13,500 BP (11,550 BCE); PPNA cultivation from ~11,500 BP (9,550 BCE) | Einkorn, emmer 10,600-9,900 BP (8,650-7,950 BCE), Çayönü, Cafer Höyük. Barley, pea 10,200-9,550 BP (Tell Aswad). Lentil 10,400-9,450 BP. Non-shattering fixed ~2,000-2,500 years after start | Einkorn, emmer, barley, lentil, pea, chickpea, bitter vetch, flax | Goat (managed by 9,900 BP / 7,950 BCE at Ganj Dareh; "male sink" culling 12,000-11,500 BP), sheep (~10,500 BP / 8,550 BCE, Nevali Çori), cattle (11,000-10,000 BP / 9,050-8,050 BCE, Euphrates), pig (by 10,300 BP / 8,350 BCE, Cafer Höyük) | Winter rain 200-1,000 mm, hot dry summer, mild winter; oak park-woodland and steppe edge, 300-1,600 m; rendzina and terra rossa on limestone, alluvial fans | Fuller et al. 2014 [O]; Weiss & Zohary 2011 [O]; Zeder 2011 [O] |
| 2a | North China (Yellow River millets) | 33-42 N, 105-120 E (Loess Plateau, North China Plain, West Liao) | Millet use by ~9,500 BP (7,550 BCE); clear cultivation by ~8,000 BP (6,050 BCE) | Charred domestic millets: Peiligang 6500-6000 BCE; Cishan late 7th-early 6th millennium BCE; Xinglongwa 6200-5400 BCE; Dadiwan 6000-5400 BCE | Broomcorn millet, foxtail millet; later soybean [M] | Pig, dog [M]; (chicken later, see 2.2) | Summer monsoon 300-900 mm; cold dry winters (Jan -10 to 0 C) [M]; loess soils, light and easy to dig | Fuller et al. 2014 [O]; Stevens & Fuller 2017 [O] |
| 2b | Yangtze (rice) | 28-33 N, 110-122 E (lower and middle Yangtze, Huai) | Shangshan culture ~7000 BCE; Jiahu I (Huai) 6900-5800 BCE | Huxi (Shangshan) 9000-8400 BP: 8.7% non-shattering; Kuahuqiao 8000-7700 BP: ~58% still shattering; Tianluoshan 7000-6500 BP; mostly domestic by Majiabang 6300-6000 BP (4350-4050 BCE) | Japonica rice; water chestnut, foxnut, acorns as side foods [M] | Pig, dog [M]; later water buffalo (swamp type) [M] | Hot wet summer, mild winter (Jan > ~2 C) [M], > 1,000 mm; marsh edges, lake shores, later paddies | Zheng et al. 2016 [O]; Stevens & Fuller 2017 [O] |
| 3 | New Guinea highlands | 5-7 S, 142-147 E; 1,200-2,800 m | Kuk Swamp (1,560 m) Phase 1 10,000-7,000 BP (8,050-5,050 BCE): pits, runnels, staking (debated) | No clear morphological marker. Phase 2 7,000-6,400 BP (5,050-4,450 BCE): banana and yam on mounds, taro on wet edges. Phase 3 4,350-3,980 BP: drainage ditches | Taro, yams, bananas (Australimusa and Musa acuminata), sugarcane [M] | None native. Pig, dog, chicken arrive late, ~3,500-3,000 BP [M] | Wet tropical montane, 2,000-4,000 mm, no dry season, frost-free below ~2,200 m [M]; swamp margins | Denham et al. 2003 (via UNESCO dossier [O]); Fuller et al. 2014 [O] |
| 4 | Mesoamerica | Balsas core 17-20 N, -103 to -98; wider 14-22 N, -105 to -90 | Squash (C. pepo) ~10,000 BP at Guilá Naquitz [M] | Maize 8,700 BP (6,750 BCE), Xihuatoxtla, central Balsas. Slash-and-burn landscape from 7,600 BP. Villages only ~1900-1500 BCE [M] | Maize, squash, common bean, chili, avocado [M] | Turkey (~300 BCE-100 CE) [M], dog; Muscovy duck in S America [M] | Seasonal tropical forest, 400-1,800 m; summer rain 600-1,500 mm, dry winter | Piperno 2011 [O]; Fuller et al. 2014 [O] |
| 5a | Northern S America lowlands and SW Amazonia | Llanos de Moxos 12-16 S, -67 to -63; NW S America 10 S-10 N, -80 to -70 | Forest islands built from 10,850 BP; squash 10,250 BP, manioc 10,350 BP (Moxos); C. moschata ~10,200 BP in N S America; arrowroot 9,320 BP (Las Vegas, Ecuador) | Manioc reaches Panama by 7,600 BP; maize arrives Moxos 6,850 BP | Manioc, squash (C. moschata), arrowroot, peanut, cotton, chili, sweet potato [M] | Muscovy duck [M] | Seasonally flooded savanna and seasonal forest; 1,000-2,000 mm with a dry season | Lombardo et al. 2020 [O]; Piperno 2011 [O] |
| 5b | Andes (western slopes and puna) | 3-22 S, -80 to -65 | Zaña / Ñanchoc valley (N Peru, ~500 m): beans 8,800-7,700 BP, peanut ~8,500 BP, manioc ~8,500 BP | Cotton ~6,300 BP (Zaña). Alpaca 6,000-5,500 BP, Telarmachay, Junín puna; llama ~4,000 BP. Potato and quinoa ~5000-3000 BCE [M] | Potato, quinoa, oca, ulluco (highland); beans, peanut, cotton, squash (lowland and valley) | Alpaca, llama, guinea pig [M], Muscovy duck | Puna 3,500-4,800 m: cold nights, frost most months, 400-800 mm summer rain; dry coastal valleys need irrigation | Piperno 2011 [O]; Senri Ethnological Studies 112, ch. 13 (Minpaku) [O]; Spooner et al. 2005 PNAS [M] |
| 6 | Eastern North America | 33-42 N, -95 to -82 (Illinois, Wabash, Duck rivers) | Late Archaic settled fisher-gatherers ~5,000 BP | Squash (C. pepo) 5,025 BP (3,075 BCE); sunflower 4,840 BP (2,890 BCE); marshelder (Iva) 4,400 BP (2,450 BCE); chenopod 3,800-3,700 BP (1,850 BCE). Maize arrives ~200 BCE; dominant only after ~900 CE [M] | Squash, sunflower, marshelder, chenopod; later knotweed, maygrass, little barley | Dog only | Oak-hickory forest and oak savanna; 800-1,300 mm, cold winters (Jan -5 to 5 C); small tributary floodplains | Smith 2011 [O]; Fuller et al. 2014 [O] |
| 7 | Sahel / West Africa | Pearl millet core 14-20 N, -10 to 5 E (Tilemsi valley, N Mali ~16-19 N, 0-1 E). Yams and African rice farther south 6-14 N | Wild Pennisetum use ~5000 BCE (AZ22, N Mali); cultivation hypothesised ~4000 BCE | Pearl millet domestication under way 3020-1940 BCE (MK36); non-shattering fixed ~2000 BCE. African rice inner Niger delta ~1500 BCE-0 [M]; yam, cowpea, oil palm undated [M] | Pearl millet, African rice, cowpea, Guinea yam, fonio, oil palm | Cattle first appear in the Sahara by ~6000 BCE (domestic status of earlier Nabta finds disputed) [M]; donkey domesticated in Africa ~5000 BCE (Todd et al. 2022) | Sahel 250-700 mm summer rain, hot, long dry season; drying Sahara pushed herders south | Fuller et al. 2021 [O]; Fuller et al. 2014 [O]; Todd et al. 2022 (via phys.org [O]) |
| 7b | Eastern Sudan (sorghum) | 13-17 N, 34-37 E (Butana, near Kassala) | Intensive cultivation of wild sorghum, Butana Group, 4th millennium BCE | Domestic-type sorghum spikelets at KG23, 3500-3000 BCE | Sorghum | Cattle, sheep, goat (introduced) | Savanna, 300-600 mm summer rain | Winchell et al. 2017, Current Anthropology (via Sci.News [O]) |
| 8 | Ethiopian highlands | 6-15 N, 35-42 E; 1,500-3,000 m | Poorly dated; indigenous crops not found early in the record | Wheat and barley arrive from ~1600 BCE; teff, finger millet, enset, noog thought older but little direct evidence | Teff, finger millet, enset (south), noog, Ethiopian mustard, coffee (wild) | None native; cattle, sheep, goat introduced | Two rainy seasons, 800-2,000 mm; cool (tw 18-24 C); vertisols | Beldados 2025, J. Ethiopian Studies [O] |
| 9 | South India (south Deccan) | 13-17 N, 75-80 E (Karnataka, Andhra; north of Kurnool) | Southern Neolithic begins first half of 3rd millennium BCE (~2800 BCE) | Mungbean, horsegram, browntop millet, bristly foxtail domesticated locally; wheat and barley arrive 1900-1800 BCE | Mungbean, horsegram, browntop millet, bristly foxtail millet | Zebu cattle dominant (ashmounds); whether local or brought in is unclear | Semi-arid, 500-800 mm monsoon rain [M]; drought 2300-1900 BCE | Boivin, Fuller, Korisettar & Petraglia 2006, Prāgdhāra 18 [O] |

Also worth a row in the engine, though not a classic "independent" centre:
- **Indus borderlands (Mehrgarh, 27-30 N, 66-69 E):** barley and wheat from ~7000 BCE; zebu cattle domesticated
  locally from wild Indian aurochs ~6500-5500 BCE [M].
- **Ganges plain:** local rice cultivation (Lahuradewa) perhaps ~5000-3000 BCE; hybrid with japonica after ~2000 BCE [M].
- **Highland New Guinea and the Bismarcks** feed banana and taro to Island SE Asia and the Pacific [M].

### 1.2 Pace of domestication (useful rates)

| Quantity | Value | Source |
|---|---|---|
| Time from start of cultivation to fixed non-shattering (wheat, barley) | 2,000-2,500 years | Fuller et al. 2014 [O] |
| Grain size increase over domestication | most seeds +20-60% (peas +19%, pearl millet +76%) | Fuller et al. 2014 [O] |
| Rice: shattering share | 8.7% non-shattering (9000-8400 BP); ~58% still shattering (8000-7700 BP); mostly domestic by 6300 BP | Zheng et al. 2016 [O] |
| Pearl millet grain width over wild | +28% (4th mill. BCE), +38% (3rd mill. BCE) | Fuller et al. 2021 [O] |
| Island acceleration (Cyprus einkorn) | ~10x faster than mainland | Fuller et al. 2014 [O] |
| ENA: cultivation to domestication | ~500-1,000 years | Fuller et al. 2014 [O] |
| Maize: domestic forms only, no teosinte, at the first site | by 8,700 BP | Piperno 2011 [O] |

Suggested model: a people "tending" a wild ancestor builds a 0-1 domestication score at ~1/2,000 a real year while
it keeps tending (faster, ~1/500, for small isolated groups and islands). At ~0.5 the crop yields more and can be carried
outside the wild range. At 1.0 it is fully domestic. Animals: start with herd management (culling young males),
morphological change ~500-1,500 years later [M, from Zeder 2011's sequence].

---

## 2. Wild ancestors and their ranges

Ranges are for the early Holocene (~9000-6000 BCE). Climate envelopes are rough; use them like types.rs `lim`
(coldest month tc, warmest month tw, rain p, height z). Most envelopes are [M] from general botany and zoology.

### 2.1 Plants

| Crop | Wild ancestor | Early Holocene range (box or region) | Climate envelope (rough) | Engine bit | Source |
|---|---|---|---|---|---|
| Einkorn | Triticum monococcum ssp. baeoticum | W Asia; centre N Syria, S Turkey, N Iraq, W Iran, W Anatolia: 33-40 N, 28-48 E | tc -2 to 8 C, tw 22-30 C, p 300-800 mm winter rain; oak park-forest, steppe; to ~1,700 m | WA_WHEAT | Weiss & Zohary 2011 [O]; envelope [M] |
| Emmer | T. turgidum ssp. dicoccoides | Narrower: Israel, Jordan, SW Syria, Lebanon, SE Turkey, N Iraq, W Iran: 31-38 N, 35-47 E | tc 2-10 C, tw 24-32 C, p 300-800 mm; basalt and limestone slopes | WA_WHEAT | Weiss & Zohary 2011 [O] |
| Barley | Hordeum vulgare ssp. spontaneum | E Mediterranean and W Asia, wider than wheat; also N Africa, to Central Asia: 28-42 N, 20-70 E | tc 0-12 C, tw 22-34 C, p 150-900 mm; open herbaceous and oak belt; tolerates drier land than wheat | WA_WHEAT (wider) | Weiss & Zohary 2011 [O] |
| Lentil | Lens orientalis | E Mediterranean, SW and Central Asia | Shallow stony soils, gravel slopes, 1,200-1,600 m | WA_WHEAT | Weiss & Zohary 2011 [O] |
| Pea | Pisum sativum var. pumilio | Fertile Crescent deciduous oak belt | Steppe-like herbaceous openings | WA_WHEAT | Weiss & Zohary 2011 [O] |
| Chickpea | Cicer reticulatum | Only SE Turkey: ~37-38.5 N, 38-41 E | Limestone slopes, ~600-1,700 m [M] | WA_WHEAT (narrow) | Weiss & Zohary 2011 [O] |
| Flax | Linum bienne | W Europe, Mediterranean, N Africa, SW Asia, Iran, Caucasus | Wet places: moist grass, springs, clay, marsh | WA_WHEAT (much wider) | Weiss & Zohary 2011 [O] |
| Rice (japonica) | Oryza rufipogon | S China to India and SE Asia, wetlands; in the early Holocene its northern edge reached the Yangtze and Huai (~33 N) | tc > ~4 C, tw > 25 C, p > 1,000 mm; standing shallow water | WA_RICE | [M] (Fuller and colleagues' reviews) |
| Rice (indica, aus) | O. rufipogon / O. nivara | Ganges, Bengal, Indochina | as above; nivara tolerates seasonal drying | WA_RICE | [M] |
| African rice | O. barthii | Inner Niger delta and Sahel floodplains, 10-16 N, -10 to 0 | flood-recession; 500-1,500 mm | (new) | [M] |
| Foxtail millet | Setaria viridis | Very wide, temperate Eurasia; domestication in N China | tw > 20 C, p 300-900 mm, summer rain | WA_MILLET | Stevens & Fuller 2017 [O] for place; range [M] |
| Broomcorn millet | wild Panicum miliaceum (ssp. ruderale), ancestor uncertain | N China, maybe also Central Asia | short (60-90 day) season, drought tolerant, needs summer warmth tw > 18 C | WA_MILLET | [M] |
| Maize | Teosinte, Zea mays ssp. parviglumis | Balsas basin, Guerrero and Michoacán: 17-20 N, -103 to -98 | 400-1,800 m, seasonal tropical forest, tc > 14 C, 600-1,500 mm summer rain; short-day plant | WA_MAIZE | Piperno 2011 [O] ("lower, warmer, moister habitats"); altitude [M] |
| (Highland teosinte) | Z. mays ssp. mexicana | Central Mexican plateau, 1,600-2,700 m | cooler; gene flow gave highland maize | WA_MAIZE | [M] |
| Potato | Solanum candolleanum (S. brevicaule complex) | S Peru and N Bolivia, near Lake Titicaca: 13-18 S, -73 to -67 | 3,000-4,000 m; tw 10-16 C; frost tolerant to ~-3 C; 500-1,000 mm | WA_POTATO | Spooner et al. 2005 [M] |
| Quinoa | Chenopodium hircinum (with others) | Andes and lowlands; domestication around Titicaca | to 4,000 m, frost and salt tolerant | WA_POTATO | [M] |
| Squash | Cucurbita pepo (Mexico; also ssp. ovifera in E USA), C. moschata (NW S America lowlands), C. maxima from C. andreana (Argentina, Bolivia) | Three separate wild ranges | warm season, frost-free; riverbanks, disturbed ground | WA_MAIZE, WA_EASTNA | Piperno 2011, Smith 2011 [O]; ranges [M] |
| Common bean | Phaseolus vulgaris (wild) | Two pools: W Mexico (Jalisco, Oaxaca), and Andes (S Peru to N Argentina) | 500-2,000 m, seasonal | WA_MAIZE, WA_POTATO | [M] |
| Sorghum | S. bicolor ssp. verticilliflorum | Wide in African savanna; domestication E Sudan | tc > 15 C, tw > 25 C, p 300-1,000 mm summer rain | WA_SORGHUM | Winchell et al. 2017 [O] for place; range [M] |
| Pearl millet | Pennisetum glaucum ssp. violaceum (monodii) | Saharan-Sahel fringe, Mauritania to Mali: 14-20 N, -15 to 5 | the driest grain: 200-600 mm, tw 30-36 C | WA_SORGHUM | Fuller et al. 2021 [O]; range [M] |
| Guinea yam | Dioscorea praehensilis, D. abyssinica | Forest-savanna edge, W Africa: 6-12 N, -10 to 10 E | 1,000-1,800 mm with a dry season, tc > 20 C | WA_YAM | [M] |
| Greater yam | D. alata (no wild form known) | SE Asia, New Guinea | wet tropics | WA_YAM | [M] |
| Taro | Colocasia esculenta (wild) | NE India to S China, SE Asia, New Guinea, N Australia | wet ground, tc > 15 C, p > 1,500 mm | WA_YAM | [M] |
| Banana | Musa acuminata (Malesia, New Guinea), M. balbisiana (S China, India) | 10 S-25 N, 75-155 E | tc > 15 C, p > 1,500 mm, no frost | WA_YAM | Denham et al. 2003 (via UNESCO [O]) for New Guinea; ranges [M] |
| ENA crops | Helianthus annuus, Iva annua, Chenopodium berlandieri | Mississippi basin floodplains, oak-hickory zone | tc -5 to 5 C, tw 24-28 C, p 800-1,300 mm | WA_EASTNA | Smith 2011 [O] |
| Teff | Eragrostis pilosa | Ethiopian highlands | 1,500-2,800 m | (new) | [M] |
| Enset | Ensete ventricosum (wild) | SW Ethiopia highlands | 1,500-3,000 m, 1,100-1,500 mm | (new) | [M] |
| Mungbean, horsegram | Vigna radiata var. sublobata; Macrotyloma uniflorum (wild) | Peninsular India; Eastern and Western Ghats margins | semi-arid to sub-humid, 500-1,200 mm | (new) | Boivin et al. 2006 [O] for place |

### 2.2 Animals

| Domesticate | Wild ancestor | Early Holocene range | Climate and terrain | Domesticated where and when | Engine bit | Source |
|---|---|---|---|---|---|---|
| Sheep | Asiatic mouflon, Ovis orientalis (gmelini) | Taurus, Zagros, E Anatolia, Caucasus foothills: 32-41 N, 35-55 E | Hill steppe and open woodland, 400-2,500 m; tc -8 to 8 C | Upper Euphrates, ~10,500 BP (8,550 BCE) | WA_SHEEPGOAT | Zeder 2011 [O]; range [M] |
| Goat | Bezoar, Capra aegagrus | Zagros, Taurus, Caucasus, to Afghanistan, Crete: 28-42 N, 25-70 E | Rocky slopes, to ~4,000 m | Central Zagros, managed by 9,900 BP (7,950 BCE) | WA_SHEEPGOAT | Zeder 2011 [O]; range [M] |
| Taurine cattle | Aurochs, Bos primigenius primigenius | All of Europe to Central Asia, N Africa, the Near East; forest edges and floodplains | tc -15 to 12 C, p 300-1,500 mm; wet meadows, river woods | Upper and middle Euphrates, 11,000-10,000 BP (9,050-8,050 BCE) | WA_AUROCHS | Zeder 2011 [O] |
| Zebu cattle | Indian aurochs, B. p. namadicus | Indian subcontinent | warm, monsoon; tc > 10 C | Indus borderlands (Mehrgarh), ~6500-5500 BCE | (new: split WA_AUROCHS by realm 2) | [M] |
| African aurochs | B. p. africanus | N Africa | | Probably added genes to African cattle, not a separate domestication [M] | (WA_AUROCHS in realm 1 north of 20 N) | [M] |
| Pig | Wild boar, Sus scrofa | Eurasia and N Africa except deserts, tundra and deep taiga; to ~50-55 N | tc > ~-15 C (snow depth limits it); forest, marsh | Anatolia, by 10,300 BP (8,350 BCE); separately China (~6500 BCE) [M] | WA_BOAR | Zeder 2011 [O]; China [M] |
| Water buffalo | Wild Asian buffalo, Bubalus arnee | S and SE Asia, also S China in the early Holocene [M]: 5-30 N, 70-120 E | swamps and floodplains; tc > 12 C, p > 1,000 mm | River type: Indus/India ~3000-2500 BCE [M]. Swamp type: China/SE Asia ~2000 BCE [M] | WA_BUFFALO | [M] |
| Horse | Wild horse, Equus ferus | Steppes and open land across Eurasia; separate clusters in Europe, Central Asia, Siberia before 3000 BCE | Dry grassland, p < 600 mm, cold winters fine | Modern lineage (DOM2): lower Volga-Don, ~2200 BCE; spread across Eurasia from ~2000 BCE. Botai (~3500 BCE) was a separate, dead-end lineage | WA_HORSE | Librado et al. 2021 [O] |
| Donkey | African wild ass, Equus africanus | NE Africa: Nubia, Red Sea hills, Somalia: 8-24 N, 30-50 E | Arid hills, p 100-300 mm | Africa ~5000 BCE; reached Eurasia ~2,500 years later | (new) | Todd et al. 2022 (via phys.org [O]) |
| Dromedary | Wild dromedary (extinct) | SE Arabia (and probably all Arabia) | Hot desert, p < 200 mm | SE Arabia; ~3,000 years of use as a domestic animal; wild animals mixed in after domestication | WA_CAMEL (Arabia part) | Almathen et al. 2016 (via ILRI [O]) |
| Bactrian camel | Extinct wild Bactrian (not the living wild Camelus ferus, a separate lineage) | Central Asian deserts and steppe: 35-48 N, 55-110 E | Cold desert, tc to -20 C, p < 250 mm | Central Asia / Iran ~3000-2500 BCE | WA_CAMEL (Asia part) | [M] |
| Llama | Guanaco, Lama guanicoe | Andes and Patagonia, sea level to 4,500 m: 8-55 S | Arid to semi-arid grass and shrub | Central Andes ~4,000 BP; Bolivia ~3,500 BP; NW Argentina and N Chile 3,100-2,400 BP | WA_LLAMA | Minpaku SES 112 ch. 13 [O] |
| Alpaca | Vicuña, Vicugna vicugna | Puna, 3,500-4,800 m: 9-29 S | Cold, high grassland near wet meadows | Telarmachay, Junín, 6,000-5,500 BP (4,050-3,550 BCE) | WA_LLAMA | Minpaku SES 112 ch. 13 [O] |
| Guinea pig | Cavia tschudii | Andes, 2,000-4,000 m | | ~5000-2500 BCE [M] | (new) | [M] |
| Chicken | Red junglefowl, Gallus gallus (mainly ssp. spadiceus) | Forest edge in S China, SE Asia, NE India: 10-28 N, 80-115 E | tc > 10 C, p > 1,000 mm; bamboo and forest edge | Central Thailand, Ban Non Wat, ~1650-1250 BCE, drawn in by stored rice and millet; China, S Asia, Mesopotamia late 2nd mill. BCE; Ethiopia and Mediterranean ~800 BCE | WA_FOWL | Peters et al. 2022 (via LMU record [O]); subspecies [M] |
| Turkey | Wild turkey, Meleagris gallopavo | Mexico to E and SW USA | Oak and pine woods | S Mexico ~300 BCE-100 CE; SW USA separately ~0-500 CE [M] | (new) | [M] |
| Dog | Grey wolf, Canis lupus (Pleistocene population) | Whole Holarctic | Any | Late Pleistocene, before farming; place still argued | WA_WOLF | Larson et al. 2014 PNAS [O, text only] |

### 2.3 How the existing `wild_ancestors()` boxes compare

Checked against the tables above (no code changed):

- WA_WHEAT box 29-40 N, 30-52 E with tc -2..12, tw > 22, p 200-1,000 mm fits einkorn and emmer well. Barley and flax
  range wider (to 20-70 E, p down to ~150 mm). Chickpea is far narrower (SE Turkey only).
- WA_MILLET 30-45 N, 100-125 E, p 300-900 fits. Good.
- WA_MAIZE 14-22 N, -105 to -95, tc > 12: the core is the Balsas (17-20 N, -103 to -98). Fine; could add a height test
  (400-1,800 m).
- WA_POTATO -22 to -3, -80 to -63, 6 < tw < 18: covers the puna and more. Potato wild core is 13-18 S near Titicaca.
- WA_SORGHUM: realm 1, 4-18 N, p 250-1,000, tc > 18: fits sorghum and pearl millet. Pearl millet's core is drier
  (to 200 mm) and farther north (to 20 N).
- WA_YAM: also used for New Guinea (realm 5, la > -10). Kuk is at 1,560 m: the coldest month there is ~17-19 C [M], right at the
  tc > 18 cutoff, and higher valleys (to 2,800 m) are colder. The New Guinea centre may fire only in the lowlands, not
  where it really began. Suggest tc > 12 for realm 5, or a height exception.
- WA_SHEEPGOAT 28-42 N, 25-70 E, tw > 18: fits.
- WA_LLAMA tw < 20: fits the puna; guanaco also lives in Patagonia (to 55 S).
- WA_FOWL realm 2, tc > 12, p > 1,000: fits.
- WA_CAMEL p < 250, 15-48 N, 30-110 E: fits both camels together.
- WA_BUFFALO realm 2, tc > 14, p > 1,200: fits.
- WA_AUROCHS (realm 0 only): misses India (zebu) and N Africa.

### 2.4 Suggested new bits (if the owner wants them)

| Bit | Box | Climate | For |
|---|---|---|---|
| WA_ZEBU | realm 2 (India and Pakistan), 8-32 N, 65-90 E | tc > 10 C | Zebu cattle; South India and Indus |
| WA_ETHIOPIA | 6-15 N, 35-42 E | height 1,500-3,000 m, tw 16-26 C | Teff, enset, finger millet, noog |
| WA_SINDIA | 12-20 N, 74-82 E | p 450-1,000 mm, tc > 18 C | Mungbean, horsegram, small millets |
| WA_AFRICE | 10-17 N, -10 to 0 | floodplain (riparian) | African rice |
| WA_DONKEY | 8-24 N, 30-50 E | p 50-400 mm | Donkey |
| WA_TURKEY | 15-40 N, -110 to -75 | | Turkey |
| WA_MANIOC | 15 S-10 N, -80 to -45 lowlands | tc > 18 C, p > 1,000 mm with a dry season | Manioc, C. moschata, arrowroot, peanut (S American lowlands; now missing) |

---

## 3. Spread rates, routes and barriers

### 3.1 Rates

| Spread | Rate or dates | Source |
|---|---|---|
| Near East to Europe, overall | 0.6-1.3 km/yr (95% range); 735 early Neolithic sites; origin N Levant / N Mesopotamia near Çayönü | Pinhasi, Fort & Ammerman 2005, PLoS Biology 3:e410 [O] |
| Ammerman & Cavalli-Sforza 1971 | ~1 km/yr (0.8-1.3) from 53 sites | as cited in Pinhasi et al. 2005 [O] |
| Europe, uneven | Fast along the Mediterranean coast (Cardial, by sea, several km/yr) and through the loess (LBK, ~5500-5300 BCE); long pauses at the Aegean edge, the Hungarian Plain and the North European Plain (~1,000 years before Scandinavia and Britain, ~4000 BCE) | [M] (Zilhão 2001 PNAS; Bocquet-Appel et al. 2012 JAS) |
| N China millet to Korea | millets 3500-3000 BCE | Stevens & Fuller 2017 [O] |
| Rice to Taiwan | 2700-2300 BCE | Stevens & Fuller 2017 [O] |
| Rice to SE Asia mainland | maybe 2500-2000 BCE, mainly after 2000 BCE | Stevens & Fuller 2017 [O] |
| Rice to Yunnan | ~2500 BCE onwards | Stevens & Fuller 2017 [O] |
| Rice to Korea, Japan | Korea ~1300-1000 BCE; Japan 980-380 BCE | Stevens & Fuller 2017 [O] |
| Rice and millet farming together | ~4000 BCE; the big expansion 3000-2500 BCE | Stevens & Fuller 2017 [O] |
| Wheat and barley to S India | 1900-1800 BCE | Boivin et al. 2006 [O] |
| Wheat and barley to Ethiopia | from ~1600 BCE | Beldados 2025 [O] |
| Herding into the Kenyan Central Rift | between 4500 and 3000 BP; S-central Kenya from 3000 BP; sheep and goats several hundred years before cattle | MPI-SHH wildebeest project page [O]; Gifford-Gonzalez 2017 [O] |
| Maize from Mexico to Moxos (SW Amazonia) | by 6,850 BP (~1,850 years after 8,700 BP) | Lombardo et al. 2020; Piperno 2011 [O] |
| Manioc from SW Amazonia to Panama | by 7,600 BP | Piperno 2011 [O] |
| Maize into eastern North America | ~200 BCE; dominant only ~900-1000 CE | Smith 2011 [O]; dominance [M] |
| Domestic horse (DOM2) across Eurasia | from ~2000 BCE, Bohemia, Danube, Anatolia by 2200-2000 BCE | Librado et al. 2021 [O] |
| Donkey Africa to Eurasia | ~2,500 years after 5000 BCE | Todd et al. 2022 (phys.org [O]) |
| Chicken from Thailand to Europe and Ethiopia | ~1650 BCE to ~800 BCE | Peters et al. 2022 [O] |
| Bantu farming, Cameroon to S Africa | ~1000 BCE (Great Lakes) to ~300 CE (Natal); ~1-2 km/yr through forest and savanna [M] | [M] |

Model rule of thumb: farming spreads by **colonists budding** at ~1 km/yr on suitable land (meso.rs budding already
exists), much faster by sea along good coasts (3-10 km/yr), and by **adoption** where foragers are dense and settled,
after a long contact pause (often 500-1,500 years).

### 3.2 Barriers

| Barrier | What it stops | Evidence | Model hook |
|---|---|---|---|
| Day length | Short-day tropical crops (teosinte, early maize, sorghum, rice) flower late or not at all at high latitude | Teosinte's flowering is delayed by long days; ZmCCT loss of day-length response let maize go north (Hung et al. 2012, PNAS, via CSHL record [O]) | A crop carries a "latitude reached" trait; it can move only ~2-3 degrees beyond the latitude it is adapted to, then adapts at ~1 degree per few centuries [M] |
| Growing season and cold | Rice north of ~35-38 N; maize where < ~120 frost-free days; Near Eastern winter crops in frost-heavy continental winters (need spring-sown forms); millet needs summer warmth | [M] | Need tw and frost-free months per crop |
| Rain regime change | Winter-rain crops (wheat, barley) fail in summer-rain tropics; summer-rain crops (millets, sorghum) do not suit the Mediterranean | Wheat stopped at the Indian monsoon and the Ethiopian highlands for millennia [M] | Crop season must match rain season |
| Tsetse belt | Cattle (and horses) south of ~14 N in humid Africa | Tsetse needs mean temps 16-32 C (diseases.md); herders reached S Kenya only from ~3000 BP; sheep and goats before cattle | Cattle loss where tsetse > 0 |
| Wildlife diseases | Cattle in E Africa: wildebeest malignant catarrhal fever, East Coast fever from buffalo | Gifford-Gonzalez 2017 [O] | Herds near wild wildebeest calving grounds suffer |
| Altitude | Lowland crops above ~2,000-2,500 m; highland crops (potato, barley) below | [M] | Height test per crop |
| Dense forest | Slows grain farmers (clearing work); LBK avoided heavy forest soils at first | [M] | Forest cost on clearing |
| Dense settled foragers | Farmers stalled at the Baltic and Japan (Jomon) for 1,000+ years | [M] | Contact pause scaled by forager density |
| Sea | Only crossed by peoples with boats; Cyprus by ~8500 BCE [M] | | Existing boat tech |

---

## 4. When chiefs and inequality appear (short, for the game AI)

### 4.1 What the evidence says

- **Farming does not bring inequality at once.** Western Eurasian farmers stayed fairly equal for millennia. Steep
  inequality came from ~4000 BCE, with the **ox plough**: land, not labour, became the limiting input, and land could be
  inherited (Bogaard et al. 2019, Antiquity 93:1129 [O], 39 sites, 90 phases, 9,000 years).
- Even "egalitarian" Çatalhöyük had top households at ~3x average wealth (Bogaard et al. 2019 [O]).
- **Eurasia went further than the Americas** after the Neolithic: large domestic animals (traction, mounted warriors)
  let inequality keep rising; in North America and Mesoamerica it plateaued ~2,000 years after farming
  (Kohler et al. 2017, Nature 551:619, Gini on house sizes [O]). Pompeii (1 CE) Gini ~0.48 [O, Nature Middle East
  summary]. Typical Ginis: foragers ~0.17, horticulturalists ~0.27, agriculturalists ~0.35 [M, Kohler et al.].
- **Circumscription (Carneiro 1970, Science 169:733):** where land is hemmed in by desert, sea or mountains (or by
  neighbours: "social circumscription"), or resources are concentrated (river fish, floodplain soil), losers of wars
  cannot leave; they are absorbed as subjects. Chiefdoms, then states, form. Carneiro later added resource
  concentration as a second route (Feinman 2012, Social Evolution & History 11(2) [O]). Paper itself [M].
- **Storage** of grain lets one household or chief build a surplus and lend it (debt, feasting). Storable staples
  (grain) favour states over root crops [M: Mayshar, Moav & Pascali 2022, J. Political Economy].
- **Scale:** villages without leaders split at a few hundred people; decision hierarchy appears as groups pass ~6
  decision units (Johnson 1982 "scalar stress") [M]. In the Seshat data, polity population, territory and levels of
  hierarchy rise together (Turchin et al. 2018 PNAS) [M]. Rough ladder [M]: village 100-500; simple chiefdom (one
  level above village) 1,000-10,000; complex chiefdom (two levels) 10,000-100,000; state (3+ levels, full-time
  officials) > ~100,000.
- **Warfare** drives scaling: states expand territory fast at their birth (Spencer 2010 PNAS, Monte Albán, Moche) [M].
- **Trade control:** chokepoints (passes, river mouths, obsidian and copper sources) give one lineage leverage [M].

### 4.2 Dates for key regions (first chiefdoms, first states)

| Region | Farming began | Chiefdoms (ranked villages, big men, elite burial) | First state | Gap farming to state | Source |
|---|---|---|---|---|---|
| Mesopotamia | ~9500 BCE (cultivation) | Ubaid, ~5500-4000 BCE (temples, ranked settlements) | Uruk, ~3500-3100 BCE | ~6,000 yr | [M] |
| Egypt | ~5500 BCE (introduced) | Naqada I-II, ~4000-3300 BCE | Unified ~3100 BCE | ~2,400 yr | [M] |
| Indus | ~7000 BCE (Mehrgarh) | Early Harappan, ~3300-2600 BCE | Mature Harappan ~2600 BCE (state status argued) | ~4,400 yr | [M] |
| North China | ~6000 BCE (domestic millet) | Yangshao late / Longshan, ~3500-2000 BCE | Erlitou ~1900-1500 BCE; Shang ~1600 BCE | ~4,000 yr | [M] |
| Yangtze | ~7000 BCE | Liangzhu ~3300-2300 BCE (walled city, possibly a state) | Liangzhu or later Shang reach | ~4,000 yr | Zheng 2016 for Liangzhu date [O]; rest [M] |
| Mesoamerica | ~6750 BCE (maize) | Olmec, San José Mogote, ~1200-900 BCE | Monte Albán ~300-100 BCE; Teotihuacan ~100 CE | ~6,500 yr | [M] |
| Andes | ~6800 BCE (Zaña crops) | Caral/Norte Chico ~2600 BCE (monuments, little inequality) | Moche ~200 CE; Wari ~600 CE | ~7,000 yr | [M] |
| Eastern N America | ~3000 BCE | Hopewell ~100 BCE-400 CE; Mississippian Cahokia ~1050 CE (paramount chiefdom) | none | | [M] |
| Hawaii | settled ~1000 CE | ~1200-1400 CE | archaic states ~1600 CE (Kirch) | ~600 yr | [M] |
| Western Europe | ~6000-4000 BCE | Late Neolithic and Bronze Age elites ~3000-1500 BCE | Iron Age oppida, Rome | | Bogaard 2019 [O] for timing |

### 4.3 Practical triggers for the game AI

A people can raise the "inequality" and "group scale" dials toward a chiefdom when several of these hold:

1. **Storable surplus**: grain or dried fish stores > ~1 year of need, for some households more than others.
2. **Land is limiting**: good land is full (circumscribed by desert, sea, mountain or strong neighbours); exit is costly.
3. **Population**: the polity has > ~1,000-2,000 people in reach of one centre.
4. **War**: raids are frequent and lethal; a war leader's role persists between wars.
5. **Concentrated resource or route**: floodplain, river mouth, pass, quarry, salt or copper.
6. **Traction animals** (Eurasia, Africa, from ~4000 BCE): allow inequality to keep rising (Kohler; Bogaard).

A chiefdom becomes a state when: polity > ~100,000, three tiers of settlement, sustained conquest, and tribute.
Expect long gaps (2,000-7,000 years) between farming and the first state, shortest where farming arrives ready-made
into a circumscribed valley (Egypt).

---

## 5. Sources

Opened this session (paper, abstract, or a summary page of that paper):
- Almathen F. et al. 2016. Ancient and modern DNA reveal dynamics of domestication and cross-continental dispersal of the dromedary. PNAS 113:6707-6712 (ILRI record).
- Beldados A. 2025. The state of archaeobotanical research in Ethiopia. Journal of Ethiopian Studies.
- Bogaard A., Fochesato M., Bowles S. 2019. The farming-inequality nexus: new insights from ancient Western Eurasia. Antiquity 93:1129-1143 (ORA copy); and ScienceDaily summary.
- Boivin N., Fuller D.Q., Korisettar R., Petraglia M. 2006. First farmers in South India. Prāgdhāra 18.
- Denham T. et al. 2003. Origins of agriculture at Kuk Swamp in the Highlands of New Guinea. Science 301:189-193 (via the UNESCO World Heritage dossier).
- Feinman G.M. 2012. Circumscription theory and political change. Social Evolution & History 11(2).
- Fuller D.Q., Denham T., Arroyo-Kalin M. et al. 2014. Convergent evolution and parallelism in plant domestication revealed by an expanding archaeological record. PNAS 111:6147-6152 (ANU copy).
- Fuller D.Q., Barron A., Champion L. et al. 2021. Transition from wild to domesticated pearl millet revealed in ceramic temper at three Middle Holocene sites in northern Mali. African Archaeological Review 38.
- Gifford-Gonzalez D. 2017. "Animal disease challenges" fifteen years later. Quaternary International 436:283-293 (abstract).
- Hung H.-Y. et al. 2012. ZmCCT and the genetic basis of day-length adaptation underlying the postdomestication spread of maize. PNAS 109 (CSHL record).
- Kohler T.A. et al. 2017. Greater post-Neolithic wealth disparities in Eurasia than in North America and Mesoamerica. Nature 551:619-622 (abstract and Nature Middle East summary).
- Larson G. et al. 2014. Current perspectives and the future of domestication studies. PNAS 111:6139-6146 (text only, no figures).
- Librado P. et al. 2021. The origins and spread of domestic horses from the Western Eurasian steppes. Nature 598:634-640.
- Lombardo U. et al. 2020. Early Holocene crop cultivation and landscape modification in Amazonia. Nature 581:190-193.
- MPI-SHH project page: Human-wildlife interactions in East Africa (wildebeest and pastoral expansion).
- Peters J. et al. 2022. The biocultural origins and dispersal of domestic chickens. PNAS 119(24) (LMU record).
- Pinhasi R., Fort J., Ammerman A.J. 2005. Tracing the origin and spread of agriculture in Europe. PLoS Biology 3:e410.
- Piperno D.R. 2011. The origins of plant cultivation and domestication in the New World tropics. Current Anthropology 52(S4).
- Senri Ethnological Studies 112, ch. 13 (National Museum of Ethnology, Osaka): camelid husbandry in the Formative Andes.
- Smith B.D. 2011. The cultural context of plant domestication in eastern North America. Current Anthropology 52(S4).
- Stevens C.J., Fuller D.Q. 2017. The spread of agriculture in eastern Asia. Language Dynamics and Change (UCL copy).
- Todd E.T. et al. 2022. The genomic history and global expansion of domestic donkeys. Science 377 (via phys.org).
- Weiss E., Zohary D. 2011. The Neolithic Southwest Asian founder crops. Current Anthropology 52(S4).
- Winchell F. et al. 2017. Evidence for sorghum domestication in fourth millennium BC eastern Sudan. Current Anthropology (via Sci.News).
- Zeder M.A. 2011. The origins of agriculture in the Near East. Current Anthropology 52(S4).
- Zheng Y. et al. 2016. Rice domestication revealed by reduced shattering of archaeological rice from the Lower Yangtze valley. Scientific Reports 6:28136.

From memory (not opened): Ammerman & Cavalli-Sforza 1971 (Man 6); Carneiro 1970 (Science 169:733); Johnson 1982;
Spooner et al. 2005 (PNAS, potato); Zilhão 2001 (PNAS, Cardial); Bocquet-Appel et al. 2012 (JAS); Spencer 2010 (PNAS);
Turchin et al. 2018 (PNAS); Mayshar, Moav & Pascali 2022 (JPE); Kirch 2010 (Hawaii); state dates in 4.2.

Could not read: PNAS pages directly (403), PubMed Central and PubMed (captcha), Europe PMC (blocked), the HAL archive
(access denied), Springer/BMC after a rate limit (429).
