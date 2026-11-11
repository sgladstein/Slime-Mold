# Larder: build plan for the RTS demo (2026-11-11)

Repo copy of the Claude Doc https://claude.ai/artifact/5ricncRubPz7WTBGJZFnHs,
as written at 04:05Z, before the independent reviews. The reviews and the
revised plan follow in this folder (`larder-build-plan-2026-11-11/`).

## Summary

Build Larder as a native RTS on today's CPU dish in six milestones, each closed by a census before anyone plays it. You command a squad that starts at 4 soldiers and can grow to 40; they burn the mould off food piles, hold lines of fire and carry food home, and the mould hunts and kills them. The mould wants the same food, and its pull on your larder grows as you hoard.

- **Route:** CPU first, in the existing native window (`src/bin/mould.rs`, winit and pixels). The GPU port waits for S4 (game rules on the GPU) and is a later milestone, not a blocker.
- **The core change from the tests:** fire is priced per cell per second and the mould must answer the squad, so that plans counter each other instead of forming a ladder.
- **First playable:** milestone M3, the squad with move, burn line and clear-pile orders on the slowed field.
- **Owner's rulings:** at 03:38Z, the squad starts at 4 and grows to 40, soldiers die to the mould, and the round ends when either side is wiped out. At 04:04Z: food stops arriving late in the round, and a 15-game-minute cap ends any stalemate on share of the map held.

## What the tests settled

Four things are settled on 16 paired seeds; two are still open. All runs are from `examples/larderprobe.rs`, written up in `Reports/mould-shooter-2026-11-11.md`.

| Finding | Evidence | What it means for the build |
| --- | --- | --- |
| A pile feeds the mould sitting on it, so cutting the supply line does nothing | Ring raised late: 0.5% less stolen; firing on the larder: 90% less | The core verb is clearing mould off a pile and keeping it off |
| A held ring of fire stops all theft (after the wound fix) | 0 stolen on 16 of 16; idle loses 12,301 | Fire must be priced, or the ring is the only play |
| Priced fire makes a ladder: ring while the larder is full, then mow | Best on 16 of 16 at shot prices 10 and 40; idle best at 120 | Price works as a lever; one plan still wins because the mould never reacts |
| The mould must be slowed to be fought | At 120 steps a second it crosses the field in about 4 s; at 15 steps a game second the first bite comes at about 32 s | The demo runs the mould slowed, and fronts become visible and shootable |
| Fed mould forms loops and starved mould forms trees | 10 loops against 1 at 45 s, 16 of 16 | Starving the mould changes its shape: a real lever, not tested in play yet |
| A hoard's smell draws the mould sooner | First at 4.1 s against 4.6 s, 16 of 16 | Real but tiny at full speed; untested on the slowed dish |

**Still open:** no test has a mould that answers the squad, and no test has soldiers with a position, so range, travel and getting engulfed are all untried.

## The demo

A round on the 23-pile field ends when every soldier is dead or all the mould is dead, or at a 15-minute cap on share of the map held; food stops arriving late in the round to force a finish. You start on the right home pile, the mould on the left. The result is a spread, not a yes or no: who was wiped out, how long it took, and how many soldiers were left. Every number below is a starting value for the census to tune, not a ruling.

**Map.** The skirmish field (`skirmish::HOMES`, 23 piles), with the mould slowed to 15 steps a game second so a front takes about 30 s to cross. Your larder is the right home pile. Piles you hold are those with a soldier within range and no mould eating on them.

**Squad.** 4 soldiers to start, up to 40 bought with food. Each has a position, health, a walk of about 8 cells a second and a range of about 30 cells. Soldiers smell like food to the mould, so it grows toward them and attacks: every walker touching a soldier drains its health. A dead soldier becomes a food pile the mould smells at once, so losses feed the enemy.

**Orders** (all RTS-style, shown in Controls below):

1. **Move:** walk to a point.
2. **Burn line:** hold a drawn line or ring of fire. Each soldier keeps up to 12 cells of line burning, using the snip with the 2026-11-11 wound fix. It costs food per cell per second, shown while you draw.
3. **Clear pile:** fire at every walker on one pile until it is bare, then keep it bare.
4. **Free fire:** shoot the densest mould within range. This is the default when idle, and it costs per shot.
5. **Carry:** a soldier on a held pile harvests about 30 food a second and walks it to the larder. Food is banked only at the larder.

**Economy.** Spend the bank on fire (per cell per second), more soldiers (up to 40), and two sidegrade upgrades a round: a burner (wider, shorter, costly fire) or a long gun (narrow, long range). The larder has a smell that grows with the bank, so hoarding draws the mould harder.

**The mould's answers**, which are what make plans counter each other:

- **Larder smell:** a full bank pulls fronts to the larder, so a big hoard needs a ring.
- **Starving mould fights harder:** a mould that has eaten little in the last 30 s spreads faster and sends thin, fast tree branches; a fed one grows slow, thick loops that are cheap to ring but slow to burn through.
- **Engulfed soldiers are food,** so walking a squad into a mesh feeds it.
- **Timed drops:** new food lands every 60 s, with its smell spreading 10 s before it lands, so both sides race for it.

**Four plans the census must keep alive:** Fortress (ring the larder, bank everything), Hunter (spend on fire, clear piles, starve the mould), Farmer (hold three piles, carry food, let the mould stay fat and slow) and Baiter (feed a decoy pile to pull fronts away). The intended counters: Hunter starves the mould into fast branches that slip past a Farmer's lines; Fortress beats Hunter by not paying for fire; a starving mould beats Fortress by rushing the hoard's smell; Baiter beats a fat mould but loses to a starving one.

## Engine route

Build on the CPU dish now and port to the GPU after S4. The browser is out for this demo (owner, 00:30Z).

| | CPU dish now | GPU later |
| --- | --- | --- |
| Size | 256 x 160 field, a few thousand walkers | 1024 x 640, 100,000 walkers (1.45 ms a frame on the GTX 1070) |
| Game rules | All there: piles, `pieces`, wounds, beacons, the snip | Not yet: S4 puts the game rules on the GPU |
| Window | `src/bin/mould.rs` (winit and pixels) already runs dishes | `src/bin/mouldgpu.rs` is a sandbox with no game layer |
| Census | `larderprobe` runs 16 seeds in minutes in the cloud | Needs the home PC through Remote Control |

The squad, orders and economy sit in a new module, `src/mould/larder.rs`, that calls the dish only through `Dish` methods (step, stroke, pile food, walker positions). That boundary is what lets the same game layer drive the GPU dish once S4 exposes the same calls.

## Milestones

Six milestones; M1 and M2 are harness work with no window, M3 is the first thing you play, and each one closes on a gate measured before the next starts. M6, the GPU port, waits on S4.

| # | Milestone | Code changes and files | Tests | Gate to pass |
| --- | --- | --- | --- | --- |
| M1 | Soldiers in the harness | New `src/mould/larder.rs`: `Soldier` (position, range, cooldown), `Order`, engulfing, carrying food; `larderprobe` drives it with scripted plans | Unit tests for engulf, range and carry; census of idle, random and the four plans on 16 seeds | Every plan beats random on 16 of 16; idle loses the larder; a soldier left in the mould's path dies, and its body is eaten |
| M2 | Priced fire and the mould's answers | Fire priced per cell per second; larder smell grows with the bank (via `Dish::beacons`); starving mould spreads faster; engulfed soldiers become piles; timed drops with early smell | Census of the four plans against each other on 16 seeds, for each answer switched off in turn | No plan is best on more than 12 of 16 seeds, and at least one counter pair holds (A beats B, B beats C, C beats A) |
| M3 | First playable | Native window: a `larder` entry in `src/bin/mould.rs`; box select, right-click move, drawn burn lines, clear pile, free fire; price shown while drawing | Frame cost per step quoted; a recorded scripted game replays to the same bank | Frame under 10 ms with 40 soldiers on the owner's machine; owner playtest |
| M4 | Economy and upgrades | Buying soldiers up to 40, burner and long gun; bank and income shown on screen | Census with upgrades bought by each plan | Each upgrade is bought by the best plan on some seeds and not on others |
| M5 | Round shape and polish | Phases (quiet start, first drop, rush), round end screen with counts, sound cues from `src/sound` | Owner playtest of 3 rounds | Owner says at least two plans felt different and both could win |
| M6 | GPU port | Game layer on the GPU dish once S4 lands, on the 1024 x 640 map | Twin test of the game layer, CPU against GPU, on one tick | Same measures as M2 within noise after correction; GPU frame on the GTX 1070 under 16 ms |

## Controls and screen

You give orders; soldiers aim for themselves. There is no manual aiming of single shots except one key.

| Input | What it does |
| --- | --- |
| Left drag | Box select soldiers; left click selects one; Shift adds |
| Right click on ground | Move there |
| Right click on a pile | Clear that pile, then hold it and carry its food home |
| F, then drag | Draw a burn line; the price per second shows by the cursor while drawing; a closed loop makes a ring |
| G, then click | One direct shot at that point, for cutting a branch now |
| Number keys 1 to 4 | Recall a control group; Ctrl and a number sets one |
| Space | Pause and give orders while paused |
| Delete | Cancel the selected soldiers' line |

**What is drawn.** Everything on screen is the simulation. The mould is drawn from its trail, exactly as in the other dishes. Fire is the wound itself: burnt ground, not a painted effect. Soldiers are small marks with a faint range circle when selected. The bank, income per second, fire cost per second and the clock sit in the existing HUD (`src/hud.rs`).

## How each milestone is judged

- **The deciding measure** is the outcome, named before each census run: which side is wiped out, at what time, and with how many soldiers left.
- **Controls:** every census includes an idle squad and a random squad.
- **Paired seeds:** 16 seeds, the same for every plan; paired counts with a sign-test p, Holm correction.
- **Counters, not a ladder:** M2 and M4 report a table of each plan against each other plan.
- **Look first:** each census comes with pictures on an Artifact page with the counts beside each picture, saved under `Reports/`.
- **Frame cost:** anything that runs every step has its cost quoted.
- **Owner playtests** from M3 on.

## Risks

| Risk | Sign it is happening | Fallback |
| --- | --- | --- |
| One plan still dominates after M2 | A plan best on 13 or more of 16 seeds | Give the mould one more answer (for example, it routes around burning lines) before building the window |
| The slowed mould looks wrong or dull | It reads as frozen in the M3 playtest | Raise the speed and enlarge the map instead, which pulls M6 forward |
| Soldiers make the CPU frame too slow | Frame over 10 ms with 40 soldiers | Soldiers fire every 4th step, as the harness does now |
| Burn lines are too strong or too weak at any price | The best price in the census is 0 or over 120 | Tune the line length per soldier rather than the price |
| S4 slips | No GPU game rules by the end of M5 | Ship M5 on the CPU; the port stays a separate milestone |

## Revised after three independent reviews (04:30Z)

Reviews and the prompts given: [prompts](larder-build-plan-2026-11-11/reviews/prompts.md), [engineering](larder-build-plan-2026-11-11/reviews/engineering.md), [design](larder-build-plan-2026-11-11/reviews/design.md), [method](larder-build-plan-2026-11-11/reviews/method.md). All three found the M2 counter-cycle gate impossible: on one seed, plan scores form a straight order, so A>B, B>C, C>A can't all hold, and at least one link holds on 10 of 16 seeds or fewer (p >= 0.45). Changes taken:

| Change | Why (reviewer) |
| --- | --- |
| M2 gate is a crossover: under one visible condition A beats B on 15+ of 16, under another B beats A; a reader plan that picks from what it sees beats every fixed plan | All three |
| A bait order (spend food to drop a pile) and one structure, a turret | Design: Baiter had no verb; the 00:18Z brief asked for defences |
| The mould's home pile is its core: burnt bare, the mould stops dividing | Design: piles regrow the mould, so the cap would decide most rounds |
| Soldiers 1.5-2x a front's speed, range 15 cells, 6 cells of line each | Design: at 8 cells/s no retreat; at 30 cells 40 soldiers cover the field 2.7 times |
| Default stance defend; stances and attack-move | Design, method: free fire spends the bank to nothing |
| One ordered score: win +(1 + left/40 + (900-T)/900), cap share held - 0.5, wiped -(1 + (900-T)/900) | Method |
| Plans are fixed rules with named settings and equal tuning; tune on seeds 1-16, gate on 17-32; controls idle and spend-matched random | Method |
| M1 owns game time, round end, death to pile, stable pile ids, a fast public `fire` | Engineering |
| Rough window right after M1, in `src/bin/larder.rs` | Design, engineering |
| Buying soldiers in M2; M2 gate re-run after M4 | Engineering |
| Scent-field view toggle; no painted shot flash | Design, engineering |

Kept against advice: the 15-minute cap (owner's ruling, 04:04Z); the census reports how rounds end so it can be revisited.

### Revised milestones

| # | Milestone | Code | Gate |
| --- | --- | --- | --- |
| M1 | Soldiers and rules in the harness | `src/mould/larder.rs`: game time, `Dish::fire`, stable pile ids, soldiers (health, speed, range, scent), death to a pile, round end, the score; six plans as fixed rules; look renders first | Soldier layer < 0.1 ms a step; every plan beats idle and spend-matched random on 15+/16 held-out seeds after Holm; no score band holds 14+/16 runs; a front travels 10 s+ before contact on 14+/16; how rounds end reported |
| M1b | Rough window | `src/bin/larder.rs`: marks, move and burn, fractional-step clock | Owner looks |
| M2 | The mould answers, and spending | Price per cell-second, bait, turret, buying soldiers to 40; larder smell, starving spreads faster, drop smell (dish settings, off by default, hashed) | Crossover 15+/16 each way after Holm; reader plan beats every fixed plan; each answer switched off changes the winner somewhere or is cut |
| M3 | Full controls | Box select, right-click, stances, priced burn lines, groups, pause, scent view; replay to the same hash | Frame < 10 ms with 40 soldiers on Scott-Desktop; owner playtest |
| M4 | Upgrades | Burner, long gun | Each helps one plan and not another on paired seeds; M2 gate re-run |
| M5 | Round shape and polish | Phases, end screen, sound | Owner plays 3+ rounds; three plans each win one |
| M6 | GPU port | Game layer behind a trait; GPU dish after S4 | Twin test at 256 x 160 within a named bound |
