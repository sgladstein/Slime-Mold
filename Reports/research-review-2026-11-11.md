# Independent review of the creature research (2026-11-11)

The owner asked for an independent review of the looks and creatures
thread's work at 15:08Z. They changed the first prompt at 15:14Z: "the
primary goal should be bigger picture. Did you miss something in your tests?
Is your overall approach good? What other directions should be explored. I
want it to tell us how to improve your work, not just validate a report",
and "biological realism should be an inspiration, not fully ignored".

The reviewer was a fresh agent. It had no view of the thread's conclusions
beyond what the reports say. It changed no repo files, and its reruns used
`threads=1`. Its scripts are in
[`research-review-2026-11-11/scripts/`](research-review-2026-11-11/scripts/).
The thread's response, and what it changed, is at the end.

## Prompt

> You are an independent reviewer of a research thread's work on a
> slime-mould agent simulation (Rust). The repo is at /home/user/Slime-Mold,
> on branch `claude/project-thread-c6c5y2`. Read CLAUDE.md first for the
> project's rules.
>
> The owner wants interesting simulations that can be turned into a game.
> Biology and the published physarum models (Jones 2010 and later, Jenson's
> GPU version, and others) are inspiration, not the bar, but they should not
> be ignored. The owner wants "creatures" for gameplay, not just organic
> patterns. The current game direction is a persistent world ("Larder World",
> plan in Reports/larder-world-roadmap-2026-11-11.md). There, energy crystals
> run dry, several mould species form a difficulty ladder, and the player
> fights the mould.
>
> Your main job is the big picture. Tell us how to improve this work, not
> just whether the reports are right.
> 1. **Approach.** Is the overall approach good, as a way to find
>    game-worthy mould creatures? Consider looks on a big dish, one rule per
>    BigParams field, and paired seeds. What is it optimising for, and is
>    that the right target?
> 2. **What the tests missed.** Missing controls or confounds, such as
>    walker count, start size, the dish edge, no food, or run length.
>    Questions that should have been asked and weren't.
> 3. **Biology and the literature as inspiration.** Name the behaviours of
>    real Physarum, or results in published agent models, that the work
>    ignored and that could make better creatures or play. Examples are
>    shuttle streaming, oscillation, memory and avoidance of its own slime,
>    sclerotia and sporulation, fusion and rejection of separate plasmodia,
>    and migration. Cite sources where you can (the web and paper search
>    tools are fine to use).
> 4. **Directions.** Rank 3-6 directions to explore next, by expected value
>    for the game. For each, give the first experiment and the measure that
>    would decide it.
> 5. **Correctness, secondarily.**
>    - Spot-check numbers against the raw files, and recompute the sign
>      tests.
>    - Check that claims are stated at the strength their evidence supports.
>      Looks on 3 seeds are not wins.
>    - Check that the rules are off by default, and that the default hash
>      `869141a7bf398adf` holds (`preset=dense seconds=2 blob=0 piles=1`).
>    - Look for code bugs in species conversion, species_share and the mixed
>      walkers (`kind`, `mix_*`).
>    - Rerun one or two results on 1-2 seeds at short lengths. The machine
>      has 4 cores, and another job is using them, so keep reruns small. Use
>      `threads=1`, and write outputs only to /tmp/review/.
>
> **In scope:**
> - `Reports/jenson-looks-2026-11-11.md`, `Reports/creatures-2026-11-11.md`
>   and `Reports/spread-mix-2026-11-11.md`, with their folders.
> - Their entries in `Reports/dead-ends.md`.
> - `wiki/model/scale.md`.
> - The `BigParams` rules in `src/mould/big.rs`, and the options in
>   `examples/bigdish.rs`.
> - For background only: `Reports/jenson-compare-2026-11-10.md`.
> - A test about to run, mixes on crystals that run dry (the script, now
>   `Reports/crystal-mix-2026-11-11/run.sh`). Critique its design, including
>   the deciding measure, but do not run it.
>
> Don't modify any repo files, and don't commit. Report back in markdown:
> - The big-picture verdict.
> - What was missed.
> - The ranked directions, each with its first experiment.
> - Correctness findings ranked by severity, each with the file and line,
>   and the evidence.
> - A short list of what holds up.
>
> Be direct and adversarial where it is warranted.

## The review (verbatim)

Line numbers refer to the branch at 51fac80.

I modified no repo files and committed nothing; the working tree is still clean. My reruns used `threads=1` and wrote only to `/tmp/review/`.

### 1. Big-picture verdict

The individual tests are careful: measures named before each run, paired seeds, sign tests computed correctly, new rules off by default, and the default hash holds. Two findings are real:
- Grazing makes droplets move.
- A mixed two-species population is decided by majority.

**But the work optimises for the wrong target.** It is chasing two things:
- Jenson's looks.
- Stills that look like creatures, on one operating point: no food, a fixed walker count, an elliptical dish, a centred disc start, runs of 20-30 s.

None of these is the game's operating point. The game has growth, crystals that run dry, a rectangular world with rock, and minutes of play. None of the three reports puts a player verb against a creature, which skips CLAUDE.md method 3 (a plan must win where random loses). "Outriders are easy to cut" (`spread-mix:74`) is asserted, not tested. Several headline behaviours come from the dish wall, and the pictures that showed them drew control and treatment with different tone curves.

**One `BigParams` field per rule** keeps defaults safe and A/B tests cheap, but it hides a blocker for the Larder ladder. Sensor angle, offset, step, deposit, keep, crowd, `blocked_deposit` and grazing are all global, and `mix_*` is a second axis independent of species. So a Drifter (droplet settings: crowd 1, SA 90, deposit 5, 30k walkers) and a Sprawler or Trunk (network settings: crowd 0, SA 45, deposit 0.5, 1M walkers) **cannot coexist on one dish**. No one has flagged this, and W3 needs it.

**Paired seeds are fine. The 8-seed sign test is weak:** the smallest possible p is 0.0078, so after any correction only an 8 of 8 result passes. Droplet dishes cost about 13 ms a tick, so 16-32 seeds and a paired permutation or Wilcoxon test on the continuous measure are cheap.

**What it should optimise for instead:** a scorecard for each creature, then a play probe. The scorecard:
- How long a body keeps its identity.
- Self-propulsion: net displacement against time lag.
- Whether it is stationary over 2-5 min.
- Whether it still works at 0.25x to 4x the walkers.
- Whether it does the same thing away from walls.
- Its response to food and to tools.
- Whether it coexists with other species.
- Its frame cost.

The play probe is idle, random and plan hands against the "what beats it" column of the roadmap.

### 2. What the tests missed

1. **The dish edge is a confound.** For trail, the wall is reflective: `spread_rows` averages over agar neighbours only (`big.rs:479-487`). For sensors, the wall reads as -1e6 (`big.rs:675`). So it is a stable rail.
   - My rerun of the plain network at 2 s (seed 1): 5,204 walkers already ride the wall, and **half of all tube area** (12.4k of 25.1k fine cells) lies outside the ring.
   - That wall halo is invisible in the soft still: the curve is normalised to the 99.9th percentile (trail 2155), while the wall trail is at most 17.
   - Every "reaches the wall" result in spread-mix is at least partly this effect. Rock in the Larder world is not agar either, so long-reach walkers will hug rocks.
   - No torus or large-world control was run.
2. **No food.** A Jones-type network with no stimulus contracts and coarsens, as Jones 2010 already reports. "Closes into a ring" is that textbook behaviour, not a trait of the creature. Spread with no food is the wrong question for a crystal world.
3. **A fixed walker count.** "Spread", "eat" and "fight" are all measured with conserved mass. The game will have births and starving (W0), and every conclusion may change under it.
4. **Walker count and density were never swept.** Each look was found at one N (1M or 30k). Under growth, N moves 10-100x during play.
5. **The start causes its own effects.** About 1% of walkers burst out of the 50-walkers-a-cell disc in the first second (10.2k beyond 0.3 of the radius at 1 s). That burst seeds the wall halo.
6. **Run length is too short.** Networks are still coarsening at 20 s: loops 48-58 at 9 s fall to 17-25 at 20 s. My 60 s rerun of grazers (seed 1): bodies go 227 → 147 → 108 → 79 at 5/10/20/60 s, and the median size grows from 171 to 391 px. The 10 s test caught them mid-merge.
7. **No control with the second kind alone** (`mix_share=1`). So "the kinds sort themselves" can't be told apart from two populations doing what each does alone.
8. **Species with avoid 0 and crowd 0 don't interact.** In `fight.sh` and `numbers.sh`, each walker senses `(1+0)·own − 0·total = own` (`big.rs:692`). With no crowd limit, the two networks pass through each other: there is no border physics, only conversion. The repo's own Jones 2015 card says borders need an explicit mutual-avoidance rule.
9. **Body identity was never measured.** The creatures report sells "countable units". In my seed 1 rerun there are 1-11 splits and 1-11 merges every 0.1 s between 4 and 7 s, and only 26 of 295 components keep an unbroken track for 8 s.
10. **The timing of the intervention** in the knife test (see C1 below).

### 3. Biology and published models the work ignored

PubMed sources, with DOIs under Sources. Several already have cards in `research/sources/` (Jones 2011, Jones-Adamatzky 2012, the Jones 2015 survey, Tero 2010), but none of the three reports used them.

- **Avoiding its own slime (Reid et al. 2012).** The plasmodium avoids ground it has already explored, which lets it escape a U-shaped trap. `graze_*` is functionally this mechanism, credited only to Primordia. Generalised per species ("avoid my own old ground"), it gives migrating fronts that leave a dry crystal behind, which is exactly what a run-dry world needs.
- **Flow-reinforced tubes and memory in tube width (Tero et al. 2010; Kramar & Alim 2021).** Tubes that carry flow thicken; the location of food is stored in the tube-width hierarchy and redirects later migration. Here, trunks came only from a dense start. Food-made trunks would give "cut the supply line" a target; that idea failed on the small dish (`dead-ends.md:26`).
- **Shuttle streaming and peristaltic oscillation (Alim et al. 2013).** Contraction waves scale to the organism's size. In agent models, Jones & Adamatzky 2012 get oscillation and travelling waves from jammed walkers, blobs that migrate toward food and away from light, and blobs that split and fuse. `blocked_keep` and `blocked_reset` exist from that paper, but oscillation was never measured; outside-rules only looked for motion on seed 1. A visible heartbeat gives timing play, and the picture stays the data.
- **Fusion and lethal incompatibility (Schrauwen 1981).** After a killer strain fuses with a sensitive one, the sensitive strain's DNA is broken down and the fused plasmodium behaves as the killer. That is the contact-mixing fight rule the creatures report lists as untested.
- **Habituation (Boisseau et al. 2016), and transfer of learned behaviour through fusion (Vogel & Dussutour 2016).** The mould learns to ignore a repellent it meets again and again, recovers after a rest, and passes this on when it fuses. In play, an overused repellent loses force: a counterplay rule.
- **Nutrient balancing (Dussutour et al. 2010).** The plasmodium grows onto patches in the proportions of an optimal protein-to-carbohydrate diet. Two crystal types would give the player a lever: deny one type.
- **Speed-accuracy trade-off and decoys (Latty & Beekman 2011, two papers).** Stressed moulds decide faster and err more, and adding a worse option changes their preference. Hungry moulds should be reckless, which makes bait matter.
- **Anticipation (Saigusa et al. 2008).** The plasmodium slows at the time the next periodic shock is due. A late-tier idea: a mould that learns the player's rhythm.
- **Sporulation and sclerotia.** Starving plasmodia sporulate when given far-red light, blue light or heat shock (Starostzik & Marwan 1995). The repo's Jones 2015 card already has a sclerotium rule. These are life-cycle responses to crystals running dry: the Sleeper and Sporer tiers.
- **Migration.** Microplasmodia move with back-to-front waves (Lewis et al. 2015). No creature here migrates as a body: tracked grazers move at about 2 fine cells a second, roughly 1.7% of walker speed.

### 4. Ranked directions, by expected value for the game

1. **Growth and starving on the CPU world (W0 early), then re-judge every creature under it.** Everything above is conditional on a fixed walker count.
   - *First experiment:* Jones 2011's density-window rule (divide with 1-10 walkers in a 9x9 window, die above 20 in a 5x5) gated by crystal energy, on the 512x320 crystal world.
   - *Decider:* walker count rises while a crystal is in reach and falls after it runs dry, on 8 of 8 seeds. Secondary: droplets keep a body count per walker within 2x across the walker-count range they pass through.
2. **A parameter set for each species, and a coexistence test.** This blocks the ladder.
   - *First experiment:* Drifter (grazing-droplet settings) and Sprawler (explorer network) on one world.
   - *Decider:* each species' body count or tube area at 60 s stays within 2x of its solo run, paired over 8 seeds.
3. **Contact rules from fusion and incompatibility, replacing pure trail-majority conversion.** Fights are W3's gate.
   - *First experiment:* rerun `hunt.sh ZONES=1` (the failed test, 5 of 7, p 0.45) with fusion on contact: other-species trail above a threshold within r makes both sides species-blind for T steps, then conversion over r applies.
   - *Decider:* the 55% side's gain at 30 s, 16 seeds, paired sign test.
4. **Flow-reinforced trunks (Tero; Kramar & Alim).** Make `keep` rise with the walker flux through a cell.
   - *First experiment:* two crystals 100 dish cells apart.
   - *Decider:* share of tube in the shortest corridor at 60 s, plus re-route time after a knife cut, against plain, 8 seeds. Add an idle and plan probe for "cut the supply line".
5. **Heartbeat and identity for droplets (Jones & Adamatzky 2012).**
   - *First experiment:* sweep `blocked_reset` over 1, 0.1, 0.01 and 0.001, crossed with `blocked_retry` 0 and 3, on grazing droplets. Note that all-at-once moves with crowd 1 may suppress the waves.
   - *Decider:* a spectral peak in per-body area or in flux across a line, on 8 of 8 seeds against reset 1; and the share of bodies tracked for 10 s.
6. **Life cycle when crystals run dry.** Per-species self-avoidance gives migration, plus sclerotium dormancy and sporulation. This depends on direction 1.
   - *Decider:* mould survival across a 60 s crystal drought with and without the rule, plus an idle and plan probe (attack the dormant disc).
   - This meets the re-test conditions of M3's rejected spore and crust rules.

Cheap side experiment: re-test the knife properly, timed to the front's arrival. With short immunity, cutting a wavefront is the classic way to get spirals in an excitable medium. That would give contagion a visible, lasting player effect.

### 5. The crystal-mix script

It was already running when I checked, 4 jobs at once, from the `cry` worktree's own build. Its deciding measure, `crystals`, prints on the `tube` line, so the grep keeps it. The design has these problems:

- **The start disc covers the crystals.** I replicated `place_piles` and checked it against `piles k/12` reached at 0.5 s: predicted about 7, 8 and 5 on seeds 1, 2 and 7; the binary gave 8, 8 and 5.
  - The droplet disc (`blob_r` 50) covers 3-6 of the 12 crystals on every seed; the network disc covers 0-2.
  - So "charge eaten by 60 s" mostly measures where the start landed.
  - *Fix:* keep crystals out of the start disc plus a margin, or place them on rings at fixed distances. Score first-contact time per crystal, or crystals reached by 30 s, not charge eaten. With a fixed walker count, eating has no feedback anyway.
- **`grazescouts` is compared against plain `droplets`, not `grazers`.** That confounds grazing with the scouts.
- **Scouts don't smell farther.** The scent reach `sso` is computed once from the base sensor offset (`big.rs:732`), and the `dense` preset sets `scent_offset` 36. `mix_reach` scales trail and wall sensing only. Either state this or add a scent multiplier.
- **Pile 0 is muted.** It counts as reached at t=0 (`big.rs:614`), so with a centred start it calls at only 15% scent (`fed_scent`).
- **The wall confound again.** Wall riders pass the crystals near the rim. Classify crystals by distance to the wall, or use `rect` with the start far from the walls.
- **The correction can't be met.** With 8 seeds the smallest p is 0.0078. Bonferroni over 4 mixes needs 8 of 8; over all 8 comparisons nothing can pass. Use 16 or more seeds for droplets (about 1.5 min a run), Holm, and a paired test on the continuous measure.
- **No pure second-kind arms** (`mix_share=1`).
- **Cost.** The network group is 1M walkers for 60 s: about 13-22 min a run on a loaded core, 40 runs.
- **Provenance.** The worktree shows unmerged paths (UU on `big.rs` and `bigdish.rs`), though there are no conflict markers. Record the exact resolved source state with the raw output.

### 6. Correctness findings, by severity

**C1 (high): the knife test never tested a cut.**
- Where: `jenson-looks-2026-11-11/cut.sh` (`infect=4` and `cut=4` on the same tick; `bigdish.rs:411-426`); claims at `jenson-looks:39-43`, `dead-ends.md:28`, `wiki/model/scale.md:160,200`.
- The front reached the cut line (32 dish cells) only at 5.0 s (`raw/Ccut-s1.txt`: front 33.1).
- My rerun dumped the trail: the 8-column band's tube share was back to the flank's level within 6 ticks (0.314 against 0.321). The cut had healed 0.95 s before the wave arrived.
- Clearing germ at 4 s did nothing: there was no germ at the line yet.
- So "a knife cut does not stop the wave" and "walkers carry it across" are unsupported. The dead-end's re-test condition should say "the cut is made as the front arrives".

**C2 (high): spread-mix's outriders and "one thin ring".**
- Outriders (`spread-mix:52`):
  - At 2 s the scouts' median distance is 34.6-39.2 fine cells, against the core's 58-62 (`raw/mix-net.txt`). Most scouts form an inner ring.
  - My rerun: 18,591 walkers lie beyond 0.6 of the dish radius, against 8,351 in the plain network. So at most about 19% of scouts reach the wall.
  - The plain network's own wall halo is hidden by the soft curve, while the mixes use `style=kinds`, where one walker in a cell shows about 45% bright. Control and treatment pictures are not comparable.
- "One thin ring" (`spread-mix:28`) is wrong:
  - `reach` 189.6-192.1 is measured from the home pile (`big.rs:1211`), which puts tube within about 5 dish cells of the far wall at 20 s.
  - "0.4% of the dish" is a unit slip: `tube_area` is in dish cells (`big.rs:389-390,1265`), so 1832-1939 is **6.1-6.4%** of the roughly 30,240 dish cells of agar.
  - The wiki row at `wiki/model/scale.md:209` inherits "(one ring)".

**C3 (medium-high): the boost mechanism contradicts the model.**
- Where: `creatures:50`, `dead-ends.md:32`.
- With `species_avoid` 0, each species steers by its own field alone, and the field is linear. Steering compares sensors (`big.rs:798-805`), so scaling the deposit by 1.5 cannot change where species 0's tubes go.
- The boost only shifts the conversion thresholds (`big.rs:986`): species 1 joins species 0 when raw0 > 0.8·raw1, and species 0 joins species 1 only when raw1 > 1.8·raw0. That *favours* species 0.
- "Louder tubes reach into the other half and convert" can't happen. The null result stands, but its story and its re-test condition are wrong.
- `wiki/model/scale.md:241` ("sorted bodies hold their border against a louder trail") misdescribes two ghost networks.

**C4 (medium): provenance.**
- `creatures/raw/fight.txt` is a hand-typed relay summary with a runner "correction" note, not program output.
- `spread-net.txt` and `mix-net.txt` are partial lines "as relayed".
- The trunk test's deciding measure (ring share per seed) is not saved anywhere in the repo; `trunk.py` reads dumps from `/tmp`. So the one "survives" result in jenson-looks can't be recomputed.
- The values themselves check out (see "What holds up").

**C5 (medium): the crystal-mix design flaws in section 5.**

**C6 (low-medium): overstatements.**
- Splitter (`spread-mix:53`): "fly out to the wall" holds on 2 of 3 seeds; on seed 2 the ordinary walkers' median is 137.5 fine cells, and the wall is 308 or more away.
- Roamer (`spread-mix:54`): its end state is everything on the wall.
- "Bodies stay bodies" (`creatures:24`) is a 10 s snapshot: 79 bodies by 60 s.
- Rewiring: the 9 s loop count is a secondary measure labelled "survives". The conclusion is unchanged at the corrected threshold for 5 measures (0.01).

**C7 (low).**
- `new_options_default_off` (`big.rs:1505-1510`) doesn't check that `kind` and `grass` are empty.
- The hash check uses `blob=0`, so it never runs the `blob=1` start path the game uses.
- The `bigdish.rs` header (lines 55-76) doesn't list the `mix_*`, `graze_*`, `species_share`, `species_convert_r` or `species_boost` keys.
- `reach` is measured from pile 0 even with a centred start.
- The doc comment for `mix_reach` should say the scent reach is unchanged.

**I found no bugs** in the species conversion (r=0 or r>0, cyclic chaser correct, double-buffered), in `species_share` (the floor test is equivalent to the doc's formula), or in `kind` and `mix_*` (keyed on walker index, kept through respawn; `lay` applies `mix_deposit` to moving, blocked and capped deposits).

### 7. What holds up

- **The default hash holds:** `869141a7bf398adf` on `preset=dense seconds=2 blob=0 piles=1` (rerun). Every new field defaults to off.
- **All recomputed sign tests match:**
  - trunk loops 7 of 8, p 0.07;
  - rewiring 8 of 8, p 0.008 at 9 s, and 5-2-1, p 0.45 at 20 s;
  - cut 4-4, p 1;
  - boost 6 of 8 lower, p 0.29 (also higher at 8 s on 6 of 8, falling on 7 of 8);
  - numbers 3 of 8, p 0.73;
  - hunt r0 and r8 both 8 of 8 (gains match);
  - zones 5 of 7 with a tie at printed precision, p 0.45;
  - blind 5 of 8, p 0.73;
  - graze 8 of 8.
- **Reruns reproduce the raw lines exactly:**
  - network seed 1 at 2 s: tube 1571.6, reach 151.2;
  - scouts seed 1 at 2 s: kinds line identical;
  - knife seed 1, 4-5 s: identical;
  - grazing seed 1: 176 bodies, drift 6.59, p90 32.19.
- **Grazing gives real self-propulsion.** For tracked bodies, net displacement grows about linearly with lag: 4.1 fine cells at 2 s, 17.1 at 8 s.
- **The trunk ring is genuine trail memory.** With respawn 0.001 for 2400 steps, about 91% of walkers have been re-placed by 20 s, yet the ring holds.
- **Majority wins in a mixed start, and sorted bodies hold their colour.** The reports state this at the right strength.
- **The spread-mix report labels itself as looks with no paired test,** which is correct.

### Sources (via PubMed)

- Reid et al. 2012, PNAS: [10.1073/pnas.1215037109](https://doi.org/10.1073/pnas.1215037109)
- Tero et al. 2010, Science: [10.1126/science.1177894](https://doi.org/10.1126/science.1177894)
- Kramar & Alim 2021, PNAS: [10.1073/pnas.2007815118](https://doi.org/10.1073/pnas.2007815118)
- Alim et al. 2013, PNAS: [10.1073/pnas.1305049110](https://doi.org/10.1073/pnas.1305049110)
- Jones & Adamatzky 2012, Bioinspiration & Biomimetics: [10.1088/1748-3182/7/1/016009](https://doi.org/10.1088/1748-3182/7/1/016009)
- Jones 2010, Artificial Life: [10.1162/artl.2010.16.2.16202](https://doi.org/10.1162/artl.2010.16.2.16202)
- Schrauwen 1981, Archives of Microbiology: [10.1007/BF00414693](https://doi.org/10.1007/BF00414693)
- Boisseau, Vogel & Dussutour 2016, Proc. R. Soc. B: [10.1098/rspb.2016.0446](https://doi.org/10.1098/rspb.2016.0446)
- Vogel & Dussutour 2016, Proc. R. Soc. B: [10.1098/rspb.2016.2382](https://doi.org/10.1098/rspb.2016.2382)
- Dussutour et al. 2010, PNAS: [10.1073/pnas.0912198107](https://doi.org/10.1073/pnas.0912198107)
- Latty & Beekman 2011, Proc. R. Soc. B: [10.1098/rspb.2010.1624](https://doi.org/10.1098/rspb.2010.1624) and [10.1098/rspb.2010.1045](https://doi.org/10.1098/rspb.2010.1045)
- Saigusa et al. 2008, Physical Review Letters: [10.1103/PhysRevLett.100.018101](https://doi.org/10.1103/PhysRevLett.100.018101)
- Starostzik & Marwan 1995, Photochemistry and Photobiology: [10.1111/j.1751-1097.1995.tb09158.x](https://doi.org/10.1111/j.1751-1097.1995.tb09158.x)
- Lewis et al. 2015, J. R. Soc. Interface: [10.1098/rsif.2014.1359](https://doi.org/10.1098/rsif.2014.1359)

## The thread's response

The thread accepted every correctness finding it could check. It confirmed
three itself:
- `tube_area` is in dish cells (`big.rs:390,1265`).
- `cut.sh` sets `infect=4 cut=4`.
- `raw/Ccut-s1.txt` has the front at 0.0 until after 4 s.

Fixed in the same change as this file:
- **C1:** the knife result is marked untested, in `jenson-looks`, its page,
  `dead-ends.md` and `wiki/model/scale.md`.
- **C2:** the tube share, the wall rail, the outriders' inner ring, and the
  splitter on 2 of 3 seeds, in `spread-mix` and its page. A *Corrections*
  section is added there.
- **C3:** the boost story, in `creatures`, its page and `dead-ends.md`.
  The null result stands; why the boosted side lost is open.
- **C6:** the bodies count over 60 s, in `creatures`.

Left for the next code change:
- **C7:** the `bigdish` header keys, the `mix_reach` doc noting that scent
  reach is unchanged, `new_options_default_off` checking `kind` and
  `grass`, and a hash check on the `blob=1` start.
- **C4:** provenance. New raw output is saved as program output, not
  relayed lines. The trunk ring shares can't be recovered: the dumps are
  gone.

**The crystal-mix test (C5).** The droplet arm had already run: 8 seeds,
60 s, charge eaten. Grazers, grazing outriders and fast droplets ate more
than plain droplets on 8 of 8, and bridged droplets less on 8 of 8. But
the start disc covers 3-6 of the 12 crystals, so these numbers mostly
measure where the start landed. They are not reported as a result. The
1M-walker network arm was stopped after 3 runs. The test will be
redesigned before it runs again:
- crystals kept clear of the start;
- first contact per crystal, and crystals reached by 30 s;
- `grazescouts` against `grazers`;
- arms with the second kind alone;
- 16 seeds or more, with Holm and a paired test on the continuous measure;
- the wall's share named.

**Directions.**
- The thread takes the per-species parameter sets and the coexistence test
  (direction 2), since the ladder is blocked on it.
- W0 growth (direction 1) belongs to the GPU thread. It was sent the
  density-window rule from Jones 2011.
- The contact and fusion fights, flow trunks, heartbeat, life cycle and the
  biology list (directions 3-6 and section 3) went to the "Find new mould
  behaviours" thread.
- The idea of a scorecard for each creature, plus a play probe, is adopted
  as the bar for new creatures. That includes testing away from the wall,
  at 0.25x to 4x the walkers, and for 2-5 min.

