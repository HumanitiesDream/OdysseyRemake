# Chiefdoms, borders, rulers and war (Neolithic to Chalcolithic)

Companion to `neolithic.md` section 4 and `neolithic-plan.md` ("Chiefs and proto-states"). Those notes already give the
when-and-why triggers, the population ladder (village / simple / complex chiefdom / state) and the yearly AI menu.
This file adds war numbers, territory sizes, rulers and traits, and a concrete model spec. It does not repeat them.

Tags: **[O]** = I opened the source (paper, publisher page, or a page quoting the paper). **[M]** = from memory, not
checked this session; treat as a plausible default. A search snippet was never used as a source.

Access notes: PubMed and PMC returned a captcha page; Europe PMC returned "rate limited" and I stopped. So Bowles 2009,
Drennan & Peterson 2006 and Keeley's tables are cited through pages that quote them, or as [M].

---

## 0. Summary for the model (one screen)

| Item | Default for Odyssey | Basis |
|---|---|---|
| Men of fighting age | ~24% of a farming population (males 15-49) | [M] model life tables |
| Village defence turnout | 70-90% of fighting-age men | [M] Keeley, Gat |
| Raid party | 10-30 men (stealth); up to a few hundred when several villages join | Dani ~12 [O]; Turkana 12 and 315 [O] |
| Chiefdom war levy | 30-50% of men from the core, 15-30% from subject villages | [M], tuned to Hawaii 1795 (~12,000 men [O]) |
| Deaths per raid, per raider | ~1% | Turkana 1.1% [O] |
| Massacre of an overrun hamlet | 50-100% of 50-100 people | LBK mass graves [O] |
| War deaths, small farming societies | ~600 per 100,000 per year (0.6%) | Thomson & Halstead via WWOTF [O] |
| Share of all deaths from violence | ~14% (foragers, Bowles) to ~25% (tribal farmers, Pinker) | [O] via WWOTF |
| Chiefly reach | half a day's walk, 25-30 km, from the centre | Spencer 2010 [O] |
| Simple chiefdom area | up to ~2,000-2,800 km² (25-30 km radius) | Spencer [O]; Mississippian 25 mi across [O] |
| Paramount (complex) chiefdom | ~7 simple chiefdoms, up to ~50,000 people | Coosa [O] |
| Chiefdom lifespan | median 75-150 years, then collapse or takeover | Hally; Blitz & Livingood, via Gavrilets et al. [O] |
| Chief's time in power | 10-25 years | Tonga 23.5 yr/gen [O, computed]; model τ 5-20 [O] |
| Fortified villages | from ~8300 BCE (Jericho), common by 5700-4000 BCE | [O] |

---

## 1. War in tribal societies and chiefdoms

### 1.1 How common and how deadly

- 90-95% of known societies made war; among native American peoples only 13% did not fight neighbours at least once
  a year (Keeley 1996, via P2P summary) [O]. Keeley also reports ~66% of nonstate societies at war continuously or at
  any time, and ~71% of tribal societies at war at least once every two years [O, archive.org text, partial].
- Bowles (2009, Science 324:1293): hunter-gatherer samples average ~14% of deaths from warfare [O, via WWOTF report].
  Pinker's "hunter-horticulturalists and other tribal groups" average ~24.5% violent deaths [O, same].
- Rates per year: small-scale farmers average ~595 violent deaths per 100,000 a year, ~4.8x nomadic foragers
  (median 124) [O, WWOTF report citing Thomson & Halstead]. OWID lists Dani 1,000, Piegan 1,000, Kato 1,450 per
  100,000 a year; Andamanese 20 [O].
- Turkana (pastoralists): 14% of men die in war between puberty and reproductive age; 9% during reproductive age;
  war is 20% of all male deaths [O, Mathew & Boyd 2011].
- Neolithic Europe: of >2,300 skeletons from 180 sites (6000-2000 BCE), more than 1 in 10 show weapon injuries
  (Fibiger et al. 2023, PNAS) [O, Edinburgh press release].
- Yanomamö: more than a third of males die from warfare (Chagnon) [O, via P2P Keeley summary]. Murngin: 25% of adult
  males killed in 20 years [O, same].

**Game reading.** Farming peoples at war lose ~0.3-1% of population a year in bad times and close to 0 in peaceful
spells. Averaged over a century this should come out at 10-25% of male deaths. Use that as a calibration target.

### 1.2 Who fights (military participation)

- In bands, tribes and simple chiefdoms there is no army. Every able man is a potential warrior. Keeley stresses that
  tribal societies mobilise a far higher share of their manpower than states, but recruitment is voluntary and leaky
  [O, archive.org text]. The often-quoted contrast is "about all adult men" versus ~1% of the population for modern
  states [M, Keeley; the 1% figure appears in the P2P summary for casualty rates, not mobilisation].
- Turkana force raids are joined by men from ~5 age-sets, 4 settlements and 3 territorial sections; desertion happens
  in at least 43% of force raids, cowardice in 45% of those with combat [O, Mathew & Boyd]. So turnout is high but
  far from total.
- Andreski's "military participation ratio" (share of adult men under arms) is the classic term: high in tribes, low
  in early states with elite warriors, high again in mass-levy states [M].
- Chiefdoms add a small core of specialists: chiefly retainers. Hawaii by 1778 had full-time elite warriors and a
  large standing army under Kalaniʻōpuʻu (Seshat) [O].

| Society type | Who goes | Share of all people under arms (big war) | Basis |
|---|---|---|---|
| Village / tribe (no chief) | volunteers, kin of victims; whole village defends | 15-20% (most men) for defence; 2-5% for a raid | [M] Keeley, Gat |
| Simple chiefdom | the chief's kin and followers plus village levies | 8-15% | [M] |
| Complex chiefdom | retainers (1-3%) plus levies of subject chiefs | 5-12% | Hawaii 1795 [O, see 1.3] with [M] population |
| Early state | professional core plus levy | 2-8% | [M] |

### 1.3 Sizes of war groups

| Case | Size | Source |
|---|---|---|
| Dani (New Guinea) raid party | about a dozen men from one neighbourhood | Gardner & Heider, via culture-of-peace [O] |
| Dani formal battle | hundreds of men, a few hours at midday | same [O] |
| Dani surprise attack, 1966 | ~125 killed in one morning, women and children too | same [O] |
| Turkana stealth raid | mean 12 warriors | Mathew & Boyd 2011 [O] |
| Turkana force raid | mean 315, median 248 warriors, from a section of ~25,000 people | same [O] |
| LBK massacre victims (Talheim, Kilianstädten, Schletz) | 34, 26+, 67 bodies; hamlets of 50-100 people | Meyer et al. 2015, via Brewminate [O] |
| Hawaii, Kamehameha at Nuʻuanu 1795 | ~12,000 men and 1,200 war canoes | Wikipedia [O] |

**How many warriors does a village of N field?** (defaults; [M] except where marked)

| Village N | Men 15-49 (~24%) | Stealth raid | Full turnout (defence or big raid) |
|---|---|---|---|
| 50 (LBK hamlet) | 12 | 3-6 | 10 |
| 150 | 36 | 8-15 | 30 |
| 300 | 72 | 10-20 | 55-65 |
| 1,000 (chiefly centre) | 240 | 20-40 | 170-210 |

### 1.4 Seasons and range

- Hawaii: all war was banned during the four lunar months of Makahiki (about Oct/Nov to Feb/Mar), when tribute was
  collected on a circuit of each island [O, Wikipedia citing Malo and Kirch].
- Dani formal battles were frequent: 9 battles in 5.5 months [O].
- Turkana herds move 10-50 km every 15-40 days, so raids reach tens of km [O]. Monte Albán's first conquests reached
  75-80 km from the capital (~300 BCE) [O, Spencer 2010].
- Farmers raid mostly in the slack season after harvest and before planting, and in the dry season when paths are open
  [M, general ethnography]. A war party on foot covers ~20-30 km a day [M].

**Game reading.** Two war seasons a year at most for farmers; raids reach 1-2 days' walk (20-60 km); conquest war by a
chiefdom reaches up to ~3 days (80 km) only at the chiefdom-to-state transition.

### 1.5 Casualties per fight

| Fight type | Deaths | Source |
|---|---|---|
| Formal line battle (Dani ritual phase) | often 0 per battle; 6 killed in raids over 5.5 months | [O] |
| Force raid | ~1.1% risk of death per raider; 1.3% if combat | Turkana [O] |
| Surprise dawn raid on a village | 10-60% of the village | Crow Creek >60% [O, P2P]; Dani 125 [O] |
| Overrun hamlet (massacre) | near all; young women and children 9-16 often missing (taken) | LBK [O] |
| Close combat, tribal | up to 60% of combatants in the worst cases | Keeley via P2P [O] |

### 1.6 Fortifications

| Date | Place | What | Source |
|---|---|---|---|
| ~8300-7800 BCE | Jericho (PPNA) | wall 3.6 m high, tower 8.5 m, ditch 8.2 m wide x 2.7 m deep; defence or flood debated | Wikipedia [O]; Bar-Yosef 1986 flood view [M] |
| ~5730 BCE | Tell es-Sawwan (Samarra) | 3 m ditch and mudbrick wall | Wikipedia [O] |
| ~5300-4900 BCE | LBK Central Europe | ditched enclosures, more at the end of LBK; Darion ~2.7 m (9 ft) ditch with palisade; arrowheads along palisade and gates | Tübingen project [O]; Keeley [O] |
| ~4000 BCE | Chengtoushan (Daxi, China) | moat and rammed-earth wall | Wikipedia [O] |
| ~1000-1500 CE | Mississippian (King site) | palisade and ditch; scalping marks | Georgia Encyclopedia [O] |

**Game reading.** Let a village build a ditch-and-palisade once it has had a raid in the last ~20 years and has >150
people; it doubles defence. Walls of mudbrick or rammed earth come with chiefdoms in the Near East and China.

### 1.7 Weapons by era

| Era (Near East dates; Europe ~1,500-2,000 yr later) | Main weapons | Source |
|---|---|---|
| Epipalaeolithic / PPNA (to ~8500 BCE) | bow and flint-tipped arrows, spear, club | [M] |
| First maceheads, ~9500-8500 BCE (Hallan Çemi, Körtik Tepe) | stone mace | Li 2018 [O] |
| Late PPN / Yarmukian ~6400-5800 BCE (S. Levant) | maceheads spread | Rosenberg 2010 [O] |
| LBK ~5000 BCE | stone adze/axe (skull wounds), arrows | Meyer 2015 via Brewminate [O] |
| ~5000 BCE | first copper macehead (Can Hasan) | Li 2018 [O] |
| ~4th millennium BCE | Nahal Mishmar hoard, >400 metal objects, many maceheads; maces become symbols of power | Li 2018 [O] |
| ~3300 BCE (Alps) | Ötzi: 99.7% copper axe, flint dagger, 1.82 m yew bow, 14 arrows; killed by an arrow | Wikipedia [O] |
| Chalcolithic-EBA | copper daggers, copper/arsenical axes, spears; later bronze | [M] |

The mace is more a sign of rank than a battle weapon (Li 2018) [O]. Use it as a prestige item that boosts a ruler's
legitimacy, not combat power.

### 1.8 War leaders, alliances, feasts, captives

- **War leader versus chief.** Many tribes split a peace chief and a war leader [M]. Polynesia split the sacred ruler
  from the active one: in Tonga about 1450 CE the Tuʻi Tonga kept religious authority and a younger brother's line took
  political power (Claessen 2018) [O]. Big men (Melanesia) gain rank by action and generosity; chiefs (Polynesia)
  inherit office (Sahlins 1963) [O, Wikipedia].
- **Alliances.** Dani alliances fought formal battles together; alliances could break and turn on each other (1966) [O].
  Elite marriage in Tonga was a deliberate tool to win allies and resources (Biersack) [O].
- **Feasting.** Hayden (2014) argues "aggrandizers" used feasts to create debt, allies and rank; feasting drove early
  inequality [O, review].
- **Captives.** LBK massacre sites lack young women and children aged 9-16, suggesting they were taken (Meyer et al.
  2015) [O]. Captives (mostly women and children) were a main loot of small-scale war and a road to slavery and
  servile labour in chiefdoms (Cameron 2016) [M].
- **Loot.** Turkana force raids bring ~11 cattle per raider [O].

---

## 2. Territory

### 2.1 Village territory

- Trypillia model: a 2 ha village of ~50 people had fields within 0.5 km and grazing within 2 km; farmers generally
  kept fields within 1-2 km and grazing within 5 km. A ~10 ha village held ~270 people. Villages lasted 50-150 years
  before moving (Rassamakin/Nikitin-type model, arXiv 1705.03398) [O].
- Classic site-catchment: farmers rarely walk more than ~1 hour (~4-5 km) to fields; foragers ~2 hours (~10 km)
  (Vita-Finzi & Higgs 1970; Chisholm 1968) [M].

| Item | Value |
|---|---|
| Village core (fields) | 1-3 km radius, 3-30 km² |
| Village territory incl. grazing and woods | ~5 km radius, ~50-80 km² |
| LBK-style hamlets | 50-100 people, several per 10 km of valley [O for size] |

### 2.2 Chiefdoms

| Unit | Area | People | Tiers above households | Sources |
|---|---|---|---|---|
| Village territory | 50-80 km² | 50-500 | 1 (village) | above |
| Simple chiefdom | ≤25-30 km radius (≤2,000-2,800 km²); Mississippian ≤25 miles across (~40 km) | 2,500-5,000 (Coosa core 2,500-4,650) | 2 (centre + villages) | Spencer 2010 [O]; Smith, Georgia Enc. [O] |
| Paramount / complex chiefdom | linear 400 miles (Coosa) of allied chiefdoms; or an island district | up to ~50,000 (Coosa); Hawaii island 60,000-150,000 | 3 | Georgia Enc. [O]; Seshat [O] |
| Archaic state (Hawaii 1778-1819) | 10,832 km² (Big Island + Hana) to 16,624 km² (archipelago) | ~180,000 (1810) | 4-5 admin levels | Seshat [O] |
| Primary state | 4-tier settlement hierarchy; conquests 75-80 km out | Monte Albán 10,200-20,400 at capital | 4 | Spencer 2010 [O] |

- **The half-day rule.** A chief can only control land about half a day's walk from his centre, 25-30 km, so he or his
  agent can reach the edge and get back in a day; states break this by delegating partial authority (Spencer 2010) [O].
- **Villages per chief.** Coosa paramountcy held at least seven chiefdoms [O]. Halaf: largest villages (~8 ha) ringed
  by 1-3 ha satellites; Oaxaca's chiefly village (60-70 ha) had satellites ~5 km out (Marcus 2012) [O]. Model span of
  control in Gavrilets et al. is 5-7 subordinates per chief [O]. Default: 3-10 villages per simple chief.
- **Hawaii's nested land units.** Island (mokupuni) > district (moku) > ahupuaʻa (ridge-to-reef strip under a local
  chief with a konohiki manager) > ʻili (2-3 per ahupuaʻa). Oʻahu had 6 moku, Maui 12 [O, Wikipedia]. This is the
  best real-world template for CK-style counties: **ahupuaʻa = barony/village, moku = county, island = duchy/kingdom.**
- **Drennan & Peterson (2006, PNAS 103:3960).** Comparing Oaxaca (~1st millennium BCE), Alto Magdalena (Colombia,
  1st millennium CE) and Hongshan (NE China, 4th millennium BCE): Oaxaca elites were set apart by wealth; Alto
  Magdalena elites by ritual power without much wealth; Hongshan had both, but not in the same people. Wealth-based
  chiefdoms moved fast toward states; ritual-based ones stayed stable for long periods [O, Pitt and Stonepages news].
  In Oaxaca most people lived in villages under 50 people; San José Mogote had 500+ [O].

### 2.3 Rise and collapse (chiefly cycling)

- Hally: most Mississippian mound centres were used for one period of 75-100 years; 29 of 45 had a single period; as
  many as 47 chiefdoms rose and fell at 27 locations in 1000-1500 CE [O, via Gavrilets, Anderson & Turchin 2010].
- Blitz & Livingood: mound-centre use lasted 100-450 years, mean 183, median 150 [O, same].
- Anderson (1994, Savannah River Chiefdoms) named the pattern "cycling": simple chiefdoms grow into complex ones and
  fall back [O for book scope; details M]. Hally & Chamblee (2019) on 351 mound sites: polities rose and fell with
  some regularity, often reborn at the same place [O].
- Model (Gavrilets et al. 2010): strength F = own production + tribute (θ = 10-30%); attack success
  P = F_a^α / (F_a^α + F_d^α); on a chief's death (mean tenure τ = 5-20 yr) a random number of subordinates break away;
  subordinates also rebel when they think they can win. Result: complex chiefdoms usually live under 55-68 years;
  2-3 levels of hierarchy at most [O].
- Hawaiian ahupuaʻa formed in an expansion phase about 1500-1550 CE and the island polities became archaic states by
  ~1600 (Kirch) [O for ahupuaʻa date, M for the state date].
- Mississippian chiefdoms were separated by empty buffer zones ~20-30 km wide [M, Hally 1993].

### 2.4 How borders were marked

| Marker | Example | Source |
|---|---|---|
| Stone heap altar with offerings (often a pig's head) on the border | Hawaii ahupuaʻa | Wikipedia [O] |
| Ridge lines, streams | Hawaii (ridge-to-reef strips) | [O] |
| Empty no-man's-land buffers | Mississippian | [M] |
| Monuments, barrows, ditched enclosures seen from far | Neolithic Europe | [M] |
| Tribute circuit (the chief or his god walks the boundary yearly) | Hawaii Makahiki | [O] |

---

## 3. Rulers

### 3.1 Legitimacy and succession

| Driver | Detail | Source |
|---|---|---|
| Descent from gods/ancestors (mana) | Polynesian chiefs held mana from divine descent; Tuʻi Tonga from the sky god Tangaloa | Claessen 2018 [O] |
| Ranked lineages (conical clan, ramage) | one descent group split into senior and junior lines, ranked by older vs younger brother | Britannica [O] (term from Kirchhoff 1955, Firth) [M] |
| Birth order, but not only | Tongan succession weighed birth order, mother's rank, maternal kin support, age, experience and ability; sisters' sons could succeed | Biersack [O] |
| Achievement (big man) | rank by feasts, gifts, protection; not inherited; competitive and unstable | Sahlins 1963 [O] |
| Wealth vs ritual | some chiefdoms ranked by wealth, others by ritual power | Drennan & Peterson [O] |
| War | defeated Tahitian rulers could regain rank by marriage and by holding sacred objects (Oro images, feather girdles) | Claessen [O] |

**Reign lengths.** Tuʻi Tonga: 39 generations 950-1865 CE, i.e. ~23.5 years each (my arithmetic on Gifford's count) [O].
European monarchs 600-1800 CE: of 1,513, 22% died violently and 15% were murdered; ~10 murders per 1,000 reign-years;
young rulers at most risk; succession fights were the top reason (Eisner 2011) [O]. Default for chiefs: mean reign
~15 years, a violent-death hazard of ~1.5-2% a year (higher than kings, [M]).

**Why chiefdoms split.**
1. Succession: on a chief's death subordinate villages walk away (model mechanism, Gavrilets) [O]; brothers of near-equal
   rank each claim (Tonga) [O].
2. Distance: lands beyond half a day's walk drift off (Spencer) [O].
3. Defeat and loss of prestige; failed feasts; famine [M].
4. Sacred vs secular split (Tonga dual kingship) [O].
5. Village size: villages without strong leaders fission at a few hundred people [M; see neolithic.md].

### 3.2 Early trait set (EU4-like, ~40 traits)

A ruler gets **2-3 personality traits plus 0-1 from each other group**. Effects are small and stackable. "Levy" = share
of men raised; "Legit" = legitimacy; "Prestige" = gain rate.

**A. Temper and courage (pick at most one per pair)**

| Trait | Opposite | Effects |
|---|---|---|
| Brave | Craven | +15% battle strength when leading; +risk of death in battle x2 / -10% battle, safer, -prestige |
| Wrathful | Calm | +raid chance, +fear of vassals; -alliance acceptance / +peace deals, +succession stability |
| Ambitious | Content | +war and expansion weight, +prestige; +rival chance / +vassal loyalty, -expansion |
| Cruel | Merciful | massacres and captive-taking on wins, +fear (fewer revolts), -alliances / spares villages, absorbs them, +legit |
| Proud | Humble | +feasting and monument weight, +prestige; +offence to rivals / +big-man style loyalty |

**B. Giving and gathering**

| Trait | Effects |
|---|---|
| Generous | feasts cost +20% but give +50% loyalty and allies (Hayden) |
| Greedy | +10% tribute; -loyalty; +raid for loot |
| Feast-giver | can hold great feasts once a year; +prestige |
| Hoarder | +storage, better famine survival; -prestige |

**C. Mind and craft**

| Trait | Effects |
|---|---|
| Shrewd | +diplomacy, better war odds estimate (fewer bad wars) |
| Dull | -diplomacy; AI misjudges strength by ±30% |
| Cunning | +surprise-raid success (+20%) |
| Honest | treaties last longer; +alliance trust |
| Deceitful | breaks alliances when profitable |
| Builder | -25% cost for palisades, mounds, shrines |
| Wise elder | +legit, +peace, -battle |

**D. Sacred standing (ritual power, Drennan & Peterson; mana)**

| Trait | Effects |
|---|---|
| Sacred-born (high mana) | +30% legit; succession stable; cannot lead in battle without penalty (Tongan sacred chief) |
| Seer / priest-chief | +legit from rituals, +cohesion; -tribute |
| Profaner | -legit; +freedom to break taboos (e.g. war in sacred season) |
| Blessed by omens | random +prestige events |
| Ill-omened | random -legit events |

**E. War style**

| Trait | Effects |
|---|---|
| Raider | +stealth raid success, +loot; -pitched battle |
| Champion | +battle when outnumbered; personal duels |
| Defender | +50% when defending own villages; builds palisades |
| Captive-taker | +captives per win (+labour, +wives for followers); +enemy hatred |
| Peacemaker | +truce chance, -war weight |
| Coward in war | lowers levy turnout -15% |

**F. Body and health (visible, early)**

| Trait | Effects |
|---|---|
| Strong | +battle, +big-man prestige |
| Frail | +death risk; -prestige |
| Wounded | -battle; -fertility small |
| Old (age-driven) | +legit, -battle, +death risk |
| Child ruler (age <16) | regent needed; +split risk (Eisner: young rulers at risk) |

**G. Standing and origin**

| Trait | Effects |
|---|---|
| Senior line | +legit (conical clan) |
| Junior line / upstart | -legit, +ambition |
| Big man (self-made) | high loyalty while generous; heir gets no inherited legit |
| Usurper | -legit for 10 years; +fear |
| Beloved | +turnout +10% |
| Hated | -turnout, +revolt |

That is 42 traits (counting each pair as two).

### 3.3 Growing toward CK3 (Chalcolithic onward)

Turn on layers one at a time as the "proto-country" dial rises (more people, more tiers, metal, writing later).

| Stage | Trigger | New layer | Notes |
|---|---|---|---|
| 1 Neolithic chiefdom | chiefdom forms | name, 2-3 personality, 1 sacred, 1 war, age, health basic | above |
| 2 Lineage | polity lasts 2 rulers | named lineage (house), family tree of the ruler's parents, siblings, spouses, children; senior/junior line; claims | conical clans give rank by birth order (Britannica, Biersack) [O] |
| 3 Education | complex chiefdom | 5 education tracks (war, diplomacy, stewardship, ritual, craft) x 4 levels, set at age 16 by guardian | mirrors CK3 [M] |
| 4 Lifestyle | ~Chalcolithic | lifestyles earned over life: Feast-master, Smith-patron, War-lord, Priest, Trader (copper, salt, obsidian routes) | ties to economy |
| 5 Congenital | lineage depth ≥3 | inheritable: Strong/Weak, Tall/Small, Clever/Slow, Fair/Ugly, Fertile/Barren; tiered (1-3); inbreeding risk on close marriage | CK3-style genetics [M] |
| 6 Health | always on but grows | Wounded/Maimed, Ill, Lame, Blind, Pox (link to diseases.md), Depressed, Lunatic | |
| 7 Relations | complex chiefdom | rivals, friends, lovers, sworn allies; marriages make alliances | Tonga affinal politics [O] |
| 8 Commander traits | ruler or kin leads armies | Ambusher, Holder of the Ford, Hill-fighter, Canoe-raider | |
| 9 Stress / coping | late Chalcolithic | acting against personality builds stress; breakdowns give flaws | CK3 [M] |

Target size "beyond CK3": CK3 has roughly 30 personality traits plus education, lifestyle, congenital, health and
commander groups [M]. Odyssey can exceed it by adding culture-specific sacred traits and craft traits as technology
grows (e.g. "Copper-giver" only after smelting).

---

## 4. Model spec for the sim

### 4.1 When a chiefdom forms

Use the triggers in `neolithic.md` 4.3. Add these war-based steps (from the Gavrilets-Anderson-Turchin model [O]):

1. Each village has a **strength** F = its own surplus + tribute in (θ = 0.1-0.3 of each subordinate's surplus).
2. Once a year, in the war season, each independent village/polity may attack a neighbour within 1-2 days' walk.
3. If the attacker wins, the loser becomes a **subordinate** (pays θ) instead of being destroyed, but only when it
   cannot flee: circumscribed land or neighbours all around. Otherwise it migrates or is massacred.
4. A centre with ≥2 subordinate villages becomes a **simple chiefdom**. A chief with ≥2 subordinate chiefs becomes a
   **complex chiefdom**. Span of control L = 5-7 direct subordinates.
5. On a chief's death, each subordinate breaks away with probability p = 0.2 + 0.3·(1 - legit) + 0.2·(distance >
   half-day); subordinates also rebel when their own F exceeds ~0.6 of the overlord's.

### 4.2 Cutting territory into counties

Goal: counties look like CK counties (black outer borders), each about one village territory to one small cluster.

| Level | Size | How |
|---|---|---|
| **Barony** (optional, = village) | 20-80 km² | each village's catchment |
| **County** | 150-600 km² (≈ 7-14 km radius, 2-3 hours' walk) | below |
| **Chiefdom core** | all counties whose seat is ≤ half a day's walk (25-30 km cost-distance) from the capital | Spencer [O] |
| **Vassal chiefdom** | counties beyond that, held by sub-chiefs | |

Algorithm (once, at map generation; counties are fixed geography like CK3):
1. Build a travel-cost raster: flat open land 1, forest 2, hills 2-3, marsh 4, rivers crossable only at fords unless
   boats, mountains/sea ∞ (seas cost 1.5 for canoe peoples later).
2. Scatter county seeds by Poisson-disk sampling with spacing ≈ 2 × target radius on habitable land; push seeds to
   good soil or river spots (where villages will be).
3. Grow each county by cost-distance (Dijkstra flood from all seeds at once) = a weighted Voronoi that follows
   ridges and rivers, like ahupuaʻa and moku [O].
4. Merge counties smaller than 40% of target into a neighbour; split those >2x target.
5. Store county adjacency and the border polyline. Draw: thin grey between counties of the same polity, **black 2 px
   between polities**, coloured fill by polity, dashed for tributary ties.
6. Uninhabited counties between chiefdoms are left unclaimed: they show the buffer zones [M].

### 4.3 Levy size

```
men        = pop * 0.24                          # males 15-49
turnout    = base[type] * mods
levy       = sum over counties ( men_c * turnout_c ) + retinue
retinue    = capital_pop * r      # r = 0 (village), 0.01 (simple), 0.02-0.03 (complex), from Chalcolithic
```

| Call-up | base turnout of men |
|---|---|
| Stealth raid | 0.05-0.15 of one village (cap 30) |
| Village defence | 0.8 |
| Chiefdom war, core counties | 0.4 |
| Chiefdom war, vassal counties | 0.2 x vassal loyalty |
| Allied village/chief | 0.1-0.2 if alliance and feast held this year |

Mods: Beloved +10%, Hated -15%, Coward ruler -15%, harvest failed -30%, planting/harvest season -50%, sacred season
(e.g. Makahiki-like) 0. Check against Hawaii 1795: ~12,000 men [O] from a kingdom of ~150,000-250,000 [O] = 5-8%.

### 4.4 War declaration and resolution

**Declare** (yearly AI weights; picks the best target within reach):
- Reasons: revenge for a killing (most common in tribes), loot (food, cattle, captives), land when circumscribed,
  subjugation/tribute (rises with centralisation: Zinkina et al. 2016 found subjugation-war correlates rho 0.58 with
  political centralisation, plunder-war not at all) [O].
- Weight = greed + ambition + revenge + (1 - own food) + land shortage - fear(enemy F) - distance.

**Resolve** (each season):
1. Armies march along the path (see 4.5). Meeting on open ground = battle; reaching a village = assault.
2. Battle: P(win) = F_a^α / (F_a^α + F_d^α) with α ≈ 1.5 [O for form; α from model range 1-2].
   F = men × quality × leader trait × terrain; defender in palisaded village ×2; surprise ×1.5.
3. Losses: winner 1-3%, loser 3-10% in a battle; rout of loser adds 5-20%. Raids: ~1% of raiders die [O].
4. Assault on a village that falls: 20-60% of residents killed, 10-20% (young women and children) taken as captives
   [O pattern: LBK, Crow Creek]; or, if the attacker's goal is tribute, the village is made a subordinate.
5. Peace: when either side's war weariness > 0.5 or after one season. Outcome: tribute, county transfer, captives,
   marriage alliance.
6. Calibrate so that in war-prone regions war deaths average 0.3-0.6% of population a year [O].

### 4.5 Showing armies on the map

| Element | Look |
|---|---|
| Unit marker | a shield or standard in the polity colour, with the **number of men** (e.g. "240"); ruler's lineage emblem if led by him |
| Raid party | small marker with a spear icon and count; moves fast, hides in forest (half visible) |
| Path | dotted line along the cost-path (Dijkstra on the same raster) to the target; arrow head at target |
| Speed | 20-30 km/day on flat ground, half in forest or hills [M] |
| Battle | crossed weapons icon on the spot for one turn; numbers tick down |
| Siege/assault | flame icon on the village; burned villages leave a ruin mark for ~20 years |
| Occupation | county striped in attacker colour until peace |
| Retinue | gold rim on the marker (Chalcolithic+) |

Merge markers in the same county (CK3 style) and split when paths diverge. Sum counts on hover with a breakdown by
county of origin.

---

## Sources (opened this session)

- Spencer, C. S. 2010. Territorial expansion and primary state formation. PNAS 107:7119. https://pmc.ncbi.nlm.nih.gov/articles/PMC2867764
- Mathew, S. & Boyd, R. 2011. Punishment sustains large-scale cooperation in prestate warfare. PNAS. https://pmc.ncbi.nlm.nih.gov/articles/PMC3136302
- Gavrilets, S., Anderson, D. G. & Turchin, P. 2010. Cycling in the complexity of early societies. Cliodynamics 1:58-80. https://journalpub.escholarship.org/cliodynamics/article/38141/galley/28708/download ; https://peterturchin.com/publications/cycling-in-the-complexity-of-early-societies/
- Hally, D. J. & Chamblee, J. F. 2019. Temporal distribution and duration of Mississippian polities. American Antiquity 84(3). https://www.cambridge.org/core/journals/american-antiquity/article/temporal-distribution-and-duration-of-mississippian-polities-in-alabama-georgia-mississippi-and-tennessee/1EC8669E11FD761AA7A328E6055C2C44
- Smith, M. T. Late Prehistoric/Early Historic Chiefdoms. New Georgia Encyclopedia. https://www.georgiaencyclopedia.org/articles/history-archaeology/late-prehistoric-early-historic-chiefdoms-ca-a-d-1300-1850/
- Marcus, J. 2012. The formation of political hierarchies and the loss of autonomy. Social Evolution & History 11(2). https://www.sociostudies.org/journal/articles/148706/
- Zinkina, J., Korotayev, A. & Andreev, A. 2016. Circumscription theory: a cross-cultural re-analysis. Cliodynamics 7. https://journalpub.escholarship.org/cliodynamics/article/38221/galley/28770/download
- Drennan & Peterson 2006 (PNAS 103:3960), via news reports: https://www.stonepages.com/news/archives/001746.html ; https://news.pitt.edu/node/18297 ; record: https://www.anthropology.pitt.edu/publication/patterned-variation-prehistoric-chiefdoms
- Keeley 1996, War Before Civilization: https://wiki.p2pfoundation.net/War_Before_Civilization ; text (partial read): https://ia803006.us.archive.org/10/items/warbeforecivilization/War%20Before%20Civilization_text.pdf
- How violent was the pre-agricultural world? (WWOTF supplement; Bowles 2009, Pinker, Thomson & Halstead). https://whatweowethefuture.com/wp-content/uploads/2023/06/How-violent-was-the-pre-agricultural-world.pdf
- Our World in Data, Ethnographic and archaeological evidence on violent deaths. https://ourworldindata.org/ethnographic-and-archaeological-evidence-on-violent-deaths
- Meyer, C. et al. 2015 (PNAS), Schöneck-Kilianstädten, via Brewminate: https://brewminate.com/mass-grave-reveals-organized-violence-among-europes-first-farmers/
- Fibiger, L. et al. 2023 (PNAS), via University of Edinburgh: https://www.ed.ac.uk/news/2023/violence-was-widespread-in-early-farming-society
- Tübingen project on LBK ditched enclosures and Herxheim: https://uni-tuebingen.de/en/146322
- Gardner & Heider (Dani), via: https://www.culture-of-peace.info/books/history/prehistory-war-2.html
- Li Shuicheng 2018. The mace-head. Social Evolution & History 17(2). https://www.sociostudies.org/journal/articles/1843104/
- Rosenberg, D. 2010. Early maceheads in the southern Levant. J. Field Archaeology 35(2). https://cris.haifa.ac.il/en/publications/early-maceheads-in-the-southern-levant-a-chalcolithic-hallmark-in/
- Claessen, H. J. M. 2018. Sacred kingship: cases from Polynesia. https://sociostudies.org/journal/articles/1842910
- Biersack, A. Rivals and wives: affinal politics and the Tongan ramage. ANU Press. https://press-files.anu.edu.au/downloads/press/p63701/pdf/ch1212.pdf
- Britannica, Ramage. https://www.britannica.com/topic/ramage
- Eisner, M. 2011. Killing kings (Br. J. Criminology), via Cambridge: https://www.cam.ac.uk/research/news/killing-kings
- Hayden 2014, The Power of Feasts, review: https://bcstudies.ubc.ca/book_film_review/the-power-of-feasts-from-prehistory-to-present/
- Seshat, Kingdom of Hawaiʻi: https://seshat-db.com/core/polity/20
- Kirch, How Chiefs Became Kings (publisher page): https://www.ucpress.edu/books/how-chiefs-became-kings/paper
- Trypillian settlement model: https://arxiv.org/pdf/1705.03398
- Wikipedia: https://en.wikipedia.org/wiki/Ahupua%CA%BBa ; https://en.wikipedia.org/wiki/Makahiki ; https://en.wikipedia.org/wiki/Big_man_(anthropology) ; https://en.wikipedia.org/wiki/Chiefdom ; https://en.wikipedia.org/wiki/Ancient_Hawaiian_population ; https://en.wikipedia.org/wiki/Battle_of_Nu%CA%BBuanu ; https://en.wikipedia.org/wiki/Wall_of_Jericho_(Neolithic) ; https://en.wikipedia.org/wiki/Tell_es-Sawwan ; https://en.wikipedia.org/wiki/Chengtoushan ; https://en.wikipedia.org/wiki/%C3%96tzi

Not reachable: PubMed/PMC pages for Bowles 2009 and Glowacki & Wrangham 2015 (captcha); Europe PMC (rate limited);
PNAS page for Drennan & Peterson (403); Annual Reviews PDF of Earle 1987 (did not load). Earle, Johnson & Earle,
Carneiro 1981, Anderson 1994 and Kirch's numbers are therefore [M] where used.
