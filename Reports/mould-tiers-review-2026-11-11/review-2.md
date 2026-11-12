# Review 2: unexplored directions for the mould ladder

Verbatim final report of the second independent review (fresh agent,
2026-11-11 20:18-20:42Z, clean checkout of `main` at 3dabcb7). Its pulse
prototype is kept as a diff against `src/mould/big.rs` with its driver
([`review-2-probes/pulse-rule.diff`](review-2-probes/pulse-rule.diff),
[`pulse-main.rs`](review-2-probes/pulse-main.rs)) and its raw output in
[`review-2-probes/runs/`](review-2-probes/runs/); paths inside are the
reviewer's scratch, `/tmp/review-2/`.

---

## Unexplored directions for the Larder World mould ladder, ranked (review 2, 2026-11-11)

I changed no repo files and posted nothing. All runs used `threads=1` and were short. Scratch work is in /tmp/review-2/ (paths at the end). After the container restart I reran only the pulse arms I still needed.

### What is built and what isn't
The earlier review (`Reports/research-review-2026-11-11.md`) already proposed flow trunks, a heartbeat, contact fights and a life cycle.
- **Built on main:** only the life cycle (Sporer, Sleeper, Swarmer).
- **Never got a rule or an experiment:** flow trunks, heartbeat, contact fights, plus habituation, anticipation and two foods. The last three appear only in that review's biology list.
- **In progress, not on main:** another thread is rerunning the three tiers at the world's operating point (`Reports/mould-tiers-2026-11-11/world/` in the main checkout). That covers part of the method point below.

### Hunch checks I ran (seed 1-2, looks rather than tests)
- **The existing Sleeper relay does not migrate a body.** Setup: bigdish, a bait every 4 s or 8 s at the crust's edge, seed 1.
  - Each beat wakes most of the crust: 13,952-17,417 walkers awake at the peaks.
  - The centre drifts 0.7-1.1 dish cells *away* from the beat over 20-24 s.
- **A relay with a one-shot directed step (prototype in a scratch copy of `big.rs`).** Setup: 20,000 walkers, dense preset, 7 piles, a beat every 6 s at the left edge.
  - Each beat fires 17,340-19,173 of the 20,000 walkers; the wave crosses the body in under 2 s.
  - The relay alone, with no step, moves nothing. That is the expected control.
  - With a 2 s directed step, mean x at 24 s was 127.5 and 126.0, against 129.5 and 126.2 with no rule. That is only 2.0 and 0.2 dish cells toward the beat after 4 beats.
  - The step did turn the mould outward: piles reached at 24 s were 7 and 7 of 7, against 4 and 5 with no rule. Laying 4 times the trail while stepping also gave 7 and 7, and moved it 2.3 and 0.6 cells.
  - So a fixed-count, fed network is held in place by its trail and its piles. The beat gives a visible heartbeat that sends the mould outward in pulses, not a whole body that moves.
- **Turnover in the world.** The other thread's in-progress raw (`world/raw/plain-s1.txt`, seed 1): 30,998 born and 32,359 starved by 60 s with 3,639 alive. The plain body's centre moved from x 62 to 90 between 30 s and 60 s. Two consequences:
  - Migration may come almost free under growth.
  - Any memory kept per walker (`sleep_rest`, `spore_rest`, habituation) is wiped in a few seconds. Learned state must live in a field or pass to children at division.

### Ranked directions (most promising first)

**1. Supply lines: trunks made by traffic that carry food**
- **Source:** Tero 2010; Kramar & Alim 2021; Alim 2013; Matsumoto 2008. Also the small dish's own "Food that flows" (`Params::flow`): with it, a cut-off piece really starved, and the storm cost a pile on 4 of 4 seeds against 2 of 4.
- **Rule:** a non-diffusing conductance field C.
  - Growth and decay: `C += flux_grow × walkers entering the cell`, then `C *= flux_keep` (half-life about 10 s).
  - Where C is high: trail keep rises toward 1 by `flux_hold × min(C,1)`; `energy_flow` is weighted by `min(C,1)`; and the step is multiplied by `1 + flux_speed × min(C,1)`.
  - A strike clears C, so a cut must be rebuilt by traffic.
- **Why it's needed:** today's diffusive `energy_flow` reaches only about 17 fine cells from a crystal. That is my estimate: D = 3 × flow = 0.15 cells² a step, upkeep 0.001, source energy 1. Food reaches the front only by walkers carrying it, and a trail cut regrows within about 6 ticks (review C1).
- **Body:** a few bright arteries from crystals to the front. Outposts on dry crystals live on what arrives through them.
- **Struck:** a cut stays cut, and the far tips dim first.
- **Counter:** find the artery and cut it near the crystal, or wall it. This gives the Trunk and Hunter tiers a learnable weakness, and it meets the re-test condition of the `larderprobe` supply-cut dead end.
- **First experiment:** world race (512×320, growth, charge 512, flow 0.01), 16 seeds, a 2×2 design.
  - Cut the highest-C tube cell on the path to the farthest held body, against a cut on a random tube cell with the same trail; rule on against rule off.
  - **Deciding measure:** walkers within 20 dish cells of the far end, 20 s after the cut. The test is the interaction (targeted beats random only with the rule on): Wilcoxon, Holm.
- **Cost:** one per-cell update, about the size of the world passes (0.004-0.018 ms each on the 1070); a few percent on the CPU.

**2. Habituation: the mould learns to ignore your trick**
- **Source:** Boisseau 2016; Vogel & Dussutour 2016 (habituation passes to a partner through fusion).
- **Rule:** a tolerance field U on the dish grid, one per stimulus kind (fire's alarm, bait smell).
  - Where the stimulus is above `hab_on`: `U += hab_gain × (1−U)`. Elsewhere: `U *= hab_keep`, so the mould recovers after rest.
  - Its answers are scaled by (1−U): `alarm_weight`, `spore_burst`, and the wake threshold.
  - Walkers carry U and lay it back where they go, and children inherit it, so it spreads to mould that touches it.
- **Body:** a "Veteran". Its first answers are strong (the Sleeper's wake wave, the Sporer's burst, the Swarmer's rush) and weaker on each repeat.
- **Counter:** vary the trick, space repeats past the recovery time, and kill the experienced part before it merges with naive mould. This is a structural answer to the "one strategy wins" result seen in every RTS demo here.
- **First experiment:** the Sleeper's bait-then-strike, 5 cycles 15 s apart, habituation on against off, 16 seeds. Then a 60 s rest and a sixth cycle.
  - **Deciding measure:** the bait's advantage on cycle 5 (walkers at the strike 2 s after it, bait arm minus no-bait arm), and whether it comes back on cycle 6.
- **Cost:** one coarse field with no spreading; under 1% on the CPU, about 0.01 ms on the GPU.

**3. Fusion and rejection between clones, with a scar**
- **Source:** Border & Carlile 1974; Schrauwen 1981 (lethal reaction after fusion). The scar draws on Reid 2012's avoidance of its own slime; fusion on touch on Jones & Adamatzky 2012.
- **Rule:**
  - Each walker carries a clone tag (one byte, e.g. 2 loci × 4 alleles), inherited by children and by landed spores (`clone_mut` lets a spore mutate).
  - Each cell keeps the tag of the last walker to lay trail there.
  - Meeting a different tag over the trail cut: if compatible, the walker joins that clone with chance `p_fuse`. If not, it dies with chance `reject_kill` a step and lays into a scar field S (no spreading, slow decay). Sensors read −`scar_weight` × S, and trail deposit is multiplied by (1−S).
- **Body:** "Clans", colonies of one species that fight along dark seams.
- **Counter:** lure two clans into each other. This gives the Rival-pair tier's "lure one into the other" real teeth: today species with avoid 0 and no crowd limit pass through each other. The scar also makes any cut last.
- **First experiment:** two colonies 60 dish cells apart, compatible against incompatible. The probe is a bait between them at 30 s against an equal-cost bait at a random place, 16 seeds.
  - **Deciding measure:** walkers alive at 120 s.
- **Cost:** one byte per walker and per cell, one compare a step, one decay pass.

**4. Anticipation: the mould learns your rhythm**
- **Source:** Saigusa et al. 2008 (plasmodia slow down when the next periodic shock is due).
- **Rule:** per dish cell, keep the last alarm onset L, an average gap I between onsets, and a confidence c (rises on an onset within ±`ant_tol` of L+I, halves otherwise).
  - When c ≥ `ant_need` and the time is within [L+I−`ant_lead`, L+I+`ant_tol`], the cell reads −`ant_weight` to sensors: the mould flinches.
  - It keeps flinching for a missed beat or two, then forgets.
- **Body:** an "Anticipator" that empties a turret's aim area just before each shot and floods back. Larder's soldiers and turrets fire on a fixed `shot_every` of 1.0 s.
- **Counter:** break the rhythm (manual fire, a jitter upgrade, turrets out of phase), or exploit it: a harmless regular beat keeps it off a crystal you harvest.
- **First experiment:** 2×2 of anticipation on/off × 10 strikes regular every 2 s against keyed jitter of ±0.8 s (same count), 16 seeds.
  - **Deciding measure:** walkers killed by strikes 6-10. The test is the interaction.
- **Cost:** three small dish-grid arrays, updated only where the alarm is above threshold; negligible.

**5. Pacemaker waves: a visible heartbeat**
- **Source:** Matsumoto 2008, Lewis 2015 and Alim 2013 (phase waves set Physarum's flow and direction); Marée & Hogeweg 2001 (Dictyostelium relay).
- **Rule (as prototyped):**
  - A pulse field P: 3×3 mean, keep 0.98.
  - A walker that is not refractory fires when P on its cell is over 1. It adds 20 to its cell after all decisions, faces the strongest of 8 readings 12 fine cells out, and walks that way for `pulse_burst` steps. It is then deaf for 600 steps.
  - Fed walkers fire on their own with chance `pace_rate`, and the player's strike adds to P, so firing becomes a drum.
- **Hunch result:** above. It gives a heartbeat and pulsed outward reach, not migration, while the mould has a fixed count and is fed.
- **Body:** a "Drummer" with rings running out from its fed core.
- **Counter:** kill the pacemaker core so the pulses stop or fall out of step, or drum it into a kill zone.
- **First experiment:** in the world race with growth, 16 seeds.
  - **Deciding measure:** the body's net displacement toward a player drum over 60 s, against no drum.
  - Second probe: piles reached at 30 s after the fed core is struck, against a random strike.
- **Cost:** one field spread with a 3×3 mean. On the 1070 that is about the trail pass (0.127 ms a tick at 1024×640). On the CPU the alarm field measured +31% at 100,000 walkers; in my 20,000-walker runs it was within noise on a shared machine.

**6. Day and night: light it avoids, and fruiting in the light**
- **Source:** Nakagaki et al. 1996 (photoavoidance across the spectrum, linked to sporulation); Starostzik & Marwan 1995; Jones & Adamatzky 2012 (light as sensor × (1−L)).
- **Rule:**
  - A global light level L(t) on a 120-240 s day, and a shade map from rock and walls.
  - Sensor readings on lit cells are multiplied by `1 − light_dim × L`.
  - Starving walkers in light fruit (`spore_light`).
- **Why now:** this meets the Skittish dead end's re-test condition ("the push reaches the whole body").
- **Body and play:** by day the mould retreats into shade and spores; by night it spreads. Walls cast shade, so a wall is also a daytime lure and a kill zone.
- **Counter:** raid exposed crystals at noon.
- **First experiment:** the world with rock 0.2, light on, 4 min, 16 seeds.
  - **Deciding measure:** the share of walkers in shade at noon against midnight.
  - Probe: a noon raid against a midnight raid.
- **Cost:** one number and one static map; about nothing.

**7. Nomad: slime it won't recross**
- **Source:** Reid et al. 2012 (avoiding its own explored ground solves the U-trap).
- **Rule:** walkers below `slime_hungry` energy lay slime X with no spreading (half-life 60-120 s), and sensors read −`slime_weight` × X.
- **Body:** a swarm that strips a region and moves on, leaving a dull wake.
- **Counter:** predict its path and wall ahead of it; harvest behind it.
- **First check:** grazing (`graze_weight`) already does this. Test grazing with growth at the world's operating point before building anything.
  - **Deciding measure:** crystals reached by 240 s, and time to escape a U-shaped wall.

**8. Two foods**
- **Source:** Dussutour et al. 2010 (the mould balances two nutrients).
- **Rule:** crystals of kinds A and B, energy per kind per walker; it divides only when both are enough. Two scent fields, each weighted by the walker's shortfall in that kind.
- **Counter:** deny the scarce kind, and it stalls even on plenty.
- **First experiment:** remove the 5 B crystals at 30 s against 5 random A crystals of equal charge.
  - **Deciding measure:** walkers alive at 120 s.
- **Cost:** one more coarse scent field and one float per walker.

### Where the method should change
1. **Operating point.** Judge every new rule in the world race: growth, crystals that run dry, a rectangular world with rock, runs of 2-5 minutes. Don't judge on the 20,000-walker fixed-count disc on 7 piles over 20 s. Keep memory in fields or pass it to children at division, because of the turnover above.
2. **Counter tests need controls.**
   - Test each counter against a random action of equal cost and against another tier's counter: a tier × counter matrix. The ladder holds when the diagonal wins after Holm.
   - The current Sleeper test, bait plus strike against strike alone, partly tests that two actions beat one.
   - Add "rule on, stimulus off" and "stimulus on, rule off" arms.
3. **Agency spread.** For each tier, run a small grid of player choices (where × when) on the same seeds. Report the spread of outcomes across choices against the spread across seeds. If the choices barely matter, the tier is a stat, not a creature. This fits "an outcome is a spread".
4. **Pace and time.** Give every tier a pace number (time to take N crystals, checked against the owner's 10 s rule). Behaviour in time (heartbeat, flinching, habituation) needs GIFs and time series; stills hide it.
5. **Statistics and cost.** Use 16+ seeds with Wilcoxon on the continuous measure and Holm (the tier tests still used 8 seeds and Bonferroni). Quote GPU cost per pass: a spreading field costs about one trail pass, a per-cell update about one world pass.

### Files
- Prototype: /tmp/review-2/scratch/src/big.rs (copy of `big.rs` plus the pulse rule) and /tmp/review-2/scratch/src/main.rs.
- Raw output: /tmp/review-2/runs/pulse/ (prototype arms) and /tmp/review-2/runs/pace/ (Sleeper relay with periodic bait).

### Sources
Biology papers checked on PubMed:
- Matsumoto 2008, [DOI](https://doi.org/10.1529/biophysj.107.113050)
- Lewis 2015, [DOI](https://doi.org/10.1098/rsif.2014.1359)
- Alim 2013, [DOI](https://doi.org/10.1073/pnas.1305049110)
- Kramar & Alim 2021, [DOI](https://doi.org/10.1073/pnas.2007815118)
- Saigusa 2008, [DOI](https://doi.org/10.1103/PhysRevLett.100.018101)
- Boisseau 2016, [DOI](https://doi.org/10.1098/rspb.2016.0446)
- Vogel & Dussutour 2016, [DOI](https://doi.org/10.1098/rspb.2016.2382)
- Reid 2012, [DOI](https://doi.org/10.1073/pnas.1215037109)
- Schrauwen 1981, [DOI](https://doi.org/10.1007/BF00414693)
- Border & Carlile 1974, [DOI](https://doi.org/10.1099/00221287-85-2-211)
- Nakagaki 1996, [DOI](https://doi.org/10.1111/j.1751-1097.1996.tb01847.x)
- Dussutour 2010, [DOI](https://doi.org/10.1073/pnas.0912198107)
- Marée & Hogeweg 2001, [DOI](https://doi.org/10.1073/pnas.061535198)

Taken from the earlier review or the repo's cards, not checked again:
- Tero 2010, [DOI](https://doi.org/10.1126/science.1177894)
- Starostzik & Marwan 1995, [DOI](https://doi.org/10.1111/j.1751-1097.1995.tb09158.x)
- Jones & Adamatzky 2012 (`research/sources/jones-adamatzky-2012.md`)

(The reviewer's note that one paper-search service was out of quota is left out here; it used PubMed and alphaXiv instead.)
