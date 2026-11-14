# Larder campaign and onboarding: the design (2026-11-12)

> **Approved by the owner, 2026-11-12 04:44Z**, on the thread's card ("Approve the campaign plan, and what carries between missions?"): "Approve; carry unlocked verbs only". Nothing below is built or measured yet; every number is a first guess for the census to set. Live doc: https://claude.ai/artifact/5zijLBLkgoZNKcaX1ZTrYz (this file is its repo copy).

Asked for by the game designer review (2026-11-12 04:31Z) after the owner's first Larder World playtest (04:13Z), which ended in a soft lock: the bank was empty, shots cost food, and the mould killed anyone sent to harvest. The owner approved the review's new threads at 04:35Z.

## Summary

The campaign is eight missions on the five v2 maps ([maps](larder-maps-2026-11-11.md)). Each mission puts one mould tier against one new verb, so the player learns one thing at a time and the mould climbs the [tier ladder](../wiki/world/mould-tiers.md). Every mission shows one goal with a progress bar, ends in one to three stars, and lasts 8 to 15 minutes.

- **One verb, one tier, one map per mission.** The census checks that the plan with the verb beats the same plan without it.
- **The goal is a sentence and a bar**, from numbers the game already tracks (food banked, charge taken, mould walkers, seconds a crystal is held). `Game::score` stays for tests only.
- **No soft lock.** While the player has a soldier, a move exists that can raise the bar; if none exists for 30 s, the mission ends and says why.
- **Dead time is measured**, with a ceiling per mission.

Choices, all as recommended: only unlocked verbs carry between missions (the persistent world, W4, is its own mode after the campaign; the owner's 11-11 14:06Z "A longer persistent world. Maybe a campaign" left this open); the 15:00 cap is a graded result; the first mission teaches by first-time hints, not a scripted tutorial.

## The missions

| # | Mission | Map | Mould | New verb | Goal on screen [bar] | Needs built first |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Landfall | Flats | Drifter (slow, grazing) | Harvest and hire | "Grow your squad to 8 soldiers" [squad size] | Drifter as a built tier (`graze_weight` exists; mould tiers thread) |
| 2 | First contact | Fork | Drifter | Fire and stances | "Clear the west fork crystal and hold it for 60 s" [seconds held with no walker on it] | Free shots; soldiers smell like prey (Larder thread) |
| 3 | The gaps | Rim | Sprawler (fast, thin) | Walls | "Keep the mould inside the Rim until 10:00" [time with no walker outside the ring] | Sprawler as a built tier (`explore_spread` exists); soldiers build walls (controls thread) |
| 4 | Cut the cord | Fork | Trunk (thick, heals) | Cutting a tube | "Starve the Trunk: cut its cords until it is a fifth of its size" (built 2026-11-14) [walkers now against its peak] | Trunk as a built tier; one body: cut pieces starve (Larder thread) |
| 5 | Kill zone | Flats | Swarmer (floods where hit) | Turret | "Drive the swarm off the three middle crystals for a minute" [seconds all three are clear, once the swarm has been on them] (built 2026-11-14) | Turrets (buildings batch 1) |
| 6 | The nest | Nest | Sleeper (dormant crust) | Strike | "Break the nest: cut the Sleeper to half its size" [walkers now against its peak] (built 2026-11-14) | Sleeper and strike in the GPU game |
| 7 | Spore season | Rim | Sporer (spores fly to crystals) | Pump | "Keep the spores off your landing crystal for a minute" [seconds it is clear] (built 2026-11-14; was "Hold 6 of the 8 outer crystals", then a held charge race) | Pumps (buildings batch 1) |
| 8 | Two moulds | Passes | Rival pair (sand fast, clay slow) | Lure | "Take 60% of the valley's charge" [take share] | Lure (buildings batch 1); the rivals as two species |

Missions 1 to 3 are the first playable slice. Mission 4 is the owner's "cut-off hunk" (11-12 04:18Z) made into a lesson. Hunter, Converter and Mixes are a second chapter once they run in the world; the squad toolkit's verbs slot in as missions against the tier each counters. Mission 5's goal was a kill count until the design review pointed out that free shots and turrets reward turtling for it.

## The goal on screen

- **Stars, not a score.** One star: the goal is met, or the player stands at the cap with the bar past half. Two: met before 12:00. Three: met with a bonus the mission names ("lose no more than 2 soldiers"), never a time, so the climax stays open.
- **Markers are UI**, from the existing marks system; the mould and ground stay pure simulation.
- **First-time hints** when the simulation first does something (the mould smells a soldier, a crystal runs dry, a wall wears, a piece is cut off), once per campaign, naming the verb that answers it.
- **A loss names its cause**, read from the game, with the bar's best value.

## Win, lose and pacing over 15 minutes

Four phases, set by the map's crystals and the mould's start, not by scripted waves:

1. **Landing, 0:00-2:00**: the mould is out of reach; harvest a safe crystal, hire. Check: no contact before 2:00.
2. **First contact, 2:00-6:00**: the mould smells the squad and the tier shows its trick. Check: the trick is seen by 6:00.
3. **Pressure, 6:00-11:00**: the mould is near full size and home crystals run low; the verb wins or loses it. Check: the idle hand is losing by 11:00 (owner, 11-11 14:54Z: "If you do nothing, the slime should get much stronger than you and kill you").
4. **Climax, 11:00-15:00**: dry crystals force a move; the mould pushes on the larder. The cap grades by the bar.

**Win:** the bar fills; the mission ends at once. **Lose:** the last soldier dies, or the mould holds the larder with the bank empty for 30 s, or the stuck backstop ends it.

## No soft lock

In every state, either the player has a move that can raise the bar, or the mission ends within 30 s and says why.

1. **The basic gun is free** (the Larder thread's free shots). Food buys more, never permission to fight.
2. **A home crystal** within a minute's walk of the landing that the mould reaches last (census: no walker on it before 6:00 in any idle seed).
3. **A reserve recruit**: with fewer than 2 soldiers and less than one soldier's price banked, the larder makes a free soldier every 90 s while the mould does not hold it.
4. **The stuck backstop**: 30 s of stuck in a row ends the mission with its cause and the stars so far.

## Dead time

A game second is dead when nothing the player owns does anything useful: no food harvested, no walker killed by the squad, no wall or building gaining health, no purchase, no soldier walking an order toward its goal, and no move of the mission bar.

| Kind | When | Fix |
| --- | --- | --- |
| Stuck | No affordable or reachable move could raise the bar | The rules above |
| Waiting | A useful move exists, but the hand's plan has nothing worth doing | The map or the mission's numbers |

The Larder thread adds the stuck count to the census; waiting shares its definition of useful. Targets on the intended plan's hand: dead share under 15% and no phase above 25%; longest dead run under 45 s; no stuck run over 30 s; **decided at** (the earliest minute after which the stars no longer change, in 14 of 16 seeds) after 9:00. The idle hand is the control.

## How a world ends, and what carries over

HANDOFF open decision 8, for the campaign: a mission world ends on its goal, on a loss, or at 15:00; crystals running dry drive the climax rather than end the world. Only unlocked verbs carry (owner, 04:44Z). Stars unlock nothing required; later they may unlock challenge variants (a map against the next tier).

## Gate and build order

Each mission ships only when, on 16 paired seeds (sign tests, Holm across the missions): idle and random lose, and random meets the goal on at most 2 seeds; the intended plan meets the goal with a median between 6:00 and 12:00; it beats the same plan without the new verb, and from mission 4 at least two distinct hands meet the goal (the new verb's best, not the only answer); the dead-time targets hold; and each mission is rendered at 0:00, 3:00, 6:00 and 11:00 and looked at first.

Build order: (1) a mission layer over `larder_world::Game`, as data (a mission's lines in the map's `.txt`: tier, goal, numbers, verbs allowed), no engine change; new end conditions go through the Larder thread, who owns `End` and `Game::score`, in the integration thread's merge queue; (2) missions 1 to 3 with the gate; (3) missions 4 to 8 as their pieces land.

## The mission layer, built (2026-11-12)

`mould::campaign` reads a mission file (`larder-campaign-2026-11-12/missions/*.txt`: name, goal sentence, map, tier, verbs, `rule` keys, bonus, one goal) and follows a running `larder_world::Game` once a game second: the bar, its best, the verdict (goal, cap, squad wiped, overrun, stuck, mould wiped) with stars, the line a mission ends on, and dead time by phase with the longest dead and stuck runs and "decided at". It never changes the game: the verdict is its own, and `End` and `Game::score` stay the Larder thread's. A mission names its map rather than adding lines to the map's `.txt`, because `map::Items::parse` refuses unknown lines and one map serves several missions.

- **Window:** `cargo run --release --bin larder-world -- mission=1-landfall` plays a mission's map and rules, shows its goal sentence and bar percentage as the HUD objective, and ends on the mission's screen ("Mission lost", the cause line). A tier name not yet built (drifter, sprawler, trunk) plays the plain mould and says so.
- **Census:** `cargo run --release --example campaign -- mission=NAME seeds=1-8` plays idle, random and the player AI's styles on the CPU big dish and prints each round's verdict, stars, best, dead time by phase, longest dead and stuck runs, and decided-at.
- **Positional goals on the GPU:** missions 2 and 3 (clear a crystal, keep the mould in the Rim) read `World::mould_grid`, which the GPU world does not give yet (it needs an occupancy read-back). In the window their bar is not shown and they end only on the other causes; the CPU census measures them.
- **Stuck** mirrors the Larder thread's census definition (a soldier alive, nothing done, a bank below a soldier's price) with a priced shot; with free shots a squad is never stuck. It will read `Game::stuck_run` once the Larder thread's PR lands, with `Rules::overrun` and `Rules::reserve` replacing this layer's overrun check.
- **Mission 1's goal raised from 8 to 24 soldiers:** on the Flats the AI's boom style reached 8 soldiers by 1:28 on seeds 1 and 2 (three stars both), far under the design's 6-12 minutes. 24 is the next guess; the census sets it.

## Missions 1-3 tuned by the census (2026-11-12)

Four censuses (`examples/campaign.rs`, seeds 1-4; idle, random and the player AI's boom, claim, late, fort and strike; CPU big dish, scale 2, 5,000 walkers; the plain mould, since the Drifter and Sprawler are not built; raw in [`census/`](larder-campaign-2026-11-12/census/), v1 to v4). Four seeds is a tuning run, not a gate: no sign test is quoted.

| Mission | Goal tried | What the census found | Now |
| --- | --- | --- | --- |
| 1 Landfall | 8 soldiers | Every AI style met it at 1:28 (3 stars) | — |
| | 24 soldiers | No hand met it: the AI's styles stop at 20 (best 0.73-0.95) | — |
| | 8 soldiers on 2 crystals at once (the design review's idea) | Every style at 1:28-1:29 | — |
| | **12 soldiers on 3 crystals** | **Every AI style at 2:31-2:48, 3 stars (20 of 20); idle and random 0 of 8** | kept: short, as a first mission should be; retune once the Drifter is built |
| 2 First contact | clear the west crystal 60 s | Idle's bar reached 0.22-0.87: the crystal was clear before the mould came | — |
| | **retake** it (counts only once the mould has been on it) | **AI 20 of 20 at 1:42-5:31, 3 stars; idle 0 of 4 (best 0.03-0.87); random 1 of 4** | kept |
| 3 The gaps | no walker outside the Rim for 10 min | No hand held it for a second (best 0.00-0.05) | — |
| | at most 2% outside | 1 of 20 AI rounds; planned hands peaked at 3.8-5.8% outside, idle at 17-33% | — |
| | **at most 5% outside** | **AI 18 of 20 at 10:00-11:41, 2 stars; idle and random 0 of 8** | kept |

Dead time on the planned hands is 0-9% (the landing's first seconds), idle's 64-96%; no hand was ever stuck (free shots). The new stall measure (bar flat, mould not smaller than 30 s ago, nothing banked) runs 7-17 s at most on planned hands and 105-284 s on idle and random.

**Open, by the design's own gate:**
- **Mission 3 does not yet teach walls.** Boom, claim, late and strike, which build no walls, meet it as fort does, so gate item 3 (the verb's plan beats the plan without it) fails. It needs either a Sprawler that leaks through open gaps, or a smaller leak with walls in the plan; it is measured against the plain mould today.
- Every number here was set on the plain mould and before the Larder thread's flee default (its PR 1: the mould turns from fresh gunfire, and a firing squad is far harder to overrun), so all three missions are re-run when that merges.
- **The positional goals (missions 2 and 3) are CPU-only for now:** the window passes `World::mould_grid`, which the GPU world gives once the GPU thread's PR #101 merges; until then the window says "progress not shown in this build yet".
- Mission 1's 2:31-2:48 is under the design's 6-12 minutes; the AI acts at once, so a first-time player will be slower. Checked by hand once the owner plays it.

**Changes from the reviews (05:38-05:43Z):** kills count as useful only while the mould is smaller than 30 s ago (firing at a mould that regrows is not progress); the stall measure above; the window says when it can't show a bar rather than hiding it; one test per goal kind on a small CPU world (clear and retake, contain, take, stuck, squad with crystals; 10 tests in all). `take`'s bar now starts from an even share: it showed 83% of a 60% goal at 0:00. On the race world an idle mould halves by itself in the first 40 s as walkers starve, so mission 4's `shrink` numbers must allow for that.

### Pictures (page)

[Campaign Missions 1-3](https://claude.ai/artifact/Pu2j5ComXhUPTAMv4MyMRJ), copy in
`larder-campaign-2026-11-12/page/`: the game window (GPU world, software
Vulkan) playing each mission with the claim hand, seed 1, counts under each
frame. Landfall met at 1:21 (3 stars, 12 soldiers, 3906 harvested). On the Rim
a mould trail leaves through the north-east gap by 5:00 and reaches the
larder. The GPU census (GPU thread, `campaign gpu=both`, claim, seeds 1-4,
after #101) holds seed 1 on the GPU too (peak share outside 0.050 at 696 s,
against the CPU's 0.058 at 701 s), so the trail the player sees sits inside
the 5% allowance. That is the problem: a leak must mean what the player sees
(a trail out of a gap) before the window shows this bar. Seed 2 failed on
the GPU (peak 0.168, Cap) where the CPU held; four seeds can't call a bias
(sign p 0.63), so the 32-seed `gpu=both` run decides it.

### What the player sees outside the Rim (census v5)

Question (design review, 06:26Z): on the Rim the window shows a mould trail
out of the north-east gap by 5:00 while the goal counts as held; what does a
leak have to mean for the bar and the picture to agree?

Measure: each second, the 4 x 4-cell bins outside the Rim holding at least
1, 4 and 16 walkers (`census/3-the-gaps-s1-4-v5.txt`, CPU, seeds 1-4,
seven hands; the last columns), and the same in the GPU window on seed 1
every 30 s (`census/3-the-gaps-claim-s1-gpu-window-bins.txt`, from a
throwaway print in the window, not committed).

| | bins >= 1 (most) | bins >= 4 (most) | bins >= 16 (most) | s with a bin of 4+ outside |
|---|---|---|---|---|
| planned hands that held (18 rounds) | 166-239 | 5-17 | 0 | 148-431 |
| planned hands that hit the cap (2) | 231-269 | 83-85 | 38-42 | 673-869 |
| idle (4) | 295-438 | 85-220 | 40-110 | 883-892 |
| random (4, ended by 6:06) | 277-432 | 19-105 | 0-44 | 201-349 |
| GPU window, claim, seed 1, 0:30-7:00 | 92-226 | 0-5 | 0 | |

The trail in the picture is thin: in the window at 5:00, 153 walkers (2.4%)
outside over 128 bins, at most 3 in any bin. The drawn body is trail times
`1 - exp(-walkers in the 5 x 5 round / 6)`, so a few walkers on a strong trail
draw a clear tendril. Nothing is wrong with the count; the goal allows 5% and
the player is not told. No hand keeps every walker in (leak 0 held for no
second, v1), so "no visible mould outside" can't be the goal on the plain
mould. A dense body outside (a bin of 16+) separates the winners (0) from
idle (40-110) as well as the share does.

Change: the goal line now says the share outside against the allowance,
for example "Keep the mould inside the Rim for 10:00 (50%; 2.4% outside, 5% allowed)"
(`Tracker::progress_line`, used by the window); the sentence says "for", as
the hold restarts when the share passes 5%. Whether a dense-body rule plays
better than the share is for the Sprawler re-tune, where the mould moves
differently.

### Missions on the Game's own no-soft-lock rules (after Larder PR #106)

The Larder thread's #106 put the design's no-soft-lock rules in the Game:
`Rules::overrun` (the larder held with the bank under 1 for N s ends the
round, `End::Overrun`), `Rules::reserve` (a free recruit every N s while
fewer than two soldiers live and the bank can't buy one), and
`Game::stuck_run`. A mission now turns on overrun 30 s and reserve 20 s
(`campaign::OVERRUN_END`, `RESERVE`; a mission file can set `rule overrun`
or `rule reserve`), and the tracker reads the Game's overrun end and stuck
count instead of its own. A wiped squad is a loss only when the Game says so
(no recruit can come). Seen in the window test: a rush on Landfall, lost by
2:00 before, now keeps coming back and is not over by 6:40; the test now
caps an idle round at 1:00. The window also gave a recruit made inside a
step no look until the next frame (an index panic); fixed. Missions 1-3 are
re-run once shot craters (Larder PR 2) and the tiers land.

### Missions 1-3 on the real tiers, with craters (census v6-v8)

The tiers thread's #115 made `tier drifter` and `tier sprawler` real (until
then they fell back to the plain mould), and the Larder thread's #113 turned
shot craters on. Census: CPU, seeds 1-8, seven AI hands plus idle and random;
raw files in `larder-campaign-2026-11-12/census/` (v6 and v7 before craters,
v8 on main 71ed0f9 with craters). Deciding measure: goal met by a plan, and
never by idle or random.

| Mission | Hand | v6 (tiers) | v8 (tiers + craters) |
|---|---|---|---|
| 1 Landfall (Drifter) | boom, claim, late, fort, strike | 40/40, 3 stars | 40/40, 3 stars |
| | idle, random | 0/16 | 0/16 |
| 2 First contact, old goal (retake the west fork) | strike | 8/8 (by wiping the mould) | |
| | the other six hands | 0/48, best 0.00 | |
| 2 First contact, new goal (retake its home) | strike | | 8/8, 3 stars, 6:16-6:19 |
| | boom, claim, late, fort; idle, random | | 0/48 |
| 3 The gaps, 5% share rule (Sprawler) | every hand | 0/56, 46% outside | |
| 3 The gaps, thick-mould rule | gaps+claim, gaps+boom, gaps+fort | 24/24 (v7) | 24/24, 2 stars, held at 10:00 |
| | boom, claim, late, fort, strike; idle, random | 0/56 (v7) | 0/56 |

Mission 2: the real Drifter never left its home crystal in 15:00 with any
hand (seeds 1-4, crystals-reached column, v7), so "take back the west fork"
could never start. The goal is now "Drive the mould off its home crystal and
hold it for 60 s" (`retake 1 60 0`). Only strike meets it, by wiping the
mould; holding the home crystal against a living Drifter happened in no
round. A tutorial that only one plan passes is acceptable for the second
mission; the designer's counter-push idea needs a mould that comes back,
which the Drifter doesn't.

Mission 3: the Sprawler leaves the Rim at once (46% of its walkers outside
for every hand), so the 5% share could not hold. The goal now uses the
thick-mould rule (`contain ... 600 1 16`: no 4 x 4 bin outside with 16+
walkers, share off), which the design review preferred. A new census hand,
`gaps+STYLE` (`examples/campaign.rs`), walls the four gaps at the start
(one soldier each, raising what it places) and then plays STYLE: it holds
24 of 24, and every hand that does not wall holds 0 of 56 (paired per seed,
gaps+claim vs claim 8-0, sign p 0.008, and the same for boom and fort; all
survive Holm over the three). So the mission now needs walls, the open item
from the first census. The bank (2000) pays for three of the four walls; the
west gap is left open on most seeds and the hold still comes, so the rule is
not too strict.

Two notes. #122 (soldiers raise buildings over time) made placed walls
ghosts until a builder raises them: the first v8 run of the gaps hand placed
walls and stood beside them, raised nothing, and lost its squad by 1:50 on
8 of 8 (`...-v8-gaps-no-builder.txt`); the hand now gives the soldier
`Order::Build`. The window's goal line says "thick mould outside in N
places" for this rule.

**Smoke test (tech lead, 14:13Z).** Mission 3's result rests on rules other
threads own (wall health and bite, raise time, craters, core defence), so CI
plays the fragile mission on seed 1 at census size
(`campaign::tests::the_gaps_needs_walls_and_landfall_wins`, 11.8 s on four
cores): Landfall claim meets its goal; The gaps with walls has no thick mould
outside and its bar at 0.2+ at 2:20; without walls thick mould is outside by
about 2:00. The first version (#130) also played First contact's strike to
7:00 and took 105 s, too long for every PR (tech lead: under 30 s); it stays
as `each_missions_plan_still_wins`, `#[ignore]`d. The wall plan is in the
library (`campaign::wall_rim_gaps`).

### The gaps and Landfall on the CPU and the 1070 GPU world, 32 seeds (census v9)

The tech lead asked for missions 1-3 on both engines after core defence
(#126). Run on the owner's GTX 1070 at main 9f5e4a4, `gpu=both`, seeds
1-32, hands idle, random, claim and gaps+claim, 70 min wall time (other
work shared the PC, so no timing is quoted). Raw file:
`larder-campaign-2026-11-12/census/3-the-gaps-s1-32-v9-cpu-vs-gpu-1070.txt`.
Deciding measure, named before the run: goal met per engine, and whether
the engines agree seed by seed.

| Hand | CPU met | GPU met | How the others end |
|---|---|---|---|
| gaps+claim | 32/32, 2 stars | 32/32, 2 stars | (held at 10:00 on every seed) |
| claim | 0/32 | 0/32 | squad wiped, CPU median 10:42, GPU 10:17 |
| random | 0/32 | 0/32 | squad wiped (2 stuck), median 3:01 / 2:54 |
| idle | 0/32 | 0/32 | overrun, median 7:36 / 7:38 |

The engines agree on every seed: walls hold 64 of 64 rounds, and no
other hand holds any of 192. Walls against claim, paired per seed, 32-0
on each engine (sign p 5e-10). One difference: the GPU world wipes a
squad that did not wall a little sooner, claim 24-8 and random 19-5
(paired, GPU earlier; sign p 0.007 each, both survive Holm over the three
hands; idle 14-18, p 0.6). The median gap is 25 s for claim and under
10 s for random, which does not change any verdict.

Landfall, the same check (re-run at 7e3a7a3 after the first try ran out
of memory on the shared PC; 20 min; raw file
`...census/1-landfall-s1-32-v9-cpu-vs-gpu-1070.txt`): claim meets the goal
32/32 on each engine, 3 stars, median 1:08 on both (the same second on 31
seeds); idle 0/32 on each (the 30 s stuck backstop ends every round,
median 0:58 / 1:08); random 0/32 on each (stuck 48, capped 15, one wiped;
end times 12-11 paired, p 1). The engines agree on every seed here too.
First contact was not re-run: its goal changes when the Drifter drifts,
and it gets a smoke check then (owner's testing rule, 17:26Z: name the
question, and anything over about an hour needs the owner's OK).

### First contact on the drifting, headless Drifter (census v10-v11)

Tiers #133 made the Drifter drift (hunger scent), #143 made soldiers wade
in thick slime, and #164 (the owner, 20:21Z: no king rule) makes every
mould headless. The old goal, "drive it off its home crystal and hold it
60 s", was met only by strike's wipe. Question for each run: does a goal
that teaches fire get met by a plan, and not by idle or random? CPU,
seeds 1-4, about 20 min a run; raw files in
`larder-campaign-2026-11-12/census/` (v10 on main a56f644 / fc4a718, v11
with #164 merged in locally).

| Goal tried | idle | random | claim | strike | other |
|---|---|---|---|---|---|
| retake home, hold 60 s (old; v10) | 0/4 | 0/4 | 0/4 | 4/4 (wipe) | boom, fort 0/4 |
| retake any crystal it reached, 60 s (v10) | 0/4 | 2/4, by 1:35-2:49 | 2/4, at 14:17-14:53 | 4/4 | boom, fort 0/4 |
| same, a soldier on it (`goal hold`; v10, v11) | 0/4 | 0/4 (v10), 3/4 (v11) | 0/4 | 4/4 | hold+claim, hold+boom 0/4 |
| cut to half its size, from the start (v11) | 4/4 at 0:20 | 4/4 | 4/4 | 4/4 | |
| **wipe it out, bar from its peak after 1:00 (`shrink 0`; v11)** | 0/4, best 0.06 | 2/4 at 10:00-11:49 | 0/4, best 0.33 | 4/4 at 7:17-8:01 | boom 0/4 |

Two things decide it. Since #143 the Drifter leaves home only when
soldiers come close (claim's games: it reached no crystal but its own on
4 of 4 seeds), so any goal about crystals it has taken waits on the
player provoking it, and an AI hand written to stand on such a crystal
(`hold+STYLE`) never got the chance. And a new mould halves by itself in
its first 40 s, so a shrink counted from the start fills for idle. Mission
2 now says "Wipe out the Drifter" (`goal shrink 0`), with the peak counted
from `SHRINK_FROM` (60 s): the bar is the share of the mould cut away,
which spreads the hands (strike 1.00, random 0.86, claim 0.33, idle 0.06).
Random wipes the headless Drifter on 2 of 4 seeds; that is a tuning note
for the Drifter (the tiers thread already calls it gentle), not a veto.
Then the designer (22:58Z): with every mould headless a wipe means
hunting the last stragglers, which the owner called tedious in the shape
D playtest. The goal is now "Break the Drifter: cut it below a tenth of
its size" (`shrink 0.1`), so the mission ends when the fight is won;
strike's goal comes at or before its wipe, and the times above were not
re-measured. On main after the Larder economy PR (#170: rations, the
overrun on) strike no longer breaks the Drifter on seed 1: bar 0.51 at
15:00, two soldiers left (the `#[ignore]`d plan test fails there for
that reason, before and after this change). #174 doubles the bite next,
so First contact is re-tuned after it lands rather than twice. `goal hold` stays in the mission format, tested, for a mould that comes
back; `hold+STYLE` was dropped with no mission to use it. The `#[ignore]`d
plan test now gives First contact's strike 9:00.

**Re-tuned after the economy (#170) and shapes (#174) PRs (census v12,
e425fed, seeds 1-4, 20 min a run).** #174 keeps missions on their own
bite (`campaign::MISSION_DRAIN` 1). With #170's rations every hand got
stuck or capped and none broke the Drifter (strike best 0.50, stuck by
12:56-14:09). Mission 2 now turns rations off (`rule ration 0`; `ration`
joins the mission rule keys): strike 4/4 at 7:27-8:53 with 2 stars,
random 0/4 (best 0.47), claim 0/4 (0.27), boom and idle 0/4 (0.07).
Upkeep is a later mission's lesson. The tiers thread's #180 (Drifter
hunger 8 to 16) followed (census v12c, a1a7156): strike 4/4 at 7:10-7:43
with 2-3 stars, random 2/4 at 11:32-13:18 (a tuning note; it lost its
squad or capped on the other two), claim 0/4 but its bar now reaches
0.49 since the Drifter comes out to its crystals, boom and idle 0/4
(0.08).

**The gaps re-tuned the same night (census v12, ab7a819, seeds 1-4).**
On main the walled squad (gaps+claim) was wiped by 2:22-8:52 and held at
best 0.30 of 10:00; claim, random and idle were wiped or overrun by 2:09-
6:17. Rations off alone: walls lived to 13:46-14:58 but best 0.49-0.83;
a 3000 bank on top was worse (best 0.19); rations off with the bite at
0.5 (`rule drain 0.5`): walls lived to the cap and held 4:42-8:00 at best,
never 10:00. So the mission is now "Keep the mould inside the Rim for
5:00" with rations off and bite 0.5: walls 3/4 at 5:00 with 2 stars (the
fourth held 4:42), claim, random and idle 0/4 (best 0.43, 0.42, 0.35:
the thick mould is out by about 2:00). The `#[ignore]`d plan test now
counts The gaps in held seconds; it passes on main, and so does the CI
smoke test.


**Design review of the re-tune (01:33Z).** Rations off in missions 2-3
is good onboarding (one thing at a time). Two follow-ups for the ladder,
taken: (1) rations are now the main economy pressure, so a mission must
teach them on purpose, most likely 4 ("keep a squad fed while you hold a
crystal", the ration on the HUD), before the player meets upkeep in a
skirmish; (2) walls being the only plan that wins The gaps is fine for a
teaching mission if the goal names them, so it now reads "Wall the gaps:
keep the mould inside the Rim for 5:00". From mission 4 on, at least two
plans should win each mission (the several-strategies bar). Random's 2/4
on First contact is acceptable for mission 2.


## Missions 4-7 built (census v13, 2026-11-14)

Asked by the cloud plan (`Reports/cloud-plan-2026-11-14.md`, C3): build
missions 4-7 from what is on `main`, each with two winning plans, each
mission's census under an hour. The question each census answers is
mission difficulty: does at least one scripted plan that uses the
mission's verb win, does a second one, and do idle and random lose. CPU
world, scale 2, 5,000 walkers, seeds 1-4, after the file split (#210).
Raw output: `census/{4-cut-the-cord,5-kill-zone,6-the-nest,7-spore-season}-s1-4-v13.txt`;
the seed-1 probes that chose the rules are in each mission file's
comment.

**Plans the census can play.** The world census's scripted building
plans (turrets by the farmed crystals, pumps, a ring, a kill zone) moved
from `examples/larderworld.rs` into the library (`mould::plans`), and a
new one cuts the cords: for each crystal a soldier farms, a cut across the
line to the core, kept burning by one of three cutters while mould lies on
it. The campaign census takes `PLAN+STYLE` (`cut+strike:...`): the player
AI's style farms each second and the plan builds or gives its soldiers
their orders on top. `keys=` tries mould keys over a tier (CPU only).

| Mission | Goal | Plan A | Plan B | boom | idle | random |
| --- | --- | --- | --- | --- | --- | --- |
| 4 Cut the cord (Fork, Trunk, rations on) | `shrink 0.2` | strike from 3:00 with ten: 4/4, 5:27-8:37 | cut + strike: 4/4, 4:26-7:53 | 0/4 (0.07) | 0/4 (0.08) | 0/4 (0.61) |
| 5 Kill zone (Flats, Swarmer) | `retake 3 60 1 2 3` | claim from 2:00 with ten: 4/4, 3:52-6:49 | kill zone + claim: 4/4, 4:12-6:51 | 1/4 | 0/4 (0.00) | 0/4 (0.00) |
| 6 The nest (Nest, Sleeper, rations off) | `shrink 0.5` | strike from 2:00 with eight: 4/4, 5:07-9:29 | turrets + strike: 3/4, 4:44-5:35 | 0/4 (0.25) | 0/4 (0.24) | 0/4 (0.22) |
| 7 Spore season (Rim, Sporer, rations off, bite 0.5) | `take 0.6 300` | boom with eight: 4/4, 5:40-5:50 | turrets + boom: 3/4, 5:41-5:47 | (plan A) | 0/4 (0.00) | 0/4 (0.00) |

Smoke checks: idle and random lose all 16 rounds of each mission; no
round of a winning plan ends before 3:52; no stuck run reaches the 30 s
backstop for a winning plan (idle's and random's do, which ends their
round). Every mission file plays 60 s in the window's CI smoke test.

**What changed from the design, and why.**

- **Cut the cord** keeps rations on: it is the mission that teaches them
  (design review, 01:33Z). Farming with the cuts alone never shrank the
  Trunk (seed 1, best 0.08); the cut wins with a strike. Giving the Trunk
  shape C's one-body keys (`energy_flow` 0.05, `wither` 0.01) made it
  bigger, so the strike reached only 0.83 on seed 1: the tier keeps its
  keys, and "cut pieces starve" is still a promise the mission's text
  makes more than the census shows.
- **Kill zone** counts the three middle crystals clear for a minute once
  the swarm has been on them. Every winning plan claims them; the kill
  zone's turrets and lure win too but lose more soldiers (2 stars, not 3).
- **The nest**: with rations on every hand ran out of food by 3:02-7:44
  (the Nest's outer crystals hold 160-192), so they are off, and the best
  cut on seed 1 was to 0.38 of the peak, so the goal is half.
- **Spore season**: no hand ever had more than 3 of the 8 outer crystals
  clear at once (spores land on every crystal with food), so "hold 6 of
  8" became the charge race the design gave mission 8. The `take` goal
  now counts only once a tenth of the map's charge is gone (`TAKE_FROM`)
  and can be held (`goal take F SECS`): met at once, `take 0.6` fell at
  0:41-0:51 to every farming hand. **Tuning note:** the squad leads the
  race from the start and pumps win only 2/4, so the mission is five
  minutes of farming; whether the Sporer should contest it is for the
  design review.

**Checks in the code.** `missions_4_to_7_plans_still_win` (`#[ignore]`d,
with the other full one) plays one plan a mission on seed 1;
`a_held_take_waits` checks the held take in CI; `the_campaign_files_parse`
checks `Mission::list`, the start menu's list (campaign order by the
file's number).

Mission 8 (your own mould against the Grudge on the Passes, D4) is next:
the window's `own=1` plays only the race world today, so it needs the
mission and map path first.

### Spore season, second cut (census v14, 2026-11-14)

The design review of v13 (03:16Z): five minutes of farming is dead time,
and the lesson is "starve it", with a first decision by about 0:45. So the
goal is now "Keep the spores off your two home crystals for a minute"
(`clear 2 60 5 9`: crystal 5 by the landing and 9, the west one, counted
from the start, so the spores must be kept off rather than chased off).
Longer holds or three crystals were met by no hand on seed 1 (pumps best,
0.08-0.71). On seeds 1-4 (`census/7-spore-season-s1-4-v14.txt`):

| Hand | Met | When | Best |
| --- | --- | --- | --- |
| pumps, twelve farming | 3/4 | 2:50-10:20 | 0.93 |
| pumps and turrets (turtle) | 2/4 | 6:10-11:51 | 0.60 |
| pumps, eight farming | 2/4 | 11:55-12:40 | 0.89 |
| farming alone (boom) | 0/4 | | 0.63 |
| idle | 0/4 | | 0.20 |
| random | 0/4 (wiped or overrun by 3:55) | | 0.16 |

The pump is now the verb that wins: every winning round pumps, and
farming alone never does. It is the hardest mission so far for the AI
(3/4 at best), which the owner's 01:08Z note on the C4 stalemate allows
("I have easily beat the game in all my playtests"). The held `take` goal
stays in the code for mission 8.

### After elbow room (census v15, 2026-11-14)

Elbow room (#227: living soldiers keep 1.6 touch radii apart, a census
baseline break by its own note) spread the strike armies, and the plans
of v13 stopped winning: Cut the cord's strike and cut+strike with ten fell
to 0/4 (best 0.89 and 0.63), The nest's strike with eight to 1/2. Retuned
by the plans' numbers, not the missions': Cut the cord's plans strike with
fourteen (strike 4/4 at 4:07-5:36, cut+strike 4/4 at 4:39-5:57), The
nest's with twelve (strike 4/4 at 3:40-4:29, turrets+strike 4/4 at
4:06-4:57). Kill zone's plans still win (2/2, 6:07-8:12).

Spore season is now a coin toss for every plan: pumps and turrets 2/4,
farming alone 2/4, pumps with eight 1/4, pumps with twelve 0/4 (it was
3/4); idle and random 0/4. The pump no longer decides it. Raw output: `census/*-v15*.txt`.

**Spore season, third cut (census v16).** The larder stays in this game
(the squads work is a separate mode, owner 04:49Z), so it is re-tuned now:
"Keep the spores off your landing crystal for a minute" (`clear 1 60 5`).
Seeds 1-4: pumps and turrets 4/4 at 3:43-9:01, farming alone 4/4 at
2:50-8:56, pumps alone 2/4; idle 0/4 (best 0.32), random 0/4 (wiped by
4:07). The pump helps but no longer decides it, a tuning note for the
next pass.

