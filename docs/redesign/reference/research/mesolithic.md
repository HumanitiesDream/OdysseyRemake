# Mesolithic settling: evidence and suggested model changes

Research notes for engine/src/meso.rs (v0.39.1). 5 Oct 2026.
Dates are calibrated unless marked "uncal". BP = before 1950. BCE = BP minus 1950.
"Opened" means I read the paper, its abstract, or a reliable summary page. "Memory" means not checked this session.

## 0. What the model does now (short)

- A people settles when `g = wp * hold * reason * brake` is high. Then `up` must beat `down`.
- `catch = 0.12 + 0.88 * near_s`. The first settler of a region gets only 0.12 of the push.
- `reason` needs `crowd0 >= 0.6-1.0` (land full by forager standards). HANDOFF item 19 says the food model is ~4x real densities and people use 0.1 of it. So crowd0 may rarely reach 0.6.
- `brake` cuts coast settling by up to 85% when the sea rises 4-14 mm a year.
- `down` has a floor of 0.0011 a real year, plus 0.004 * (1 - g).
- `tile_patch`: nuts go to zero above ~20-26 C mean. River richness is halved above ~14-22 C. So the wet tropics get no rich patch.
- A culture ends the year its last member drops below settled 0.15. Members leave on any scatter.

A quick check of the balance. For a lone first settler with g = 0.6 and wp = 1:
up = 0.004 * 0.6 * 3 * 0.12 * 1.6 = 0.0014; down = 0.0011 + 0.0016 = 0.0027 (per real year, at s near 0).
Down wins. Only peoples with g near 1 can ever start a region. That is why settling starts late.

## 1. Settling starts too late

### Evidence: when settling began (opened)

| Place | Date | What | Source |
|---|---|---|---|
| Levant, Natufian | 15,000-11,500 cal BP (13,050-9,550 BCE) | Semi-sedentary; stone-footed round houses; big cemeteries; sickles and ground stone | Barzilai et al. 2017, Antiquity 91:1137-1154 |
| Tanegashima, S Japan, Incipient Jomon | ~13,000 cal BP | Sedentism appears with rich food AND rising population; "highly stable sedentism depends ... also on the increase in population size" | PLOS ONE, vol. 20 no. 1 (year shown as 2026 on NSF PAR), "More on mobility and sedentism ... Tanegashima" |
| Star Carr, England | ~9000 BCE | 2 ha lakeside site, 30 m waterfront, at least one post-built house | Conneller, Milner, Taylor & Taylor 2012, Antiquity 86:1004-1020 |
| Iron Gates, Danube | Proto-Lepenski Vir 9500-7300 BCE | Rapids fishing; repeated use of the same spots | Brami et al. 2022, J. World Prehistory 35:109-133 |
| Llanos de Moxos, Bolivia | from 10,850 cal BP (8,900 BCE) | ~4,700 man-made forest islands; squash ~10,250, manioc ~10,350 cal BP | Lombardo et al. 2020, Nature 581:190-193 |
| Lake Turkana, Kenya | early Holocene, wavy-line pottery 12,000-5,000 BP | Harpoon fishers used the same spot all through the Holocene | Goldstein et al. 2017, Antiquity 91(360) e5 |
| Uenohara, S Japan | ~9,500 cal BP (7,550 BCE) | 52 pit houses, 16 storage pits; one of Japan's oldest permanent villages | Wikipedia summary of Japanese agency sources (not peer-reviewed) |
| Namu, British Columbia | last ~7,000 years | Permanent settlement on stored pink salmon | Cannon & Yang 2006, American Antiquity 71:123-140 |
| Denmark, Kongemose | 6500-5400 BCE | "Large sedentary settlements" even while the sea rose fast | Rowley-Conwy 1999, Proc. British Academy; dates from Plonka et al. 2023, Antiquity |

### Two biases that matter for the targets

- Drowned coasts. Big shell-midden clusters start ~8-7 ka. "Earlier sites are not unknown, but they are far fewer and smaller." Much of this is loss to the rising sea, not absence (Bailey & Flemming 2008, Quaternary Science Reviews 27:2153-2165). So coastal settling before ~7000 BCE was likely more common than the record shows.
- Rising seas did not stop settling. The Kongemose had large sedentary sites while the sea rose fast (Rowley-Conwy 1999). Denmark was also lifting, so the local rise was slower than the global one.

### Conditions that went with early settling

- A dense, reliable food at one spot: lake edge (Star Carr), rapids (Iron Gates), wild cereals and nuts (Natufian), fish runs (Namu), forest islands (Moxos).
- Storage: pits (Uenohara), stored salmon (Namu), sickles and ground stone (Natufian).
- Population pressure. Tanegashima: lasting sedentism needed more people, not just rich land (PLOS ONE, Tanegashima paper).
- Hamilton et al. 2024 (J. Archaeological Science: Reports 54) on Binford's data: mobility costs fall with density (doubling density about halves mobility cost). Storage cuts how often people move, not how far they go in a year. So density is the stronger lever; storage is a helper.

### Are the 9000 and 7000 BCE targets sound?

Yes, roughly. By 9000 BCE settling had begun in at least five far-apart regions (Levant, Japan, Britain, Danube, Amazonia, East Africa). That fits 3-8% part-settled. The drowned-coast bias argues for the upper half.

### Suggested changes

1. Raise the first-settler push. `catch = 0.12 + 0.88*near_s` to `catch = 0.3 + 0.7*near_s`. Settling began in many regions within ~1,000-2,000 years of the melt, independently. It did not wait thousands of years per region.
2. Lower the base `down` from 0.0011 to ~0.0006, so peoples with g ~ 0.6 can hold part-settled (s ~ 0.4-0.5).
3. Loosen packing. `smooth(0.6, 1.0, crowd0)` to `smooth(0.3, 0.8, crowd0)`, or measure crowd against the *used* food (0.1 of the model's), not the model's full food. The model's food is ~4x real (HANDOFF 19), so crowd0 is deflated by ~4x.
4. Soften the sea brake. `0.85 * smooth(4, 14, sl_rate)` to `0.4 * smooth(6, 20, sl_rate)`. Keep it only for low, flat shores (small relief) if possible.
5. Fresh start mid-melt: spin up the dials. At the start, run `meso_year` ~20-50 times with no news, or set `dial[0]` to the value the balance gives (s* where up(1-s)^2*1.6 = down*s with near_s from a first pass). Real settling at 9000 BCE was already 2,000+ years old in places.

## 2. The settled share plateaus at ~15% by 5000 BCE

### Is 15-20% well founded?

I tabulated Murdock's Ethnographic Atlas (D-PLACE data, EA030 settlement pattern) for foragers (agriculture 0-5%, herding 15% or less). n = 240 societies.

| Group | n | Semisedentary or more | Permanent villages |
|---|---|---|---|
| All foragers | 240 | 28% | 11% |
| Fishing first | 90 | 53% | 24% |
| Gathering first | 79 | 14% | 4% |
| Hunting first | 60 | 3% | 2% |
| Latitude 0-15 | 16 | 12% | 12% (2 societies) |
| Latitude 15-35 | 29 | 10% | 0% |
| Latitude 35-50 | 124 | 27% | 11% |
| Latitude 50+ | 71 | 38% | 14% |

Weighted by population (EA202, where given), semisedentary-or-more is ~26%, villages ~9%. Close to the group shares.

What this means:
- The ethnographic world is the end point after 5,000+ years more of growth. Many big villages came late (NW Coast plank houses, Middle Jomon peak at up to ~300 people per 100 km2: Keally, Jomon web page; Plateau pithouse villages after ~4,000 BP: Chatters & Pokotylo 1998 via a WA state report).
- Bailey & Flemming 2008: shell middens boom only after the sea stopped rising ~7-6 ka.
- So 15-20% part-settled at 5000 BCE is reasonable. It should keep rising to ~25-30% by ~2000 BCE among people still foraging. Fully settled ~3-7% at 5000 BCE, ~10% later. The current targets are sound.
- The model's ~15% is at the low end of the band, not wrong. The bigger problem is that it stops rising.

### Suggested changes

1. Add a late-Holocene target to meso_check: ~22-30% part-settled and ~8-12% fully settled among foragers by ~2000 BCE (if the sim runs that long without farmers taking the rich land).
2. Let sea-level stillstand help. When `sl_rate < 2 mm/yr`, add a coast bonus to `wp` (estuaries fill, shellfish beds form). E.g. shore patch x 1.2 once the sea is still.
3. Let fish be the main driver. Fishers are 4x more likely to settle than gatherers and ~15x more than hunters. Check that `hold` and `reason` are not equal across ways. Consider `g *= 1.3` for fish ways and `g *= 0.5` for hunting-only peoples.
4. Let density keep pushing. A slow late rise needs a term that grows with density, not just packing past a threshold. E.g. add `0.3 * dial[2]` (density dial) to `reason`.

## 3. Tropical forest foragers never settle

### Evidence (opened)

| Place | Date | What | Source |
|---|---|---|---|
| Hoabinhian, SE Asia | ~29,000 to 3,700 BP (Huai Hin, Thailand, latest) | Long-lived pebble-tool tradition; rockshelters by streams in karst | Forestier, Zeitoun, Winayalai & Metais 2013, Comptes Rendus Palevol 12 |
| Togi Ndrawa, Nias (off Sumatra) | 12,170 to 5,540 BP (uncal) | Hoabinhian tools with shells | Forestier et al. 2013 |
| Da But culture, N Vietnam | ~6,500-4,800 BP (uncal, freshwater shell) | Shell-midden sites | Oxenham et al. 2018, Antiquity 92(364) |
| Con Co Ngua, N Vietnam (Da But) | 6,700-6,200 cal BP (4,750-4,250 BCE) | "Apparently sedentary"; 272 burials; large sealed pots for storage; Canarium nuts as main starch; wild cattle, deer, fish, shellfish; no rice | Oxenham et al. 2018, Antiquity 92(364) |
| Taperinha, lower Amazon | 8,029-6,311 cal BP | Freshwater shell mound ~4 m high, with pottery | Roosevelt et al. 1991 dates, via XRONOS database |
| Llanos de Moxos, SW Amazonia | from 10,850 cal BP | Forest islands in flooded savanna; early cultivation | Lombardo et al. 2020, Nature 581 |
| Sambaquis, S Brazil coast (subtropical, 23-28 S) | at least 8,000 to 1,000 cal BP | Shell mounds up to 30 m high and 400-500 m across; fish-based; one site used 800+ years without a break | Mendes Cardoso et al. 2024, PLOS ONE (Galheta IV) |
| Contemporary S China | ~same time as Con Co Ngua | Dingsishan: 331 graves | Oxenham et al. 2018 |

Cross-cultural (opened):
- Binford 1990 (via HRAF summary): the longer the growing season, the more likely foragers are fully nomadic.
- Ethnographic Atlas (my tabulation above): foragers at 0-15 degrees are 12% settled (n = 16, small sample); 15-35 degrees 10%.
- Roscoe 2006 (via HRAF): New Guinea foragers who depend on fishing have higher density and big settlements.

From memory (not checked):
- Central African forest foragers (Mbuti, Aka, Baka) were highly mobile in historic times. Holocene sites in the Congo basin are small and few. No case of settled forest foragers there that I know of. (Cornelissen 2013, Oxford Handbook of African Archaeology, is the review; I only saw its record.)
- Sago-palm stands in New Guinea and Melanesia carried semi-settled groups in recent times.

### What the evidence says

- Tropical foragers did settle, but not in the closed rainforest interior. They settled at:
  - estuaries and freshwater shell beds (Da But, Taperinha, Sumatra),
  - floodplain and savanna-forest edges (Moxos),
  - with tree nuts or palms as starch (Canarium at Con Co Ngua).
- Timing: mid-Holocene, ~8,000-6,000 cal BP. That is after the sea stopped rising and during the Holocene thermal maximum. Moxos is earlier (~10,800 BP).
- Share: low. ~10% of tropical foragers, like the ethnographic figure.

### Suggested changes in tile_patch / way_for

1. Tropical river floodplains. Change the river factor `0.5 + 0.5*smooth(0,6,tm)*(1-smooth(14,22,tm))` so big tropical rivers are not halved. E.g. for `river >= 6` use 0.85 in the tropics. Amazon and Mekong floodplains are among the richest fisheries.
2. Estuary and freshwater shell beds. If a tile has a sea neighbour AND river >= 4, or a lake neighbour, with tm > 18, set shore/lake to at least 0.6. That gives Da But, Taperinha and Sumatra-type middens.
3. Tropical tree nuts and palms. Add a fourth nut branch for tm > 20: `0.8 * tree * seasonality` where seasonality is a dry season (monsoon forest, forest-savanna edge), not ever-wet forest. Canarium, palms, sago. Keep it below temperate nut woods (0.8 not 1.25).
4. Keep the rainforest interior roving. No patch for ever-wet closed forest away from big rivers and coasts. This matches the Congo and the ethnography.
5. Gate tropical settling on a still sea (sl_rate < ~3 mm/yr), so it comes ~6000 BCE, not earlier. Except inland floodplains (Moxos), which can start ~9000 BCE.
6. Expect ~10% of tropical foragers part-settled by 4000 BCE. Add that to meso_check.

## 4. Cultures churn too fast

### Evidence: how long forager cultures lasted (opened unless marked)

| Culture | Dates | Length | Source |
|---|---|---|---|
| Natufian | 15,000-11,500 cal BP | ~3,500 yr (two phases ~1,500-2,000 each) | Barzilai et al. 2017, Antiquity |
| Maglemose | ~9600-6500 BCE | ~3,100 yr | Plonka, Adamczyk & Diakowski 2023, Antiquity |
| Kongemose | ~6500-5400 BCE | ~1,100 yr | same |
| Ertebolle | ~5400-4100 BCE | ~1,300 yr | same |
| Iron Gates Mesolithic (Proto-Lepenski Vir) | 9500-7300 BCE, then on to 6200-5900 BCE | ~2,200 yr phase; ~3,500 yr tradition | Brami et al. 2022, J. World Prehistory |
| Capsian | 9000-5400 BCE | ~3,600 yr (Typical ~2,600; Upper ~1,000) | Perrault 2024, African Archaeological Review 42:95-124 |
| Jomon | ~16,000-2,800 cal BP | ~13,000 yr; Middle Jomon (Kanto) 5,400-4,400 cal BP ~1,000 yr | Keally, Jomon web page (not peer-reviewed) |
| Green River Shell Mound Archaic | shell build-up 6,500-4,500 BP | ~2,000 yr; one mound (Indian Knoll) grew ~400-500 yr | Morey et al. 2002, Geoarchaeology 17:521-553 |
| NW Coast phases (Locarno Beach, Marpole, etc.) | 6,500-4,500; 3,500-2,500 BP ... | ~1,000-2,000 yr each | Stein 2000, cited in a WA state report (grey) |
| Namu settlement | last ~7,000 yr | one place, ~7,000 yr | Cannon & Yang 2006, American Antiquity |
| Hoabinhian | ~29,000-3,700 BP | 20,000+ yr technocomplex | Forestier et al. 2013, C. R. Palevol |
| Sambaqui tradition | 8,000-1,000 cal BP | ~7,000 yr; one site 800+ yr | Mendes Cardoso et al. 2024, PLOS ONE |

Three scales show up:
- Long traditions / technocomplexes: 5,000-20,000 years (Hoabinhian, Jomon, sambaqui).
- Named cultures: ~1,000-3,600 years. Median ~2,000-3,000.
- Phases within them: ~400-1,000 years.

Also: Mesolithic cultures were NOT all settled. Maglemose was mostly mobile. Late Natufian was more mobile than Early Natufian and is still called Natufian. A culture outlives the settling of its members.

### The model in the same units

At the fast culture pace, 400 game years is ~3,200-6,400 real years (pace 8-16). With ~200 living and ~900 founded, the mean life is about 200 x 400 / 900 = ~90 game years, or ~700-1,400 real years. The mean is at the "phase" scale. The median is likely far shorter (many cultures die within a few decades of game time). A southern Scandinavia-sized area has 3 cultures in 5,500 years: founded/living ~ 3 over 5,000 years.

Why it churns (from the code):
- `if s < 0.15 { culture = 0 }`. Any scatter (cold snap, regional fall-back, failed patch) strips the culture.
- `if c.n == 0 { ended }` the same year. No grace time.
- Re-settling founds a new culture, not the old one.
- Splits at > 8 cells or 10 kit tools drifted.

### Suggested changes

1. Membership by people, not by settled state. Once a people joins a culture it keeps it while it lives in the culture's area (cdist <= 8), settled or not. Drop it only on a long move or a kit split.
2. Grace before death. When `n == 0`, mark the culture dormant. End it only after ~500 real years (`500 / cult * 12` months). A people of the same line or a neighbour that re-settles within 5 cells rejoins it.
3. Founding needs a gap. Found a new culture only if no living or dormant culture of a kindred way is within ~5 cells. (Already true for living ones; add dormant.)
4. Slow the kit split. 10 drifted tools out of 26 can happen in a few centuries. Raise to 13, or require the drift to last 300+ real years.
5. Add a check to meso_check: median culture life >= 1,000 real years; mean >= 1,500; founded/living over 5,000 real years <= 4.
6. Optional: a parent "tradition" layer (e.g. Jomon, Hoabinhian, Danish Mesolithic) that lasts 5,000+ years and holds a chain of named cultures. The existing `parent` field can chain them.

## Sources opened

- Bailey G. & Flemming N. 2008. Archaeology of the continental shelf. Quaternary Science Reviews 27:2153-2165.
- Barzilai O. et al. 2017. Radiocarbon dating of human burials from Raqefet Cave and contemporaneous Natufian traditions at Mount Carmel. Antiquity 91(359):1137-1154.
- Brami M. et al. 2022. Was the fishing village of Lepenski Vir built by Europe's first farmers? Journal of World Prehistory 35:109-133.
- Cannon A. & Yang D.Y. 2006. Early storage and sedentism on the Pacific Northwest Coast: ancient DNA analysis of salmon remains from Namu. American Antiquity 71(1):123-140.
- Conneller C., Milner N., Taylor B. & Taylor M. 2012. Substantial settlement in the European Early Mesolithic: new research at Star Carr. Antiquity 86(334):1004-1020.
- D-PLACE, Ethnographic Atlas (Murdock 1967 and later), variables EA004, EA005, EA030, EA031, EA042, EA202. My tabulation.
- Forestier H., Zeitoun V., Winayalai C. & Metais C. 2013. Comptes Rendus Palevol 12 (Hoabinhian, Huai Hin).
- Goldstein S. et al. 2017. New archaeological investigations at the Lothagam harpoon site at Lake Turkana. Antiquity 91(360) e5.
- Hamilton M.J., Buchanan B., Lobo J. & Walker R.S. 2024. Food storage, mobility, and the density-dependence of hunter-gatherer movement ecology. J. Archaeological Science: Reports 54.
- HRAF, Explaining Human Culture: Hunter-gatherers summary (Binford 1990; Roscoe 2006).
- Keally C.T. Jomon culture web page (not peer-reviewed).
- Lombardo U. et al. 2020. Early Holocene crop cultivation and landscape modification in Amazonia. Nature 581:190-193.
- Mendes Cardoso J. et al. 2024. Late shellmound occupation in southern Brazil: Galheta IV. PLOS ONE.
- Morey D.F. et al. 2002. The fluvial and geomorphic context of Indian Knoll. Geoarchaeology 17(6):521-553.
- Oxenham M.F. et al. 2018. Between foraging and farming: strategic responses to the Holocene Thermal Maximum in Southeast Asia. Antiquity 92(364).
- Perrault N. 2024. Contemporaneity of the Typical and Upper Capsian. African Archaeological Review 42(1):95-124.
- Plonka T., Adamczyk M. & Diakowski M. 2023. New radiocarbon dates for ornamented Mesolithic objects from north-west Poland. Antiquity.
- PLOS ONE, vol. 20 no. 1 (NSF PAR record). More on mobility and sedentism: Upper Paleolithic to Incipient Jomon, Tanegashima Island (authors not seen).
- Roosevelt A. et al. 1991, Taperinha dates, via the XRONOS radiocarbon database.
- Rowley-Conwy P. 1999. Economic prehistory in southern Scandinavia. Proceedings of the British Academy.
- Stein 2000 and Chatters & Pokotylo 1998, as cited in a Washington EFSEC archaeological context appendix (grey literature).
- Wikipedia, Uenohara site (for the 52 houses and 9,500 cal BP date).
