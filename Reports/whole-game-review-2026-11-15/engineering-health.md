# Whole-game review, 2026-11-15: engineering health

Reviewer role: engineering-health (what will slow every thread down next
week). Main at 17f2e5b8. I read `src/`, `.github/workflows/`, the last 400
Actions runs, the view-cost, pc-batch and bug-check reports, and ran six
captures on llvmpipe (`larder-world look=DIR`; scratch in
`/tmp/wgr/engineering-health/`, five stills in `engineering-health/`). The
scripted hands are controls, not players. One `bench=50` run on llvmpipe
was stopped at its 300 s limit and gave no numbers. Its question (the GPU
cost of payload hits at 40 soldiers) goes to the 1070 in finding 2.

## The game as a codebase

**What it is.** The real game is the roguelike run in squad mode. That is
the rules in `Game<W: World>` (lib, `src/mould/larder_world/`, 8.9k lines,
already split by C9/D6), the run and shop (`run.rs`, `rounds.rs`,
`guns.rs`, `bullets.rs`), and the slime. The slime runs on **two
engines**: the CPU `BigDish` (`big.rs`, 5.9k lines) and the GPU engine
(`engine.wgsl` 3.3k plus `gpu/mod.rs` 3.3k, 49 storage bindings). A
3.4k-line view shader draws it. A 74-field `App` in the `larder-world`
binary holds it all together. The source is 93k lines. About 24k of them
belong to paused dishes and RTS demos.

**Is the architecture right?** The core seam is right. Rules on the CPU
talk to the slime only through one squad frame and a `World` trait, so the
GPU needs one small read-back a frame (`larder_world.rs:8-18`). The
`squad::Burst` primitive (kill, burn, crater, fling, alarm, gas, germ,
bait) is a good effect algebra, and it runs on both engines. Three things
scale badly with every new system:

1. **Game state has leaked into the window binary.** Payload casting
   (`Rounds`), tools (`Kit`), the run, the mission tracker and larder
   theft all live in `App`. So the census, the player AI, the replays and
   the CPU tests play a different game from the one the owner plays.
2. **Every slime mechanic is written twice**, and every parameter is
   plumbed by hand through about six tables.
3. **The payload path syncs the GPU once per hit.** That cliff sits right
   where the owner plays (40 soldiers) and right where he wants the game
   to go (much richer guns).

The workflow costs are fixable in a day. CI takes 11-14 min, and 9 of
those minutes are three test binaries running one after another. 45 PRs
have bounced off the queue on conflicts, mostly in index files. The docs
every thread must read first are stale (HANDOFF) or 3.6x over their cap
(`wiki/world/larder.md`, 43 KB).

**The one refactor:** move the window's game state into a lib `Session<W>`
(finding 1). Every feature after it gets a CPU test, a census, a replay and
a player AI for free, instead of only a window. The next most valuable
change is to batch payload bursts into the frame's submit (finding 2),
because richer guns are blocked on it.

Of the bug check's open items (`Reports/bug-check-2026-11-15/README.md`),
two matter most for play. **Spread on bullet cards does nothing** on the
rifle and long gun: a card that does nothing is the opposite of a
satisfying gun. **Designed maps get plants only in the window** is the
same disease as finding 1. **Fire kills counted unevenly** only matters
once a census reads kills.

## Findings, most important first

### 1. Game state lives in the window binary, so tests, census, AI and replays play a different game
- **Kind:** issue (architecture)
- **Evidence:**
  - `src/bin/larder-world/main.rs:617` `kit: Kit`, `:620` `rounds: Rounds`, `:629` `mission: Option<Tracker>`, `:652` `run: Option<Run>`, `:595` `stolen`. 74 fields on `App` in all.
  - `Rounds::after_step` (`src/mould/rounds.rs:728`) lands every payload card. It is called only from `main.rs:1085,1111,1256` (plus `examples/rounds.rs` and one sandbox test). Neither `larder_world_ai.rs` nor any census example touches `Rounds`.
  - `main.rs:616`: "Tools are not in the game's replay (the kit is outside `Game`)".
  - The bug check's open "plants only in the window" item is the same pattern.
- **Why it matters for play:** "No recharge" (11-14) made every shot carry a card. So a CPU census or player-AI result on a loaded gun measures plain bullets, and a replay of a payload round is not the round that was played. Every new gun card, tool or enemy event the owner asks for can only be tested through the window harness. Those are the slowest tests (197 s in CI).
- **Smallest next step:**
  1. Create `mould::session::Session<W: World> { game, rounds, kit, tracker, level }` with one `step(dt)`.
  2. Move `after_step`, `use_tool` and the theft bookkeeping into it.
  3. Have `App` own a `Session<GpuWorld>`, and give the census, AI and replay a `Session<BigDish>`.
  4. First proof: a CPU test where a bomb card kills, through `Session` only.
- **Impact:** 5 · **Effort:** M · **Owner:** unowned (the integration thread is the natural home)

### 2. Each payload hit stalls the GPU with a flush and 1-3 blocking read-backs
- **Kind:** issue (frame cost)
- **Evidence:**
  - `GpuWorld::burst` (`src/gpu/larder_world.rs:617-690`) first calls `self.flush()`, which waits for the step in flight.
  - A kill then calls `Engine::struck` → `read_back` (`gpu/mod.rs:2588`). That function allocates a new staging buffer and runs `device.poll(wait_indefinitely)` (`gpu/mod.rs:3037-3049`).
  - A fling reads `gas_word(3)` before and after (`gpu/mod.rs:2614-2626`), which is two more round trips. Gas does the same.
  - Capture `b40` (fork, `hand=farm bank=60000 rounds=bomb`, 40 soldiers by 40 s; still `engineering-health/farm40-bomb-041s-near.jpg`): 338 → 350 casts in one game second while most of the squad was harvesting. That is 12 bombs a second, so about 36 blocking read-backs a second.
  - A 40-soldier assault at rifle rate (`shot_every` 1.0, `rules.rs:476`) is about 40 casts a second, about 120 syncs. A scattergun is 2-6 bullets a shot.
  - The 1070's "bombs cost nothing" line (`Reports/pc-batch-1070-2026-11-14.md`, 1.80 / 2.40 ms) was measured with the scripted 8-soldier hand.
- **Why it matters for play:** It hits the owner's own pattern (40 soldiers attacking) and the owner's top look ask (guns that are "really satisfying"). Each sync also throws away the step/draw overlap that the split frame was built to get.
- **Smallest next step:**
  1. On the 1070: `bench=60 hand=farm bank=60000 rounds=bomb` against `rounds=` (none), reading the bench's "read-backs a step" and the frame p95.
  2. Then record bursts into the frame's command buffer and take kill and fling counts from the frame readout's `RO_WSTAT` words at frame end. That is the ROADMAP L-K item "bursts batched into the frame's submit".
- **Impact:** 4 · **Effort:** M · **Owner:** performance thread / gun thread

### 3. Richer gun effects: compose `Burst`s, not new fields, and lift the light and bullet caps
- **Kind:** direction
- **Evidence:**
  - Bombs already look like the simulation at its best: flung slime, craters and fire (`farm40-bomb-041s-near.jpg`).
  - A plain shot is a streak plus a point light, and leaves nothing on the ground beyond the strike (`run3-squads-plain-27.1s-near.jpg`; `draw.rs:621-720`).
  - `MAX_LIGHTS = 64` (`world/view.rs:43`), and past it the rest are dropped (`view.rs:665`). Lights are pushed in this order: building glows, bullets, turrets, bursts, hits, then muzzles (`draw.rs:637-715`), so at 40 soldiers the muzzle flashes are the first lost.
  - `SHOTS_MAX = 256` bullets in flight, the newest dropped (`squad.rs:47`). `shaded` loops over every light for every pixel (`view.wgsl:2080`).
- **Why it matters for play:** The owner wants the guns to be "one of the draws of the game". The cheapest juice that also obeys "everything on screen is the simulation" is a small `Burst` on every plain hit: a fling of a few walkers (splash), a crater, a scorch in plants. Both engines already run those, so this needs no new twin code. New fields need twins and bindings.
- **Smallest next step:**
  1. After finding 2, give plain bullets `fling` 0.3 and a `crater` of 1 cell.
  2. Film it at zoom 12 against main, as a blind A/B.
  3. Bin lights into 16 x 16 screen tiles in `prep`, or at least push muzzles before bullets. Raise `MAX_LIGHTS` to 256.
  4. Quote the frame cost on the 1070.
- **Impact:** 4 · **Effort:** M · **Owner:** gun/graphics thread

### 4. Every slime mechanic costs double, and the owner's top asks are slime mechanics
- **Kind:** issue (architecture) / owner decision
- **Evidence:**
  - Condense (`bea09d4f`): `big.rs` +223, `engine.wgsl` +137, `gpu/mod.rs` +83, twin test +56, view +98 lines.
  - Since 11-11, 26 commits touched both `big.rs` and `engine.wgsl`.
  - The parameter tables are written by hand: `tiers::set_key` (`tiers.rs:226`), `gpu::set` (`gpu/mod.rs:700`), `race_settings` with renamed fields (`sensor_offset`→`reach`, `rotate_angle`→`turn`, `deposit`→`laid`; `gpu/larder_world.rs:46-98`), the WGSL `Params` struct, campaign rule keys (`campaign.rs:239`), `larder_world_ai.rs:413` and `larder_ai.rs:841`. That is 145 `Settings` fields against 191 `BigParams` fields.
- **Why it matters for play:** The owner wants enemy progression, aggression and a reason to defend. Built as new slime rules, each costs two implementations plus a twin test.
- **Smallest next step:**
  - Owner decision, put as two cards: (a) new slime rules are built GPU-first, held by a pinned llvmpipe hash test, with a CPU twin only when a census needs one; or (b) keep full twins.
  - Either way, generate the key, `Settings`, `BigParams` and WGSL `Params` plumbing from one `params!` table (one macro listing the name, type, default and CPU field) and delete the hand mappings.
  - Meanwhile, prefer game-rule levers that write fields both engines already have (scent, alarm, `Burst`; finding 19).
- **Impact:** 4 · **Effort:** S (decision) + M (macro) · **Owner:** unowned

### 5. CI runs three test binaries in series: about 9 of its 11-14 minutes
- **Kind:** low-hanging
- **Evidence:**
  - CI run 43697525730, job "test + clippy": compile 1 m 30 s, then lib 229.6 s (484 tests), `larder-world` 196.6 s (35 tests), `mouldgpu` 126.6 s (39 tests), one after another.
  - A merge-queue batch takes 14.0 min (run 43697143013).
  - Flakiness is low: of 400 runs, 8 CI failures, 5 of them wiki lint and 1 clippy. None was a test flake.
- **Why it matters for play:** About 10 threads land through one queue that runs one batch at a time. Halving CI halves the wait to land, and the time a red batch blocks everyone.
- **Smallest next step:** Split `test` into a matrix (`--lib`, `--bin larder-world`, `--bin mouldgpu`, and the rest), each job with the shared rust-cache. Or run `cargo nextest`, which runs the binaries in parallel. Expected time is about 5-6 min. The repo is public, so the minutes are free.
- **Impact:** 3 · **Effort:** S · **Owner:** unowned

### 6. Queue bounces come mostly from shared index files: add a union merge
- **Kind:** low-hanging
- **Evidence:**
  - 45 PRs carry the queue's "does not merge cleanly" comment.
  - Most-touched files since 11-14 (non-merge commits): `Reports/README.md` 56 (118 KB), `Reports/pages.md` 48, `larder-world/main.rs` 35, `view.wgsl` 32, `draw.rs` 32, `larder-world/tests.rs` 28, `wiki/decisions-game.md` 23.
  - Commit `fbfeca60` ("condense lines above the tail, clear of other PRs' appends") shows threads already working around this by hand.
- **Why it matters for play:** Each bounce costs a thread a merge and a 14-min re-queue.
- **Smallest next step:** Add `.gitattributes` with `Reports/README.md merge=union` and `Reports/pages.md merge=union`. The queue merges with plain `git merge` on the runner, so git's built-in union driver applies. Add `wiki/decisions-game.md` too if its rows are append-only. Measure the bounce count a week later.
- **Impact:** 3 · **Effort:** S · **Owner:** unowned

### 7. "Matches the CPU" means "matches the CPU on llvmpipe, at small scale, for a few ticks"
- **Kind:** issue (tests prove less than their names)
- **Evidence:**
  - The twin tests (`src/bin/mouldgpu.rs:2331-5196`, `gpu/larder_world.rs:1024-1576`) run at scale 1-2, 1,000-5,000 walkers, and 1-4 ticks or at most 30 s. They are exact only on llvmpipe.
  - On a card, `tol()` loosens them (`gpu/larder_world.rs:878`; plants 0.05, fire 0.2). The 1070 gave 824 vs 933 walkers at 30 s, 988/1068 vs 992/1064 births, and one flung walker off (`Reports/pc-batch-1070-2026-11-14.md`).
  - The CPU also differs between Windows and Linux (libm; HANDOFF decision 6). The window plays at scale 2 (`main.rs:1466`) on planets up to 6x.
- **Why it matters for play:** A census on the cloud CPU is not the owner's GPU game. A report's "the GPU matches" is a proof of logic, not of play. No report should quote CPU census numbers as what the owner will see.
- **Smallest next step:** Rename or document the suite as llvmpipe logic twins. Add one *statistical* twin to the 1070 batch: 8 paired seeds, mould count and kills at 60 s on CPU and GPU, compared by sign test. Note in `wiki/method/census.md` which engine each census used.
- **Impact:** 3 · **Effort:** S · **Owner:** unowned

### 8. No window test plays the owner's game (squad mode in a run)
- **Kind:** issue (test coverage)
- **Evidence:** `run_base()` sets `squads: false` (`larder-world/tests.rs:806`). `the_window_plays_a_runs_planet` plays the RTS mode on planet 3 at scale 1 with 1,250 walkers (`tests.rs:982`). Only one test in the file sets `squads: true`. No test plays a planet past 3, where the 6x code (sliced walking maps, the gather refresh) runs.
- **Why it matters for play:** The panic-in-the-window class of bug (#114) would reach the owner's actual mode unseen.
- **Smallest next step:** Copy that test with `squads: true` at depths 3 and 6, hand `every`, 20 game seconds. Depth 6 at 2.8x costs about 3 s of wall time per game second on llvmpipe, about 60 s.
- **Impact:** 3 · **Effort:** S · **Owner:** squad-mode thread

### 9. Two tracked replay files can silently stop `play.cmd` from updating
- **Kind:** low-hanging
- **Evidence:**
  - `git ls-files` lists `larder-world-replay-planet-1-1-1.txt` and `larder-world-replay-planet-2-1-11018803426441713926.txt`, committed in `bcbc3f1c`.
  - `.gitignore:8-12` records that #192 did exactly this, and that "play.cmd stopped updating".
  - `play.ps1` refuses to pull on any tracked change.
  - The game rewrites `larder-world-replay-{map}-{seed}.txt` at Esc or at the end of a round (`main.rs:1044`).
- **Why it matters for play:** If the owner plays one of those two planet and seed pairs, every later playtest runs stale code. The only warning is a yellow line in a console that is scrolling past.
- **Smallest next step:** `git rm --cached larder-world-replay-*.txt`, plus a CI check that no `larder-world-replay-*` file is tracked.
- **Impact:** 3 · **Effort:** S · **Owner:** unowned

### 10. `Args` has no `Default`, so every new flag edits 18 struct literals
- **Kind:** low-hanging
- **Evidence:** 17 full `Args { ... range: None }` literals in `larder-world/tests.rs`, 1 in `main.rs:1466`, and `Args` is not `Default` (`main.rs:375`). `tests.rs` is the fourth most-touched source file since 11-14.
- **Why it matters for play:** Every new feature flag is a guaranteed conflict with every other open PR that touches the tests.
- **Smallest next step:** `#[derive(Default)]` (the default map is `None`), then rewrite the literals as `Args { map: Some("fork".into()), scale: 1, walkers: 1250, ..Args::default() }`.
- **Impact:** 2 · **Effort:** S · **Owner:** unowned

### 11. HANDOFF is stale and the Larder wiki page is 3.6x its cap; every thread pays for it
- **Kind:** issue (docs as infrastructure)
- **Evidence:**
  - HANDOFF's "Where the game stands" says "next W1b". It does not mention the run, squad mode, guns or pumps.
  - `CLAUDE.md` tells every game thread to read HANDOFF first.
  - `wiki/world/larder.md` is 43 KB against the 12 KB cap (owner, 11-11 15:16Z), and `lint.py:62` only warns.
  - `wiki/world/planets.md`'s Now section is one line of about 10 KB.
  - `ROADMAP.md` is 76 KB and `Reports/README.md` is 118 KB.
- **Why it matters for play:** Each thread spends about 15k tokens on orientation that points at the wrong next step.
- **Smallest next step:** Rewrite HANDOFF's Larder lines (run plus squads plus guns plus pumps as the game). Split `larder.md` by phase. Make the Now cap a lint error.
- **Impact:** 3 · **Effort:** S · **Owner:** wiki / integration thread

### 12. Paused code is a quarter of the source, and the live game still imports from it
- **Kind:** roadmap
- **Evidence:**
  - The paused files total 23.8k lines and 95 lib tests: `mould/mod.rs`, `look.rs`, `level.rs`, `render.rs`, `play.rs`, `lesson.rs`, `organ.rs`, `wander.rs`, `tune.rs`, `larder.rs`, `larder_ai.rs`, `fever.rs`, `skirmish.rs`, and the `mould`, `organ` and `larder` binaries.
  - The live code depends on them: `campaign.rs:28` and `sound/world.rs:36` take `End` from the M3 `larder.rs`, and `larder_world.rs` takes `Rng` from the dish's `mod.rs`.
  - 42 GPU twin tests live in the 5.3k-line sandbox binary `mouldgpu.rs`.
- **Why it matters for play:** Compile and clippy time, lib test time, and readers mistaking dish code for game code.
- **Smallest next step:**
  1. Move `End` and `Rng` into a small `mould::core`.
  2. Put the dishes behind a `dishes` cargo feature, tested in a separate CI job.
  3. Move the twin tests to `src/gpu/twins.rs`.
- **Impact:** 2 · **Effort:** M · **Owner:** unowned

### 13. Gun upgrades are applied on two paths with copied constants
- **Kind:** issue (two code paths, one mechanic)
- **Evidence:**
  - `Run::rules` (`run.rs:517-518`: `shot_every *= 0.85^l`, `range += 3l`) and `Run::armed` (`run.rs:552-556`: the same constants on the `Gun`). The bug check's fix 4 added the second path.
  - The player AI still prices fire from `rules.shot_every` (`larder_world_ai.rs:241`), not from the gun.
- **Why it matters for play:** The next upgrade retune will change one path and not the other, and the AI already misjudges a rolled gun.
- **Smallest next step:** Write one `Upgrades::gun(&Gun) -> Gun`, read only that, and drop `shot_every` and `range` from `Rules` once the squad's gun is always set.
- **Impact:** 2 · **Effort:** S · **Owner:** gun thread

### 14. Real play leaves no record the threads can read
- **Kind:** direction
- **Evidence:**
  - The window prints frame stats to the console only (`main.rs:1341`).
  - Replays leave out tools (`main.rs:616`) and cannot reproduce across machines: card atomics and platform libm (HANDOFF decision 6).
  - The owner's 08:30Z notes are the only human-play evidence the project has.
- **Why it matters for play:** "How he wins" (40 soldiers, split to flank) is exactly what a play log would show in numbers, on every playtest, at no cost to him.
- **Smallest next step:** Write `larder-world-log-{map}-{seed}.txt` beside the replay, gitignored: once a game second, the headless t line plus the commands given plus frame p50 and p95. Add a menu line telling the owner where it is, so he can drop it in the thread.
- **Impact:** 4 · **Effort:** S · **Owner:** unowned

### 15. 49 storage bindings in one layout shared by about 60 pipelines
- **Kind:** roadmap / owner decision (platforms)
- **Evidence:**
  - `engine.wgsl` has 49 `@binding`s. `gpu/mod.rs:1190` builds one bind-group layout for every engine pipeline. The view has 27 (`view.rs:224`).
  - Devices are requested with `required_limits: adapter.limits()` (`events.rs:25`).
  - CI's macOS check builds only `organ` and `mould` (`ci.yml`).
- **Why it matters for play:** Every new field adds a binding. On Metal, a stage takes about 31 buffers (guess, from wgpu's Metal limits), so the game probably cannot create its device on a Mac. If shipping beyond Windows matters, this grows harder with every feature.
- **Smallest next step:** Ask the owner which platforms the game targets. If more than Windows, add `cargo check --bin larder-world --target aarch64-apple-darwin` to CI, and pack the per-cell f32 fields into one buffer with offsets.
- **Impact:** 2 · **Effort:** S (ask) / L (pack) · **Owner:** owner decision

### 16. The condensed slime counts as shrinking toward the run's goal
- **Kind:** issue
- **Evidence:**
  - `Goal::Shrink` compares `g.mould` (living walker slots) with its peak (`campaign.rs:459-465`).
  - A condensed walker counts once but is two walkers of mass (`big.rs:2016-2020` `mass`). The kit measured the count roughly halving with condense on.
  - Condense is a menu option (`menu.rs:86`), off by default.
- **Why it matters for play:** With condense on, up to half of the "cut it to 2%" progress comes from the slime clumping up, not from the player's fire.
- **Smallest next step:** Make `World::living` return `mass()` for the goal and the HUD, or add a `World::mass` and use it in `Goal::Shrink`.
- **Impact:** 2 · **Effort:** S · **Owner:** run thread

### 17. The run's slime starves itself to about 28% in 30 s; a cost note in the kit misleads
- **Kind:** issue (evidence for the enemy review) + low-hanging
- **Evidence:**
  - `run=1:3 mode=squads hand=squads`: mould 7,016 at 4 s, 1,973 at 30 s, with 31 killed (`run3-squads-004s-all.jpg`, `run3-squads-030s-all.jpg`). The body shrinks onto its home crystal. That matches the owner's "just hang at their home crystal".
  - Separately, the headless `CPU: ground field N ms (once a game second)` line times the whole-field rebuild (`headless.rs:419-431`). Since the gather refresh (`draw.rs:257+`), the window does that rebuild only when a world is built. `kit.md` quotes "19.5 ms" for planet 8 from that line.
- **Why it matters for play:** The first point is the enemy review's to act on: the slime gives up its pressure by itself. The second leads readers to think late planets hitch.
- **Smallest next step:** Tag the first for the enemy review. Change the headless line to time the live refresh path.
- **Impact:** 2 · **Effort:** S · **Owner:** enemy review / unowned

### 18. Enemy progression is a fixed ladder; nothing scales with the player's power
- **Kind:** direction (game design, in scope)
- **Evidence:**
  - Per depth, the run sets only a `LADDER` tier and walkers = (1 + `WALKERS_PER_DEPTH`·(d−1))·area (`run.rs:486-508`).
  - Meanwhile the player's upgrades multiply hp, strike, rate, range and squad size (`run.rs:512-530`), and guns roll bigger budgets deeper (decision 11-14 16:46Z).
- **Why it matters for play:** "Especially when your guns get stronger": the slime's challenge cannot respond to a strong gun.
- **Smallest next step:** Make a per-planet pressure table (data, beside `LADDER`) that sets the tier keys from depth plus the carried gun's grade. First check: planet 4 with a graded-high gun against a standard one, 4 paired seeds, time-to-goal.
- **Impact:** 4 · **Effort:** S-M · **Owner:** run thread, with the enemy review

### 19. A pump that smells like food already exists, switched off: a twin-free reason to defend
- **Kind:** low-hanging (game rule; tag for the enemy review)
- **Evidence:** `BuildRules::pump_smell` defaults to 0.0 (`buildings.rs:197, 265`). When it is non-zero, a working pump lays scent through the squad frame (`buildings.rs:572`), which both engines already run.
- **Why it matters for play:** "They don't attack my pumping crystals so no need to defend." This makes pumps bait at no engine cost, so turrets and walls get a job.
- **Smallest next step:** Set `pump_smell` to 0.5 × depth / 4 and film planet 3 at 90 s with two pumps, against 0. Judge by look, then playtest.
- **Impact:** 4 · **Effort:** S · **Owner:** pump-economy thread, with the enemy review

### 20. Unit and shot caps fail silently
- **Kind:** issue
- **Evidence:**
  - `squad.rs:542,585` drop units past `UNITS_MAX` 96 without a word. `buildings.rs:30` still says 64.
  - Bullets past 256 are dropped and counted (`bullets_dropped`) but not shown anywhere.
  - At 40 soldiers with a 6-bullet scattergun, one volley is 240 bullets.
- **Why it matters for play:** Shots vanish exactly at the most satisfying moment.
- **Smallest next step:** Print `bullets_dropped` and units over the cap in the headless t line. Raise `SHOTS_MAX` to 1024 after finding 2. Fix the stale comment.
- **Impact:** 2 · **Effort:** S · **Owner:** gun thread
