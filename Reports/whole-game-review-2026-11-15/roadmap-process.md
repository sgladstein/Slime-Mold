# Whole-game review: roadmap and process (2026-11-15)

Role: roadmap-process. The question I was given: do the plans match the game and what the owner wants?
Main at `17f2e5b8` (#329). I read ROADMAP.md, HANDOFF.md, `wiki/decisions*.md`, the
merges since 11-12 (220 merge commits, about 110 PRs), the "Next / Open / Not
done" sections of the 11-14 and 11-15 reports, the project's thread list
and the open PRs. I made three captures on llvmpipe (run seed 1, squad
mode, idle hand). The idle hand is a control, not a player.

## The game as a whole, and whether the plans fit it

The game the owner plays now is the **roguelike run in squad mode**. You get
eight generated planets, one slime on each, and a shop between them that sells
rolled guns and bullet cards. Since #322 pumps are the main income. The
repo is not organised around that game. ROADMAP's *Now* band still
describes the 11-12 Larder RTS: free shots, the census gate, controls
part 1, a morning playtest. Its first line still reads "while the owner
can't playtest", but the owner has been playtesting since 11-14 22:00Z. It
was last edited at 11-14 18:52Z, and about 35 PRs have merged since. Its
rows cover none of these: squad mode, the guns, the pump economy, the gun
look, the UI review, condensing or the slime-enemy thread. HANDOFF is still
the front page of the paused dish game. Its *Next steps* are six dish
fixes, and five of its nine open decisions are about dishes the owner set
aside on 11-10.

The output is very high: about 40 PRs on 11-14 and 15 by 07:11Z on 11-15,
nearly all on owner asks or on reviews of owner asks. The aim is wrong in
one place, and it is the place the owner named. **No thread owns the
enemy's progression across a run**, and nothing on the roadmap makes the
slime press the player. "Build a roguelike run mode" closed at 09:16Z. "Improve the slime enemy"
owns the slime's tuning numbers, not the run's ladder. Meanwhile the
level-generator and ground-look threads put 12 PRs into planet shape and
look overnight. My captures agree with the owner's words. On planet 1 the
slime held 1,445 walkers at 1:00 and 1,515 at 5:00. It took 3% of the
crystals in five minutes and never once touched an idle squad. A slime
behaviour that already exists in the code, off by default, changes that on
the first try (finding 3). Defence, turrets, walls and "a reason not to
build 40 soldiers and walk in" all follow from a slime that leaves home.
The plan should be re-cut around that one problem, the guns' feel and the
run, and the rest parked.

## Findings

### 1. The real game has no owner: no thread, and no ROADMAP row, holds the run's enemy progression
- **Kind:** roadmap
- **Evidence:** thread list at 09:35Z. "Build a roguelike run mode" is *done* (09:16Z). "Integrate the Larder game's shared rules" is *done* (11-15 01:57Z), and it kept ROADMAP and the merge order. "Improve the slime enemy" (started 08:03Z) owns tuning, and this review's brief puts tuning out of scope. W6's only row says "Next: the owner's playtest". `wiki/world/run.md` *Open* passes the slime's size on big planets on as "the slime's economy, not the run's". The run report (`Reports/roguelike-run-2026-11-14/README.md` l.185-191) says the same and was "passed on and not tuned here".
- **Why it matters for play:** the owner's top complaint is an enemy that doesn't progress or press, and it falls between three threads.
- **Smallest next step:** name one thread *Run and enemy progression* and give it the ladder (`run::LADDER`, `WALKERS_PER_DEPTH`, what each depth switches on). Give it a ROADMAP row and findings 2-5 below. The coordinator asks for it today.
- **Impact:** 5. **Effort:** S (to name); the work is M. **Owner:** unowned.

### 2. Scaling the enemy by depth doesn't reach play: the slime sheds what it is given and then sits
- **Kind:** issue
- **Evidence:**
  - `roadmap-process/run1-p1-idle-300s.jpg`. Run 1, planet 1 (Drifter), squad mode, idle hand: mould 1,445 at 1:00, 1,516 at 3:00, 1,515 at 5:00. Crystals left 0.994, then 0.982, then 0.969. Shots 0, squad touched 0.
  - `roadmap-process/run1-p4-idle-180s.jpg`. Planet 4 (Swarmer): 5,277 at 0:10, 3,326 at 1:30, 3,363 at 3:00, crystals left 0.969, squad untouched. Both nests sit in rock bowls, and only a trickle runs toward one crystal.
  - The run report: planet 5 goes from 18,027 to 4,053 in 30 s, and the core world from 65,654 to 7,335.
  - The only escalation is the tier ladder plus "walkers a fifth more a planet" (`src/mould/run.rs:75,102`).
- **Why it matters for play:** this is the owner's "no rush ... no need to defend". It is also why the 40-soldier blob is safe to build: there is no clock and no pressure.
- **Smallest next step:** before tuning anything, write down what the slime does *at each depth*. For example: planet 1 sits, planet 2 sends breakaways, planet 3 hunts your pumps, planet 5 also grudges. Then try one switch per depth (findings 3 and 4).
- **Impact:** 5. **Effort:** M. **Owner:** the new run/enemy thread, with "Improve the slime enemy" for the numbers.

### 3. Low-hanging: breakaway (`roam`) is built and off, and on one seed it turns the sitting slime into a threat
- **Kind:** low-hanging
- **Evidence:** `roadmap-process/run1-p1-idle-roam-180s.jpg`. Same run, planet and hand as above, plus `condense=300 roam=300`. Mould 1,602 at 1:00 and 3,334 at 3:00 (1,516 without). It took the big crystal, and pieces reached the squad: the idle squad's free fire made 420 shots and killed 916 walkers, and the squad was being bitten. That is one seed, so it is a direction, not a result. The condense report (`Reports/condense-2026-11-14/README.md` l.163-167) found the same on the Fork: "the pieces reach every far crystal by 1:00", and the farm hand ended with 0 soldiers against 2. Both are still "off by default" and wait on "the owner's playtest" (same report, *Open*).
- **Why it matters for play:** it is the cheapest available answer to "they just hang at their home crystal". It is already twin-tested on the GPU.
- **Smallest next step:** have the run turn on `condense` and `roam` from planet 2 or 3 (one line in `Run::level`'s keys). Put it in the next playtest pack with a 20 s clip. One caveat on the look: on the way, a piece "stretches into a file of specks" (report l.168). The owner disliked specks on 11-12, so the art lead should look at it before or alongside.
- **Impact:** 5. **Effort:** S. **Owner:** run/enemy thread (unowned); numbers to "Improve the slime enemy".

### 4. Nothing gives the slime a reason to come for the player's income
- **Kind:** direction
- **Evidence:** the pump report (`Reports/pump-economy-2026-11-15/README.md` l.80-82): only "the pump on the slimed crystal" was worn down, so the slime hits a pump only where it already lives. In the shared rules (ROADMAP l.115-116) scent is `UnitIn.smell` on units, and a pump is a still unit that *takes*, not one that smells. Walls already leak scent (`wall_scent` 0.9).
- **Why it matters for play:** the owner wants a reason to defend. Pumps that draw the slime would make turrets, walls and split squads worth having, which is the strategy the owner says the game hasn't needed.
- **Smallest next step:** give a working pump a `smell` (one number; it already rides the units pass), or let `roam` prefer pumped crystals. Film one planet 2 seed at 0:30, 1:30 and 3:00 with `hand=pumps`.
- **Impact:** 4. **Effort:** S. **Owner:** run/enemy thread; the buildings rules are the Larder rules' path.

### 5. The player's power grows by multiplying while the enemy's grows by adding, and nobody has drawn the two curves
- **Kind:** direction
- **Evidence:**
  - Card rulings: range +100% a card and rate +100% a card, stacking to 2-4x (`wiki/decisions-game.md`, 11-14 15:06). Lucky rolls per stat (17:32). Gun price `(80 + 40 x depth) x grade^2` (run report l.101).
  - The enemy gets +20% walkers a planet, which then sheds (finding 2).
  - The owner: "especially when your guns get stronger".
  - The curve itself is a guess; no report draws it.
- **Why it matters for play:** if a mid-run gun deletes a planet in seconds, no tier ladder will feel like progression.
- **Smallest next step:** one page with two lines by depth: the kills a second of a typical shop's squad (from `range=` loadouts) and the slime's mass at 1:00 from the captures above. No census.
- **Impact:** 4. **Effort:** S. **Owner:** run/enemy thread with the gun thread.

### 6. The tier that learns (Grudge) is left out of the run, and its GPU port's thread is blocked
- **Kind:** roadmap
- **Evidence:** run report l.25-29: "The Grudge is left out: `Tier::settings` refuses it on the GPU engine (no scar keys yet)". Binding 47's second block was reserved for the scar on 11-12 (ROADMAP *GPU engine*). "Port mould species and tiers to the GPU" has been *blocked* since 11-14 00:47Z.
- **Why it matters for play:** the most "progressing" enemy already designed is one that remembers where you hurt it. It is built on the CPU and can't be used in the game the owner plays.
- **Smallest next step:** port the scar into the reserved block, twin-test it on llvmpipe, and add Grudge as a late rung.
- **Impact:** 4. **Effort:** M. **Owner:** unowned (the GPU thread is blocked).

### 7. The pump change pushes the 40-soldier blob back by about a minute; it doesn't remove it
- **Kind:** issue
- **Evidence:** the pump report's own smoke test (l.83-88; one seed, Fork, squad mode). The greedy hand had 40 soldiers at 1:30 before and 22 after. The pumping hand banked 5,124 by 1:30 with 4 soldiers, which buys about 12 more at 400. The owner: "I mostly ... create new soldiers until I have 40 soldiers and then I attack the slime."
- **Why it matters for play:** with a passive slime (finding 2) a slower route to the same blob is still the only plan. Economy changes can't fix this without enemy pressure.
- **Smallest next step:** measure time-to-40 soldiers on run planet 1 in squad mode with `hand=greedy` and `hand=pumps`, with roam off and on. That is four short captures, not a census. Read it beside the owner's next play.
- **Impact:** 4. **Effort:** S. **Owner:** run/enemy thread.

### 8. Squad mode is the owner's game, but the run starts in RTS control unless he changes an option every launch
- **Kind:** low-hanging
- **Evidence:** `src/bin/larder-world/menu.rs:75` (Control defaults to `"rts"`). `run_args` takes it from that option (`menu.rs:182-183,393`). Only the UI size is saved (`src/world/settings.rs:129`). `play.ps1` passes no `mode=`.
- **Why it matters for play:** the real game's control has to be picked by hand each time, and anyone else who opens the run plays the RTS version.
- **Smallest next step:** make squads the run's default control, or save the Control option the way the UI size is saved. This touches the menu, so it is tagged **for the menus/HUD review**.
- **Impact:** 3. **Effort:** S. **Owner:** "Review menus, shop and HUD layout".

### 9. Two control modes and two game modes double every feature; whether to keep both is an open owner decision nobody has asked
- **Kind:** owner decision
- **Evidence:**
  - The pump change works one way with the larder and another without (pump report l.21-26: "With the larder on, carrying is its own brake").
  - Missions force RTS (`menu.rs:385`, "The missions are written for the larder").
  - W5's next step is "a tuning pass over all eight" missions.
  - The owner, 11-14 15:21Z: "it might become the main game. I don't want to erase the campaign yet". Then 11-15 08:30Z: squad mode in the run is the real game.
- **Why it matters for play:** every hour spent keeping RTS, the larder and the campaign working is an hour not spent on the run.
- **Smallest next step:** a card. "Freeze the campaign and RTS control: they keep working and get no new features (recommended)" / "Keep building both" / "Retire them". This keeps the owner's "don't erase".
- **Impact:** 4. **Effort:** S. **Owner:** the owner; asked by the coordinator.

### 10. ROADMAP's *Now* band is out of date and too long to read
- **Kind:** roadmap
- **Evidence:**
  - l.41 "While the owner can't playtest" (he is playing).
  - L-C "part 1 built (branch …), census gate … running": built and merged 11-12, and the thread is done.
  - L-G "next a hole that is real in the simulation": done by 11-12 (#113), with craters at 2 s since 11-14.
  - S-P "planned 11-12": done (`Reports/gpu-readout-2026-11-12.md`).
  - L-PT "done 11-12".
  - Two rows are both labelled **L-O** (l.50 one body, l.79 own mould).
  - L-T2's next step is "per-tier core values", but the core rule was removed 11-12 20:21Z.
  - The *Caps* paragraph says `UNITS_MAX` is 64; the code says 96 (`src/mould/squad.rs:46`).
  - L-F's cell is about 5,000 characters of history.
  - There are no rows for squad mode (#240, #254, #276, #321), the guns, cards and bullets (#257 … #325), pumps (#322), the gun look, the UI review (#324, #334), plants (#290-#315), condense (#266, #274) or the slime enemy (#338).
  - The page was last edited 11-14 18:52Z.
- **Why it matters for play:** new threads read ROADMAP to choose work (CLAUDE.md), so a stale band sends them to the 11-12 game.
- **Smallest next step:** replace the band with the draft at the end of this file. Move the history cells to their reports, and move the Params ledger (l.334-435) to a comment by `struct Params`, where the PRs that claim words already edit.
- **Impact:** 3. **Effort:** S. **Owner:** unowned (the integration thread is done); suggest the coordinator.

### 11. HANDOFF is the front page of a game the owner paused five days ago
- **Kind:** roadmap
- **Evidence:**
  - l.26 Larder World: "next W1b" (done 11-11). l.30 own mould: "contact is next" (it was built on the GPU on 11-12). l.32 the player AI: points to the old small-dish `larder` binary.
  - No line covers the run, squad mode, the guns, pumps, the start menu, the gun range or `play.cmd`.
  - *Next steps, in order* (l.83-100) is six dish-game steps. *Decided and not yet built: the forest floor* (l.61) is superseded by Larder World.
- **Why it matters for play:** this is the page CLAUDE.md tells every game thread to read first.
- **Smallest next step:** rewrite *Where the game stands* as eight lines: run, squad mode, guns, economy, slime enemy, planets, looks, campaign (frozen?). Move the dish *Next steps* under a *Paused* heading. Lint passes (rule 5 only fixes where open decisions live).
- **Impact:** 3. **Effort:** S. **Owner:** unowned.

### 12. Owner rulings given but not recorded in `wiki/decisions-game.md`
- **Kind:** owner decision
- **Evidence:**
  - 11-15 08:30Z, three priorities: squad mode in the roguelike run is the real game; "the gun graphics … should really be one of the draws of the game. They need to be really satisfying"; "there needs to be more interesting progression in the enemy … especially when your guns get stronger".
  - 11-14 15:33Z: "15/20 minutes to 1 hour+" a level. This is only in the run report (l.155).
  - 11-15 02:17Z: judge the run's pace in play first. This is only in `run.md`.
  - The planets' 6x growth, "the owner's 6x" (#308). This is in no decisions row.
  - The newest row in the file is 03:07Z.
- **Why it matters for play:** these are the priorities every thread should weigh. Without a row, a thread reading decisions-game sees the 11-14 framing ("separate, it might become the main game").
- **Smallest next step:** add four rows with the quotes, and update the 11-14 15:21 row's run status.
- **Impact:** 3. **Effort:** S. **Owner:** whichever thread next touches the wiki; the coordinator can ask.

### 13. Open decisions that are not on HANDOFF's list (the one place they may live), and stale ones that are
- **Kind:** owner decision
- **Evidence:**
  - Missing from the list:
    - condense and breakaway on in shape C (condense report *Open*);
    - the run's default control (finding 8);
    - freezing the campaign and RTS (finding 9);
    - a breather planet near 6 (`run.md` *Open*; deferred to play);
    - the UI's 0.7 floor (`Reports/ui-review-2026-11-15/README.md` *Next*; the menus/HUD review's);
    - the 20 s landing preview (decision 1's remainder, dish game).
  - Stale on the list:
    - 2-5 and 7 (tube look, grains, food flow, Organ/Wanderer, Bloomfront) belong to paused games;
    - 6's "upload Jones 2011 before W0": W0 merged 11-11;
    - 8's "two soldier teams allies or rivals" and "fog of war" have no place in the run as built.
  - Answered but still listed as open in a report: `Reports/trail-cards-2026-11-14.md` *Not done* asks whether crater and alarm trails are drawn. The owner said "They should be visible though … 100%" (decisions-game, 11-14 08:39).
- **Why it matters for play:** the owner is asked about dishes and isn't asked about the switch that would change his next session.
- **Smallest next step:** move 2-7 under *Paused games' questions*, add the run/enemy decisions at the top, and fix the trail-cards line.
- **Impact:** 3. **Effort:** S. **Owner:** unowned.

### 14. The level generator's overnight work went mostly into planet polish the owner didn't ask for, and the nest bowls may be part of why the slime stays home
- **Kind:** issue
- **Evidence:**
  - 9 levelgen PRs between 11-14 22:18Z and 11-15 07:11Z (#302, #306-#309, #312, #323, #327, #329): doors, moats, lakes and rims. Plus 3 ground-look PRs (#305, #317, #328).
  - The work comes from three review rounds (`Reports/levelgen-review-2026-11-14`, `-2026-11-15`, `art-review-2026-11-14`) more than from owner asks. The owner's asks behind it are 6x growth and "any map beatable by any strategy".
  - Both of my stills show each nest inside a rock bowl with one door (#327 "a front door and a back road for every nest").
  - That the bowl holds the slime in is a **guess**, but a cheap one to test.
- **Why it matters for play:** the planets look good, and the enemy on them is idle. More planet look is now the wrong next hour.
- **Smallest next step:** one capture pair, planet 1 seed 1, idle, 3:00, with the bowl as generated and with its rim opened (a levelgen flag or a hand-edited map in `larder-edit`). If the bowl is the cause, "doors" become the slime's way out on purpose. Hold twists, rescue planets and per-biome landing crystals until the enemy moves.
- **Impact:** 4. **Effort:** S. **Owner:** "Generate roguelike levels procedurally".

### 15. "Satisfying guns" is more than the gun look thread's pictures: there are no new gun sounds since the sound thread closed
- **Kind:** direction
- **Evidence:**
  - `src/sound/world.rs` voices bullets in flight (l.355-360). It has no voice per gun kind (rifle, carbine, long gun, scattergun) and none per payload: no bomb, gas or germ in the file.
  - Its last edits were the gun thread's (#288, #299). The sound thread is done.
  - The owner's 11-14 20:03-20:22Z rulings shrank every hit light to the size of its damage, so "satisfying" has to come from the simulation's response (holes, flung walkers, fire) and from sound, not from bigger flashes.
  - "Make the guns look satisfying" is active.
- **Why it matters for play:** the owner says guns should be a draw of the game. Feel is roughly half sound.
- **Smallest next step:** give the gun look thread the gun's sound too. First, a scattergun-on-blast clip with a per-kind report and a per-payload thump, put on its next page beside the picture.
- **Impact:** 4. **Effort:** M. **Owner:** "Make the guns look satisfying".

### 16. What should stop, or be parked with a line saying so
- **Kind:** roadmap
- **Evidence:**
  - Player AI (thread *blocked* since 11-14 08:23Z). The owner, 11-14 01:08Z: "I have easily beat the game in all my playtests so I am not too concerned about the slime winning your automated test."
  - C4/D3's counter table and the plan-by-tier matrices are strategy censuses on RTS plans.
  - W5's "tuning pass over all eight" missions.
  - The own mould's GPU follow-ups (`species_cap`, an attacker's edge), which are not in the run.
  - L-T2's per-tier core values (the core is gone).
  - Archive bands G, E and R, and the RTS demos' wiki section ("Other games in design").
- **Why it matters for play:** each competes for runner time and review attention with the enemy and the guns.
- **Smallest next step:** mark each *parked (11-15, why)* in the new band. Restart one only when an owner ask names it.
- **Impact:** 3. **Effort:** S. **Owner:** coordinator.

### 17. Reviews now set more of the backlog than the owner does
- **Kind:** issue
- **Evidence:**
  - From 11-14 to 11-15: an art review (#248, #279), planet reviews (#312, `levelgen-review-2026-11-15`), a bug sweep and two bug checks (#311, #313), a UI review (#324). The technical lead and game designer reviews are still open, and now this five-way review.
  - Their *Next* lists became the night's PRs: #317 and #328 come from the art lead, #323 from the planet reviews.
- **Why it matters for play:** reviews catch real bugs (the link soft-lock in #311). But the polish they generate crowds out the two things the owner asked for.
- **Smallest next step:** every review's ranked list states, for its top 3, which owner priority it serves. Items serving none go to a *later* list, not to a thread.
- **Impact:** 3. **Effort:** S. **Owner:** coordinator.

### 18. The roadmap rule "update your row in the same PR" fails because new work never gets a row
- **Kind:** low-hanging
- **Evidence:** ROADMAP l.437-447 (*Keeping this page true*). There was no row for squad mode, guns or pumps to update, and none was added. The wiki lint enforces citations, not the roadmap.
- **Why it matters for play:** the same drift will come back within a day of any rewrite.
- **Smallest next step:** make `wiki/lint.py` warn when a PR adds a report but touches no ROADMAP row and names no row id in the report's front matter. Also: a new thread's first PR adds its row.
- **Impact:** 2. **Effort:** S. **Owner:** unowned.

### 19. Three pillars of the real game have no wiki page: guns and cards, squad mode, the economy
- **Kind:** low-hanging
- **Evidence:** `wiki/index.md` l.40-51 has the run, planets, plants, tiers, toolkit and own mould. It has no guns/cards/bullets page (spread over `guns`, `no-recharge`, `rolled-bullets`, `gun-budget`, `gun-order`, `gun-range`, `card-text`, `shot-lights`, `trail-cards` and 11 decisions rows), no squad-mode page (`squad-mode-2026-11-14`, `free-camera`, `narrow-squads`) and no economy page (`pump-economy`, `no-recharge`, the upkeep rulings).
- **Why it matters for play:** the gun look thread and any new enemy thread need one page saying what a gun is today.
- **Smallest next step:** three short wiki pages, each a *Now* paragraph plus links (wiki rule: no number tables).
- **Impact:** 2. **Effort:** S. **Owner:** the gun thread, the squad-mode owner and the pump thread respectively; the coordinator assigns.

### 20. Where a confident report could be wrong: the core world's ~7,300 walkers
- **Kind:** issue
- **Evidence:** the run report l.179-191 reads the core world's fall from 65,654 to 7,335 as "the slime's own economy". 7,300 is also the number at which the GPU once dropped its crowd limit (`Reports/gpu-readout-2026-11-12.md` l.493-497; fixed 11-12 for start walkers at most half the agar). The two are probably unrelated, since the threshold scales with agar. That is a **guess**.
- **Why it matters for play:** if an engine cap is hiding behind "economy", tuning the slime's food won't make deep planets bigger.
- **Smallest next step:** run one core-world capture at 1:00 with `capacity` doubled. If the walker count moves, it was a cap. #339 (the 1070 timing at run scale, open) is the natural place to note it.
- **Impact:** 2. **Effort:** S. **Owner:** "Run jobs on Scott's desktop PC" / #339's thread.

## Draft: ROADMAP's *Now* band as it should read today

*(Draft only; ROADMAP.md is not edited by this review.)*

### Now: the roguelike run in squad mode (11-15)

The game is the run (owner, 11-15 08:30Z): eight planets, squad control,
pumps as income, rolled guns and cards from a shop between planets. Every
row serves one of the owner's three asks: **an enemy that progresses and
presses**, **guns that are a draw**, **menus that don't get in the way**.
Owner rulings that bound all rows: no upkeep (11-14 04:53Z), build first,
balance later (11-12 17:26Z), everything on screen is the simulation.
History lives in each row's report, not here.

| # | Step | Owner | Waits on | State |
| --- | --- | --- | --- | --- |
| N1 | **Enemy progression across the run.** What the slime does at each depth: sits, breaks away (`condense` + `roam`), hunts pumps, remembers (Grudge); per-depth switches in `Run::level`; the player-power and slime-mass curves by depth on one page | *new thread: Run and enemy progression* | nothing | Planet 1 idle: 1,515 walkers at 5:00, 3% of crystals taken; with `roam` 3,334 at 3:00 and the squad attacked (one seed, this review). First: roam from planet 2-3, in the next playtest pack |
| N2 | **The slime's behaviour and numbers** (spread, sharing, thickening, breakaway pace, the speck look on the way) | *Improve the slime enemy* | nothing | #338 open (a fed slime that cannot divide thickens) |
| N3 | **A reason to defend**: pumps the slime wants (a pump's `smell`, or `roam` choosing pumped crystals); turrets and walls judged by whether they get used | N1's thread, with the buildings rules' owner | N1's first switch | not started |
| N4 | **Guns that are a draw**: shot, hit and payload looks from the simulation, and their sounds per gun kind and payload | *Make the guns look satisfying* | nothing | working; sound has no per-kind voices (`src/sound/world.rs`) |
| N5 | **Menus, shop, HUD and keys**; the run's default control is squads (`menu.rs:75`) | *Review menus, shop and HUD layout* | nothing | #324 merged, #334 open |
| N6 | **Planets**: hold new look and feature work until N1 moves; one check of whether the nest bowls keep the slime home | *Generate roguelike levels procedurally* | N1 | #329 merged; twists, rescue planets and per-biome landing crystals wait |
| N7 | **Leftover bugs** from the 11-15 check | *Fix leftover bug-check items* | nothing | working |
| N8 | **The 1070**: twins pending since 11-14 (germ, gas, explosions, condense, run scale) in one batch | *Run jobs on Scott's desktop PC* | the PC on | #339 open |
| N9 | **Playtest pack** for the next session: what changed since the owner's 08:30Z play (pumps #322, Esc/WASD #321, narrow gaps #326, gun range #325, card text #318, win at 2% #320, N1's switch), each with its key and a short clip | coordinator | N1's first switch | not started |

**Parked (11-15), restart only on an owner ask:** the player AI and
strategy censuses (owner 11-14 01:08Z); the campaign and RTS control, which
keep working with no new features (pending the owner's card, finding 9);
the own mould's GPU follow-ups; the per-tier core values (no core since
11-12); the paused dish games and RTS demos (archive).

**Owner decisions to ask, in order:** freeze the campaign and RTS control;
condense and breakaway on in the run (from which planet); the run's
default control is squads.
