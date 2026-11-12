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
| 4 | Cut the cord | Fork | Trunk (thick, heals) | Cutting a tube | "Starve the mould to a fifth of its size" [walkers now against its peak] | Trunk as a built tier; one body: cut pieces starve (Larder thread) |
| 5 | Kill zone | Flats | Swarmer (floods where hit) | Turret | "Drive the swarm off the three middle islands" [middle islands with no mould] | Turrets (buildings batch 1) |
| 6 | The nest | Nest | Sleeper (dormant crust) | Strike | "Break the nest before it wakes" [crust walkers left; the wake time is the clock] | Sleeper and strike in the GPU game |
| 7 | Spore season | Rim | Sporer (spores fly to crystals) | Pump | "Hold 6 of the 8 outer crystals at 12:00" [outer crystals with no mould] | Pumps (buildings batch 1) |
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
