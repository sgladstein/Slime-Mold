# Whole-game review, 2026-11-15: merged findings

## Re-check on main after the review (11-15 10:00-11:50Z)

The reviewers worked on main at 17f2e5b8 (#329). While they ran, #330-#340
merged, and three of them change the enemy the review describes: #336 (the
run plays shape C under the tier, 25,000 walkers times the level's factor;
before, a run played the bare race slime by mistake), #338 (a fed slime
that cannot divide thickens, on in shape C) and #339 (thick slime breaks
away). #330 made squads the window's default control, #335 gave the rifle a
base spread, and the wiki gained pages for squad mode, the economy and guns.
The thread that committed this folder re-filmed the key claims on main at
40f34e48, seed 1, llvmpipe, one seed each (stills in [recheck/](recheck/)):

| Capture | Before (review, 17f2e5b8) | Now (40f34e48) |
|---|---|---|
| Planet 1, idle | 1,445 at 1:00, 1,515 at 5:00, squad untouched | 13,119 at 0:01, 15,233 at 1:00, 22,410 at 4:00; 460 shots, 4 soldiers down to 2 ([recheck/planet-1-1-idle-240s-all.jpg](recheck/planet-1-1-idle-240s-all.jpg)) |
| Planet 3, idle | 1,581 at 1:00, 1,658 at 2:30, 15 of 16 crystals untouched | 43,304 at 0:01, 46,434 at 1:00, 28,554 at 2:00 as a network across the map; squad bitten ([recheck/planet-3-1-idle-120s-all.jpg](recheck/planet-3-1-idle-120s-all.jpg)) |
| Planet 1, `mode=squads hand=squads` | wiped by 0:26-0:30 | 16,241 at 2:00, 7 of 8 soldiers ([recheck/planet-1-1-squads-120s-all.jpg](recheck/planet-1-1-squads-120s-all.jpg)) |
| Planet 1, `mode=squads hand=pumps` | both pumps 200/200 at 1:37, then "stuck" | one pump 200/200, the other 169, 137, 75 hp at 1:00, 2:00, 3:00 ([recheck/planet-1-1-pumps-180s-all.jpg](recheck/planet-1-1-pumps-180s-all.jpg)) |
| Planet 3, `mode=squads hand=pumps` | (not filmed) | two of four pumps worn to 0 hp by 3:00, the others 25 and 87 ([recheck/planet-3-1-pumps-180s-all.jpg](recheck/planet-3-1-pumps-180s-all.jpg)) |
| Planet 1, `mode=squads hand=greedy` | 38 soldiers at 2:00 | 13, 22, **40** soldiers at 0:30, 1:00, 2:00; slime 10,461 at 2:00 ([recheck/planet-1-1-greedy-120s-all.jpg](recheck/planet-1-1-greedy-120s-all.jpg)) |

So on main:

- **No longer true:** the run's slime shedding to its home crystal and
  sitting (W24, W2/D1, W7's evidence, W11, W16), idle being unable to lose
  (W9), and pumps never being touched (W1's evidence: the slime now wears
  pumps down on planet 3 within 3 minutes, though `pump_smell` is still 0
  and still has no rule key). The run's default control is squads (W20,
  D11), the spread card works on the rifle (W39), and the three wiki pages
  exist (W52).
- **Still true:** the 40-soldier snowball (W5: 40 soldiers at 2:00 against
  a live slime), plain shots barely visible and the gun look (W26, W18,
  W19, W27, W10, W55), no thread owning the run's enemy progression (W3,
  W8, W25 as a "does to you" ladder, W14), the stuck rule (W17, code
  unchanged), no play log (W13), crystals never regrowing (W21), the stale
  first-read docs (W22, W23, W34), and every engineering item (W31-W53).
- Planet 1's slime still fills its rock bowl and leaves in a trickle, while
  planet 3's covers the map. Whether a planet's bowl holds the slime (W12)
  is now a per-planet question, not the cause of a passive enemy.

The sections below are the merge as written against 17f2e5b8. Read each
item with the list above.

## How the review was run

The review ran on main at 17f2e5b8 on a cloud machine with software Vulkan
(llvmpipe). First a starter kit ([kit.md](kit.md), [kit.sh](kit.sh)) proved
one headless capture command for each kind of picture: squad mode, missions,
run planets 1, 4 and 8, the shop, buildings, fire, the player's own mould,
condensing, the gun range, the player AI, close zoom and bursts. Five
reviewers then worked independently from that kit, each in one role, and
each wrote a ranked file with stills beside it:
[first-time-player.md](first-time-player.md) (FTP),
[game-director.md](game-director.md) (GD),
[simulation-to-play.md](simulation-to-play.md) (STP),
[engineering-health.md](engineering-health.md) (EH) and
[roadmap-process.md](roadmap-process.md) (RP). All five files are present.
The owner's notes (Scotttt, 11-15 08:30Z) are the only evidence of real
human play, and they outrank every scripted hand. The scripted hands are
controls, not players. This merge did four things:

1. It joined findings that say the same thing, keeping all the evidence and
   naming the reviewers who raised each.
2. It checked the 30 findings with the highest impact one at a time,
   against the source line, the still or a fresh capture. Three fresh
   captures were made: run seed 2 planet 1 idle, the same with breakaway
   on, and run seed 1 planet 1 with shape C's keys. Two stills from them
   are in [merge/](merge/). Each finding is marked confirmed, plausible or
   dropped. Items below the top 30 by impact carry the reviewer's verdict
   and are marked *not re-checked*.
3. It scored each finding as impact / effort (S = 1, M = 2, L = 4), plus 1
   when two or more reviewers raised it. Ties are broken by impact, then by
   the number of reviewers, then by whether the item answers something the
   owner named on 11-15.
4. It wrote this page. Disagreements are listed under owner decisions,
   with both sides given. This page does not resolve them.

Every count is from one seed: these are looks, not measurements. No census
was run.

*Not done by this merge:* the `Reports/README.md` line and the wiki updates
that go with a new report. This task was limited to this folder, so the
thread that commits it adds them.

## The five openings

**First-time player.** It played the game in the order a newcomer finds it,
and found two games that never meet. The Fork sandbox is a horror game: the
squad is dead by 0:15 and the slime eats the bank by 0:30. The run, which
the owner calls the real game, is an economy game against a slime that
lands at about 3,000 walkers, halves on its own and then sits round its
home crystal for five minutes. Seen that way, the owner's plan (drain the
biggest crystal, buy to 40, march in) is not a failure of imagination; it
is the only plan the run asks for. The slime looks rich and alive at close
zoom, and where it is big enough to move it makes the game a fight over the
map. The first fix is to make the run's slime act like the Fork's. Then
come the guns: the starting rifle is the least exciting thing on screen,
and the blast is one of the most exciting.

**Game director.** The run is meant to be claim, hold, strike. In play it
is an economy race with no opponent, then one fight against a fixed-size
sponge that regrows. On every planet filmed, the slime drops to whatever
its home crystals feed within 30 s and then sits in its nest bowl. It never
touches a pump. So each planet asks one question: what damage per second
beats the nest's regrowth? The answer is 40 soldiers with the best gun. The
approach (an enemy whose body is its hit points and whose crystals are its
economy) is distinctive, roughly Creeper World with a squad. What is
missing is the property that makes Creeper World and They Are Billions
work: the enemy keeps coming, and it wants what you hold. Every system
should feed claim, hold or strike, and the rest should be parked.

**Simulation to play.** The model is a good engine for a game. It can grow
networks, reach for food, heal, wither, panic, spore and sleep. The run
uses almost none of that, and the economy around the model gives the slime
no reason to leave home:

- a fed walker has no drive to leave;
- spare walkers starve at home instead of reaching out;
- food smell reaches about 25 cells while crystals are 100 or more apart;
- every food rule makes a drained crystal less attractive.

Two cheap changes moved the picture a lot: shape C's body, which the run
never uses, and a bigger appetite. Gun hits are honest but invisible in
dense bodies because the body's picture saturates. The model can already
supply a flinch, a spatter and a spill.

**Engineering health.** The core seam is right. Rules on the CPU talk to
the slime through one squad frame and a `World` trait, and `squad::Burst`
is a good effect algebra on both engines. Three things scale badly:

- Game state (payload cards, tools, the run, missions) lives in the window
  binary, so tests, census, AI and replays play a different game from the
  owner's.
- Every slime mechanic is written twice, and its parameters are plumbed by
  hand through about six tables.
- Every payload hit syncs the GPU, which is a cliff at the owner's 40
  soldiers and in the way of richer guns.

The workflow costs are fixable in a day: CI runs three test binaries one
after another, shared index files bounce PRs off the queue, and the
first-read docs are stale.

**Roadmap and process.** The repo is not organised around the game the
owner plays. ROADMAP's *Now* band still describes the 11-12 Larder RTS and
says the owner can't playtest. HANDOFF is the front page of the paused dish
game. The output is very high and nearly all on owner asks. The aim is
wrong in one place, the place the owner named: no thread owns the enemy's
progression across a run, and nothing on the roadmap makes the slime press
the player. Meanwhile the planet and ground threads put 12 PRs into map
shape and look overnight. A behaviour already in the code and off by
default (breakaway) changes the sitting slime on the first try. The plan
should be re-cut around that problem, the guns' feel and the run, with the
rest parked. A draft *Now* band is at the end of the RP file.

## Top 10: where to focus

These are ranked by score. **The owner's gun ask is underweighted.** The
highest gun item by score is 10, a payload card in the starting gun. The
gun-feel pass itself (W26) scores 3.5, because three of four reviewers
sized it M. The saturation fix (W18, 4) and the flinch (W19, 4) are the
cheapest gun items. If the owner's named priorities should outrank the
formula, move W26 up.

### 1. W1. Pumps don't draw the slime, so nothing needs defending
- **Score:** 6 (impact 5, effort S, raised by five reviewers). **Verdict:** confirmed as the cause; the fix is untested.
- **Reviewers:** FTP 4, GD 7 (+18), STP 3, RP 4, EH 19.
- **Evidence:**
  - Three pumps and a turret on run planet 1 stood at full hp for 3:20, and the turret never fired (FTP; [first-time-player/run1-pumps-untouched-200s.jpg](first-time-player/run1-pumps-untouched-200s.jpg)).
  - Pumps at hp 200/200 on planet 1 (GD; [game-director/01-run-p1-lost-to-stuck-097s.jpg](game-director/01-run-p1-lost-to-stuck-097s.jpg)).
  - A building loses hp only when walkers touch it (`buildings.rs:624-627`, checked).
  - `BuildRules::pump_smell` exists and defaults to 0.0 (`buildings.rs:197,265`, checked). When set, it lays scent through the units pass at line 572, which both engines run.
  - A reached crystal smells at `fed_scent` 0.15 (`big.rs:1153`), and the latch is set in `engine.wgsl:1450`. Breakaway skips crystals under `ROAM_FRESH` 0.8 (`big.rs:80`). All checked.
  - Owner: "They don't attack my pumping crystals so no need to defend."
- **Caveat found in the merge:** `pump_smell` cannot be set from the command line. No rule key reaches `BuildRules`, so no reviewer filmed it on. The lure lays the same kind of scent, and it drew nothing at 130 units (FTP) or 250 units (GD). STP's numpy model of the scent kernel falls to 1e-8 at 54 cells. Scent alone may not reach far enough, which ties this item to W4, W7 and W45.
- **Smallest next step:**
  1. Add `pump_smell` (and `lure_r`) as rule keys.
  2. Film run planets 1-3 with two pumps at `pump_smell` 0, 0.5 and 1, at 0:30, 1:30 and 3:00.
  3. Run GD's smoke checks: an undefended pair of pumps falls within 3 minutes, a turret beside a pump keeps it standing, and idle still doesn't win.
  4. If the scent doesn't reach, try STP's flow scent: a crystal smells of the charge drawn from it this second, and "reached" holds only while the slime holds the crystal.
- **Owner:** unowned. The pump-economy thread owns the pump; "Improve the slime enemy" owns the numbers.

### 2. W2. The run lands a different slime from the sandbox: no shape C body, and too few walkers to regrow
- **Score:** 6 (5, S, three reviewers). **Verdict:** confirmed.
- **Reviewers:** STP 2, FTP 1 and 2, GD 4.
- **Evidence:**
  - Shape C is applied only when no run, mission or own mould is set (`main.rs:1668`, checked). `run_args` gives the run the tier's mould, "not shape C" (`menu.rs:178-183`, checked).
  - Planet 3 with C's four keys: 7,565 walkers at 1:00 and 9,324 at 2:00, still rising, with two cords out. Without them: 1,658 and no cords (STP; [simulation-to-play/planet3-appetite.jpg](simulation-to-play/planet3-appetite.jpg)).
  - **Merge capture:** run seed 1, planet 1, idle, with C's keys gave 4,321 at 1:00 and 6,793 at 2:00, against about 1,450-1,500 without them (FTP, RP). But at 2:00 it still fills its rock ring and has sent no cord ([merge/run1-p1-shapeC-keys-120s.jpg](merge/run1-p1-shapeC-keys-120s.jpg)).
  - Walker count, sandbox rules with no shape C: `walkers=` sets `chosen`, which skips shape C (checked, `main.rs:1496-1498`). FTP's sweep on planet 1 found that 3,000 walkers sits (1,734 at 2:00), while 8,000 regrows to 3,355 and runs a cord to the squad ([first-time-player/planet1-walkers-3k-8k-25k.jpg](first-time-player/planet1-walkers-3k-8k-25k.jpg)).
  - GD's free play on planet 1 (C at about 25k) made a sheet of 26,541 at 1:00 ([game-director/02-freeplay-p1-sheet-360s.jpg](game-director/02-freeplay-p1-sheet-360s.jpg)).
- **Smallest next step:** an owner card (decision D1). Meanwhile, film planets 1-4 in squad mode at 1:00, 2:00 and 5:00 under C's keys, under 8,000 walkers, and under both.
- **Owner:** the run's levels (unowned since "Build a roguelike run mode" closed), with "Improve the slime enemy".

### 3. W3. No thread owns the run's enemy progression
- **Score:** 6 (5, S, two reviewers). **Verdict:** confirmed.
- **Reviewers:** RP 1, GD 1 and its opening (FTP's owner lines point at "the run thread", which is done).
- **Evidence:**
  - `wiki/world/run.md:99` hands the slime's size off: "the slime's economy, not the run's" (checked).
  - The run report says it was "passed on and not tuned here".
  - Per RP's read of the thread list at 09:35Z: "Build a roguelike run mode" is done (09:16Z), and "Improve the slime enemy" owns the tuning numbers, not the ladder.
- **Smallest next step:** the coordinator names one thread, *Run and enemy progression*, and gives it `run::LADDER`, `WALKERS_PER_DEPTH`, what each depth switches on, W1, W2, W4, W6 and W8, plus a ROADMAP row (RP's draft N1).
- **Owner:** the coordinator.

### 4. W4. Breakaway (condense + roam) is built and off, and it gives the sitting slime reach
- **Score:** 6 (5, S, two reviewers). **Verdict:** plausible. The direction holds on two seeds, but the size of the effect varies.
- **Reviewers:** RP 3, STP 13.
- **Evidence:**
  - RP, run seed 1 planet 1, idle with `condense=300 roam=300`: 3,334 walkers at 3:00 against 1,516. It took the big crystal, the squad was bitten, 420 shots and 916 kills ([roadmap-process/run1-p1-idle-roam-180s.jpg](roadmap-process/run1-p1-idle-roam-180s.jpg) against [roadmap-process/run1-p1-idle-300s.jpg](roadmap-process/run1-p1-idle-300s.jpg)).
  - **Merge capture**, run seed 2 planet 1 at 2:00: 1,308 with roam against 1,247 without. Pieces crossed the lake into the crystal field, and the idle squad fired 53 shots against 0, but was not touched ([merge/run2-p1-idle-vs-roam-120s.jpg](merge/run2-p1-idle-vs-roam-120s.jpg)).
  - STP, planet 3 with a pump: a file of specks reached the pumped crystal by 0:40, and nothing settled by 2:00 ([simulation-to-play/home-blob-and-breakaway.jpg](simulation-to-play/home-blob-and-breakaway.jpg)).
  - Roam steers straight at a chosen crystal (`panic_to`) and skips crystals under 80% charge (`ROAM_FRESH`, checked), so a pumped crystal drops out as a target.
- **Smallest next step:** depends on decision D2. Either turn it on from planet 2 or 3 in `Run::level` and put a 20 s clip in the next playtest pack (RP), or first let roaming pieces follow a far smell and drop `ROAM_FRESH` (STP).
- **Owner:** the run/enemy thread (W3); the condense thread for the rule.

### 5. W5. The 40-soldier snowball survives the pump change
- **Score:** 6 (5, S, two reviewers). **Verdict:** confirmed.
- **Reviewers:** FTP 3, RP 7.
- **Evidence:**
  - Run planet 1, squad mode, `hand=greedy`: 12 soldiers at 0:30, 19 at 1:00, 38 at 2:00, and the slime untouched at 1,479 ([first-time-player/run1-squads-greedy-38-soldiers-120s.jpg](first-time-player/run1-squads-greedy-38-soldiers-120s.jpg), checked).
  - The pump report's own smoke test (Fork, one seed): 40 soldiers at 1:30 before the change and 22 after (`Reports/pump-economy-2026-11-15/README.md` l.83-88, checked).
  - Hand income is linear in soldiers, so the army compounds.
  - Upkeep is a dead end (`Reports/dead-ends.md` l.9; owner: "no maintenance costs"). That entry names the allowed brakes: "prices, the mould eating the bank, or things worth buying".
- **Smallest next step:** depends on decision D7. FTP: one look each at a per-crystal drain cap past four soldiers, and a soldier price rising 10% a soldier, reading soldiers at 2:00. RP: time to 40 soldiers with `hand=greedy` and `hand=pumps`, roam off and on, beside the owner's next play.
- **Owner:** the run/enemy thread, with the economy (pump) thread.

### 6. W6. Each planet is a damage threshold, and nobody has drawn the player's kill rate against the slime's regrowth by depth
- **Score:** 6 (5, S, two reviewers). **Verdict:** plausible. The regrowth is filmed; the "switch, not spread" is an inference.
- **Reviewers:** GD 2, RP 5.
- **Evidence:**
  - Run planet 4, `hand=rush` (RTS): 4 soldiers killed 12,262 walkers by 2:30, while the slime went from 2,725 to 3,031 and its crystals fell only from 0.990 to 0.976. The squad stood at the nest's edge with 0 walkers touching it ([game-director/04-run-p4-four-at-the-nest-090s.jpg](game-director/04-run-p4-four-at-the-nest-090s.jpg), checked).
  - Card rulings stack range and rate to 2-4 times (decisions-game 11-14 15:06), while the enemy gets +20% walkers a planet, which it then sheds.
- **Smallest next step:** one page with two lines by depth: the kills a second of a typical shop's squad (from `range=` loadouts), and the slime's mass and regrowth at 1:00 on each planet (one seed each, no census).
- **Owner:** the run/enemy thread, with "Make the guns look satisfying".

### 7. W7. Surplus walkers starve at home instead of exploring
- **Score:** 5 (5, S, one reviewer). **Verdict:** plausible. The rule is a proposal, and the probe has not been run.
- **Reviewers:** STP 1 (and 11.1).
- **Evidence:**
  - Planet 3 went from 7,016 walkers to 1,581 at 1:00 and 1,658 at 2:30, with 15 of 16 crystals untouched and 0.978 of the charge left at 2:30.
  - EH 17 saw the same shed: 7,016 to 1,973 by 0:30 with 31 kills ([engineering-health/run3-squads-004s-all.jpg](engineering-health/run3-squads-004s-all.jpg), [engineering-health/run3-squads-030s-all.jpg](engineering-health/run3-squads-030s-all.jpg)).
  - `Reports/dead-ends.md` l.61 and l.73 record that lone explorers matter to spread. Line 61: `lone_upkeep` at 20 left the mould on its home crystal.
- **Smallest next step:** key `explores()` on energy rather than age (under a hunger threshold a walker becomes an explorer). Film planet 3 idle from 0 to 150 s, counting crystals reached and walkers at 2:30 against 1 and 1,658.
- **Owner:** "Improve the slime enemy".

### 8. W8. The enemy should answer the player's build and gun, not only the depth
- **Score:** 5 (4, S, three reviewers). **Verdict:** plausible (a design direction).
- **Reviewers:** GD 9, EH 18, STP 11.5.
- **Evidence:**
  - Per depth, the run sets only a `LADDER` tier and a walker count (`run.rs:486-509`, checked). Upgrades multiply hp, strike, rate, range and squad size (`run.rs:512-530`).
  - The Grudge's scar and the Swarmer's alarm already exist.
  - Owner: "especially when your guns get stronger".
- **Smallest next step:**
  - GD: pick the next tier from the last planet's log (largest squad, shots per spot, buildings). One function in `run.rs`, with one sentence on the shop's planet card.
  - EH: a pressure table beside `LADDER` keyed on depth and the carried gun's grade.
  - STP: payloads that killed most raise the next slime's alarm threshold to them (paper design first).
- **Owner:** the run/enemy thread (W3).

### 9. W9. Nothing presses the player: idle can't lose, and the only danger is walking in
- **Score:** 5 (4, S, two reviewers). **Verdict:** confirmed.
- **Reviewers:** FTP 9, GD 23.
- **Evidence:**
  - Planet 1 idle at 5:00: squad at 100% health, 0 shots, slime 1,515 at home (FTP; RP's [roadmap-process/run1-p1-idle-300s.jpg](roadmap-process/run1-p1-idle-300s.jpg)).
  - **Merge capture**, run seed 2: 0 shots and the squad untouched to 2:00 (1,227 then 1,247).
  - GD: standing at a nest's edge was safe.
  - Owner: "So there is no rush."
- **Smallest next step:**
  - Add "idle loses, or is clearly losing, by 5:00" to the run's smoke checks (one capture a planet).
  - Judge W1, W4 and W43 by GD's measure, named before their films: the seconds per minute the enemy spends pressing on something of the player's.
- **Owner:** the run/enemy thread.

### 10. W10. Start the run with a payload card; the satisfying gun is behind shop luck
- **Score:** 5 (4, S, two reviewers). **Verdict:** plausible. One shop roll was seen.
- **Reviewers:** FTP 6 and 16, GD 6 and 11.
- **Evidence:**
  - Rifle against blast at close zoom ([first-time-player/rifle-vs-blast-close.jpg](first-time-player/rifle-vs-blast-close.jpg)).
  - The shock payload on the Fork: 64 casts and 1,840 killed by 8 s ([game-director/06-fork-shock-payload-8s.jpg](game-director/06-fork-shock-payload-8s.jpg)).
  - The planet-1 shop offered Needle gas, Buckshot bullet, Delay, Thick and Pierce, with no blast or bomb (kit [kit/run-shop.jpg](kit/run-shop.jpg)).
  - The run starts with the plain rifle.
- **Smallest next step:** decision D5. Then one rolled payload card in the starting gun, filmed on planet 1 at zoom 12.
- **Owner:** the run/enemy thread, with "Make the guns look satisfying".

## Issues (the rest, by score)

### W11. Squad-mode run planet 1 is over in about 30 s
- **Score:** 5 (4, S, two reviewers). **Verdict:** confirmed by two independent captures.
- **Reviewers:** FTP 8, STP 1 and 9.
- **Evidence:** FTP: contact at 0:16 (3,077 walkers), 840 at 0:22, 4 at 0:26, "mould wiped". STP: wiped by 0:30 by 8 soldiers with 183 shots, none lost.
- **Smallest next step:** fix it through W2 or W4, then check that planet 1 lasts at least 3 minutes for `hand=squads`.
- **Owner:** the run/enemy thread.

### W12. Map work is ahead of enemy work, and the nest bowls may be part of why the slime stays home
- **Score:** 5 (4, S, two reviewers). **Verdict:** plausible.
- **Reviewers:** GD 12, RP 14.
- **Evidence:**
  - 9 levelgen PRs and 3 ground-look PRs overnight (RP).
  - The moat is "built but off".
  - The merge's shape C capture shows the body filling its rock ring at 2:00 without leaving ([merge/run1-p1-shapeC-keys-120s.jpg](merge/run1-p1-shapeC-keys-120s.jpg)). That is consistent with the bowl idea but does not prove it.
- **Smallest next step:** one capture pair on planet 1 seed 1 at 3:00, idle, with the bowl as generated and with its rim opened. Hold twists, rescue planets and per-biome landing crystals until the slime moves.
- **Owner:** "Generate roguelike levels procedurally"; the hold is the coordinator's call.

### W13. The owner's play leaves no record the threads can read
- **Score:** 5 (4, S, two reviewers). **Verdict:** confirmed.
- **Reviewers:** GD 20, EH 14.
- **Evidence:**
  - The window saves `larder-world-replay-MAP-SEED.txt` (`main.rs:32-33`, checked).
  - Tools are not in the replay (`main.rs:616`, checked), and replays are not portable across machines (HANDOFF decision 6).
  - The owner's 08:30Z paragraph is the project's only human-play evidence.
- **Smallest next step:** write a gitignored `larder-world-log-{map}-{seed}.txt` (the headless t line once a game second, the commands given, and frame p50 and p95), and tell the owner where it is. Ask for two run replays, and write a reader that prints time to N soldiers, splits, pumps, shots per spot and buildings.
- **Owner:** unowned (GD suggests the player-AI thread, which is blocked).

### W14. The tiers were tuned on other maps, and on run planets they barely show
- **Score:** 5 (4, S, two reviewers). **Verdict:** confirmed.
- **Reviewers:** GD 13, STP 9.
- **Evidence:**
  - `tiers.rs:115-136` records tuning on First contact seeds and the race world's 23 crystals (checked).
  - On planets, the Drifter reached no crystal and the Trunk reached one at 1,658 walkers.
  - Planet 3's Trunk has no cords, planet 1's Drifter does not drift, and only planet 5's Sporer looks like itself (GD; [game-director/03-run-p4-static-nests-150s.jpg](game-director/03-run-p4-static-nests-150s.jpg)).
- **Smallest next step:** one look-only pass of each ladder tier on its own planet (1-6), idle, 0-120 s, with walkers and crystals reached and one still each. About 10 minutes of runner time.
- **Owner:** the run/enemy thread and "Improve the slime enemy".

### W15. Strategy reports measured a different slime, and a different engine, from the one the owner plays
- **Score:** 5 (4, S, two reviewers). **Verdict:** confirmed.
- **Reviewers:** GD 4, EH 7.
- **Evidence:**
  - The census and player AI ran on the Fork or race world with shape C at about 25k walkers. `take_claim` and the tier counters were measured there.
  - The twin tests run at scale 1-2, with 1,000-5,000 walkers, for a few ticks, and are exact only on llvmpipe. The 1070 gave 824 against 933 walkers at 30 s (`Reports/pc-batch-1070-2026-11-14.md`).
- **Smallest next step:** label each strategy report with the slime and engine it measured, and make run planets 1-4 the reference for future strategy questions. Add one statistical twin (8 paired seeds) to the next 1070 batch.
- **Owner:** the census / player-AI thread; "Run jobs on Scott's desktop PC" for the twin.

### W16. A confident report may be wrong: "the slime sheds most of its walkers whatever it starts with"
- **Score:** 5 (4, S, two reviewers). **Verdict:** plausible.
- **Reviewers:** FTP 2, RP 20 (STP 17's death count folds in here).
- **Evidence:**
  - `Reports/roguelike-run-2026-11-14/README.md` l.181-182 says it (checked), and `wiki/world/run.md:99` repeats it.
  - FTP's counter-case (8,000 and 15,000 walkers shed and then regrow) ran without a tier and on one seed.
  - RP: the core world's about 7,300 is also the number at which the GPU once dropped its crowd limit. RP calls the link a guess and probably unrelated.
- **Smallest next step:**
  - FTP's table: 3k, 8k, 15k and 25k walkers at 1:00, 2:00 and 5:00, run rules against sandbox rules (about 10 min).
  - One core-world capture with `capacity` doubled.
  - Print deaths by cause (shot, starve, wither, wade) in the t line (STP 17).
- **Owner:** the run/enemy thread; the capacity check goes with #339.

### W17. The "stuck" rule ends a run while the squad is healthy
- **Score:** 4 (4, S). **Verdict:** confirmed.
- **Reviewers:** GD 5.
- **Evidence:**
  - "Mission lost: No food to buy or fire with since 1:07 ... The run ends on planet 1", with 4 soldiers at full health ([game-director/01-run-p1-lost-to-stuck-097s.jpg](game-director/01-run-p1-lost-to-stuck-097s.jpg), checked).
  - `count_stuck` counts a second as stuck when there are no new shots, no harvest and the bank is under a soldier's price, whether or not the squad is walking or pumps are standing (`turn.rs:397-406`). The end is `STUCK_END` 30 s (`campaign.rs:266,612`). Both checked.
  - Firing is free in squad mode, so "fire with" is false there.
- **Smallest next step:** in the run, count a moving squad or a standing pump as not stuck, and warn instead of ending.
- **Owner:** the run/enemy thread (the run thread is done).

### W18. A hit on a dense body can't be seen, because the body's picture saturates
- **Score:** 4 (4, S). **Verdict:** confirmed.
- **Reviewers:** STP 6 (GD 6 saw "a soft dent").
- **Evidence:**
  - `body()` is the look times 1 - exp(-n/n0), with n0 scaled by the world's average density only (`view.wgsl:701-708`, checked).
  - Planet 1 at zoom 14: kills went from 167 to 483 and the edge barely moved ([simulation-to-play/hits-invisible.jpg](simulation-to-play/hits-invisible.jpg)).
  - Owner, 11-12 04:13Z: "visually it didn't impact much".
- **Smallest next step:** normalise n0 by local density (a blurred count mip, or the body's mean per crystal). Show it as a blind A/B at zoom 6 and 14 on planet 1, and quote the frame cost (STP guesses 0.1 ms).
- **Owner:** "Make the guns look satisfying", with the slime look.

### W19. A hit should make the body flinch, and the model already can
- **Score:** 4 (4, S). **Verdict:** plausible.
- **Reviewers:** STP 7.
- **Evidence:**
  - Planet 1 at 19 s: 661 kills with `alarm_keep` 0.98 and `alarm_weight` -1, and the body pulls back, against 483 kills by default ([simulation-to-play/hit-flinch.jpg](simulation-to-play/hit-flinch.jpg)).
  - `Reports/dead-ends.md` l.40, 51 and 54 record that flee and wary failed the census gate. Their re-test condition is "the mould defends its core and crystals whatever it avoids". Under build first, balance later, that is a tuning note.
- **Smallest next step:** alarm keep about 0.9, so it is a flinch and not a retreat, on every tier. Draw the alarm at a tenth of its strength (W46). Show it as an A/B.
- **Owner:** "Make the guns look satisfying", with "Improve the slime enemy".

### W20. The menu starts the wrong game: Sandbox is highlighted first, and the Run defaults to RTS control
- **Score:** 4 (3, S, two reviewers). **Verdict:** confirmed.
- **Reviewers:** RP 8, FTP 10.
- **Evidence:** the Control option defaults to `"rts"` (`menu.rs:75`, checked), and only the UI size is saved. The menu highlights Sandbox first. The Fork rush leaves 0 soldiers at 0:15.
- **Smallest next step:** make squads the Run's default control, or save the Control option. Put Run first.
- **Owner:** "Review menus, shop and HUD layout".

### W21. Crystals never regrow, so a pumped map is settled by minute 3
- **Score:** 4 (3, S, two reviewers). **Verdict:** confirmed (code and stills); the fix is untested.
- **Reviewers:** FTP 14, STP 14.
- **Evidence:**
  - `crystal_regrow` is 0.0 (`big.rs:1284`, checked).
  - Three pumps on planet 1: crystals left 0.36 at 3:20 at +0/s ([first-time-player/run1-pumps-untouched-200s.jpg](first-time-player/run1-pumps-untouched-200s.jpg)).
  - `Reports/dead-ends.md` l.33: regrowth in the field skirmish fed the bigger mould. Its re-test condition ("income from something the leader cannot also take") may fit pumps.
- **Smallest next step:** `crystal_regrow` at about a tenth of a pump's draw, or regrowth only on crystals the slime holds (FTP). Film the planet-3 pump for 3 minutes.
- **Owner:** the pump-economy thread.

### W22. The first-read docs point at the old game
- **Score:** 4 (3, S, two reviewers). **Verdict:** confirmed.
- **Reviewers:** EH 11, RP 10 and 11.
- **Evidence:**
  - HANDOFF l.26: "next W1b", with no line for the run, squad mode, guns or pumps (checked).
  - ROADMAP l.41: "While the owner can't playtest" (checked).
  - `wiki/world/larder.md` is 43 KB against a 12 KB cap (checked).
  - ROADMAP is 76 KB.
- **Smallest next step:** see the roadmap changes below.
- **Owner:** the coordinator.

### W23. Reviews set more of the backlog than the owner, and nothing is marked parked
- **Score:** 4 (3, S, two reviewers). **Verdict:** plausible.
- **Reviewers:** RP 16 and 17, GD's systems table.
- **Evidence:** RP lists the night's PRs that came from review *Next* lists (#317, #328, #323). The owner, 11-14 01:08Z, on the player AI: "not too concerned about the slime winning your automated test".
- **Smallest next step:** each review's top 3 names the owner priority it serves. Mark *parked (11-15, why)*: the player AI and strategy censuses, the mission tuning pass, the own mould's GPU follow-ups, L-T2's core values, and the RTS demos.
- **Owner:** the coordinator.

### W24. The umbrella: the run's slime sheds to its home crystal's ceiling in about 30 s and then sits
- **Score:** 3.5 (5, M, five reviewers). **Verdict:** confirmed.
- **Reviewers:** FTP 1, GD 1, STP 1, RP 2, EH 17.
- **Evidence:**
  - Planet 1: 1,445 at 1:00, 1,515 at 5:00 (FTP, RP).
  - Planet 2: about 2,200-2,300.
  - Planet 3: 1,581-1,658.
  - Planet 4: 1,543-3,390.
  - Planet 5: 4,435-5,233.
  - The core world: about 9,000 (GD's kit [kit/run-d8.jpg](kit/run-d8.jpg)).
  - Merge capture on seed 2: 1,227 and 1,247.
- **Smallest next step:** this is the symptom. The levers are W1, W2, W4, W7, W45 and STP's appetite ladder (W25). It is scored M because none of them alone is proven.
- **Owner:** the run/enemy thread (W3).

### W25. Enemy progression is a label: the ladder changes the shape, not the pressure
- **Score:** 3.5 (5, M, four reviewers). **Verdict:** confirmed.
- **Reviewers:** FTP 5, GD 3, STP 5, RP 2.
- **Evidence:**
  - `LADDER` sets the shape (`run.rs:75`). Walkers rise 20% a planet (`run.rs:102`) and are shed.
  - Planet 3 started 43% bigger than planet 2 and was smaller at 1:00.
  - STP's appetite dial on planet 3 at 2:00: `crystal_eat` 1x gives 1,658 walkers, 3x gives 1,823, and 10x gives 4,790 with three crystals held ([simulation-to-play/planet3-appetite.jpg](simulation-to-play/planet3-appetite.jpg)).
- **Smallest next step:** an owner card on the axes (decision D3), then one switch per depth.
- **Owner:** the owner, then the run/enemy thread.

### W26. Plain shots are close to invisible; the spectacle is all in payload cards
- **Score:** 3.5 (5, M, four reviewers). **Verdict:** confirmed.
- **Reviewers:** FTP 6, GD 6 and 11, STP 8, EH 3.
- **Evidence:**
  - Zoom 20 on four rifles killing about 80 walkers a second: no bullet, flash or hit is visible across three frames ([game-director/05-run-p4-plain-rifle-zoom20-060s.jpg](game-director/05-run-p4-plain-rifle-zoom20-060s.jpg)).
  - [first-time-player/rifle-vs-blast-close.jpg](first-time-player/rifle-vs-blast-close.jpg) and [engineering-health/run3-squads-plain-27.1s-near.jpg](engineering-health/run3-squads-plain-27.1s-near.jpg).
  - A blast flings 0 walkers; only the bomb flings ([simulation-to-play/blast-wash.jpg](simulation-to-play/blast-wash.jpg)).
  - The rulings bound the fix: a hit lights only its damage, and nothing is drawn-only (decisions-game 11-14 20:03-20:22Z).
- **Smallest next step:** every plain hit is a small `Burst` that flings k walkers (k from the damage, `fling_speed` is already on the GPU) and opens a one-cell crater. The bullet is drawn in flight, and the soldier recoils. Film a burst in the gun range at zoom 12, 0.05 s apart, k from 2 to 8, as a blind A/B for the owner. STP sizes this S; EH, GD and FTP size it M, and EH would batch bursts first (W40).
- **Owner:** "Make the guns look satisfying".

### W27. A blast's glow is clipped to a square
- **Score:** 3 (3, S). **Verdict:** confirmed (the still shows straight vertical and horizontal edges).
- **Reviewers:** FTP 7.
- **Evidence:** [first-time-player/blast-square-edge.jpg](first-time-player/blast-square-edge.jpg).
- **Smallest next step:** find the light's bounding box in `view.wgsl`, then pad it to the falloff radius or fade it to zero at the box.
- **Owner:** "Make the guns look satisfying".

### W28. Splitting the squad is the owner's best moment, and linked nests would make it pay
- **Score:** 3 (4, M, two reviewers). **Verdict:** plausible.
- **Reviewers:** FTP 15, GD 10.
- **Evidence:** the owner splits the squad to hit a base from several sides. Shape C already shares food (`energy_flow`). Planets have 2-3 nests from planet 4.
- **Smallest next step:** on planet 4, two captures: one 8-soldier squad against two 4-soldier squads from opposite sides, looking at the body at 0:10 and 0:20. Then link both nests' food and count the time to 2%.
- **Owner:** unowned.

### W29. The tier that remembers the player (Grudge) is not in the run
- **Score:** 3 (4, M, two reviewers). **Verdict:** confirmed (the `run.rs:70-75` comment).
- **Reviewers:** STP 12, RP 6.
- **Evidence:** `Tier::settings` refuses the Grudge on the GPU. The thread "Port mould species and tiers to the GPU" has been blocked since 11-14 00:47Z. Binding 47's second block is reserved for the scar.
- **Smallest next step:** port the scar into the reserved block, twin-test it on llvmpipe, and add the Grudge at planet 6 or later.
- **Owner:** unowned (the GPU port is blocked).

### W30. The 2% goal measures walker slots from the peak
- **Score:** 3 (2, S, two reviewers). **Verdict:** confirmed (`campaign.rs:459-465`).
- **Reviewers:** FTP 12, EH 16.
- **Evidence:**
  - The slime halves on its own while the bar reads "(2%)".
  - With condense on, a condensed walker counts once but has two walkers' mass.
- **Smallest next step:** show "walkers now → walkers at the goal" beside the bar, and use `mass()` in `Goal::Shrink`.
- **Owner:** "Review menus, shop and HUD layout" for the bar; the run/enemy thread for the goal.

### Lower-scored issues

These carry the reviewers' verdicts, not re-checked unless marked.

| Item | Score | Verdict | Reviewers | Evidence and smallest next step | Owner |
|---|---|---|---|---|---|
| W31. CI runs three test binaries one after another (about 9 of 11-14 min) | 3 | confirmed (`ci.yml:94`, a single `cargo test`) | EH 5 | Lib 229.6 s, `larder-world` 196.6 s, `mouldgpu` 126.6 s. Split into a matrix, or use nextest. | unowned |
| W32. Queue bounces come from shared index files | 3 | plausible (bounce count not re-checked) | EH 6 | `.gitattributes` with `merge=union` for `Reports/README.md` and `Reports/pages.md`. | unowned |
| W33. Two tracked replay files can stop `play.cmd` from pulling | 3 | confirmed that the files are tracked; the risk is plausible (it needs that map and seed) | EH 9 | `git ls-files` lists two `larder-world-replay-*.txt` files. `git rm --cached` them, and add a CI check. | unowned |
| W34. Owner rulings not recorded, and HANDOFF's open-decision list is stale | 3 | plausible | RP 12, 13 | The 11-15 08:30Z priorities, "15/20 minutes to 1 hour+", and the 6x growth have no rows. Add the rows, and move paused games' questions aside. | the coordinator |
| W35. No window test plays squad mode in a run | 3 | confirmed (`run_base` sets `squads: false`, `tests.rs:806`) | EH 8 | Copy `the_window_plays_a_runs_planet` with `squads: true` at depths 3 and 6. | squad-mode thread |
| W36. Rivers disappear under reeds within two minutes | 3 | confirmed (the still) | FTP 13 | [first-time-player/river-reeds-2s-vs-120s.jpg](first-time-player/river-reeds-2s-vs-120s.jpg). Keep a blue base under reeds on deep water, or no reeds on deep cells. | plants thread |
| W37. Mission 1 teaches "farm and ignore the slime" | 3 | plausible | FTP 11 | Three stars at 1:53 with the slime never near. Depends on D4. | campaign thread |
| W38. Outposts, lures and sirens are dead buys in the run shop | 3 | confirmed (`run.rs:237-242`) | GD 16, 18 | Squad mode has no larder; the lure drew nothing at 130 or 250 units. Drop the outpost, and hold the lure and siren until the slime moves (D10). | the run/enemy thread |
| W39. Spread on bullet cards does nothing on the rifle and long gun | 3 | not re-checked (from the bug check, l.35) | EH opening | "A card that does nothing is the opposite of a satisfying gun". It waits on an owner question in the bug check. | "Fix leftover bug-check items" |
| W40. Each payload hit flushes the GPU and blocks on 1-3 read-backs | 2 | confirmed in code (`larder_world.rs:620` `self.flush()`); the frame cost is unmeasured | EH 2 | About 36 syncs a second at 12 bombs a second. `bench=60 hand=farm bank=60000 rounds=bomb` on the 1070, then batch bursts into the frame's submit (ROADMAP L-K). | performance thread / guns |
| W41. Light, shot and unit caps fail silently | 2 | confirmed (`MAX_LIGHTS` 64, `SHOTS_MAX` 256, `UNITS_MAX` 96) | EH 3, 20 | Muzzles are pushed last, so they are lost first at 40 soldiers. Print the drops in the t line, push muzzles earlier, and raise the caps after W40. | guns |
| W42. Game state lives in the window binary | 2.5 | confirmed (`main.rs:612-620` `kit`, `rounds`) | EH 1 | A lib `Session<W>` owning the game, rounds, kit, tracker and level. First proof: a CPU test where a bomb card kills. | unowned (integration) |
| W44. The model's knobs for these probes can't be set from the command line | 2 | confirmed (no `scent_keep`, `fed_scent` or `pump_smell` key; no `BuildRules` keys at all) | STP 18 | Add the keys. This blocks W1's probe. | unowned |
| W46. The alarm field washes the screen mint | 2 | confirmed (the still) | STP 19 | [simulation-to-play/hit-flinch.jpg](simulation-to-play/hit-flinch.jpg), right. Draw it only on the slime, or at a tenth. | slime look |
| W47. `Args` has no `Default`, so every flag edits 17 literals | 2 | confirmed (17 `range: None` literals) | EH 10 | `#[derive(Default)]` and `..Args::default()`. | unowned |
| W48. Gun upgrades applied on two paths with copied constants | 2 | not re-checked | EH 13 | `run.rs:517-518` and `:552-556`. One `Upgrades::gun`. | guns |
| W49. Squads draw as a stack of overlapping rings at close zoom | 2 | confirmed (the greedy still: 38 in a clump) | FTP 17 | Spread the formation by a soldier radius, and draw one ring a squad. | squad-mode thread |
| W50. The shop's planet card promises a size the slime sheds within 30 s | 2 | plausible (true at landing, misleading in play) | GD 14 | Say what the tier does instead. | the run/enemy thread |
| W51. The roadmap rule fails because new work gets no row | 2 | plausible | RP 18 | A lint warning when a report adds no ROADMAP row id. | unowned |
| W52. No wiki page for guns and cards, squad mode, or the economy | 2 | plausible | RP 19 | Three short *Now* pages. | the guns, squad and pump threads |
| W53. Paused code is a quarter of the source | 1 | not re-checked | EH 12 | Move `End` and `Rng` to `mould::core`, and put the dishes behind a feature. | unowned |

## Missed directions

None of these is built. Scores follow the same formula.

| Item | Score | Verdict | Reviewers | First experiment | Owner |
|---|---|---|---|---|---|
| W43. The slime grows on a clock and sends fronts you can see coming (Risk of Rain, They Are Billions) | 2.5 | plausible | GD 8 | The nest's birth cap rises a share each minute, and every 2-3 minutes a front goes toward the nearest pump or squad. Film it on planet 2. Smoke checks: idle loses by 10:00; one wall and turret line holds a front; the 40-rush still works but is not free. | unowned |
| W45. A far-smell channel that only hungry walkers read | 2 | plausible (STP's numpy model of the kernel, not a capture) | STP 4 | A quarter-grid scent with keep 0.999, weighted by hunger. Frame cost guessed under 0.05 ms. | "Improve the slime enemy" |
| W54. A kill spills food the body comes back to eat | 2 | plausible (`life` sets energy -1, `engine.wgsl:917-1000`, not re-checked) | STP 10 | Spill 0.5 x energy into grass at the cell of death, then count walkers within 10 cells of a holding squad over 20 s. | unowned |
| W55. Gun sound per gun kind and per payload | 2 | confirmed (no per-kind voices in `sound/world.rs`) | RP 15 | A scattergun-on-blast clip with a per-kind report and a per-payload thump. | "Make the guns look satisfying" |
| W56. Char as barren ground, a firebreak the slime avoids | 3 | not re-checked | STP 15 | Film a scorch line between home and a pump on planet 3, then count crossings. | plants thread |
| W57. Two rival slimes on a deeper planet, to bait one into the other | 1.5 | not re-checked | STP 16 | `map=planet:4` with `species` 2 and `species_convert`, one minute. | unowned |
| W58. A "heart" nest per planet; the core world as a boss | 1.5 | plausible | GD 22 | Design one special nest per planet; all of them on the core world. | unowned |
| W59. Biology-led rules: risk for reward, flux-reinforced tubes, spent-ground memory, rhythm | 2 | plausible (inspiration) | STP 11.2-11.6 | STP's ranked list, each with a probe. | "Improve the slime enemy" |

W7 (hunger to explorers), W8 (the enemy answers the build), W13 (the play
log) and W28 (linked nests) are also missed directions. They are ranked
above.

## Roadmap changes

The reviewers agree on all of these. None needs an owner ruling except
where a decision is named.

1. **Name the run/enemy-progression thread** (W3), and give it W1, W2, W4,
   W5, W6, W8, W11, W14, W16 and W17.
2. **Re-cut ROADMAP's *Now* band around the run in squad mode.** RP's
   draft rows: N1 enemy progression, N2 the slime's numbers, N3 a reason
   to defend, N4 guns that are a draw (look and sound), N5 menus, N6
   planets held until N1 moves, N7 bugs, N8 the 1070 batch, N9 a playtest
   pack. Move the history cells to their reports (W22).
3. **Rewrite HANDOFF's *Where the game stands*** as eight lines: run, squad
   mode, guns, economy, slime enemy, planets, looks, campaign. Put the dish
   steps under *Paused* (W22). Split `wiki/world/larder.md`.
4. **Hold new planet look and feature work** until the slime moves, after
   one bowl check (W12).
5. **Mark these parked** with a line saying why: the player AI and
   strategy censuses, the eight-mission tuning pass, the own mould's GPU
   follow-ups, and L-T2's core values (W23). GD's systems table, which
   says keep, merge or park for each system, is the starting list.
6. **Record the 11-15 rulings** in `wiki/decisions-game.md`, and move
   paused games' questions off HANDOFF's open list (W34).
7. **Label strategy reports** with the slime and engine they measured, and
   make run planets 1-4 the reference (W15).
8. **Engineering**, from EH's list: the `Session<W>` refactor (W42),
   batched bursts (W40), CI in parallel (W31), union merges (W32), and
   untracking the replay files (W33).
9. **A playtest pack** for the owner's next session (RP's N9): what changed
   since 08:30Z, each change with its key and a short clip.

## Low-hanging fruit (impact 3 or more, effort S)

- W1 Pump scent on, after adding the key (6)
- W2 Shape C or more walkers for the run (6; D1)
- W3 Name the enemy-progression thread (6)
- W4 Breakaway on in the run (6; D2)
- W5 One look at two snowball brakes (6; D7)
- W6 The kill-rate and regrowth page by depth (6)
- W7 Hunger makes explorers (5)
- W8 Pick the next tier from the last planet's log (5)
- W9 "Idle loses by 5:00" as a run smoke check (5)
- W10 A payload card in the starting gun (5; D5)
- W11 Planet 1 lasts 3 minutes (5)
- W12 The bowl check, and hold planet polish (5)
- W13 A play log beside the replay (5)
- W14 One still per tier on its planet (5)
- W15 Label strategy reports (5)
- W16 The walker table for the run report's claim (5)
- W17 A walking squad or standing pump is not stuck (4)
- W18 Local-density body look (4)
- W19 A short flinch from fire (4)
- W20 Squads as the Run's default control; Run first in the menu (4)
- W21 Crystal regrowth on (4)
- W22 HANDOFF and ROADMAP rewrite (4)
- W23 Parked list (4)
- W27 Blast glow square (3)
- W31 CI in parallel (3)
- W32 Union merge for index files (3)
- W33 Untrack the replay files (3)
- W34 Record the rulings (3)
- W35 Squad-mode run window test (3)
- W36 Rivers under reeds (3)
- W37 Mission 1's goal (3; D4)
- W38 Drop the outpost; hold the lure and siren (3; D10)
- W39 Spread card bug (3)
- W56 Char as barren ground (3)

## Dropped findings

- **STP 17, "the mould count falls far faster than the kill counter in a
  fight" (about 750 deaths unaccounted for on planet 1 between 16 and 20
  s).** This is the slime's own first-30-s shed, which EH 17, FTP 1 and
  STP 1 all filmed. Nothing points to a counter bug. Its next step
  (deaths by cause in the t line) is kept under W16, where it settles the
  shedding question.
- **EH 17, second half: the kit's "ground field 19.5 ms" for planet 8
  misleads.** This is a capture-kit note, not a game finding. The window
  rebuilds the field only when a world is built. The next review should
  read the kit's planet-8 cost line knowing that.
- **GD 23 as a separate issue.** It was joined into W9, not dropped for
  being wrong.
- **EH 15's Metal binding limit as an issue.** The Metal limit is the
  reviewer's guess, and nothing ships to a Mac. It is kept only as
  decision D9.

## Owner decisions

These are unranked. Each gives both sides where the reviewers differ, and
one recommended option.

- **D1. Should run planets use shape C's body under each tier, or keep the
  tiers as they are?** (W2; STP 2, with FTP's walker sweep beside it.)
  - *For C:* the owner chose it because "it networks the crystals". On
    planet 3 it grew to 9,324 with cords, and on planet 1 to 6,793 by
    2:00 (merge capture).
  - *Against, or not alone:* on planet 1 the C body still filled its rock
    ring without leaving by 2:00. FTP's evidence says the walker count
    matters too (8k regrows, 3k sits), even without C.
  - **Recommended:** C under each tier, filmed on planets 1-4 before the
    next playtest. This is STP's recommendation.
- **D2. Should breakaway be on in the run as built, or reworked first?**
  (W4.)
  - *RP:* turn on `condense` and `roam` from planet 2 or 3 now. It is
    built, twin-tested, and on seed 1 it made the slime take the big
    crystal and bite the squad. The art lead should look at the specks.
  - *STP:* breakaway steers straight at a chosen target, which is a goal
    written into the model and against "the picture is the data". It
    skips pumped crystals (`ROAM_FRESH`), and it travels as specks, which
    the owner disliked on 11-12. Let pieces follow a far smell first.
  - The merge's seed-2 capture found a weaker effect than seed 1.
  - **Recommended:** RP's option, on from planet 2 in the next playtest
    pack with a clip. It is "defaults on" and "build first", and the
    owner can judge the specks in play.
- **D3. What should rise from planet to planet, so the enemy progresses?**
  (W25, W8.)
  - GD: three axes, reach, appetite and an answer to the player's build.
  - FTP: one thing each tier does *to you* (the Drifter eats your drained
    crystal, the Sprawler cuts supply, the Trunk cords to pumps, the
    Swarmer's alarm brings the body).
  - STP: an economic ladder (eating, upkeep, smell reach).
  - EH: pressure keyed on depth and the gun's grade.
  - **Recommended:** FTP's per-tier "does to you" table (RP 2 asks for
    the same), as one card. The others can fill in the numbers behind it.
- **D4. Should the campaign and RTS control be frozen?** (W37, W38; RP 9,
  GD 15 and 16, FTP 11.)
  - *Freeze* (RP): both keep working and get no new features. This keeps
    the owner's "don't erase".
  - *Fold in* (GD): missions 1-3 become the goals of run planets 1-3.
  - *Keep building* (FTP 11 implies it): rework mission 1 so the slime
    contests the crystals.
  - *Retire* (RP, as an option).
  - **Recommended:** freeze (RP).
- **D5. Should every run start with a payload card in the gun, or every
  shop guarantee one?** (W10; FTP 6 and 16, GD 11.) Both reviewers
  favour it, and the alternative is the status quo of rolled luck.
  **Recommended:** one rolled payload card in the starting gun.
- **D6. Are camera shake and hit-stop allowed under "everything on screen
  is the simulation"?** (GD 21.) They paint nothing; they move the camera
  and the clock. No ruling covers them. **Recommended:** ask with a 10 s
  film of each. GD's reading is that they are allowed.
- **D7. Should the 40-soldier snowball be braked now through the economy,
  or after the enemy presses?** (W5.)
  - *Now* (FTP 3): one look each at a per-crystal drain cap and a rising
    soldier price. Both are allowed brakes under the upkeep dead end.
  - *After* (RP 7, GD 2): with a passive slime, a slower route to the same
    blob is still the only plan, and economy changes can't fix it alone.
  - **Recommended:** after. Measure time to 40 again once W1 or W4 is on.
- **D8. Should new slime rules be built GPU-first, with a CPU twin only
  when a census needs one, or keep full twins?** (EH 4.) Either way, EH
  proposes generating the parameter plumbing from one `params!` table.
  **Recommended:** GPU-first, held by a pinned llvmpipe hash test (EH's
  option a).
- **D9. Which platforms does the game target?** (EH 15.) If more than
  Windows, CI checks a Mac build and the 49-binding layout gets packed.
  **Recommended:** Windows only until the owner says otherwise.
- **D10. What happens to the tools that overlap payload bullets, and to
  the dead shop items?** (W38; GD 16, 17 and 18.)
  - Fold blast, gas and germ tools into gun cards, keeping mine and det
    cord as one kit slot.
  - Drop the outpost from the run shop.
  - Hold the lure and siren out of the run shop until the slime moves.
  - **Recommended:** drop the outpost and hold the lure and siren now.
    Fold the tools after the gun-feel pass.
- **D11. Should the Run default to squad control?** (W20; RP 8.)
  **Recommended:** yes. The owner calls squad mode in the run the real
  game.
- **D12. Should the own mould be parked?** (GD 19.) It is not in the run.
  **Recommended:** park it, and return to it as a late-run card once pumps
  are contested.
