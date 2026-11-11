# Skeptic review (independent agent, 2026-11-11)

Prompt: [common.txt](common.txt) + [skeptic.txt](skeptic.txt). Report as returned:

# Skeptic review: Larder World roadmap (Reports/larder-world-roadmap-2026-11-11.md)

## The three most important points

**1. The plan scales up a game that has not been shown to be fun, and W0 is the most expensive way to find out.** M3's gate still fails: no flip between fields survives Holm over 34, and the owner's playtest is "still to come" (larder-m3:7, :57; wiki/rts/shooter.md:84). The roadmap then puts the hardest item in the GPU plan first. That item is W0, births, upkeep and starving per walker on the GPU. The GPU plan calls these rules "the hardest part to make parallel and deterministic" (gpu-plan:204). They need a claim pass, segmented prefix-sum birth budgets, a slot pool and a CPU census readback that arrives about 300 ms stale (gpu-plan:40, :102-104, :154-155). The plan also wants Jones 2011 uploaded first, and that is still open (gpu-plan:216). The engine has nothing to build on: big.rs:7-8 says it leaves out "the economy (eating, upkeep, births, starving), the census", and it respawns to a fixed count (big.rs:932). So the first two weeks are spent on plumbing, and nothing in them tests the game. The CPU `Dish` already has growth, upkeep and starving (mod.rs:29-37, :167-185). The GPU plan itself names a 512 x 320, ~20k-walker CPU arm as live-capable, and 1024 x 640 at about 10 ms a step offline (gpu-plan:137).

**2. Persistence (W4) and the campaign (W5) are planned before any one round has a decision in it.** Every Larder milestone found the same shape: one dominant plan ("a ladder with a timing choice, not counter-strategies, because the mould does not adapt to the squad", shooter.md:25-26). M2 had the reader best on 13/16 (shooter.md:51). M3 has reader / striker / farmer best on 8/5/3 and 10/4/2, none significant (shooter.md:49). A persistent world multiplies that problem. Saved extractors, upgrades and bases are snowball engines, and the field skirmish already showed snowballing as the default dynamic in this model. The 1:00 leader won 80-85% under every rule set tried: hold, supply, regrowth, a 4:00 bell and ground scoring (dead-ends:24, :31). The owner then ruled the 80-85% a difficulty stat, not a gate (roadmap:45). That removes the only measured brake on the race opening. W4's gate ("the owner plays an hour and wants more", roadmap:17) is the first fun test in the plan, and it comes after four phases.

**3. Crystals that run dry repeat a failure the repo has already recorded.** Lean mould "crashes to a few hundred walkers" on its own by 6:00 (larder-m3:41), and with drops off the lean mould is wiped 16/16 (shooter.md:53). The crust failed because "the dish's lean crash comes from its own starvation rule" (dead-ends:35). Spores failed because "no new food comes with a spore" (dead-ends:34). Finite crystals are a lean field by construction, so the likely world is one where the mould starves while the player waits. In that world idle beats active play, and that breaks "random and idle are the controls". Two other findings point the wrong way:
- A dead soldier is a 400-food pile (larder.rs:28, :162), so buying soldiers feeds a starving mould (dead-ends:33). In a resource-scarce world the punishment for buying gets worse.
- Kills heal on a timer (larder.rs:542). A world that persists between sessions has to decide what happens to cuts and corpses, and the plan doesn't say.

## The progression ladder is names on rules, not measured behaviour

- **Drifter** (`graze_weight`): grounded. Bodies move 6.5-7.6 cells against 0.67-0.77, 8/8 (creatures:13-26). It is also the only tier with a measured behaviour.
- **Sprawler** (`explorer_spread`): that parameter does not exist. The code has `explore_spread` (big.rs:230, :909). It also has no measured result in a fight.
- **Trunk**: "heal around a cut" has no rule behind it. "dense start, slow respawn, W0 growth" is a wish. Cords at density sit in a cord > hunt > maze ladder on 7-9 of 10 layouts (dead-ends:17), and "heals" is the wound timer every mould already has.
- **Hunter** (Larder hunger): "hunger barely acts and needs a lower bar or should go" (shooter.md:84).
- **Rival pair / Converter** (`species_avoid`, `species_convert`): measured not to fight. Shares stayed 0.49-0.51, three cyclic species blurred into "a mixed soup", and a boost overreached and lost, p 0.29 (creatures:31-40, :44-55; dead-ends:32).

Likely to play the same: Trunk and Hunter both play as "a big mould you shoot until it starves". Rival pair and Converter will both look like a stable border until something creates an imbalance. Tiers 4-6 should be marked "unmeasured" on the page.

## Interfaces and threads

There are six threads (GPU, Larder, Camera/UI, Player AI, creatures, campaign) and they share one moving type, `Cmd` (larder.rs:217-251). Its world semantics aren't fixed yet: team ids, building squares, a visibility hook (roadmap:21-24). Camera/UI and Player AI would code against a `world` module with no state behind it until W1. The Player AI "reports paired-seed censuses" of plans that M1-M3 already show are a ladder. Building it before the mould answers the squad gives a well-measured ladder. Cut to three threads until W1 passes: GPU, Larder, Player AI. The Player AI should take over the existing `larderm2` plans rather than start fresh.

## What I would cut and reorder

1. **W-1 (new, CPU, about a day):** crystals on today's `Dish`. Piles with finite charge, scent scaled by charge, no drops (`drops_stop` 0, larder.rs:117). Use the existing M3 window, squad and plans, a few dishes across, then the race start.
2. Get the owner's M3 playtest done first. It is free and overdue.
3. W1 with walls and a second mould, still on the CPU, at the 512 x 320 arm.
4. W0 (GPU growth) only once a CPU world has a plan that beats idle and random and that the owner wants to replay. The GPU then answers a measured question: does scale change play? That fits phase 2's own gate (gpu-plan:142).
5. Cut W5 entirely, and cut W4 down to save and load for replays, until one 15-minute world passes its gate.
6. Drop tiers 5-7 from the owner-facing ladder until one species fight shows a winner the player caused.

## The cheapest test that could kill the plan

Do this before any GPU code, on the existing CPU harness (`examples/larderm2.rs`, seeds 17-32):
- Turn the 23 piles into crystals: finite, dimming, no drops.
- Run idle, random, farmer, reader and striker.
- **Deciding measure, named now:** does any plan beat idle on 13 or more of 16 seeds (sign-test p < 0.05 after Holm), and does the mould still hold more than 25% of walkers at 5:00 under idle?

If the mould starves itself under idle (the M3 lean crash predicts it will), "crystals force you to expand" is a mould problem, not a player problem. In that case the world needs income that the leader can't also take (dead-ends:31) before anything is ported. That is about an hour of runs, against weeks for W0.
