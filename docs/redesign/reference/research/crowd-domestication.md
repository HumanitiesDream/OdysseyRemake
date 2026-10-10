# Crowd diseases and full domestication

Research note for Odyssey. 5 Oct 2026. Companion to `diseases-full.md` (the 76-agent register and its stage 0-1-2
machine), `neolithic.md` (centres, wild ancestors, spread rates) and `neolithic-plan.md` (build order, crop groups
that split into varieties). It does not repeat those tables. It adds two things:

- **Part A.** The arithmetic of density in historical epidemiology: how large a linked population a given disease
  needed in order to stay, which diseases suited villages and which needed cities, the dates at which each disease
  is thought to have become a human disease, and the death rates recorded in unexposed versus long-exposed peoples.
- **Part B.** A worldwide domestication table (crops and animals), the dates of secondary uses (milk, wool, traction,
  riding), and a design for crop and herd groups that split into regional varieties.

Tags. **[S]** = a figure from a named study I am confident exists. **[≈]** = a working figure from general textbook or
agronomic knowledge, good to about +-25%, not tied to one paper. Dates: BP = before 1950; BCE/CE as given. Sources
are named by author and year only; no DOIs, on purpose.

---

# Part A. Density-driven diseases

## A1. Critical community size

An acute immunising disease (one bout, then immunity for life) burns through everyone it can reach, then has to wait
for new babies. In a small community the chain of infection breaks during that wait and the disease disappears until
someone brings it back. The smallest population in which it stays on is the **critical community size** (CCS).

| Finding | Number | Source |
|---|---|---|
| Measles in English and Welsh towns | dies out between epidemics below about 250,000-300,000 people; stays above | Bartlett 1957 [S]; Bartlett 1960 [S] |
| Measles on islands | islands under about 500,000 lose it between imports | Black 1966 [S] |
| Measles with births and spatial structure | the threshold depends on the birth rate and on how strongly towns are linked; a well-linked group of towns behaves as one big town | Keeling & Grenfell 1997 [S]; Keeling & Grenfell 2002 [S] |
| Isolated Amazonian peoples before contact | antibodies to latent and chronic infections (herpes family, Epstein-Barr), **none** to measles, mumps, rubella or influenza | Black 1975 [S] |
| Forager disease profile | heirloom and animal-borne infections only; the childhood crowd diseases are a farming-era novelty | Cockburn 1971 [S]; Barrett, Kuzawa, McDade & Armelagos 1998 [S] |
| Where crowd diseases came from | mostly from domestic and commensal animals, through a graded series from animal-only to human-only | Wolfe, Dunavan & Diamond 2007 [S] |

**Why this matters here.** A people in Odyssey tops out near 1,000. A measles-sized threshold therefore means
**250-500 closely linked villages**, or a city with its hinterland. That is a Bronze Age state, not a Neolithic
valley. The effect is exactly what the owner wants: the childhood killers arrive late, and only where the player has
built a real network.

### The threshold moves with the birth rate

Persistence depends on how fast new non-immune children appear. Pre-modern birth rates (about 40-50 per 1,000 a year)
were roughly twice those of 1950s England (15-18), which is the setting Bartlett measured. So a pre-modern measles
threshold is probably lower, perhaps **150,000-300,000** [≈]. A usable form:

```
ccs_eff = ccs_base * (17.0 / birth_rate_per_1000).powf(0.7)   // 0.7 damps it [≈]
```

### What "linked" means

The linked set is the group of peoples that exchange enough travellers often enough that an infection can pass within
the disease's own short window (two to three weeks for measles). Metapopulation work indicates that even about **1% of
contacts crossing between towns** makes them act as one population for persistence [≈, from Keeling & Grenfell 2002
and later work in that line]. For the engine:

```
n_net = sum over peoples p in the component of  pop[p] * min(1.0, link[p] / 0.01)
link[p] = share of p's yearly contacts that are with other peoples in the component
          (trade ties, a shared market centre, a river route, marriage exchange)
```

Peoples more than about ten days' travel from the core should count at a fraction even when linked: a chain of
two-week infections breaks across long, thinly travelled gaps.

## A2. Village diseases and city diseases

The most useful split in the whole subject. It follows from how a disease survives the quiet period between
outbreaks.

| How it survives the gap | Examples | Smallest home | Practical CCS |
|---|---|---|---|
| Latency: hides in the body and wakes later | chickenpox and shingles, herpes simplex, Epstein-Barr, cytomegalovirus | a band of 25 | none |
| Long illness with years of shedding | tuberculosis, leprosy, hepatitis B, yaws and the other treponemes, *P. malariae* and vivax relapses | a band or a village | 0-10,000 |
| Healthy carriers | typhoid (gall bladder carriers), diphtheria, streptococcus, meningococcus | a village or a town | 1,000-100,000 |
| An animal or environmental reservoir | plague (rodents), cholera (brackish water), hookworm (soil), schistosomes (snails), anthrax (soil spores) | wherever the reservoir is | none for persistence; density only sets outbreak size |
| Nothing: short, immunising, human-only | measles, smallpox, mumps, rubella, whooping cough | a city network | 100,000-500,000 |

The game rule: **villages get the chronic, carrier and environmental diseases early; only networks of cities keep the
acute childhood killers.** Until then an acute crowd disease that arrives strikes once and dies out.

## A3. Animal origins and dates

Molecular-clock and ancient-DNA dates. Use each as the **earliest calendar year** the sim will allow (the
`earliest_year` field in `diseases-full.md` 6.2).

| Disease | Came from | Date estimate | What it means in play | Source |
|---|---|---|---|---|
| Measles | a cattle morbillivirus of the rinderpest line | split from rinderpest about the 6th century BCE (range 1174 BCE to 165 CE), from a 1912 Berlin lung specimen | needs cattle plus a large city network; later than usually assumed; Greek and Roman writers do not clearly describe it | Düx et al. 2020 [S] |
| Smallpox | a rodent and camel poxvirus group (gerbil taterapox, camelpox are the closest known) | ancient genomes from Viking-age northern Europe, 600-1050 CE; the diversity of the modern virus is young (17th century onward) | allow spillover early, a recognisable human smallpox from the Bronze Age at the earliest, with the classic form late | Mühlemann et al. 2020 [S]; Duggan et al. 2016 [S] |
| Plague, early ("LNBA" line) | rodents, on and around the steppe | a basal strain at Frälsegården, Sweden, about 4,900 BP, with divergence about 5,700-5,100 BP; the authors point to the very large Trypillia settlements as a likely setting and plague as one cause of the **Neolithic decline** | these early strains lacked the gene that lets fleas carry plague efficiently, so outbreaks were slower and more local | Rascovan et al. 2019 [S]; Rasmussen et al. 2015 [S] |
| Plague, flea-borne | as above | Samara, about 3,800 BP: the first strains with the full flea-borne kit | from about 1800 BCE allow bubonic plague in rodent country; it travels with Bronze Age carts and caravans | Spyrou et al. 2018 [S]; Andrades Valtueña et al. 2017 [S] |
| Typhoid and Paratyphi C | *Salmonella enterica*; the near relatives of Paratyphi C live in pigs | genomes from the Paratyphi C line in Neolithic farmers and herders, about 6,500-5,000 BP; an 800-year-old genome from Norway | comes in with settled herders and pigs, and stays on through carriers | Key et al. 2020 [S]; Zhou et al. 2018 [S] |
| Tuberculosis | human lineages; cattle tuberculosis descends from the human group, not the other way round | ancient genomes suggest a common ancestor under 6,000 years old; other genomic work argues for tens of thousands; bone lesions claimed at Atlit-Yam about 9,000 BP | treat it as present at low level from sedentism, rising with crowding, smoky houses and cattle | Bos et al. 2014 [S]; Comas et al. 2013 [S]; Brosch et al. 2002 [S]; Hershkovitz et al. 2008 [S] |
| Leprosy | human; squirrels and armadillos caught it from us later | a skeleton at Balathal, India, about 2000 BCE | allow from about 2000 BCE in South Asia; spreads slowly | Robbins et al. 2009 [S]; Schuenemann et al. 2018 [S] |
| Falciparum malaria | gorilla parasites | the jump to humans long predates farming | present from the start; farming makes it much worse (below) | Liu et al. 2010 [S] |
| Hookworm and soil worms | ancient human parasites | eggs recovered from pre-Columbian American coprolites | present from the start; the load climbs once people stay put | Araújo and colleagues, coprolite studies [≈] |
| Cholera | a brackish-estuary bacterium of the Bay of Bengal | endemic in the Ganges delta; worldwide pandemics only from 1817 | keep as delta diarrhoea before 1500; no pandemics in a game that ends there | Devault et al. 2014 [S] |
| Pandemic influenza | wild waterfowl, by way of pigs and ducks | well-recorded pandemics only from the 16th century | needs ducks, pigs, rice and cities together; rare | [≈] |

### Malaria gets worse with farming

Livingstone (1958) argued that West African yam farmers, by clearing forest, created the sunlit puddles that *Anopheles
gambiae* breeds in, which raised transmission and in turn selected for the sickle-cell gene. Later work supports the
clearing-and-standing-water link (Sallares 2002 on Roman Italy; Webb 2009 as a world history). Paddy rice and
irrigation canals do the same in Asia. Keep the multiplier already in `diseases-full.md`: a tile going from forest to
farmland roughly triples falciparum.

### Hookworm and the cost of staying put

Hookworm larvae need several days in warm moist soil before they can infect. Mobile foragers have left the camp by
then; settled people walk the same ground daily. A usable form:

```
worm_load = suit(tile) * min(1.0, years_settled / 50.0) * density_factor
```

Load then costs work (5-15% less) and raises child deaths by 1-3 per 1,000 a year [≈], mostly through anaemia.

## A4. How crowding, water, animals and trade spread them

| Driver | Mechanism | Diseases | Engine input |
|---|---|---|---|
| Crowding | more close contacts a day | all airborne ones; tuberculosis above all | people per hectare of settlement, people per house |
| Water | wells beside latrines, shared rivers, canals | typhoid, dysentery, hepatitis A, polio, cholera | years settled, density, irrigation, town size |
| Animals | more contact with animal diseases; pigs and ducks mix influenza; cattle carried the ancestor of measles | measles, influenza, bovine tuberculosis, brucellosis, Paratyphi C | herd head per person, and whether animals are stalled or on open range |
| Stored grain | granaries draw rats and mice, and their fleas | plague, murine typhus, hantavirus | stored surplus, granary towns |
| Trade and war | join networks together, so the linked total rises; carry strains far | every human-adapted disease; plague most of all | trade ties, caravans, ships, armies |
| Famine | weaker resistance, people crowding at relief points, migration | typhus, dysentery; measles deaths rise sharply | a famine flag that multiplies deaths by 1.5-3 [≈] |

## A5. Epidemic or endemic: the numbers

### Reproduction numbers in pre-modern conditions

| Disease | R0 | Infectious span | Source |
|---|---|---|---|
| Measles | 12-18 | about a week | Anderson & May 1991 [S] |
| Whooping cough | 12-17 | about three weeks | Anderson & May 1991 [S] |
| Smallpox | 3.5-6 | about two weeks | Gani & Leach 2001 [S] |
| Mumps | 4-7 | about a week | Anderson & May 1991 [S] |
| Rubella | 6-7 | about a week | Anderson & May 1991 [S] |
| Diphtheria | 6-7 | two to four weeks, carriers longer | Anderson & May 1991 [S] |
| Pneumonic plague | about 1.3 | two to four days, almost always fatal | Gani & Leach 2004 [S] |
| Pandemic influenza | 1.5-3 | about four days | [≈] |

### Three rules that fall out of R0

1. **Can an outbreak start?** Only when the non-immune share is above `1/R0`. For measles that is 6-8%. With about 4%
   of the population born each year, a community below the threshold is ready for measles again **two to five years**
   after the last wave [≈].
2. **How many catch it?** The final size solves `z = 1 - exp(-R0 * s0 * z)`. Fully unexposed: R0 = 15 gives nearly
   100%; R0 = 5 gives 99%; R0 = 3 gives 94%; R0 = 2 gives 80%; R0 = 1.5 gives 58%.
3. **What age, once endemic?** Mean age at infection is roughly `life_expectancy / (R0 - 1)`. Measles with a mean life
   of 30 years: **about two years old**. Smallpox (R0 near 5): seven or eight. This is why these became children's
   diseases and why adults were immune.

### Death rates: unexposed versus long-exposed

| Disease | Endemic, in children | First arrival, all ages | Recorded cases | Source |
|---|---|---|---|---|
| Measles | 1-5% if well fed; 10-25% in famine or with poor nutrition | 20-30% | Fiji 1875: roughly a quarter of about 150,000 died. Faroe Islands 1846: about 6,000 cases in 7,800 people but few deaths, in a small well-fed population with nursing | Panum 1847 [S]; Cliff, Haggett & Smallman-Raynor 2000 [S] |
| Smallpox, major form | 15-30% | 30-50% | Iceland 1707-09 lost roughly a quarter to a third of its people; this is one of the few diseases whose death rate barely fell with familiarity | Fenner et al. 1988 [S]; share for Iceland [≈] |
| Plague, bubonic | 40-60% | the same | pneumonic and septicaemic forms near 100% | [≈] |
| Whooping cough | 1-5% of infants | 5-15% of infants | | [≈] |
| Diphtheria | 10-20% | 20-50% in children | | [≈] |
| Typhoid | 10-20% | 10-30% | | [≈] |
| Tuberculosis, active and untreated | about 70% dead within ten years of falling ill, but only 5-10% of the infected ever fall ill | | | Tiemersma et al. 2011 [S] |

### First-contact ("virgin soil") epidemics

Crosby (1976) gave them the name: a people with no earlier exposure meets a crowd disease and **every age group falls
sick at once**. Jones (2003) added the necessary correction, that the toll also depended on war, famine, forced labour
and the collapse of ordinary care. Both belong in the model:

```
deaths_frac = cfr_first_contact
            * (1.0 + 0.8 * (attack_rate - 0.5).max(0.0))  // nobody left to fetch water or food [≈]
            * famine_mult                                  // 1.0, or 1.5-3.0 in a famine year [≈]
            * (1.0 - host_resist)                          // see A7
```

The large American losses (often put at 50-90% over a century: Crosby 1976, McNeill 1976, Livi-Bacci 2003) came from
**several diseases in sequence** plus conquest, not from one epidemic. A single first-contact measles or smallpox wave
is about 10-30% of a people [≈]. In Odyssey this is the contact shock for the Americas, the Pacific, Australia and
also for cut-off valleys inside Eurasia.

## A6. Suggested rules: when a new crowd disease appears

The historical pattern is that an animal disease touched people many times before one of those occasions started a
chain that lasted, and the chain only lasted where there were enough people linked together. Model it as a yearly
chance, gated hard by the calendar floor from A3.

```
// for each candidate disease d with animal source s, in each linked component k, each year
if year < floor[d] { continue; }                       // the molecular-clock floor from A3
contact = herd_per_person[k][s] * housing[d];          // cattle per person for measles, pigs+ducks for influenza,
                                                       // stored grain * rodents for plague
                                                       // housing: animals stalled indoors 1.0, open range 0.4 [≈]
dens    = (settled_density[k] / 300.0).min(1.0);       // people per km2 of settled land [≈]
size    = ((n_net[k] / ccs[d]).powi(2)).clamp(0.0, 1.0); // next to no chance below a third of the threshold
lambda  = base[d] * contact * dens * size;
p_year  = 1.0 - (-lambda).exp();
```

Set `base[d]` so that with contact, density and size all at 1.0 the average wait is **300 to 1,000 years**. That puts
measles in the late first millennium BCE in a Near Eastern, Indian or Chinese network, matching Düx et al. 2020, and
smallpox earlier where Egypt, Mesopotamia or Arabia grew large networks.

**Once it exists** it is a stage 2 disease in that component. Each year it may still die out:

```
p_fade = 1.0 / (1.0 + (n_net / ccs_eff).powi(4));   // 0.94 at half the threshold, 0.5 at it, 0.06 at twice it [≈]
```

On fade-out the component keeps its immune adults and the disease is gone from there until reimported. While any
component anywhere still holds it, reimport runs on trade: `p_import = 1 - exp(-traffic[k][j] * prevalence[j])`. The
first component over the threshold becomes the **source** that throws an epidemic at every smaller neighbour every few
years. That is Bartlett's town-to-village pattern, and the measured Icelandic measles waves (one per import, every
five to ten years, in Cliff and Haggett's work) are the same thing.

**The endemic switch.** When a disease has held in a component for about 50 years without fading, mark it endemic
there: deaths move to small children (A5 rule 3), adult deaths stop, and the endemic death rate replaces the
first-contact one. That single switch is most of what "becoming a childhood disease" means.

**Village diseases** (the first four rows of A2) skip the `size` term: once a people is settled, `size = 1`. They
still need their reservoir and their density.

### Checkpoints for tuning

| Date | What should be possible | Why |
|---|---|---|
| 4100-3600 BCE | the large Trypillia settlements (10,000-15,000 people, up to about 320 ha) suffer early plague and break apart | Müller, Rassmann & Videiko 2016 for the sizes [S]; Rascovan et al. 2019 for the plague argument [S] |
| 3500-3000 BCE | Uruk-period Mesopotamia, 50,000-100,000 linked people: tuberculosis, typhoid and dysentery endemic; pox-type diseases only as rare epidemics | [≈] |
| 3000-2500 BCE | steppe movement carries the early plague line across Eurasia | Rasmussen et al. 2015 [S] |
| 2000-1000 BCE | flea-borne plague on the steppe; Egypt, Mesopotamia and the Indus each hold half a million to two million; the first networks pass the smallpox threshold | Spyrou et al. 2018 [S]; population figures [≈] |
| 800-300 BCE | measles becomes possible where cattle and a big city network meet | Düx et al. 2020 [S] |
| 165-180 CE, 541 CE | the Antonine and Justinianic pandemics: empire-wide networks, sea routes | Harper 2017 [S] |

## A7. Why death rates fall over the centuries

Three separate things, and the code should keep them apart.

**1. The age shift, which is fast and purely demographic.** As soon as a disease is endemic, the adults are immune and
only young children meet it. Deaths per head of population drop sharply even though nothing about the disease has
changed. This is the biggest effect, and it comes free from A5 and A6.

**2. Change in the disease itself, which is slow and not guaranteed.** The standard theory is a trade-off: a disease
that disables its host too quickly loses chances to pass on, so severity settles at an intermediate level rather than
falling to nothing (Anderson & May 1982; Alizon, Hurford, Mideo & Van Baalen 2009). The textbook case is myxomatosis in
Australian rabbits: released in 1950 killing nearly all infected rabbits, within a few years the common field strains
killed a lower share, while the rabbits themselves became more resistant (Fenner & Ratcliffe 1965; Kerr et al. 2012).
Syphilis in Europe was described as much less severe within a few decades of the 1495 outbreak [≈]. Ewald (1994) adds
the useful qualifier that diseases carried by insects, water or attendants stay severe, because a bedridden host still
passes them on, while those needing the host to walk about do not.

```
// per disease, per component where it is endemic, per year
severity += (severity_floor[d] - severity) / tau[d];
// tau[d] = 300-1,000 years for airborne, directly spread diseases
//        = 2,000+ years for insect-, water- or attendant-borne ones [≈]
cfr = cfr_base * severity;
```

Smallpox is the warning against making this automatic: the major form stayed at 20-30% for centuries, and the mild
form only became common in the 19th century (Fenner et al. 1988). So give each disease its own `tau` and its own floor,
and let a few have a floor close to where they started.

**3. Resistance in the people, which is slow and genetic.** Give each people a `host_resist[d]` between 0 and 0.5 that
creeps up with cumulative losses to that disease:

```
host_resist[d] += 0.1 * deaths_frac[d] * (0.5 - host_resist[d]);   // per generation [≈]
```

Real examples: the malaria-protective blood variants (Livingstone 1958 for the sickle-cell argument; Tishkoff et al.
2007 for the wider pattern of selection in human genes) and the immune-gene changes reported in plague-hit medieval
Europe (Klunk et al. 2022 [S]). Keep the cap at 0.5. Genes never made a population immune, and the simulation should
not let them.

---

# Part B. Full domestication

## B1. The shape of the process

Fuller and colleagues (2014) showed that domestication was **protracted**: two to four thousand years from the first
cultivation to fixed domestic traits in most cereals, with the traits arriving one at a time rather than together.
Purugganan & Fuller (2011) measured the rate of change in the key trait, non-shattering seed heads, at a selection
coefficient near 0.003 a year in wheat and barley, no faster than ordinary natural selection. Larson et al. (2014)
drew the same picture for plants and animals together. Zeder (2011, 2012) sorted the animals into three paths:
**commensal** (dog, cat, chicken, pig: the animal came to us), **prey** (sheep, goat, cattle, camelids: hunting turned
into herd management) and **directed** (horse, donkey, camels: deliberate and much later, mostly for transport).

Trait bundles worth having as flags in the engine. Crops: non-shattering heads, bigger seed, loss of dormancy,
ripening together, compact growth, free-threshing grain. Animals: tameness, smaller body and horns at first, patchy
coats, more fat, wool, milk yield, a longer breeding season.

## B2. Crops

**Mgmt** = first cultivation or management. **Dom** = first clearly domestic form. Climate limits are [≈] working
values for code, not biogeography.

| Crop | Centre(s) | Wild ancestor | Mgmt | Dom | Gap (yrs) | Traits gained | Climate limits for code | Spread | Splits and breakthroughs |
|---|---|---|---|---|---|---|---|---|---|
| Einkorn wheat | Karacadağ, SE Turkey | *Triticum boeoticum* | 9500 BCE | 8500-7500 BCE | 1,500-2,000 | non-shattering, bigger grain | winter rain or snow cover, 300-800 mm, not humid tropics | with the Neolithic, about 1 km a year into Europe (Pinhasi, Fort & Ammerman 2005) | fades after the Bronze Age |
| Emmer wheat | SE Turkey, S Levant | *T. dicoccoides* | 9500 BCE | 8500-7500 BCE | ~2,000 | as einkorn; hulled, so it needs pounding | as above | Europe 6500-4000 BCE, Egypt about 5000 BCE, India about 3000 BCE | **free-threshing durum** about 7000-6500 BCE |
| Bread wheat | S Caspian, Transcaucasia | emmer crossed with goat-grass *Aegilops tauschii* | — | 7000-6000 BCE | one hybrid event | free-threshing, cold and continental tolerance, gluten | winter types survive to -20 C under snow; spring types need 90-120 frost-free days | China 2500-2000 BCE, India by 2500 BCE | can only happen where emmer farming reaches the goat-grass range (36-42 N, 44-60 E): Dubcovsky & Dvorak 2007 |
| Barley | Fertile Crescent, perhaps also eastern Iran | *Hordeum spontaneum* | 9500 BCE | 8000 BCE | ~1,500 | non-shattering, six-row heads, naked grain | the hardiest cereal: 60-90 day season, 200 mm, to 70 N and 4,500 m | with wheat into Europe; China 2500-2000 BCE; Tibet about 1600 BCE | **naked high-altitude barley** opens the Tibetan plateau (d'Alpoim Guedes et al. 2015) |
| Rice, japonica | lower Yangtze | *Oryza rufipogon* | 7000 BCE | 4500-4000 BCE | ~3,000 | non-shattering (sh4, qSH1), fewer awns, more grains per head | 4-5 months above 20 C; paddy or 1,000+ mm | Korea about 1300 BCE, Japan about 900 BCE, mainland SE Asia after 2000 BCE (Stevens & Fuller 2017) | **temperate and tropical japonica** split around the 4.2 ka cooling (Gutaker et al. 2020) |
| Rice, indica | Ganges plain | local cultivated wild rice plus japonica genes | about 5000 BCE | 2000-1500 BCE, after japonica arrived | hybrid | non-shattering taken from japonica | hot lowlands; deepwater and floating forms | across South and SE Asia | arises only when japonica reaches a region already cultivating wild rice (Fuller 2011; Choi et al. 2017) |
| African rice | inner Niger delta | *O. barthii* | about 1500 BCE | about 3,000 BP | — | | wet savanna floodplain | West Africa | Cubry et al. 2018 |
| Broomcorn millet | north China | wild *Panicum miliaceum* | 8000 BCE | 6000 BCE | ~2,000 | | **60-90 day** season, 200-500 mm, to 55 N | reaches Europe about 1500 BCE (Filipović et al. 2020) | the short-season crop for steppe and northern edges |
| Foxtail millet | north China | *Setaria viridis* | 8000 BCE | 6000-5500 BCE | ~2,000 | | 70-120 days, 300-600 mm | Korea 3500 BCE, Europe in the Bronze Age | |
| Maize | Balsas valley, SW Mexico | teosinte *Zea mays parviglumis* | 7000 BCE (starch at 8,700 BP) | small cobs by 4300 BCE | staple only after about 2000 BCE | **cob and kernel row growth**, naked kernels (tga1), one stalk | 100-140 frost-free days (flint types 90), 500-800 mm, base 10 C | SW USA about 2100 BCE (Merrill et al. 2009), Peru by about 4700 BCE (Grobman et al. 2012), eastern North America about 200 BCE | one origin (Matsuoka et al. 2002); half-domesticated maize left Mexico and was improved further in South America (Kistler et al. 2018); the short-season **Northern Flint** type is what makes it a staple in eastern North America by about 900 CE |
| Common bean | Mesoamerica and the Andes, twice over | wild *Phaseolus vulgaris* | ? | about 2500 BCE [≈] | | pods that hold, bigger seeds | 90+ frost-free days | travels with maize as part of the three sisters | two separate gene pools: Schmutz et al. 2014 |
| Squash | Oaxaca (*C. pepo*), northern South America (*C. moschata*), southern Andes (*C. maxima*) | wild *Cucurbita* | 8000 BCE | 8000 BCE by seed size | short | thicker rind, bigger seeds, less bitter | 90+ frost-free days | a separate *C. pepo* domestication in eastern North America about 3000 BCE | Smith 1997 |
| Potato | Titicaca basin, south Peru and north Bolivia | the *Solanum brevicaule* complex | 8000 BCE? | 6000-3000 BCE [≈] | long | fewer bitter glycoalkaloids, bigger tubers | cool growing season 15-20 C, frost kills the leaves, 2,000-4,000 m, short day length | lowland Chile (Chiloé) | one origin (Spooner et al. 2005); the **long-day Chilean type** is the breakthrough that later suits Europe; freeze-dried *chuño* lets the crop be stored for years |
| Quinoa | Titicaca basin | wild *Chenopodium* | about 5000 BCE [≈] | about 1500 BCE [≈] | | thinner seed coat, bigger seed | frost to -4 C, salt-tolerant, 300 mm, to 4,000 m | stays Andean | |
| Sorghum | eastern Sudan (Butana) | *S. bicolor* ssp. *verticilliflorum* | 4th millennium BCE | 3500-3000 BCE | | non-shattering, bigger grain, open heads | hot, 400-800 mm, frost-free, 100-120 days | India about 2000 BCE, West Africa later | Winchell et al. 2017; the **five races** (bicolor, guinea, caudatum, kafir, durra) are the textbook case of regional splitting (Harlan & de Wet 1972) |
| Pearl millet | Tilemsi valley, Mali | wild *Cenchrus americanus* | about 4000 BCE | about 2500 BCE | ~1,500 | non-shattering fixed by 2000 BCE, grain width +38% | 200-600 mm, the hottest and sandiest ground, 60-90 days | India 2000-1700 BCE | Manning et al. 2011 |
| Finger millet | Ethiopian and East African highlands | *Eleusine africana* | ? | ? | | | 1,000-2,500 m, 500-1,000 mm | India by about 1000 BCE [≈] | |
| Guinea yam | Niger basin | *Dioscorea praehensilis* | ? | ? | | bigger, less bitter tubers | 1,000-1,500 mm, 6-10 month season | with the Bantu expansion | Scarcelli et al. 2019 |
| Greater yam | New Guinea and SE Asia | wild *D. alata* | about 5000 BCE (Kuk) | ? | | | wet tropics | Pacific, Madagascar, Africa | |
| Taro | New Guinea, SE Asia | wild *Colocasia esculenta* | about 8000 BCE (Kuk) | no clear marker | | | wet, 1,500+ mm, or pondfield irrigation | Pacific with Lapita; Africa later | Denham et al. 2003 |
| Banana | New Guinea | wild *Musa acuminata banksii* | about 5000 BCE (Kuk) | seedless types later | | **seedless fruit**, then sterile triploids that can only be cloned | frost-free, mean above 15 C, 1,200+ mm | India, then Africa (Cameroon by about 500 BCE: Mbida et al. 2000) | Perrier et al. 2011; crosses with *M. balbisiana* give the plantains, which take drought and cool better |
| Sugarcane | New Guinea | *Saccharum robustum* | about 6000 BCE? [≈] | ? | | thick sweet stems | frost-free, 1,500 mm or irrigation, 10-12 months | India, where hybrids and **crystal sugar** appear around 300 BCE to 0 [≈] | |
| Cotton | Indus (*Gossypium arboreum*), Arabia and Africa (*G. herbaceum*), Yucatán (*G. hirsutum*), coastal Peru and Ecuador (*G. barbadense*) | four separate wild species | about 6000 BCE (fibres at Mehrgarh) | | | long spinnable lint | 180-200 frost-free days, heat | each stays in its own hemisphere before 1500 | Moulherat et al. 2002; Wendel & Grover 2015 |
| Flax | Fertile Crescent | *Linum bienne* | 9000 BCE | 7000 BCE by seed size | | bigger oily seed, later tall **fibre** types | cool, 450-750 mm | with the Neolithic into Europe | wild flax fibres were twisted as early as 30,000 BP (Kvavadze et al. 2009); linen as a fibre crop about 4000-3000 BCE [≈] |
| Grape | the Caucasus and West Asia, two at once | *Vitis vinifera sylvestris* | about 9000 BCE | | | self-fertile flowers, big sweet berries, propagated by cuttings | needs winter chill, min above about -15 C, dry summer, to 50 N | wine residues in Georgia about 6000 BCE (McGovern et al. 2017) | Dong et al. 2023; table and wine lines split early |
| Olive | southern Levant, Jordan valley | *Olea europaea sylvestris* | about 5000 BCE | about 5000 BCE (oil at Kfar Samir: Galili et al. 1997) | | bigger oily fruit, cloned by grafting | Mediterranean only: coldest month above 3 C, absolute minimum above -10 C, hot dry summer | Aegean about 3000 BCE, west Mediterranean about 1000 BCE | Langgut et al. 2019 |
| Date palm | Persian Gulf | wild *Phoenix dactylifera* | about 5000 BCE (Dalma) | | | sweeter fruit, hand pollination | very hot dry summers with groundwater | North Africa, where it crossed with *P. theophrasti* (Flowers et al. 2019) | |

Other crops that deserve a group row: lentil, pea, chickpea and bitter vetch (Fertile Crescent, alongside wheat);
soybean (north China, 3000-1000 BCE) [≈]; mungbean and horsegram (south India: Fuller 2007); manioc (SW Amazonia, by
8000 BCE); peanut (Bolivia); sunflower and chenopod (eastern North America); teff and enset (Ethiopia); cowpea and
oil palm (West Africa); coconut (Indo-Pacific, two origins: Gunn, Baudouin & Olsen 2011).

## B3. Animals

| Animal | Centre(s) | Wild ancestor | Mgmt | Dom | Path | Traits gained | Limits for code | Breeds and spread |
|---|---|---|---|---|---|---|---|---|
| Dog | Eurasia, place still argued | Pleistocene grey wolf | before 15,000 BP | burial at Bonn-Oberkassel about 14,200 BP | commensal | tameness; **extra starch-digesting genes** where people farmed (Axelsson et al. 2013) | none | five distinct lineages by about 11,000 BP (Bergström et al. 2020); sled dogs at Zhokhov about 9,500 BP (Sinding et al. 2020) |
| Cat | Near East, later Egypt | *Felis silvestris lybica* | a cat buried with a person on Cyprus about 9,500 BP (Vigne et al. 2004) | Egypt, 2nd millennium BCE | commensal | mild tameness only; coat patterns are medieval and later | none | spreads with grain stores and ships (Ottoni et al. 2017) |
| Sheep | upper Euphrates, Taurus-Zagros | Asiatic mouflon *Ovis orientalis* | 11,000 BP | about 10,500 BP | prey | smaller horns; **wool** replacing hair; more milk; fat tails | hill steppe to 2,500 m; poor on waterlogged ground | into Europe with farming; fat-tailed breeds in the dry Near East; wool breeds from the 4th millennium BCE |
| Goat | central Zagros | bezoar *Capra aegagrus* | managed by 9,900 BP | 9,000-8,500 BP | prey | tameness, milk, cashmere in cold uplands | rocky ground to 4,000 m; the most heat and drought tolerant ruminant | two main maternal lines carried west and east (Naderi et al. 2008); the poor family's animal everywhere |
| Taurine cattle | upper and middle Euphrates | aurochs *Bos primigenius* | 11,000-10,500 BP | 10,500-10,000 BP | prey | big size loss (aurochs bulls near 1,000 kg down to 300-400 kg), docility, milk | needs grass and water: 300+ mm or floodplain; stopped by tsetse country | all taurine cattle trace to a small founder herd, perhaps 80 cows (Bollongino et al. 2012) |
| Zebu cattle | Indus borderlands, Mehrgarh | Indian aurochs *B. p. namadicus* | 7000 BCE? | 6500-5500 BCE | prey | heat and drought tolerance, the shoulder hump | hot monsoon lands; better than taurine above 30 C | zebu genes later cross into African and Near Eastern herds; in Africa heavily after 1000 CE |
| African cattle | north Africa, Sahara | local aurochs *B. p. africanus*, mixed into Near Eastern stock | | from about 7000-6000 BCE in the green Sahara | prey | | Sahel and savanna; tsetse keeps them out of the wet forest, except the trypanotolerant west African breeds | Sahara herders by 7,000 BP; into east Africa 4500-3000 BP (Gifford-Gonzalez 2017) |
| Water buffalo | Indus and India (river type), China and SE Asia (swamp type) | *Bubalus arnee* | | river type 3000-2500 BCE, swamp type about 2000 BCE | prey | strength in mud, very rich milk | needs water to wallow, coldest month above 12 C, 1,000+ mm | the plough animal of the paddy world; river type gives the dairy buffalo of South Asia |
| Yak | Tibetan plateau, Qinghai | wild yak *Bos mutus* | | about 2500 BCE, perhaps 5,000 BP | prey | cold tolerance, wool, pack carrying | above 2,500 m; suffers below about 2,000 m in the heat | crosses with cattle give the fertile-female dzo; opens high Tibet together with naked barley |
| Pig | Anatolia, and separately China | wild boar *Sus scrofa* | 10,500 BP | 10,300 BP Anatolia, about 6500 BCE China | commensal | fast growth, fat, large litters, shorter snout | forest and marsh; no good on open steppe; cannot be driven far | European wild boar genes later swamp the imported Near Eastern pigs (Larson et al. 2007); the village and town animal, not the herder's |
| Horse | lower Volga-Don steppe | wild *Equus ferus* | Botai about 3500 BCE was a separate, dead-end line | the modern line (DOM2) about 2200 BCE | directed | docility, a stronger back, a calmer temper (the GSDB2 and ZFPM1 changes), speed | dry grassland, 200-600 mm; needs grass under snow, so it beats cattle on the winter steppe; poor in the wet tropics and in tsetse country | DOM2 spreads over Eurasia from about 2000 BCE, reaching Bohemia, the Danube and Anatolia by 2200-2000 BCE (Librado et al. 2021) |
| Donkey | northeast Africa, Nubia and the Horn | African wild ass *Equus africanus* | | about 5000 BCE | directed | load carrying, drought tolerance, sure feet | arid hills, 100-300 mm; cheap to keep | reaches Eurasia about 2,500 years later (Todd et al. 2022); the pack animal that makes overland trade pay |
| Dromedary | southeast Arabia | the now-extinct wild dromedary | | about 1200-1000 BCE as a managed herd animal | directed | water economy, fat hump, load and riding | hot desert under 200 mm; cannot take cold wet winters | wild animals kept being crossed in after domestication (Almathen et al. 2016); makes the incense and Sahara routes possible |
| Bactrian camel | Central Asian deserts | an extinct wild Bactrian, not today's *Camelus ferus* | | 3000-2500 BCE | directed | two humps, cold desert tolerance, heavy loads | cold desert, to -20 C, under 250 mm | the Silk Road animal; crosses with the dromedary give the strong first-generation hybrid |
| Llama | central Andean puna | guanaco *Lama guanicoe* | | about 4,000 BP | prey | load carrying (25-45 kg), coarse wool, dung for fuel | 2,500-4,800 m; suffers in hot lowlands; no riding, no traction | reaches Bolivia about 3,500 BP, northwest Argentina 3,100-2,400 BP; caravans of hundreds of animals by the Tiwanaku period |
| Alpaca | Telarmachay, Junín puna | vicuña *Vicugna vicugna* | | 6,000-5,500 BP | prey | very fine fleece | 3,500-4,800 m, wet meadows | the fibre animal; fine-fleece breeds under the Inca |
| Guinea pig | central Andes | *Cavia tschudii* | | 5000-2500 BCE | commensal | bigger body, tameness, breeding year round | kept indoors anywhere warm enough | the household meat animal of the Andes |
| Chicken | central Thailand and nearby, drawn in by stored rice | red junglefowl *Gallus gallus spadiceus* | | about 1650-1250 BCE (Ban Non Wat) | commensal | laying through the year, bigger body, loss of the broody pause | coldest month above about 10 C in the wild; kept further north once housed | reaches Mesopotamia and Egypt late in the 2nd millennium BCE, Europe and Ethiopia by about 800 BCE (Peters et al. 2022) |
| Turkey | south and central Mexico, and separately the US southwest | wild turkey *Meleagris gallopavo* | | about 300 BCE to 100 CE in Mexico; 0-500 CE in the southwest | commensal | tameness, size; in the southwest kept mainly for feathers at first | oak and pine country | Speller et al. 2010 |
| Muscovy duck | lowland South America | *Cairina moschata* | | pre-Columbian, undated [≈] | commensal | | wet tropics | |
| Silkworm | north and central China | wild *Bombyx mandarina* | | about 3000 BCE; silk at Qingtai about 3600 BCE, thread at Jiahu claimed about 8,500 BP | directed | **completely dependent on people**: cannot fly, will not escape, feeds only on mulberry | needs mulberry; indoor rearing at 20-28 C | the single most jealously guarded domesticate; reaches Korea and Japan in the 1st millennium BCE, the Mediterranean in the 6th century CE |
| Honeybee | the Near East and Egypt for hive-keeping; *Apis mellifera* was already tended in the wild everywhere | *Apis mellifera* | honey-hunting from the Palaeolithic (Altamira and Spanish rock art) | pottery hives in Egypt by about 2400 BCE; beeswax in Neolithic pots from about 7000 BCE in Anatolia (Roffet-Salque et al. 2015) | directed, and never fully tame | nothing changes in the bee; what changes is the hive | needs flowering plants and a mild enough winter; no honeybees in the Americas or Australia before 1600 CE | the wax matters as much as the honey: lamps, casting, writing tablets, sealing |

## B4. Secondary uses and their dates

Sherratt (1981, 1983) called this the **secondary products revolution**: milk, wool, traction and riding came
centuries or millennia after the animals themselves, and each one changed the economy more than the first
domestication did.

| Use | Animal | Earliest date | Evidence | What it unlocks in the sim |
|---|---|---|---|---|
| Milk | sheep, goat, cattle | 7th millennium BCE, northwest Anatolia; the green Sahara by about 5200 BCE; cheese sieves in Poland in the 6th millennium BCE | milk fat residues in pottery (Evershed et al. 2008, 2022; Dunne et al. 2012; Salque et al. 2013) | 3-5 times more food energy per animal over its life; lets herders live on dry grassland. Energy reaches people as cheese, butter and soured milk from the start; fresh milk for adults only in proportion to lactase persistence, which is still under 10% in Europe at 3000 BCE (see `animals-foods.md` B4) |
| Wool | sheep | woolly sheep from about 4000-3000 BCE; textiles and spindle whorls follow | sheep figurines, flock sex ratios (more wethers), textile finds | a trade good and a reason to keep male animals alive; the Uruk and Bronze Age textile industries |
| Traction, ard ploughing | cattle | about 4000-3500 BCE | plough marks under barrows, cattle pathology, the Bronocice pot with a four-wheeled wagon, about 3500 BCE | more land per worker, so heavier soils and real surpluses; a trigger for chiefdoms in `chiefdoms-war.md` |
| Cart and wagon | cattle, then onager and donkey | about 3500-3300 BCE across Europe and Mesopotamia | wheels, wagon models, the Standard of Ur with battle carts | bulk transport, overland trade, the first war carts |
| Pack carrying | donkey, then camels, llama | donkey from about 3000 BCE in Egypt; dromedary from about 1000 BCE; llama caravans from about 1000 BCE | donkey burials at Abydos; caravan records | long-range trade without rivers; raises network linkage, which feeds Part A |
| Riding (horse) | horse | ridden by the Yamnaya from about 3000 BCE on skeletal evidence; the firm archaeological horizon is about 2000-1500 BCE | "horseman syndrome" in Yamnaya skeletons (Trautmann et al. 2023); bits, cheekpieces | scouting, raiding, herding over long distances |
| Light chariot | horse pair | spoked-wheel chariots at Sintashta, about 2000-1800 BCE; the Kikkuli training text about 1350 BCE | Sintashta burials, Anatolian and Egyptian texts | a prestige weapon that needs two trained horses, a wheelwright and a composite bow: a Bronze Age elite only |
| Cavalry proper | horse | 1000-800 BCE (Assyrian, then the steppe nomads) | reliefs, horse size increase | the mounted nomad who will not settle |
| Dung as fuel and manure | cattle, yak, camelids | with herding, everywhere | | fuel on treeless plateaus and steppe; manured fields yield 20-40% more [≈] |
| Blood and traction in Africa | cattle | with Nilotic and East African herding | | pastoral diets without grain (see `animals-foods.md` B3) |

## B5. Simulation design: crop groups, varieties and breakthroughs

This follows the owner's note in `neolithic-plan.md` (about ten crop groups, each with a wild-ancestor bit, splitting
into local varieties that can make breakthroughs).

```rust
struct CropGroup {        // ~12-16 of these worldwide: wheats, barley, rice, millets, maize,
    id: CropId,           // beans, squash, roots and tubers, bananas, fibres, tree fruits, pulses
    wild: WildBit,
    dom: f32,             // 0..1 domestication score, shared across the group
}

struct Variety {          // what a people actually sows; many per group
    group: CropId,
    home_lat: f32,        // the latitude and day length it is used to
    t_min: f32,           // coldest month it tolerates
    season_days: u16,     // frost-free days needed
    rain_mm: (u16, u16),
    yield_mult: f32,
    traits: VarFlags,     // FREE_THRESHING | NAKED | SHORT_SEASON | IRRIGATED | STORABLE | FIBRE | OIL
    parent: Option<VarId>,
}
```

**How a variety forms.** A people that has grown a variety outside its comfort zone for long enough makes a local one.

```
drift_pressure = mismatch(variety, tile) * years_grown / 200.0
p_new_variety_per_year = 0.002 * drift_pressure * (1.0 + 0.5 * is_island)   // [≈]
```

The new variety keeps the parent's traits, shifts one limit toward the tile (about 1 degree of latitude or 5-10 frost-free
days per few centuries, matching the owner's existing rule) and loses a little yield. Varieties should be named for the
region, so the player sees "Highland barley" or "Short-season maize" appear on their own.

**Breakthroughs.** Rare, cheap to code, and the moments a player remembers. Each needs a precondition, so they cannot
happen anywhere.

| Breakthrough | Precondition | Effect |
|---|---|---|
| Non-shattering (any cereal) | the group's `dom` reaches 1.0 | harvest loss drops from 30-50% to under 10% |
| Free-threshing wheat | emmer grown for 1,500+ years | processing labour down by half |
| Bread wheat | an emmer-growing people in a tile with the goat-grass bit | a new variety with `t_min` -20 C and gluten: opens temperate and continental Europe and north China |
| Naked high-altitude barley | barley grown above 2,500 m for 500+ years | opens the Tibetan plateau |
| Short-season maize (flint) | maize grown above 38 N for 1,000+ years | `season_days` drops from 130 to 90: opens eastern North America and the upper Midwest |
| Teosinte to maize cob growth | staged: each 500 years of tending raises cob yield by 15-25% until the full crop is reached, roughly 4,000 years in total | the slowest and most visible domestication in the game |
| Temperate japonica | rice grown above 32 N for 1,000+ years | `t_min` falls: opens Korea, Japan and the Huai |
| Indica rice | japonica reaching a people already cultivating wild rice in the Ganges | a new high-yield tropical variety |
| Paddy and bunded fields | rice plus 300 years of settled farming on a floodplain | yield up 50-100%, and schistosomiasis and malaria up with it |
| Seedless banana, then the plantain triploid | banana tended 1,000+ years, then crossed with the second wild species | the staple of the wet tropics; only propagates by cuttings, so it spreads with people rather than by seed |
| Freeze-dried potato (chuño) | potato above 3,500 m with hard frosts | multi-year storage: the base of an Andean state |
| Grafting and cuttings | grape, olive, date tended 500+ years | fixes a good individual as a clone: orchards become real property and a reason for inheritance |
| Fibre flax, then linen | flax plus 500 years of settled life | a cloth industry before wool |

**Herd groups** work the same way. A `HerdGroup` has a domestication score and secondary-use flags unlocked by
elapsed time and herd size, and it splits into breeds: fat-tailed sheep in dry country, woolly sheep where it is cold
and people spin, zebu-crossed cattle where it is above 30 C, trypanotolerant cattle at the edge of tsetse country,
large horses where fodder grain is grown, small hardy horses on the steppe. One breed, one or two changed limits, a
little better at something and a little worse at another: the same cheap shape as crop varieties, and over a long game
it draws a map of regional agriculture by itself.

---

## Sources

Alizon, Hurford, Mideo & Van Baalen 2009 (virulence trade-off review) · Almathen et al. 2016 (dromedary) ·
Anderson & May 1982, 1991 (R0, virulence) · Andrades Valtueña et al. 2017 (Bronze Age plague) · Araújo et al.
(coprolite parasites) · Axelsson et al. 2013 (dog starch genes) · Barrett, Kuzawa, McDade & Armelagos 1998 (third
epidemiological transition) · Bartlett 1957, 1960 (critical community size) · Bergström et al. 2020 (dog lineages) ·
Black 1966 (island measles), 1975 (Amazonian serology) · Bollongino et al. 2012 (cattle founder herd) · Bos et al.
2014 (pre-Columbian tuberculosis) · Brosch et al. 2002 (MTBC phylogeny) · Choi et al. 2017 (rice origins) · Cliff,
Haggett & Smallman-Raynor 2000 (island epidemics) · Cockburn 1971 (disease in prehistory) · Comas et al. 2013 (TB
with human migration) · Crosby 1976 (virgin soil epidemics) · Cubry et al. 2018 (African rice) · d'Alpoim Guedes et
al. 2015 (Tibetan barley) · Denham et al. 2003 (Kuk) · Devault et al. 2014 (19th-century cholera genome) · Dong et
al. 2023 (grape genomes) · Dubcovsky & Dvorak 2007 (polyploid wheat) · Duggan et al. 2016 (17th-century variola) ·
Dunne et al. 2012 (Saharan dairying) · Düx et al. 2020 (measles divergence) · Evershed et al. 2008, 2022 (milk
residues, lactase) · Ewald 1994 (transmission mode and virulence) · Fenner & Ratcliffe 1965, Kerr et al. 2012
(myxomatosis) · Fenner et al. 1988 (smallpox) · Filipović et al. 2020 (broomcorn millet in Europe) · Flowers et al.
2019 (date palm) · Fuller 2007, Fuller et al. 2014 (protracted domestication), Fuller 2011 (rice) · Galili et al.
1997 (Kfar Samir oil) · Gani & Leach 2001, 2004 (smallpox and plague R0) · Gifford-Gonzalez 2017 (African
pastoralism) · Grobman et al. 2012 (Peruvian maize) · Gunn, Baudouin & Olsen 2011 (coconut) · Gutaker et al. 2020
(rice climate history) · Harlan & de Wet 1972 (sorghum races) · Harper 2017 (Rome and disease) · Hershkovitz et al.
2008 (Atlit-Yam TB) · Jones 2003 (why American epidemics were so deadly) · Keeling & Grenfell 1997, 2002
(persistence, metapopulations) · Key et al. 2020 (Neolithic Salmonella) · Kistler et al. 2018 (South American maize)
· Klunk et al. 2022 (plague selection) · Kvavadze et al. 2009 (wild flax fibres) · Langgut et al. 2019 (olive) ·
Larson et al. 2007 (pigs), 2014 (domestication synthesis) · Librado et al. 2021 (horse origins) · Liu et al. 2010
(falciparum from gorillas) · Livi-Bacci 2003 (American population collapse) · Livingstone 1958 (farming and sickle
cell) · Manning et al. 2011 (pearl millet) · Matsuoka et al. 2002 (maize single origin) · Mbida et al. 2000 (banana
in Cameroon) · McGovern et al. 2017 (Georgian wine) · McNeill 1976 (Plagues and Peoples) · Merrill et al. 2009
(maize in the southwest) · Moulherat et al. 2002 (Mehrgarh cotton) · Mühlemann et al. 2020 (Viking smallpox) ·
Müller, Rassmann & Videiko 2016 (Trypillia mega-sites) · Naderi et al. 2008 (goats) · Ottoni et al. 2017 (cats) ·
Panum 1847 (Faroe measles) · Perrier et al. 2011 (banana) · Peters et al. 2022 (chicken) · Pinhasi, Fort & Ammerman
2005 (Neolithic spread rate) · Piperno 2011 (Balsas maize) · Purugganan & Fuller 2011 (rate of selection) ·
Rascovan et al. 2019 (Neolithic decline plague) · Rasmussen et al. 2015 (Bronze Age plague genomes) · Robbins et al.
2009 (Balathal leprosy) · Roffet-Salque et al. 2015 (beeswax) · Sallares 2002 (Roman malaria) · Salque et al. 2013
(cheese sieves) · Scarcelli et al. 2019 (yam) · Schmutz et al. 2014 (bean gene pools) · Schuenemann et al. 2018
(medieval leprosy) · Sherratt 1981, 1983 (secondary products) · Sinding et al. 2020 (sled dogs) · Smith 1997
(squash), 2011 (eastern North America) · Speller et al. 2010 (turkey) · Spooner et al. 2005 (potato) · Spyrou et al.
2018 (flea-borne plague) · Stevens & Fuller 2017 (Asian crop spread) · Tiemersma et al. 2011 (untreated TB
outcomes) · Tishkoff et al. 2007 (lactase selection) · Todd et al. 2022 (donkey) · Trautmann et al. 2023 (Yamnaya
riding) · Vigne et al. 2004 (Cyprus cat) · Webb 2009 (world history of malaria) · Wendel & Grover 2015 (cotton) ·
Winchell et al. 2017 (sorghum) · Wolfe, Dunavan & Diamond 2007 (origins of human diseases) · Zeder 2011, 2012
(animal domestication pathways) · Zhou et al. 2018 (Paratyphi C).
