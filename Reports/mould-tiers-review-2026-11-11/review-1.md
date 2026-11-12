# Review 1: Sporer, Sleeper, Swarmer (mould-tiers-2026-11-11)

Verbatim final report of the first independent review (fresh agent, 2026-11-11
20:18-20:40Z, clean checkout of `main` at 3dabcb7). Its probe scripts and raw
output are in [`review-1-probes/`](review-1-probes/) (paths inside them are
the reviewer's scratch, `/tmp/review-1/`).

---

I worked in /tmp/review-1/repo at 3dabcb7 and changed no repo files. My probe scripts are /tmp/review-1/scripts/probe1.sh and /tmp/review-1/scripts/probe2.sh, and their raw output is in /tmp/review-1/out/. They ran with threads=1, on 1-3 seeds, and are descriptive only.

## 1. Verdict

The engineering is careful:
- The rules are deterministic, use keyed draws, are off by default and port to the GPU.
- The default hash 869141a7bf398adf holds.
- Seed-1 reruns of t4-on and t2-on match the raw files digit for digit. Only the columns added since (crystals, rim, firsts) differ.
- All five sign tests recompute.

As a search for game tiers, though, it tested the wrong thing. Each 8-of-8 test shows that a rule does what it was built to do:
- on a 20 s dish,
- with a fixed walker count and no economy,
- under a bomb-sized strike at a fixed dish position.

None of the tests checks the counter it names. None runs at the game's operating point. My probes suggest the tiers change character there: the Sleeper freezes into an immortal crust, and the game's shot lays none of the alarm the three tiers answer to.

## 2. The tiers' trigger does not exist in the game yet

- **The game's shot lays no alarm.** It strikes through `squad::shots`, which calls `BigDish::kill` (src/mould/squad.rs:207; big.rs:1041). That clears trail and kills, and lays no alarm. The spore burst, the Sleeper's wake and the Swarmer's lean all read only the alarm. So as merged, all three answers to attack are inert in Larder World.
- **The test strike is far bigger than a shot.** The game's widest shot is `R_MAX` 7 fine cells (squad.rs:46). The test strike is 8 dish cells, which is 32 fine cells; T2's is 10 dish cells, 40 fine cells (bigdish.rs:389 default). That is 21-33 times the kill area. The test strike kills 600-2,200 walkers. The W2a twin killed 19 walkers with 15 hits.
- **Probe: does a bullet-sized strike still trigger the rules?** I used `strike_r=1.75`, which is 7 fine cells.
  - Burst Sporer at `spore_rate` 1e-5: the bullet killed 41-113 and launched 339-739 spores within 1 s. It held 6, 7, 7 piles at 20 s, against 4, 7, 6 unstruck (seeds 1-3).
  - Sleeper, one bullet at 14 s: it killed 57 and 27. Awake walkers went from 184 to 18,144 by 17 s on seed 1, and from 2,297 to 15,581 on seed 2.
  - So the relay and the burst turn one bullet into a response across the whole body. That is readable and dramatic.
  - What happens under sustained fire (several shots a second) is untested. The Sleeper may never sleep, and the Sporer may burst without stopping.

## 3. The counters are claimed, not tested

The Summary says "each with a counter the player can learn, all four tests at 8 of 8". The 8-of-8 tests measure the mould's response, not the counter. The wiki tiers page correctly lists the counters as open (wiki/world/mould-tiers.md, *Open*). The report and wiki/model/life-cycle.md should say the same.

**Sporer: "starve it, don't hit it".**
- T2's struck arm has no `spore_rate` (run.sh:37). It is a mould that spores only when struck, not the T1 Sporer.
- T2's control is a plain body struck, not an unstruck Sporer.
- At 1e-4 the unstruck Sporer already holds 6-7 of 7, so there is no headroom to test the claim.
- My probe at 1e-5 supports the direction on 3 seeds: struck by the T2 strike it held 7, 7, 7 against 4, 7, 6 unstruck.
- "Starve" is never tested: no arm has crystal charge.
- Starving this Sporer only switches its fruiting off, since fruiting is gated on scent over 1. That makes it a plain mould. "Starve it" is a counter to every tier.
- The report misses a real decision here. A strike empties the wound: 66-736 walkers left against 1,171-2,361. So hitting a Sporer clears a crystal locally and seeds every other one. That trade-off is worth designing around.

**Sleeper: "bait it".**
- The deaf window is real (section 9).
- The bait's position doesn't matter. A bait at (128, 30) instead of the left edge left 1,835 and 1,912 walkers at the strike at 16 s. The left bait left 1,858 and 1,987; no bait left 2,596 and 2,787.
- Any earlier shock anywhere seems to work, so a random hand that fires twice at different places would likely get the benefit too.
- Firing twice at the same spot (10 s, then 14 s) gave no benefit: 2,577 and 2,648 walkers at the strike. The second strike killed only 16 and 99.
- Only one delay (4 s) was tried. The window's shape is the player's skill curve and was never swept.
- The measure, walkers near a wound, is not a player outcome. With no squad, "rushes the wound" is not shown to be a threat.

**Swarmer: "a kill zone".**
- T4 compares the same fixed-spot fire with and without the alarm lean. No arm tests a kill zone against any other way of firing.
- After the fire the Swarmer floods the wound: 1,409-3,573 walkers there at 20 s against 831-1,224.
- In the game, mould that touches a soldier drains it. So a mould that rushes the point it was shot at may be the hardest tier, not an easy one.
- Piles held at 20 s were lower on 4 of 8 seeds and tied on 4.

## 4. A fixed walker count makes the measures hollow, and M3 already ruled on this

- **With no births or starving, "piles held" just moves walkers around for free.** Spores cost nothing, and the piles never drain.
- **The report doesn't cite M3's rejected spores and crust** (dead-ends.md:35-36), which breaks CLAUDE.md method 5:
  - Crust: "turned lean wins into stalemates".
  - Spores: "no new food comes with a spore".
  - Their re-test conditions are not met here: there is no food economy and no squad.
  - It also doesn't cite Keep It Alive's sleepers (wiki/model/water-and-sleep.md). There, sleep had a cost (`crust_life`), and long-lived sleepers made random play coast to the cap.
- **Probe at the game's operating point** (probe2.sh, seed 1, 90 s): 512 × 320 world, 23 crystals of charge 8 that run dry, growth from 5,000 walkers in 50,000 slots, upkeep 0.001.

| Arm | Alive over 10-90 s | Alive at 90 s | Charge left at 90 s | Piles held |
|---|---|---|---|---|
| Plain | 574-2,362 | 890 | 0.567 | 7-13 of 23 |
| Sporer (1e-4) | 795-1,762 | 1,379 | 0.682 | 10-16 of 23 |
| Sleeper (report's settings) | 6,052 from 30 s on | 6,052 | 0.911 | — |

  - Under growth the Sporer's edge shrinks a lot, and only 2-14 spores are in the air at any report.
  - The Sleeper had 5,131 of 5,206 walkers dormant at 10 s and all 6,052 dormant from 30 s.
  - From 30 s to 90 s nothing changes at all: alive count, births, starvings, charge left and tube area are identical at every report.
  - Dormant walkers pay no upkeep (`live_and_die` skips non-ordinary walkers, big.rs:2007). Thresholds tuned on the 20k dish then put the whole body to sleep.
  - The result is a frozen, immortal world. That breaks the owner's 14:54Z rule ("if you do nothing, the slime should get much stronger than you and kill you"), and the world can never end by its crystals running out.
  - This is one seed. The research thread appears to be running world runs in its own checkout now; this is the failure to look for.

## 5. Approach: creatures or parameter tweaks?

- **The Sporer is a real creature.** It has its own body, map-wide reach and a strategic threat.
- **The Swarmer is one sign of `alarm_weight` on the plain body.** It is a modifier, not a creature.
- **The Sleeper is weaker than the plain mould while asleep.** It is a frozen disc holding 2 of 7 piles; it is interesting only when woken.

A tier earns its place when it makes a different player verb right. A better design question for each one is "which of the player's tools does it beat?":
- **Sporer:** it should fly over walls and rock. Today a spore bounces off anything that isn't agar (big.rs:2176), so walls stop it. Its counter could be shooting spores in flight, which are immune today (big.rs:2241).
- **Sleeper:** it should wake to the soldiers' smell. `squad::smell` already adds scent, and scent wakes it. That gives a stealth harvest: approach quietly, take the leftover charge.
- **Swarmer:** it should drain the soldiers it reaches, so holding a kill zone is a positional skill.

The deciding measures should be on the player's side, against idle and random hands. That means the charge the squad banks, the charge the mould eats from the non-home crystals, the mould's size at 60-120 s, and soldiers lost. This is the scorecard and play probe the project adopted after the first review.

## 6. Other gaps in the tests

- **The strike sits at fixed dish coordinates,** while each seed's body and piles differ. T2's kills range 678-2,170. Placing it relative to the body (for example, its front toward the nearest unheld pile) would fit the game, where a shot lands at the first slime.
- **Measures that saturate.** T2's struck arm holds 7 of 7 on every seed, at the cap. Time to hold k piles, or the charge drained, would give the spread the owner asks for.
- **8 seeds with Bonferroni** means only 8 of 8 can pass. Use 16 or more seeds and a paired test on a continuous measure.
- **Pace was dialled only by rate.** A spore's flight reaches up to 240 × 3 = 720 fine cells; its straight half alone is 90 dish cells, about a third of the dish's width. That jump length is the natural dial for "can't take the map in 10 s". Hops of one crystal spacing would make a visible front of outposts the player can race.
- **One dish, one body size, 20 s runs.** The robustness checks (2 minutes; 5k and 80k walkers) were 3 seeds, at a fixed walker count.
- **The life fields are global.** None of the spore, sleep, wake or alarm fields is in `SpeciesSet` (big.rs:71-82), so Sporer, Sleeper and Swarmer cannot share a world. Mixes (tier 7) and W3 need that. Spores and sleepers also use the global `deposit`, `step`, `sensor_angle` and `rotate_angle` (big.rs:2108-2150), not their species set's.

## 7. Biology and the literature

According to PubMed:
- **Fruiting follows starvation plus a trigger.** It is not a fed walker's random act.
  - Physarum needs a long starvation before it can sporulate. A light pulse then commits it irreversibly 4-5 h later (Chapman & Coote 1982, [doi](https://doi.org/10.1099/00221287-128-7-1489)).
  - Sporulation is controlled by light, heat shock and glucose starvation. Once committed, the plasmodium loses its unlimited growth (Marwan & Starostzik 2002, [doi](https://doi.org/10.1078/14344610260450127)).
  - For play, three things follow:
    - Fruiting should be triggered by a crystal dimming. Its glow is the data, so the player sees the jump coming.
    - It should have a committed, visible delay in which the player can step in.
    - It should cost the mould something. Spores could carry the parent's energy, which would also spend the hoard of energy that crowded walkers can't divide with, a problem the roadmap reports.
- **Dormancy is a stress response.**
  - Drying induces the sclerotium, and water germinates it within hours (Furuhashi 2002, [doi](https://doi.org/10.1111/j.1550-7408.2002.tb00355.x); [doi](https://doi.org/10.1016/s1357-2725(02)00072-9)).
  - Jones's agent sclerotium (the repo's Jones 2015 card) contracts to a disc at its centroid as walkers are lost. The Sleeper instead freezes in place.
  - The project's own water-and-sleep rule (lamp dries, water wakes, sleepers die over time) is closer to both, and more playable.
- **Signals travel along the body.** In Physarum, a local stimulus releases a molecule that the cytoplasmic flow carries and that strengthens its own transport (Alim et al. 2017, [doi](https://doi.org/10.1073/pnas.1618114114)). The alarm here spreads isotropically by a 3x3 mean, "a few fine cells a second". The report itself names that as why Skittish failed. An alarm carried by walkers along tubes would reach the whole body, and a cut tube would block it, which gives the knife a role.
- **The Sleeper's relay with a deaf window is Dictyostelium's cAMP relay.** Starving amoebae relay waves with a refractory period, and spiral cores organise them (Sawai, Thomason & Cox 2005, [doi](https://doi.org/10.1038/nature03228)). The cells then stream together into slugs that migrate toward light and heat before fruiting (Flegel et al. 2011, [doi](https://doi.org/10.1111/j.1440-169X.2011.01300.x)). That is a complete, visible cycle (starve, relay, stream, migrating body, spores), and it is the migrating tier the report leaves open.
- **Habituation.** A plasmodium learns to ignore a repeated stimulus and recovers after rest (Boisseau, Vogel & Dussutour 2016, [doi](https://doi.org/10.1098/rspb.2016.0446)). A Sleeper whose deaf time grows with each wake would punish spam fire and reward rhythm, which is better than a fixed 5 s window.
- **Light as a multiplier.** Jones & Adamatzky's light multiplies sensor readings by less than 1 over a lit region. Skittish used an additive point source instead, so its dead end tested a different rule.

## 8. Improvements, ranked by expected value for the game

1. **Put the tiers on the game's attack.**
   - Make `squad::shots` and the GPU strike lay alarm per hit.
   - First experiment: rerun T1-T4 with `strike_r=1.75` and sustained fire, for example every 0.25 s along the body's front for 10 s.
   - Deciding measure: T1-T4's own measures, plus the share of time the Sleeper is awake under fire. Quote the alarm pass's frame cost.
2. **Test each counter as a plan against random and idle hands,** on the CPU world with growth and crystals that run dry.
   - Sleeper: bait then strike, against two shots at random places and times, against one shot.
   - Sporer: hold fire and wall its crystals, against firing at it.
   - Swarmer: fixed-spot fire, against spread fire.
   - Deciding measure: the mould's walkers at 60 s, or the charge it eats from the non-home crystals. 16 seeds, paired, with Holm.
3. **Give each life state a cost and an economic trigger,** so the world can't stall.
   - Dormant walkers pay part of the upkeep, or die over time like Keep It Alive's crust.
   - Fruit when a crystal's share falls, with a commitment delay; spores carry energy.
   - First experiment: my world probe on 8-16 seeds.
   - Deciding measure: an idle player's mould still grows with the charge, and the world reaches "crystals gone".
4. **Make each tier beat a tool:** spores over walls, the Sleeper waking to soldiers' smell, the Swarmer draining soldiers.
   - Deciding measure: the plan that beats the plain mould loses its edge against the tier, and a different plan wins.
5. **Life-cycle fields per species** (`SpeciesSet`). The first review's coexistence test applies: each species within 2x of its solo run at 60 s.
6. **Alarm carried along tubes,** then re-test Skittish under its dead end's conditions.
   - Deciding measures: the body's displacement at 60 s and its body count; how fast the alarm reaches along a tube compared with across open agar.
7. **A Dictyostelium-style cycle for the migrating tier.** Starvation sets off the Sleeper's relay, and walkers climb the relayed signal into aggregates.
   - Deciding measures: the number of aggregates, and the largest body's net displacement at 60 s against plain, on 8 or more seeds.
8. **Cheap fixes to the measures:** sweep the bait delay (1-8 s); dial pace by jump length as well as rate; place the strike relative to the body.

## 9. Correctness and what holds up

- **Sign tests:** T1-T4 are 8 of 8 (p = 2/256 = 0.0078); T5 is 4-1-3 (p 0.375). All match.
- **Defaults are off.** Life states and the alarm field are allocated only when a rule is on or a strike lands (big.rs:988-994, asserted at the start of the life-cycle test). The hash covers `life`, `life_t` and `alarm`.
- **Report errors:**
  - T2's strike was radius 10 (raw "r 10"), not the "radius 8, alarm out to 16" of the Method (report line 68). So its wound count is within 20 dish cells, not 16 (line 109).
  - T2 is mislabelled as "a Sporer struck" (section 3).
  - The bigdish header doesn't list the new keys (spore_*, sleep_*, wake_*, alarm_*, strike*, bait*, lifeline, style=life). This is the same gap as the first review's C7.
- **Code: no occupancy or claim bugs found.**
  - Fruiting frees the cell. Landing respects the crowd limit with keyed claims.
  - `strike` and `kill` subtract only walkers that hold a cell. `build_wall` moves the dormant and skips spores and the dead.
  - Wakes are decided before any relay is added, so walker order doesn't matter. The tests check `occ` and the thread count.
- **Smaller code issues:**
  - Species conversion checks only `lives(k)` (big.rs:1901), so dormant walkers, spores in flight and, with growth off, the struck dead can change species.
  - A dormant walker counts in `occ` and so dilutes the eaters' share on its cell. This is documented.
  - "Sleep makes the dish cheaper" won't carry to the GPU, where a dormant walker still runs its thread.
- **What holds up:**
  - The Sleeper's deaf window is real, not the body having moved off toward the bait. Walkers at the strike at 13 s: 2,192-2,635 with the bait, 2,192-2,459 without. The refill from 14 to 16 s: +392 to +696 with the bait, +1,113 to +1,326 without.
  - The Sporer's pace table, the wall check (no tier rides the wall) and the cost table check out.
  - The pictures are the data.
