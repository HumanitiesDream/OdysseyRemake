# Culture That Evolves, and Horse Nomads with Chariots

Research notes for Odyssey. Part A covers a dynamic culture system. Part B covers horse domestication, chariots and pastoral nomads. Each part ends with a concrete design for the sim. Dates are approximate and given in BCE unless marked otherwise. Numbers marked "tunable" are suggested starting values, not facts.

---

# PART A: A CULTURE SYSTEM THAT EVOLVES BY ITSELF

## A1. What Crusader Kings III does

CK3 (Paradox, 2020; overhauled by the Royal Court expansion, 2022) treats culture as a named object that provinces and characters belong to. Each culture has:

- **Four pillars.** *Ethos* (one of about seven broad value sets, such as Bellicose, Stoic, Egalitarian, Communal, Courtly, Inventive, Spiritual). *Heritage* (a family grouping, such as North Germanic or Iranian). *Language* (decides whether rulers can talk easily). *Martial custom* (who may be a soldier or knight: men only, women only, or both).
- **Traditions.** Up to about five perks, such as "Mountain Homes", "Seafarers", "Hit-and-Run Tactics", or "Polygamous". Each has a mechanical bonus.
- **Innovations.** A tech tree per era (tribal, early medieval, high medieval, late medieval). Cultures research these slowly. Some spread from neighbours.
- **Divergence.** A player (or AI ruler) with enough prestige and support can create a new culture that splits off from a parent. They keep some pillars and pick new traditions.
- **Hybridisation.** A ruler can merge two cultures present in the realm into one new hybrid culture.
- **Acceptance.** Each pair of cultures in a realm has an acceptance score from 0 to 100. It rises over time when they share a realm and lowers friction, unrest and opinion penalties.
- **Conversion.** Counties change culture when a ruler's councillor works on them for years.

### Known weaknesses (the "good ideas poorly implemented" part)

- **Cultures are static.** A culture's traditions do not change unless a ruler pays to change them. Five centuries pass and Norse culture is the same Norse culture.
- **Change is player-driven.** Divergence and hybridisation are buttons. They cost prestige. They need a ruler's decision. No culture splits because villages drifted apart.
- **No drift.** There is no slow random change. There is no isolation effect. Iceland stays Norse forever.
- **Traditions are menu picks.** They come from a fixed list and are chosen, not grown. A desert culture can pick "Seafarers" if it meets a small check.
- **One culture per county.** Places flip from one label to another. There is no mixed zone, no gradient, no gradual blending.
- **Traits do not feed back in a rich way.** Traditions give flat bonuses (+10% levies). They do not change birth rates, inheritance outcomes, or how a society develops.
- **Language is a label.** It does not drift. Dialect chains do not exist.
- **Acceptance is pairwise and abstract.** It rises with time, not with marriages, trade or shared ritual.

The lesson for Odyssey: keep CK3's idea of a culture as a bundle of parts, its divergence and hybridisation, and its acceptance score. But make all of them emerge from the bottom up. Peoples carry traits. Traits drift, copy and spread. Named cultures are labels we put on clusters, not objects that cause things.

## A2. What cultural evolution science says

### Culture behaves like an inheritance system

Cavalli-Sforza and Feldman (1981, *Cultural Transmission and Evolution*) modelled culture with the maths of population genetics. They separated three routes:

- **Vertical** (parent to child). Slow and conservative. Religion and kinship rules often travel this way.
- **Oblique** (older non-parent to young person). Teachers, elders.
- **Horizontal** (peer to peer). Fast. Fashions, new tools, slang.

Vertical routes change slowly. Horizontal routes can sweep through a population in one generation.

### Transmission biases

Boyd and Richerson (1985, *Culture and the Evolutionary Process*) named the main biases in who people copy:

- **Conformist bias.** People copy the most common variant more than its share would predict. It makes groups internally similar and keeps them different from each other. It also slows the spread of rare new ideas.
- **Prestige bias.** People copy the successful and admired. Elites and winning groups get copied. This spreads things that come with success, even useless ones (dress, names, burial style).
- **Guided variation.** Individuals try things and keep what works. Over time this pushes culture toward what suits the local environment. It is slow and depends on how easy it is to judge results.
- **Content bias.** Some ideas are easier to remember or more attractive in themselves.

Henrich and Gil-White (2001, *Evolution and Human Behavior*) set out the theory of prestige in more detail. Henrich (2016, *The Secret of Our Success*) is a readable summary of the whole field. Mesoudi (2011, *Cultural Evolution*) is the standard textbook. It argues that culture shows variation, competition and inheritance, so Darwinian tools apply.

### Groups are more different in culture than in genes

Bell, Richerson and McElreath (2009, *PNAS*) estimated that neighbouring groups differ more in their values than in their genes. Conformity and group-level selection keep cultural boundaries sharp even when people intermarry. This is why the sim should allow genes and culture to move separately.

### Population size and cultural loss

Henrich (2004, *American Antiquity*) used Tasmania as a case. Tasmania was cut off by rising seas around 10,000 years ago. Its population was perhaps 4,000 people. Over the following millennia, Tasmanians appear to have lost bone tools, fishing, and cold-weather clothing that mainland Australians kept. Henrich's model: copying is imperfect. Most copies are a bit worse than the best model. Only a large pool of learners produces enough lucky improvements to keep complex skills alive. Small, isolated groups lose complex skills.

Powell, Shennan and Thomas (2009, *Science*) and Kline and Boyd (2010, *Proceedings of the Royal Society B*) found similar links between connected population size and toolkit complexity. Critics point out that environmental risk matters too. Both effects belong in the sim.

### Phylogenies of culture

Mace and Holden (2005, *Trends in Ecology and Evolution*) argued that cultural traits can be studied on language family trees, like genes on species trees. Holden and Mace (2003, *Proceedings of the Royal Society B*) found that in Bantu-speaking Africa, groups that took up cattle tended to lose matrilineal descent. Cattle are wealth that sons can inherit and use for bridewealth. This is a clean example of subsistence driving kinship. Currie and colleagues (2010, *Nature*) found that political complexity rose step by step across Austronesian societies, but could fall by more than one step at once. Watts and colleagues (2016, *Nature*) linked ritual human sacrifice in Austronesian societies to the rise of social stratification.

Gray and Atkinson (2003, *Nature*) dated Indo-European with Bayesian trees at about 7,800 to 9,800 years ago (an Anatolian origin). Chang and colleagues (2015, *Language*) got a younger date fitting a steppe origin, which ancient DNA supports (Haak et al. 2015, *Nature*). Either way, languages and cultures form trees with some borrowing between branches.

### How fast language changes

Swadesh (1955) and Lees (1953) proposed glottochronology. Lees estimated about 81% retention of a 200-word basic vocabulary per 1,000 years. That is about 19 to 20% replaced per thousand years. Critics (Bergsland and Vogt 1962, *Current Anthropology*) showed rates vary a lot. Icelandic changed very little; Norwegian changed much more. Pagel, Atkinson and Meade (2007, *Nature*) found that frequently used words change much more slowly than rare ones. So use 20% per 1,000 years as a rough average, with real variation:

- Isolated, small, literate or conservative groups: slower (perhaps 5 to 10% per 1,000 years).
- Groups in heavy contact, after conquest, or with language shift: faster (30% or more).
- Two dialects become hard to understand after roughly 1,000 to 2,000 years apart. They become separate languages that need learning after 2,000 to 3,000 years.

### Ethnic boundaries

Barth (1969, *Ethnic Groups and Boundaries*) made a key point. Ethnic groups persist even when people and goods flow across their borders. What keeps them apart is a small set of markers the groups choose to treat as important (dress, food rules, language, a ritual). The content inside the boundary can change while the boundary stays. For the sim: identity should not equal the full trait vector. A few "marker" traits matter more for who counts as "us".

### The Ethnographic Atlas

Murdock (1967, *Ethnographic Atlas*) coded about 1,200 societies on dozens of variables. D-PLACE (Kirby et al. 2016, *PLoS ONE*) puts this data online. Key variables usable in the sim:

- **Subsistence** (shares from gathering, hunting, fishing, herding, farming).
- **Settlement pattern** (nomadic, seminomadic, hamlets, villages, towns).
- **Descent** (patrilineal, matrilineal, bilateral, double).
- **Post-marital residence** (patrilocal, matrilocal, neolocal, avunculocal).
- **Marriage** (monogamy, limited polygyny, general polygyny, polyandry).
- **Marriage payment** (bridewealth, brideservice, dowry, exchange, none).
- **Inheritance of land and movable property** (to sons, daughters, sister's sons, all children, none).
- **Political complexity** (number of levels above the local community: 0 to 4).
- **Class stratification** (none, wealth-based, elite, caste-like).
- **Slavery** (absent, present).

Regularities: patrilocality is most common; matriliny is often lost under cattle herding; dowry goes with plough farming and stratification.

### Values that differ between societies

- Nisbett and Cohen (1996, *Culture of Honor*) linked herding economies to honour cultures. Herds are easy to steal, so a reputation for violent revenge protects wealth.
- Gelfand and colleagues (2011, *Science*) measured "tight" vs "loose" cultures. Tight cultures have strong norms and punish deviance. Tightness rises with threat: war, disease, famine, and density.
- Talhelm and colleagues (2014, *Science*) linked rice farming in China to more interdependent thinking than wheat farming.
- Alesina, Giuliano and Nunn (2013, *Quarterly Journal of Economics*) linked historical plough use to more restrictive gender roles today. Boserup (1970) made the original argument.
- Schulz and colleagues (2019, *Science*) linked the breakup of intensive kin networks to individualist values. Henrich (2020, *The WEIRDest People in the World*) expands this.
- Axelrod (1997, *Journal of Conflict Resolution*) built a simple agent model. Neighbours who share more traits interact more and become more alike. The result is local convergence but lasting global diversity. This is a useful base model for the sim.

### Frontiers and cohesion

Turchin (2003, *Historical Dynamics*; 2006, *War and Peace and War*) argued that large states form on "metaethnic frontiers". These are zones where peoples with very different cultures face each other, for example farmers and steppe nomads. Pressure from the other side raises **asabiya**, a word from Ibn Khaldun meaning group cohesion and the ability to act together. High asabiya lets a group grow. In the heartland, far from the frontier, asabiya slowly decays. Elites compete and the group fragments. Turchin and colleagues (2013, *PNAS*) modelled the spread of large states across Afro-Eurasia from 1500 BCE to 1500 CE. Military technology spreading from the steppe frontier was a key driver. For the sim: cohesion should rise with nearby cultural difference and threat, and fall with size, peace and elite competition.

## A3. Design: culture as a vector of traits

### Core idea

Each people carries its own **culture vector**. Named cultures (Natufian, Yamnaya, Sintashta) are clusters of similar vectors. They are names for what emerges, not the cause. Every year, each people's vector changes a little from four forces: drift, conformity to neighbours, prestige copying, and guided adaptation. Contact pulls vectors together. Isolation lets them drift apart. When a cluster stretches too far, it splits and a new culture gets a name. When two clusters mix enough, a hybrid forms.

### Trait list

Use two kinds of traits. **Continuous** traits are numbers from -1 to +1 (or 0 to 1). **Discrete** traits are choices from a list. Each trait has a **transmission route** (vertical, horizontal, or mixed), a **base drift rate**, and a **marker weight** (how much it counts for identity, after Barth).

Continuous values (-1 to +1):

| Trait | Low end | High end | Route | Drift / yr | Marker |
|---|---|---|---|---|---|
| collectivism | individualist | collectivist | vertical | 0.002 | 0.5 |
| honour | low revenge | strong honour and revenge | mixed | 0.003 | 0.5 |
| tightness | loose norms | tight norms | mixed | 0.003 | 0.3 |
| hierarchy | egalitarian | accepts rank | mixed | 0.002 | 0.5 |
| martial | peaceful | warlike, glorifies war | horizontal | 0.004 | 0.6 |
| gender_roles | flexible | strict separation | vertical | 0.002 | 0.4 |
| xenophilia | closed to outsiders | open to outsiders | horizontal | 0.004 | 0.2 |
| innovation | traditional | open to new ways | horizontal | 0.004 | 0.2 |
| ancestor_focus | little | strong ancestor cult | vertical | 0.002 | 0.6 |
| mobility_pride | settled ideal | mobile ideal | mixed | 0.002 | 0.5 |
| feasting | modest | lavish competitive feasts | horizontal | 0.004 | 0.4 |

Discrete traits:

| Trait | Options | Route | Change chance / yr | Marker |
|---|---|---|---|---|
| descent | patrilineal, matrilineal, bilateral, double | vertical | 0.0005 | 0.6 |
| residence | patrilocal, matrilocal, neolocal, avunculocal | vertical | 0.001 | 0.3 |
| marriage | monogamy, limited polygyny, general polygyny, polyandry | vertical | 0.001 | 0.4 |
| marriage_payment | none, exchange, brideservice, bridewealth, dowry | mixed | 0.001 | 0.3 |
| inheritance | equal, sons, eldest son, daughters, sister's sons | vertical | 0.001 | 0.3 |
| religion_style | animist, ancestor cult, shamanic, temple/priestly, sky-god | mixed | 0.0008 | 0.9 |
| burial | exposure, pit, cist, mound/kurgan, cremation, house floor | vertical | 0.001 | 0.8 |
| art_style | an id in the art-style table | horizontal | 0.003 | 0.7 |
| ritual_style | an id in the ritual table | mixed | 0.002 | 0.7 |
| war_style | ambush, raid, champion duel, massed infantry, chariot, mounted archery | horizontal | 0.002 | 0.4 |
| food_taboos | a bitset of up to 8 taboos (pig, fish, horse, dog, milk, etc.) | vertical | 0.0005 per bit | 0.7 |
| dress_marker | id | horizontal | 0.003 | 0.8 |

Language is tracked separately (see below) but counts as a marker trait with weight 1.0.

"Drift / yr" is the standard deviation of the random step per year for continuous traits. A random walk spreads with the square root of time, so 0.003 per year gives about 0.095 over 1,000 years. That is gentle. Selection and copying, not drift, should drive most change. Drift matters most for markers in isolated groups.

### Yearly update for each people

For each people *p* with vector *x*, in order:

1. **Drift.** Add a random step to each continuous trait (normal, standard deviation = drift rate times a small-population multiplier). Small groups drift faster: multiply by `sqrt(500 / max(pop, 50))`, capped at 3. For discrete traits, roll the change chance; if it fires, pick a random option weighted toward the neighbours' options.

2. **Conformity to neighbours.** Build a contact list: peoples within reach (same region or adjacent region), plus trade, marriage and alliance partners. Weight each contact by interaction intensity *w* (see below). Move each trait toward the weighted mean of contacts:
   `x += c * (mean_contacts - x)`
   with `c = 0.01` per year for horizontal traits, `0.003` for mixed, `0.001` for vertical. Use Axelrod's rule: weight each contact also by current similarity (1 minus distance). So similar neighbours pull harder. Very different neighbours barely pull, unless conquest or intermarriage forces contact.

   For discrete traits, use conformist bias: the chance to switch to option *k* is proportional to (share of contacts with *k*) raised to a power of 1.5 to 2. This makes the majority option sticky.

3. **Prestige copying.** Find the most prestigious people in reach: high wealth, recent war victories, large size, chiefdom centre. Prestige score `P = log(pop) + wealth_index + 2 * recent_victories + 3 * is_paramount`. Each year, with probability `0.05 * (P_model - P_self)` (clipped 0 to 0.3), copy one horizontal or mixed trait a step of 0.1 toward the model. Elites copy elites: within a chiefdom, subject villages copy the paramount centre at double rate. This spreads art style, dress, burial and war style from winners, which is what archaeological "horizons" look like (the Yamnaya kurgan horizon, the Bell Beaker package).

4. **Guided variation (environment and economy).** Each trait has a target that local conditions push toward. Move toward the target at `g = 0.002` per year. Suggested pushes:
   - Herding (especially cattle and horses) pushes honour +, martial +, patrilineal descent, bridewealth, mobility_pride +.
   - Plough farming pushes gender_roles + (strict), inheritance toward sons, dowry when stratified.
   - Hoe farming with women's labour pushes toward polygyny or matriliny (weak push).
   - Irrigation and rice-like intensive farming push collectivism + and tightness +.
   - High war threat in the last 20 years pushes tightness +, martial +, collectivism +.
   - Epidemics in the last 20 years push tightness + and xenophilia - (Gelfand pattern).
   - Large surplus and storable wealth push hierarchy + and feasting +.
   - Foraging with sharing pushes hierarchy - and gender_roles - (more flexible).
   - Frontier with a very different culture pushes asabiya + (see below) and martial +.

5. **Contact shocks.** Some events cause jumps, not drifts:
   - **Conquest.** The conquered people moves 0.1 toward the conqueror on all horizontal traits, and has a 20% chance per decade of language shift if the conqueror rules for a generation.
   - **Intermarriage at scale.** If more than 10% of marriages in a decade are with another people, mixed traits move 0.05 toward them per decade.
   - **Merger.** Two peoples who merge average their vectors weighted by population. Discrete traits come from the larger group, except each one has a 25% chance to come from the smaller.
   - **Charismatic leader or prophet.** A rare event (one in 5,000 people-years) changes religion_style or ritual_style and gives the new form a prestige boost for 50 years.

6. **Population loss of complex traits (Henrich effect).** Every skill or craft carried by a people has a complexity score 1 to 5. If the people's **connected population** (own pop plus 0.3 times the pop of peoples with weight above 0.2) is below a threshold for that complexity, it has a loss chance each year. Suggested thresholds: complexity 1 needs 50; 2 needs 150; 3 needs 500; 4 needs 1,500; 5 needs 5,000 connected people. Loss chance = 0.002 per year per complexity step short. Example: an isolated 300-person island group can keep pottery (complexity 2), but slowly loses metalworking (complexity 4).

### Interaction intensity

`w(p, q) = proximity + 0.5 * trade + 1.0 * marriage + 0.7 * alliance + 0.5 * same_chiefdom + 0.3 * war`

Each term is 0 to 1 for the last 20 years. War counts positively for contact (people learn from enemies, especially war style) but negatively for acceptance (below). Weight drops to zero beyond the travel range of the era: about 2 regions for foragers, 3 for farmers, 6 for horse nomads.

### Cultural distance

`D(p, q) = sum over traits of marker_weight * |x_p - x_q| / sum of marker weights`

For discrete traits, the difference is 0 if equal and 1 if different. Language distance (below) counts as a trait. D runs from 0 (identical) to 1 (nothing in common).

Also compute an **identity distance** using only traits with marker weight 0.6 or more. Barth's point is that identity rests on a few markers.

### Language

Give each people a language id and a **lexical position**, a vector of, say, 20 numbers. Each year each number drifts by a small step, so the expected replacement rate is about 0.02% per year (20% per 1,000 years). Contact pulls positions together at 0.1 times the trait conformity rate. Mutual intelligibility = `1 - (distance / D_lang)`, where `D_lang` is tuned so that 1,500 years of separation gives about 50% intelligibility. When intelligibility between a cluster's branches falls below 0.5, split the language and name the daughter. Track a parent pointer so the sim builds a family tree.

### Named cultures: clustering, splitting, merging

Every 25 years, run a clustering pass:

- Each named culture has a **centroid** (the population-weighted mean of its members' vectors).
- A people belongs to the culture whose centroid is closest, if that distance is under **0.25**.
- **Split.** If a culture's members form two sub-groups (use a simple 2-means on members) whose centroids are more than **0.30** apart in identity distance, and each has at least 3 peoples or 1,000 people, the smaller group becomes a new culture. Name it from its region or its language plus a marker ("Upper Don Kurgan", "Painted-Pot Folk") or from the generated language. Record the parent.
- **Isolation split.** A group of peoples with no contact (all w below 0.05) to the rest of its culture for 200 years splits even if traits are close, once identity distance exceeds 0.15. This is the "Iceland" case CK3 misses.
- **Hybrid.** If two cultures share a region where at least 30% of each culture's local peoples have strong ties (w above 0.4) to the other, and a mixed cluster forms between the two centroids (members closer to the midpoint than to either centroid), that cluster becomes a hybrid culture after 50 years. Its name joins both ("Yamnaya-Corded"). Archaeologists name these "contact" or "transitional" cultures.
- **Absorption.** If a culture shrinks below 2 peoples, or its centroid comes within 0.08 of a neighbour's, it merges into the larger one and the name passes into history.

Keep a history log: birth year, parent(s), region, defining markers, peak size, end year. This gives the player the CK3 pleasure of a culture tree, but grown, not clicked.

### Acceptance and asabiya

**Acceptance** between two peoples (0 to 1) starts at `1 - identity_distance`. It rises 0.01 per year of peaceful contact with w above 0.3, and by 0.05 per decade of intermarriage. It falls 0.1 per war and 0.02 per raid. It decays slowly toward the baseline. Acceptance affects: alliance chance, marriage chance, revolt chance in a chiefdom, how fast traits copy, and whether conquered peoples assimilate or resist.

**Asabiya** per people or per chiefdom (0 to 1):
- Rises by `0.02 * frontier_contrast` per year, where frontier_contrast is the largest identity distance to a neighbour with strong contact (especially war) multiplied by the threat that neighbour poses.
- Falls by `0.005 * (size_factor)` per year, where size_factor grows with the number of peoples in the polity and with hierarchy.
- Falls faster (0.02 per year) during elite competition (several claimants, high inequality, many younger sons).
- Effects: war strength multiplier (0.7 to 1.3), willingness to join larger polities, resistance to fragmentation, success in conquest. A high-asabiya group on a frontier is the seed of a new power. This matches Turchin's steppe frontier pattern and will make chariot and nomad frontiers produce big states, as they did.

### How traits feed back into the sim

This is where CK3 is weakest. Each trait should change outcomes:

| Trait | Effect |
|---|---|
| collectivism | + cooperation in public works and irrigation; + sharing in famine; - innovation rate |
| honour | + revenge raids after insult or theft; + feud chance; + defence of herds |
| tightness | + survival in epidemics (quarantine norms); - trait drift; - innovation; + conformity rate c |
| hierarchy | + inequality growth; + chiefdom formation chance; + feasting; - acceptance of rival leaders |
| martial | + war party size and frequency; + war strength; - trade |
| gender_roles | strict: + plough efficiency, - female craft output; flexible: + labour in hoe farming |
| xenophilia | + trade and marriage across groups; + trait copying; + disease import |
| innovation | + adoption of new crops, animals, metal, wheels |
| ancestor_focus | + territorial attachment (less migration); + mound and tomb building; + land claims |
| mobility_pride | + migration chance; + uptake of herding; - farming uptake |
| feasting | + alliance formation; + prestige; - stored surplus |
| descent patrilineal | + fraternal interest groups, + feud and raid, + chiefdom succession stability |
| descent matrilineal | + peace between villages (men scattered), - formation of big war bands |
| marriage polygyny | + birth rate of elite lineages; + unmarried young men; + raiding for wives and wealth |
| marriage_payment bridewealth | + herd accumulation drive; + raiding for cattle |
| inheritance eldest son | + estate size held together; + younger sons leaving (colonists, war bands) |
| inheritance equal | + land fragmentation; + village splitting |
| religion_style temple | + surplus collected by centre; + hierarchy |
| food_taboos | - food supply from tabooed source; + identity strength |

Birth rates: allow `+5%` total fertility for strong pronatalist cultures (high ancestor_focus with patrilineal descent and polygyny), and `-5%` for cultures with long birth spacing (mobile, foraging, flexible roles). Keep effects modest. Economy and disease should matter more.

Farming uptake: probability per year multiplied by `(1 + 0.5 * innovation - 0.5 * mobility_pride + 0.3 * xenophilia_toward_farmers)`.

### State variables (per people)

```
culture_id            // current named culture
traits_cont[N]        // f32, -1..+1
traits_disc[M]        // u8 option ids
food_taboos           // u8 bitset
language_id
lex_pos[20]           // f32 lexical position
asabiya               // f32 0..1
prestige              // f32, recomputed each year
skills[]              // id + complexity + level
contacts[]            // (people_id, w, acceptance), updated each year
last_war_years, last_epidemic_years // for guided variation
```

Per named culture: `id, name, parent_ids, founded_year, ended_year, centroid, member_count, region_core, marker_summary, language_family_id`.

### Calibration targets

- A culture should usually last 300 to 1,500 years before splitting or merging. Archaeological cultures in Europe and the Near East mostly last a few centuries to a millennium.
- Horizons (fast spreads of a package) should happen when a prestigious, expanding group appears. The Yamnaya horizon spread across 3,000 km in about 300 years.
- Languages within a family should be about as far apart as Romance languages after 1,500 to 2,000 years.
- Isolated islands should lose some complex skills within 1,000 to 3,000 years.

---

# PART B: HORSES, CHARIOTS AND PASTORAL NOMADS

## B1. Horse domestication

### Wild horses and early use

Wild horses lived across the Eurasian steppe in the Holocene. Steppe foragers hunted them heavily. In the Volga and Ural steppe, horse bones dominate many sites from about 5000 BCE.

### Botai (Kazakhstan, about 3500 BCE)

Botai sites have huge numbers of horse bones. Outram and colleagues (2009, *Science*) reported possible bit wear, horse-milk fats in pots, and horses with leg bones similar to later domestic horses. This was long read as the first domestication. Gaunitz and colleagues (2018, *Science*) sequenced Botai horses. They were not ancestors of modern domestic horses. They were ancestors of Przewalski's horses. So Przewalski's horses are feral descendants of Botai horses, not a truly wild line. Whether Botai people rode or penned their horses is still debated. The key point for the sim: early horse management at Botai did not lead to the horses that spread.

### The modern domestic horse (DOM2)

Librado and colleagues (2021, *Nature*) sampled hundreds of ancient horses. Modern domestic horses (called DOM2) come from the western steppe, in the lower Volga-Don region, in the late 4th to early 3rd millennium BCE. Two genes under strong selection were GSDMC (linked to back structure and strength) and ZFPM1 (linked to calm temper and stress). These fit breeding for riding or draught and for docility. DOM2 horses spread across Eurasia rapidly from about 2200 to 2000 BCE, replacing other horse lines everywhere. They reached Anatolia, the Carpathians, Bohemia and Central Asia by about 2000 BCE. They spread with spoked-wheel chariots and Indo-Iranian languages in Asia.

The same study found that Yamnaya and Corded Ware expansions (about 3000 to 2500 BCE) did not spread DOM2 horses. So Yamnaya people moved with ox-drawn wagons, not mounted on horses.

Librado and colleagues (2024, *Nature*) found that from about 2200 BCE, horse generation times shortened sharply. Breeders were producing foals younger and more often. That is a sign of controlled breeding. They concluded that widespread horse-based mobility began about 2200 BCE, not earlier.

### Riding versus driving

- **Riding** probably began in a small way before chariots. Herders on foot cannot manage large herds well; mounted herders can. Anthony (2007, *The Horse, the Wheel, and Language*) argued for riding as early as the 4th millennium BCE based on bit wear. The genetic evidence makes large-scale riding before 2200 BCE less likely, but some riding is plausible.
- **Early riding was not cavalry.** Without good bits, saddles and stirrups, and with small horses, mounted warriors were not effective in formed battle before about 1000 BCE. Riders acted as scouts, messengers and raiders who fought on foot.
- **Driving** (pulling a vehicle) became the main military use from about 2000 to 1000 BCE.

### Wagons and the Yamnaya

Wheeled vehicles appear across a wide zone from Mesopotamia to Poland in about 3500 to 3300 BCE. The Bronocice pot from Poland (about 3500 BCE) shows a wagon. Early wagons had solid wooden wheels and were pulled by oxen. They moved at about 2 to 3 km per hour.

The Yamnaya culture (about 3300 to 2600 BCE) of the Pontic-Caspian steppe used wagons as mobile homes. Wagons let herders leave river valleys and live on the open steppe with cattle and sheep. Some Yamnaya graves include whole wagons. Anthony (2007) sees this as the real revolution of steppe life: mobile herding of cattle and sheep, with wagons, from about 3300 BCE. Haak and colleagues (2015, *Nature*) and Allentoft and colleagues (2015, *Nature*) showed that Yamnaya-related ancestry spread far into Europe (Corded Ware) and east to the Altai (Afanasievo) after 3000 BCE.

## B2. Chariots

### Sintashta (about 2100 to 1800 BCE)

The Sintashta culture lived in the southern Ural steppe. It built fortified settlements, mined and smelted copper and arsenical bronze on a large scale, and buried warriors with chariots, horses and weapons. Spoked-wheel chariot burials at sites such as Sintashta and Krivoe Ozero date to about 2000 BCE, with some radiocarbon dates near 2050 to 1900 BCE (Anthony 2007). Spoked wheels with bent-wood rims made vehicles light enough to be fast.

A chariot had two spoked wheels, a light platform for two or three people, and a pair of horses on a central pole. It needed a wheelwright, bent-wood skill, bronze tools and trained horses. Littauer and Crouwel doubted that the narrow early Sintashta vehicles were true war chariots; Anthony argues they were.

### Spread

- **Andronovo horizon** (about 1800 to 1200 BCE) grew out of Sintashta and Petrovka. It spread across the steppes of Kazakhstan and southern Siberia. It carried chariots, bronze, horses and probably Indo-Iranian languages (Kuzmina 2007, *The Origin of the Indo-Iranians*).
- **Near East.** Light spoked chariots appear in Anatolia and Syria by about 1800 to 1700 BCE. Before that, Mesopotamia had heavy solid-wheeled "battle wagons" pulled by donkeys or onagers (the Standard of Ur, about 2600 BCE). These were not chariots in the later sense.
- **Mitanni** (northern Mesopotamia, about 1550 to 1260 BCE) had a chariot elite called *maryannu*. Its horse training vocabulary and some gods' names are Indo-Aryan. The Hittite horse training text of Kikkuli (about 1400 BCE) is a Mitanni trainer's manual for chariot horses, a programme of about 214 days.
- **Egypt.** The Hyksos ruled the eastern Nile delta from about 1650 BCE. The chariot and horse arrive in Egypt in this period. Egypt used them to expel the Hyksos (Ahmose, about 1550 BCE) and to build an empire under the New Kingdom.
- **China.** Chariots appear suddenly in late Shang tombs at Anyang, about 1200 BCE, fully developed, with horses. No local precursors are known. They arrived from the steppe via the northwest.
- **India.** The Rigveda (oral, roughly 1500 to 1200 BCE) is full of horses and chariots. Spoked-wheel chariots fit the Indo-Aryan arrival. Disputed finds at Sinauli (about 1900 BCE) have solid wheels and are debated.
- **Greece and Europe.** Mycenaean Greece used chariots from about 1600 BCE. Further north they were prestige vehicles (the Trundholm sun chariot, about 1400 BCE).

### Chariot warfare

Drews (1993, *The End of the Bronze Age*) and Littauer and Crouwel (1979, *Wheeled Vehicles and Ridden Animals in the Ancient Near East*) are the classic works.

- **Crew.** Egypt: two men, a driver and an archer. Hittites at Kadesh: three men, a driver, a warrior with spear, and a shield bearer. Shang China: three men, a driver, an archer and a halberd fighter.
- **Weapon.** The composite bow was the chariot's main weapon in the Near East and Egypt. A chariot was a fast, mobile archery platform. It shot from range, wheeled away, and ran down broken infantry.
- **Supporting infantry.** "Runners" followed the chariots to finish off enemies and protect crews whose chariots broke down.
- **Terrain.** Chariots need flat, open, dry ground. Hills, forest, marsh and broken ground make them useless.
- **Scale.** At Megiddo (about 1457 BCE), Thutmose III claimed to capture 924 chariots. At Kadesh (1274 BCE), Egyptian accounts give the Hittite coalition about 2,500 chariots in its attack and about 1,000 more held back. Totals near 5,000 chariots for both sides are often cited, but all are royal claims. At Qarqar (853 BCE), the Assyrian record credits Ahab of Israel with 2,000 chariots.
- **Cost.** A chariot needed two trained horses (plus spares), specialist builders, bronze fittings, grooms, fodder and grain. Horses on grain are expensive. Chariot crews were an elite who were given land in return (the maryannu in Mitanni; similar systems at Nuzi and Ugarit). This pushed Late Bronze Age states toward a small warrior aristocracy and high inequality.
- **End.** Drews argued that around 1200 BCE, masses of infantry with javelins and long swords defeated chariot armies, helping cause the Late Bronze Age collapse. This is debated, but chariot importance did fall after 1200 BCE. Cavalry gradually replaced chariots from the 9th century BCE (Assyria).

## B3. Pastoral nomads

### Stages on the steppe

1. **Before 4000 BCE.** Foragers and fishers along rivers, some cattle and sheep keeping near the Black Sea.
2. **Yamnaya (3300 to 2600 BCE).** Mobile herders with wagons, cattle and sheep. Kurgan burials. Horses eaten and maybe ridden a little.
3. **Middle and Late Bronze Age (2100 to 1200 BCE).** Sintashta, Andronovo, Srubnaya. These were not fully nomadic. They lived in settlements with houses, especially in winter. They herded cattle, sheep and horses within a seasonal round, and some farmed a little. Frachetti (2008, *Pastoralist Landscapes*; 2012, *Current Anthropology*) argues that mobile pastoralism emerged in several regions at once in this period, with seasonal moves of tens of kilometres, not long-distance wandering.
4. **Final Bronze Age (1200 to 900 BCE).** A dry, cold phase in parts of the steppe. Settlements shrink. Mobility rises.
5. **Early Iron Age true mounted nomads (from about 900 to 800 BCE).** The Arzhan 1 royal kurgan in Tuva (late 9th century BCE) is an early sign of a mounted nomad elite. Scythians, Saka and Cimmerians follow by the 8th to 7th centuries BCE. They are mounted archers with herds of horses and sheep, living in tents and wagons year-round. This is the classic nomad pattern.

For the sim, the key timeline is: ox-wagon herders (from 3300 BCE), chariot-using semi-mobile herders (from 2100 BCE), and true mounted nomads (from 900 BCE).

### Ecology and herd sizes

- **Steppe grassland** has low and variable rainfall (about 200 to 400 mm per year). Farming is risky. Grass is good. Winters are cold. Herds must move between summer and winter pastures.
- **Herd needs.** Khazanov (1984, *Nomads and the Outside World*) and later studies give rough numbers. A household of five to six people in Central Asia or Mongolia needs on the order of 100 sheep equivalents to live mainly from herds, or about 15 to 25 sheep equivalents per person. One horse is roughly 5 to 7 sheep equivalents, one cow about 5, one camel about 7. Many households fall below this and must trade, raid, or farm.
- **Herd composition.** Typical steppe herds were mostly sheep and goats, plus cattle in wetter areas, horses for riding, milk (kumis) and meat, and camels in drier zones. A Kazakh or Mongol household historically might have 5 to 15 horses.
- **Losses.** Severe winters (the Mongolian *dzud*) can kill a third or more of herds in a bad year. This makes herds boom and bust. It is a major driver of raids and migrations.
- **Population density.** Pastoral nomads on steppe usually live at about 0.5 to 3 people per km². Farming villages in good land reach 10 to 50 or more. So a steppe region of 100,000 km² supports perhaps 50,000 to 300,000 herders.
- **Mobility.** Seasonal moves range from 10 to 50 km in mountain areas (up and down slopes) to several hundred km on flat steppe. Some Kazakh routes exceeded 1,000 km. A whole group can move 15 to 30 km per day with herds and wagons. Horsemen travel 50 to 80 km per day, more in relays.

### Relations with farmers

Khazanov (1984) stressed that pure nomads cannot live on herds alone. They need grain, metal, and crafts from farmers. They get them by:
- **Trade** (animals, hides, wool and dairy for grain and goods).
- **Tribute** (farmers pay to avoid raids).
- **Raids** (taking grain, animals and people).
- **Conquest** (ruling farmers directly).

Barfield (1989, *The Perilous Frontier*) argued that big nomad empires grew as "shadow empires" that extracted wealth from strong farming states, not weak ones. Nomad political structure was usually loose. Supreme leaders formed only when there was a big neighbour to extract from. When the leader died, the confederacy often split. Honeychurch (2015, *Inner Asia and the Spatial Politics of Empire*) stresses that steppe polities were real, complex states built on mobility and networks.

### Raid patterns

- Raids usually came in late autumn and winter, when horses were fit after summer grazing and farmers' harvests were stored.
- Raids were short. Small parties of tens to hundreds of riders struck, took animals, grain and captives, and left before a large force gathered.
- Herders' wealth (animals) is easy to steal. This drives honour cultures, revenge and constant small-scale raiding between herders themselves (Nisbett and Cohen 1996).
- Large invasions came after leadership consolidated, after a herd disaster, or when a farming state was weak or divided.

## B4. Design for the sim

### Phases and prerequisites

Gate horse use by stages. Each stage is a skill a people can hold.

| Stage | Earliest | Needs |
|---|---|---|
| horse_hunting | any | steppe or open woodland region with wild horses |
| horse_herding (meat, milk) | about 4000 BCE | horse_hunting, animal herding skill, steppe region |
| ox_wagon | about 3500 BCE | wheel skill (diffuses from contact), cattle, carpentry |
| horse_riding (herding, scouting) | about 2500 BCE, widespread from 2200 BCE | horse_herding, DOM2 horse stock |
| spoked_chariot | about 2100 BCE | DOM2 horses, bronze working, wheelwright skill, bent-wood skill, hierarchy above 0.2 |
| chariot_army | after spoked_chariot | surplus grain or pasture to feed horses, chiefdom or state, elite class |
| mounted_archery | about 900 BCE | horse_riding, composite bow, horse breeding skill level 3, bits |

Do not allow any of these to arise anywhere. Seed them:
- **DOM2 horse stock** appears once, in the lower Volga-Don region, around 2400 to 2200 BCE, in a herding people with horse_herding. It then spreads by trade, raid and migration. The Botai-type horse can appear in northern Kazakhstan around 3500 BCE but does not produce DOM2 and gives only herding and milk.
- **Spoked chariot** appears first in a southern Ural people with DOM2 horses and bronze, around 2100 BCE. Allow small chances elsewhere if all prerequisites are met (so alternate histories work), but tune so the default history matches.
- **Mounted archery** emerges after 1000 BCE in the steppe, ideally after a dry spell pushes mobility up.

### What makes a people a horse or chariot people

Compute a **horse score** each decade:

`horse_score = 0.4 * share_of_herd_in_horses + 0.3 * steppe_share_of_territory + 0.2 * horse_skills_level / 5 + 0.1 * mobility_pride`

- Above 0.4 with chariot skill: a **chariot people**. They gain the chariot war style and prestige.
- Above 0.5 with mounted archery and low farming share (under 20% of calories): a **mounted nomad people**.
- Between: **semi-mobile herders** (Andronovo-like), with winter settlements.

Cultural feedback: being a horse people pushes mobility_pride +, honour +, martial +, patrilineal descent, bridewealth, and the kurgan burial style.

### Herds and food

Track herds per people as sheep equivalents by type. Suggested yields per head per year: sheep or goat 15 to 25 kg meat equivalent plus milk; cattle 60 to 100 kg plus more milk; horse used for milk (mare's milk) and meat, plus transport. A simple rule: each sheep equivalent supports about 0.05 people. So 20 sheep equivalents per person is subsistence. Herds grow 10 to 20% per year in good years. A bad winter (chance 5 to 10% per year on the steppe) kills 20 to 50%. When herds fall below 12 sheep equivalents per person, raid and migration chances jump.

### Movement

- Ox-wagon herders: can move one region per 2 to 5 years as a group. Seasonal round inside a region.
- Chariot and riding herders: one region per year if needed.
- Mounted nomads: up to 2 to 3 regions per year on steppe. Range for raids: 3 to 5 regions. Range for contact (culture spread, trade): 6 regions.
- Steppe regions cost little to cross. Forest, mountain and desert cost much more (desert costs less for camel herders).

### Raiding

Each year a horse people decides to raid if `raid_drive` beats a threshold. Raid drive grows with:
- herd shortfall (below subsistence),
- honour and martial traits,
- young unmarried men (polygyny, bridewealth),
- nearby wealth (stored grain and herds of farmers or other herders),
- low acceptance toward the target,
- the season (raids in late autumn to winter).

Raid outcome uses the war system, with mobility advantages: a nomad raiding party can choose to fight or flee. If the target's defence is above 1.5 times its strength, it withdraws with a small loss rather than fighting. Farmers who build walls, keep a standing force, or pay tribute lower raid frequency. Allow **tribute** as a stable outcome: a farmer chiefdom pays a share of surplus (5 to 15%) each year and raids stop.

### Conquest and elite takeover

When a chariot or nomad people defeats a farmer people decisively, allow these outcomes:
- **Raid and leave** (most common).
- **Tribute relationship.**
- **Elite takeover.** A small group of horse warriors becomes the ruling class of a farmer chiefdom. Population numbers barely change, but culture shifts at the top: the war style, burial style, religion style and (over generations) language can switch through prestige copying. This is how Mitanni, the Kassites, and Indo-Aryan elites probably worked.
- **Migration and replacement.** Rare, but possible where the farmer population has collapsed (from disease, famine or prior war). The Yamnaya movement into Europe may have followed a plague-driven decline (see the disease notes).

### Battle effects

Give each force a composition: infantry, archers, chariots, cavalry (riders). Suggested multipliers versus basic Bronze Age infantry, on open flat terrain:

| Unit | Strength per man-equivalent | Open terrain | Broken terrain | Notes |
|---|---|---|---|---|
| Infantry (spear, axe) | 1.0 | 1.0 | 1.0 | baseline |
| Archer infantry (composite bow) | 1.2 | 1.2 | 1.0 | needs bow skill |
| Chariot (2 or 3 crew) | 4 to 6 per chariot | 1.5 | 0.3 | shock and archery; breaks down; needs runners |
| Early riders (pre-900 BCE) | 1.2 | 1.2 | 0.8 | scouting, pursuit, raiding |
| Mounted archers (after 900 BCE) | 2.5 | 1.8 | 0.6 | can refuse battle; deadly in pursuit |

Rules:
- A side with chariots against a side without any, on open ground, gets a morale shock bonus: +30% for the first battle against a people who have never seen them. This fades as the defenders gain experience.
- Mobility: a side with mostly mounted forces can choose whether to accept battle. It can retreat with low losses.
- Pursuit: after a win, mounted forces multiply enemy losses by 1.5 to 2.
- Cost: each chariot needs 2 to 4 horses (with spares), 1 specialist per 10 chariots, and fodder equal to about 2 people's food per horse if grain-fed. Chariot armies are only possible for polities with surplus.
- Chariot share is capped by elite size. A rough rule: chariots are at most 1 per 50 to 100 people in a state, and a chiefdom of 5,000 people might field 20 to 60.

### Spread rules

- **Horses and riding** spread by contact with horse peoples, by trade of animals, and by raids. Base chance per year for a contacted herding people with steppe territory: 0.01, times `(1 + innovation)`. For farmers outside the steppe, 0.002, and they gain horses as an elite good, not a way of life.
- **Chariots** spread mainly by prestige copying among elites. Once any chiefdom in contact has chariots, neighbours with bronze, hierarchy, and open land adopt with chance 0.02 per year. If they lost a battle to chariots, 0.1 per year. This produces the fast spread seen between 1900 and 1600 BCE.
- **Specialist knowledge** (wheelwrights, trainers) can travel with people. Captives and hired experts can carry the skill to distant places. Allow a captured or hired craftsman to give the skill directly.
- **Climate push.** Dry spells (for example, the 4.2 kiloyear event about 2200 BCE, and a drier phase around 1200 to 900 BCE) raise herd losses and mobility, and raise the chance that a horse people migrates and that new mobile forms appear.
- **Language and culture spread.** Horse peoples carry their language farther because their contact range is larger and their prestige is high. Elite takeover should give a 20 to 50% chance of language shift in the subject people over 200 years if the elite holds power that long.

### Diseases and nomads (links to other notes)

Mobile herders live at low density, so crowd diseases do not persist among them. But their wide contact range carries diseases between dense farming zones. Plague (*Yersinia pestis*) is found in steppe people from about 3000 BCE (Rasmussen et al. 2015, *Cell*). Herders also catch animal diseases (brucellosis, anthrax). In the sim, nomads should act as long-range carriers with low local outbreak rates.

### State variables (horse and nomad part, per people)

```
herd_se_by_type{sheep, goat, cattle, horse, camel}   // sheep equivalents
horse_stock_type      // none, wild, botai, dom2
skills: horse_herding, wheel, ox_wagon, riding, wheelwright, bentwood,
        chariot, chariot_army, composite_bow, mounted_archery, horse_breeding (0-5)
horse_score           // computed
mobility_class        // settled, semi_mobile, wagon_herder, chariot_herder, mounted_nomad
season_pasture       // summer and winter region ids
raid_drive            // computed
tribute_links[]       // payer, receiver, share
chariots, riders     // numbers available for war
```

### Default timeline the sim should be able to reproduce

| Date (BCE) | Event |
|---|---|
| 4500 to 3500 | Steppe foragers and herders eat horses; Botai horse management around 3500 |
| 3500 to 3300 | Wheels and wagons spread across a wide zone |
| 3300 to 2600 | Yamnaya ox-wagon herders spread from the Pontic-Caspian steppe |
| 2400 to 2200 | DOM2 horses bred in the lower Volga-Don |
| 2200 to 2000 | Fast spread of DOM2 horses and horse mobility |
| 2100 to 1800 | Sintashta chariots and fortified bronze towns in the southern Urals |
| 1800 to 1200 | Andronovo horizon across Central Asia; chariots in Anatolia, Syria, Greece |
| about 1650 | Hyksos in Egypt; horse and chariot arrive in Egypt |
| 1550 to 1260 | Mitanni with maryannu chariot elite |
| 1274 | Kadesh, the largest chariot battle on record |
| about 1200 | Chariots at Anyang (late Shang) |
| 1200 to 900 | Collapse in the Near East; drier steppe; rising mobility |
| 900 to 700 | Arzhan; first mounted nomad elites; Cimmerians and Scythians |

---

## Sources

Culture and cultural evolution:
- Alesina, A., Giuliano, P. and Nunn, N. (2013). On the origins of gender roles: women and the plough. *Quarterly Journal of Economics*.
- Axelrod, R. (1997). The dissemination of culture: a model with local convergence and global polarization. *Journal of Conflict Resolution*.
- Barth, F. (ed.) (1969). *Ethnic Groups and Boundaries*.
- Bell, A. V., Richerson, P. J. and McElreath, R. (2009). Culture rather than genes provides greater scope for the evolution of large-scale human prosociality. *PNAS*.
- Bergsland, K. and Vogt, H. (1962). On the validity of glottochronology. *Current Anthropology*.
- Boserup, E. (1970). *Woman's Role in Economic Development*.
- Bouckaert, R. et al. (2012). Mapping the origins and expansion of the Indo-European language family. *Science*.
- Boyd, R. and Richerson, P. J. (1985). *Culture and the Evolutionary Process*.
- Cavalli-Sforza, L. L. and Feldman, M. W. (1981). *Cultural Transmission and Evolution*.
- Chang, W., Cathcart, C., Hall, D. and Garrett, A. (2015). Ancestry-constrained phylogenetic analysis supports the Indo-European steppe hypothesis. *Language*.
- Currie, T. E. et al. (2010). Rise and fall of political complexity in island South-East Asia and the Pacific. *Nature*.
- Gelfand, M. J. et al. (2011). Differences between tight and loose cultures: a 33-nation study. *Science*.
- Gray, R. D. and Atkinson, Q. D. (2003). Language-tree divergence times support the Anatolian theory of Indo-European origin. *Nature*.
- Henrich, J. (2004). Demography and cultural evolution: the Tasmanian case. *American Antiquity*.
- Henrich, J. (2016). *The Secret of Our Success*.
- Henrich, J. (2020). *The WEIRDest People in the World*.
- Henrich, J. and Gil-White, F. (2001). The evolution of prestige. *Evolution and Human Behavior*.
- Holden, C. J. and Mace, R. (2003). Spread of cattle led to the loss of matrilineal descent in Africa. *Proceedings of the Royal Society B*.
- Kirby, K. R. et al. (2016). D-PLACE: a global database of cultural, linguistic and environmental diversity. *PLoS ONE*.
- Kline, M. A. and Boyd, R. (2010). Population size predicts technological complexity in Oceania. *Proceedings of the Royal Society B*.
- Lees, R. B. (1953). The basis of glottochronology. *Language*.
- Mace, R. and Holden, C. J. (2005). A phylogenetic approach to cultural evolution. *Trends in Ecology and Evolution*.
- Mesoudi, A. (2011). *Cultural Evolution*.
- Murdock, G. P. (1967). *Ethnographic Atlas*.
- Nisbett, R. E. and Cohen, D. (1996). *Culture of Honor*.
- Pagel, M., Atkinson, Q. D. and Meade, A. (2007). Frequency of word-use predicts rates of lexical evolution. *Nature*.
- Powell, A., Shennan, S. and Thomas, M. G. (2009). Late Pleistocene demography and the appearance of modern human behavior. *Science*.
- Schulz, J. F. et al. (2019). The Church, intensive kinship, and global psychological variation. *Science*.
- Swadesh, M. (1955). Towards greater accuracy in lexicostatistic dating. *International Journal of American Linguistics*.
- Talhelm, T. et al. (2014). Large-scale psychological differences within China explained by rice versus wheat agriculture. *Science*.
- Turchin, P. (2003). *Historical Dynamics*.
- Turchin, P. (2006). *War and Peace and War*.
- Turchin, P. et al. (2013). War, space, and the evolution of Old World complex societies. *PNAS*.
- Watts, J. et al. (2016). Ritual human sacrifice promoted and sustained the evolution of stratified societies. *Nature*.

Horses, chariots and nomads:
- Allentoft, M. E. et al. (2015). Population genomics of Bronze Age Eurasia. *Nature*.
- Anthony, D. W. (2007). *The Horse, the Wheel, and Language*.
- Barfield, T. J. (1989). *The Perilous Frontier: Nomadic Empires and China*.
- Drews, R. (1993). *The End of the Bronze Age*.
- Frachetti, M. D. (2008). *Pastoralist Landscapes and Social Interaction in Bronze Age Eurasia*.
- Frachetti, M. D. (2012). Multiregional emergence of mobile pastoralism and nonuniform institutional complexity across Eurasia. *Current Anthropology*.
- Gaunitz, C. et al. (2018). Ancient genomes revisit the ancestry of domestic and Przewalski's horses. *Science*.
- Haak, W. et al. (2015). Massive migration from the steppe was a source for Indo-European languages in Europe. *Nature*.
- Honeychurch, W. (2015). *Inner Asia and the Spatial Politics of Empire*.
- Khazanov, A. M. (1984). *Nomads and the Outside World*.
- Kuzmina, E. E. (2007). *The Origin of the Indo-Iranians*.
- Librado, P. et al. (2021). The origins and spread of domestic horses from the Western Eurasian steppes. *Nature*.
- Librado, P. et al. (2024). Widespread horse-based mobility arose around 2200 BCE in Eurasia. *Nature*.
- Littauer, M. A. and Crouwel, J. H. (1979). *Wheeled Vehicles and Ridden Animals in the Ancient Near East*.
- Outram, A. K. et al. (2009). The earliest horse harnessing and milking. *Science*.
- Rasmussen, S. et al. (2015). Early divergent strains of *Yersinia pestis* in Eurasia 5,000 years ago. *Cell*.
