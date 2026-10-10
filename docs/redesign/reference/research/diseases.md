# Endemic diseases and parasites for preindustrial peoples

Research notes for the TODO item "Disease reservoirs (plague, malaria, tsetse, rabies)" and for epidemics in people
(HANDOFF: tropical disease is density-dependent, holding tropical peoples to ~60% of their land's food). 5 Oct 2026.

Dates are calibrated. BP = before 1950. BCE = BP minus 1950.
Tags: **[O]** = I opened the paper, its abstract, or a reliable summary of it this session. **[M]** = from memory, not
checked this session. Treat [M] numbers as good to about +-20%.

---

## 0. Summary for the model

Three kinds of disease behave differently and need different code:

1. **Place diseases (vector or soil):** malaria, sleeping sickness, schistosomiasis, hookworm, filariasis, river
   blindness, leishmaniasis, Chagas, yellow fever (jungle cycle). They live where the climate suits the vector or the
   parasite's free stage. They existed before farming. They are a per-tile **burden** (more deaths, lower work and
   births) scaled by climate suitability x human density x landscape change (clearing, irrigation, standing water).
2. **Reservoir zoonoses:** plague, rabies, anthrax, brucellosis. They live in wild or domestic animals. People catch
   them in **spillover events**, sometimes as outbreaks. Need an animal map (rodents, dogs, herds).
3. **Crowd diseases:** measles, smallpox, mumps, rubella, whooping cough, influenza (as an endemic human disease).
   Immunising, acute, no animal reservoir. They die out in small groups and need a **critical community size** of
   ~250,000-500,000 connected people (measles). Before that size they come only as rare, deadly imported epidemics.
   Most arose in the last ~5,000 years; measles only ~500 BCE.

Farming made all three worse: denser, settled people; standing water in fields; dung and stored grain bring rodents
and flies; herds bring zoonoses. Falciparum malaria spread with clearing for yam farming in West Africa (Livingstone
1958, the sickle-cell link) [M].

---

## 1. Place diseases (vectors and soil)

### 1.1 Main table

| Disease | Agent | Vector / host | Range (preindustrial, rough box) | Climate limits | Before farming? | Burden notes | Sources |
|---|---|---|---|---|---|---|---|
| Falciparum malaria | Plasmodium falciparum | Anopheles mosquitoes (Africa: An. gambiae, An. arabiensis, An. funestus) | Tropical Africa (strongest), S and SE Asia, New Guinea; Mediterranean and Near East lowlands in summer; not in the Americas before 1492 [M] | Parasite needs > 16 C to develop, 111 degree-days above 16 C to finish in the mosquito; year-round transmission needs warm all year | Yes, but at low level; spread hugely with farming and clearing [M] | Kills young children; adults gain partial immunity; sickle cell, thalassaemia, G6PD are adaptations [M] | Gething et al. 2011 [O]; Sinka et al. 2012 [O] |
| Vivax malaria | P. vivax | Anopheles (many species, incl. temperate An. messeae, An. atroparvus [M]) | Wider: all warm and temperate Eurasia to ~60-64 N in summer (historic Finland, Russia) [M]; tropical Asia; rare in W and central Africa (Duffy-negative people) [M]; Americas: probably post-1492 [M] | > 14.5 C, 105 degree-days; dormant liver stages let it wait through a cold winter | Yes | Chronic, weakening; lower death rate than falciparum (~1-5% of untreated cases, falciparum 10-20%+ in children) [M] | Gething et al. 2011 [O] |
| Sleeping sickness | Trypanosoma brucei gambiense (W and central Africa), T. b. rhodesiense (E Africa) | Tsetse (Glossina) | Tsetse belt: ~14 N to ~20-29 S in Africa [M]; humid forest and moist savanna; river woods into drier land | Tsetse populations die out if held below ~16 C or above ~31-32 C mean for long; best 19-25 C; need shade, humidity, woody cover [M] | Yes (wild game reservoir) | Human disease patchy; the big cost is **nagana in cattle and horses** (T. congolense, T. vivax) | Are & Hargrove 2020 (bioRxiv version [O]); van Aardenne (IRI slides [O]) |
| Schistosomiasis (urinary) | Schistosoma haematobium | Bulinus snails in fresh water | Africa, Nile, Near East, Mesopotamia | Snails and parasite need water ~15-32 C; transmission best ~22-28 C [M] | Yes; strongly increased by irrigation (Nile, Mesopotamia) [M]; egg found in a ~6,200 BP burial in Syria (Anastasiou et al. 2014, Lancet Infect Dis) [M] | Chronic; anaemia, bladder disease | [M] |
| Schistosomiasis (intestinal) | S. mansoni | Biomphalaria snails | Africa, Arabia (Americas only after the slave trade) [M] | as above | Yes | | [M] |
| Schistosomiasis (Asian) | S. japonicum | Oncomelania snails (amphibious) | Yangtze basin, S China, Philippines, Sulawesi [M] | Oncomelania limited north of ~33-34 N by winter cold [M] | Yes; a ~2,100-year-old case in the Mawangdui body [M] | Many animal hosts (buffalo, pigs, rodents) | [M] |
| Hookworm | Necator americanus, Ancylostoma duodenale | Larvae in warm moist soil, enter through bare skin | Tropics and warm subtropics; also reached the pre-Columbian Americas [M] | Larvae need soil > ~15-20 C and moisture; killed by dry, hot (> ~40 C) or frozen soil [M] | Yes (old human parasite) | Iron-deficiency anaemia; worse where people settle and soil is soiled | [M] (Pullan & Brooker 2012 not read: rate limited) |
| Roundworm, whipworm | Ascaris lumbricoides, Trichuris trichiura | Eggs in soil and on food | Global incl. temperate Europe; common in Neolithic and later villages [M] | Eggs tolerate cold; need moisture; rare in deserts | Present in foragers at low level; boom with sedentism and manure [M] | Mild to moderate | [M] |
| Tapeworms | Taenia saginata (cattle), T. solium (pig); Echinococcus (dogs, sheep) | Undercooked meat; dog faeces | With herds and pigs everywhere | none much | Taenia in people predates domestic cattle/pigs (from wild hosts) [M]; hydatid with dog-sheep herding | Hydatid cysts serious | [M] |
| Lymphatic filariasis | Wuchereria bancrofti, Brugia malayi | Culex, Anopheles, Aedes mosquitoes | Tropics: Africa, S and SE Asia, Pacific, (Americas post-1492 [M]) | Needs warm humid; parasite development > ~22 C [M] | Yes [M] | Elephantiasis; low deaths | [M] |
| River blindness | Onchocerca volvulus | Simulium blackflies breeding in fast rivers | W and central Africa savanna rivers; (Yemen; Latin America post-1492) [M] | Needs fast-flowing rivers, warm | Yes [M] | Blindness; people abandon fertile river valleys (Volta) [M] | [M] |
| Leishmaniasis | Leishmania spp. | Phlebotomine sandflies | Cutaneous: Near East, Central Asia, N Africa, Mediterranean, S America. Visceral: E Africa, India (Bengal), Mediterranean, Brazil [M] | Warm, dry-season; sandflies need > ~15-16 C [M] | Yes; reservoirs dogs, gerbils, hyraxes [M] | Visceral form fatal untreated | [M] (Pigott et al. 2014 eLife page would not load) |
| Chagas | Trypanosoma cruzi | Triatomine bugs living in thatch and walls | Americas ~40 N to ~45 S; Andes and S Cone worst [M] | Bugs need warmth; T. infestans in Andean houses to ~3,500 m [M] | Yes; found in ~9,000-year-old Chinchorro mummies (Aufderheide et al. 2004, PNAS) [M] | Sedentary adobe and thatch houses + guinea pigs made it a household disease [M] | [M] |
| Yellow fever | Yellow fever virus | Aedes aegypti (towns), forest Aedes and Haemagogus (monkeys) | Africa only before ~1600 CE; reached the Americas 300-400 years ago with the slave trade; current strains arose in Africa within the last ~1,500 years | Aedes aegypti needs > ~10 C winters; transmission > ~18-20 C [M] | Jungle cycle in monkeys yes; urban cycle needs towns | Adults die more than children; survivors immune | Bryant, Holmes & Barrett 2007, PLoS Pathogens 3:e75 [O] |
| Dengue | Dengue virus 1-4 | Aedes aegypti, Ae. albopictus; jungle cycle in primates (SE Asia, W Africa) | Tropical Asia, Africa [M] | as yellow fever | Jungle cycle yes; human endemic cycle needs large towns [M] | Mild mostly; severe on reinfection | [M] |

### 1.2 Malaria vectors by region (for the anopheline map)

From Sinka et al. 2012 [O]:

| Region | Dominant vectors | Habitat notes |
|---|---|---|
| Africa | An. gambiae, An. arabiensis, An. funestus co-dominant; An. moucheti in central African forest | An. arabiensis takes drier land: Sahel, Ethiopia, Arabia, Namibia, Botswana |
| Europe and Middle East | An. messeae (UK to Asia; the most northern dominant vector); also An. atroparvus, An. sacharovi, An. labranchiae [M] | |
| India | An. culicifacies, An. stephensi, An. fluviatilis | |
| SE Asia | An. dirus, An. minimus complexes | forest and foothill |
| Indonesia, Borneo | An. sinensis, An. barbirostris, An. leucosphyrus/latens, An. balabacensis | |
| New Guinea, Pacific | An. farauti, An. koliensis, An. punctulatus | (no Anopheles east of Vanuatu [M]) |
| North America | An. freeborni (NW), An. quadrimaculatus (SE) | |
| South America | An. darlingi; An. albimanus, An. pseudopunctipennis in Central America; An. aquasalis on salty coasts | |

### 1.3 Climate rules ready for code

| Disease | Suitability rule (per tile, per month) | Source |
|---|---|---|
| Falciparum | Month counts if mean T > 16 C; transmission season length = months where degree-days above 16 C over ~a month exceed 111; scale x vector presence; very high T (> ~32-34 C) cuts vector survival | Gething et al. 2011 [O]; upper limit [M] |
| Vivax | as above with 14.5 C and 105 degree-days; survives winters of any cold (relapse) if summer season >= 1-2 months | Gething et al. 2011 [O] |
| Tsetse | 16 C < mean T < 31 C most of the year, plus woody cover > ~20% and rain > ~500 mm [M]; realm 1 (Africa) only | Are & Hargrove 2020 [O]; cover and rain [M] |
| Schistosomiasis | Fresh standing or slow water (lakes, rivers, irrigation) with water T 15-32 C at least part of the year; Asian type only where Oncomelania (S of ~33 N in E Asia) | [M] |
| Hookworm | Soil (air) mean T > ~18 C for 3+ months, moist (rain > ~600 mm); none above ~2,500 m in tropics | [M] |
| Chagas | Americas, mean T > ~15 C, settled people with mud and thatch houses | [M] |

Gething et al. 2011 [O] also note: year-round falciparum suitability covers ~37.6 M km2 and vivax ~43.5 M km2 today;
highlands (Tibet, Andes, East African highlands) switch sharply from suitable to unsuitable.

---

## 2. Reservoir zoonoses

| Disease | Reservoir | Range | Climate / triggers | First in people | Sources |
|---|---|---|---|---|---|
| Plague | Wild burrowing rodents: great gerbil (Central Asia), marmots and ground squirrels (Central Asia, Mongolia), prairie dogs (N America, post-1900), many African rodents (reservoirs poorly known); fleas carry it | Natural foci: Central Asian deserts and steppe (Kazakhstan, Mongolia, W China), Caucasus, Iran, Arabia [M]; E and S Africa [M] | Warmer springs and wetter summers raise plague in great gerbils (Kazakhstan) | Basal lineages split 5,700-5,100 BP; oldest and most basal strain in a ~4,900 BP Swedish farmer (Gökhem); linked to large settlements (Cucuteni-Trypillia, 6,100-5,400 BP). Flea-adapted (bubonic) form only from ~1000 BCE (Spyrou et al. 2018) [M] | Stenseth et al. 2008, PLoS Medicine 5:e3 [O]; Rascovan et al. 2019, Cell 176:295 (via The Science Breaker [O]) |
| Rabies | Dogs (Old World), foxes, wolves, jackals, skunks; vampire bats in Central and S America [M] | Global except islands like Australia, Japan (later), Britain after 1902 [M] | Rises with village dogs | Old; described in Mesopotamia (Eshnunna laws, ~1900 BCE) [M] | [M] |
| Anthrax | Spores in soil; grazing animals | Global, worst on alkaline soils in warm areas [M] | Droughts after rain, herds | Prehistoric [M] | [M] |
| Brucellosis | Goats, sheep (B. melitensis), cattle (B. abortus), pigs | With herds | Milk and birthing contact | Neolithic onward [M] | [M] |
| Tuberculosis | Mostly human-to-human; M. bovis in cattle; M. pinnipedii in seals | Global | Crowding, indoor smoke, poor food | MTBC common ancestor < 6,000 years old (so not a long Paleolithic disease); the pre-Columbian Peruvian strains came from seals and sea lions | Bos et al. 2014, Nature 514:494 (CONICET record [O]) |
| Leprosy | Humans; armadillos (Americas, post-1492); red squirrels (Britain) [M] | Old World | Long close contact | Oldest skeletal case ~2000 BCE, India [M] | [M] |
| Hepatitis B | Humans (older forms in apes) | Global | | Ancient DNA shows it in Eurasians ~10,000 years ago; pre-farming [M] (Kocher et al. 2021, Science) | [M] |

---

## 3. Crowd diseases and critical community size

| Disease | Origin | When | Critical community size (CCS) | Sources |
|---|---|---|---|---|
| Measles | From rinderpest-like cattle virus | Split from rinderpest between 1174 BCE and 165 CE (mean 528 BCE); cities passed the threshold ~300 BCE (Babylon, Athens, Rome) | 250,000-500,000 people | Düx et al. 2020, Science 368:1367 (AAAS news [O]) |
| Smallpox | From a rodent orthopoxvirus (closest: taterapox of gerbils, camelpox) [M] | Variola as we know it: ancient DNA in Viking-age Europe (~600-1050 CE), ancestor ~1,700 years ago; earlier "smallpox" claims (Egypt, Ramses V) uncertain [M] (Mühlemann et al. 2020, Science) | ~200,000+ (similar to measles) [M] | [M] |
| Influenza (human endemic) | Wild waterbirds via pigs, poultry | Unknown; recurrent pandemics in history [M] | Large; seasonal reintroduction [M] | [M] |
| Mumps, rubella | Human | Late; need cities [M] | ~200,000-300,000 [M] | [M] |
| Whooping cough | Human (Bordetella from animal relatives) [M] | Late [M] | ~200,000+ [M] | [M] |
| Chickenpox | Human (latent; reactivates as shingles) | Old; latency lets it persist in small bands | none (no CCS needed) [M] | [M] |

Origins in general: Wolfe, Dunavan & Diamond 2007 (Nature 447:279) traced 25 major diseases (10 tropical, 15
temperate) through five stages from animal-only to human-only; temperate crowd diseases mostly came from domestic
animals, tropical ones more from wild primates [abstract O; details M].

Model use:
- Track "connected population" per disease region (people within a few months' travel along ties).
- If connected < CCS: a crowd disease cannot stay; it arrives as a rare epidemic (contact with a large population)
  killing ~10-30% of a non-immune people [M], then dies out.
- If connected > CCS: it becomes a childhood disease (endemic), steady child deaths of a few %, and the people become a
  "disease reservoir" that kills neighbours on contact (the Columbian effect).
- None of the crowd diseases should exist before ~3000 BCE anywhere; measles not before ~1000 BCE.

---

## 4. Animal diseases that matter to herders

| Disease | Hosts | Range | Effect on spread | Sources |
|---|---|---|---|---|
| Nagana (animal trypanosomiasis) | Cattle, horses, camels | Tsetse belt (see 1.1) | Main barrier to cattle in humid Africa | [M] |
| Wildebeest malignant catarrhal fever | Cattle, from wildebeest calves | E African savanna | Slowed cattle south of Lake Turkana; sheep and goats came several hundred years before cattle | Gifford-Gonzalez 2017 [O] |
| East Coast fever (Theileria) | Cattle, from buffalo via ticks | E and S Africa | as above | Gifford-Gonzalez 2017 [O] |
| Rinderpest | Cattle, buffalo, wild ungulates | Asia, later Africa (1887 panzootic) | Already in the engine as "cattle plague" (eco.rs) | [M] |

---

## 5. What existed before farming, and what needs density

| Group | Diseases | Needs |
|---|---|---|
| "Heirloom" (with foragers since the Paleolithic) | Lice, pinworm, herpes, chickenpox, hepatitis B, yaws and other treponemes [M], vivax and some falciparum, hookworm, Taenia, schistosomes, Chagas (Americas), leishmaniasis, sleeping sickness | Present at low level in bands; climate sets range |
| Boosted by settling and farming | Falciparum (clearing, puddles), schistosomiasis (irrigation), soil worms (sedentism, manure), Chagas (houses), plague (stored grain, rodents), TB (crowding), brucellosis, anthrax, tapeworm, hydatid (herds and dogs) | Settled villages; density; herds |
| Crowd diseases | Measles, smallpox, mumps, rubella, whooping cough, influenza | Connected population > ~250,000; from ~1000 BCE onward |

---

## 6. Sources

Opened this session (paper, abstract, or a summary page of that paper):
- Are E.B., Hargrove J.W. 2020. Extinction probabilities as a function of temperature for populations of tsetse (Glossina spp.). PLoS Neglected Tropical Diseases (bioRxiv 767350 version). Authors and journal as I recall them; the page did not show them.
- van Aardenne L. (IRI, Columbia) slides: climate and tsetse in the Zambezi Valley.
- Bos K.I. et al. 2014. Pre-Columbian mycobacterial genomes reveal seals as a source of New World human tuberculosis. Nature 514:494-497 (CONICET record).
- Bryant J.E., Holmes E.C., Barrett A.D.T. 2007. Out of Africa: a molecular perspective on the introduction of yellow fever virus into the Americas. PLoS Pathogens 3:e75.
- Düx A. et al. 2020. Measles virus and rinderpest virus divergence dated to the sixth century BCE. Science 368:1367-1370 (AAAS news story).
- Gething P.W. et al. 2011. Modelling the global constraints of temperature on transmission of Plasmodium falciparum and P. vivax. Parasites & Vectors 4:92.
- Gifford-Gonzalez D. 2017. "Animal disease challenges" fifteen years later. Quaternary International 436:283-293 (abstract).
- Rascovan N. et al. 2019. Emergence and spread of basal lineages of Yersinia pestis during the Neolithic decline. Cell 176:295-305 (The Science Breaker summary by the lead author).
- Sinka M.E. et al. 2012. A global map of dominant malaria vectors. Parasites & Vectors 5:69.
- Stenseth N.C. et al. 2008. Plague: past, present, and future. PLoS Medicine 5:e3.
- Wolfe N.D., Dunavan C.P., Diamond J. 2007. Origins of major human infectious diseases. Nature 447:279-283 (abstract only).

From memory (not opened): Livingstone 1958 (American Anthropologist, sickle cell and yam farming); Anastasiou et al.
2014 (Lancet Infect Dis, Syrian schistosome); Aufderheide et al. 2004 (PNAS, Chagas in Chinchorro mummies); Spyrou et
al. 2018 (Nature Comms, flea-adapted plague); Mühlemann et al. 2020 (Science, Viking smallpox); Kocher et al. 2021
(Science, hepatitis B); Pullan & Brooker 2012 (Parasites & Vectors, soil worm limits); Pigott et al. 2014 (eLife,
leishmaniasis); all numbers tagged [M] above.

Could not read: Parasites & Vectors / Springer pages after a rate limit (429) (a tsetse temperature paper and a
schistosomiasis temperature paper), the eLife leishmaniasis page (did not load), PubMed and PMC (captcha), HAL
(access denied).
