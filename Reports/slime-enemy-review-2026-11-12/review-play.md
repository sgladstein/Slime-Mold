# Play and balance review: the ideal slime enemy

> Saved by the one-body thread from the reviewer's final message (subagents could not write files). Paths under `raw/` are this folder's `raw/`; pictures named in the text are in `img/` (see the report's picture list). Unedited otherwise.

**Short answer: keep Jones, add two layers on top, and fix three game rules.** Nothing in the data points at the walker rule as what stops play. A one-off script tallied raw census lines already in the repo; I ran no new games.

**Two things you should know about this report:**
- The brief asked for `report.md` in my scratch folder. The tool refused to write it, so this message is the report.
- The tally script is at `raw/play/tally.py` and its output at `raw/play/tally-core.txt`. They cover the 48-seed core census; the tier-matrix tally was a one-off and isn't saved.

## 1. The owner's question, from play

Four findings (48-seed race census with core defence, `Reports/larder-world-roadmap-2026-11-11/core/census-pull8-race-*.txt`):

- **Only one plan can end a game.**
  - Farm, wall and fort reached the 15-minute cap in 144 of 144 games.
  - Strike wiped the mould in 6 of 48.
  - In the older tier matrix (8 tiers × 8 seeds, before shot craters), boom, claim, farm and fort reached the cap in 237 of 256. Strike won 31 of 64.
- **The win is a king.**
  - A squad that holds the core clear for 5 s triggers `check_core`, which calls `stop_births`. That sets `birth_energy` to infinity for the whole mould.
  - A walker lives about 3 s, so the whole body then dies.
  - So the only winning plan is the one that walks to the core.
- **The mould can't kill a dug-in squad.**
  - Idle survived to the cap in 46 of 48 games (first touch at a median 189-294 s).
  - That breaks the owner's ruling: "If you do nothing, the slime should get much stronger than you and kill you" (11-11 14:54Z).
- **Food doesn't bind.**
  - Farm, wall and fort end with 83-85% of everything they harvested still banked (farm's median: 43,362 of 51,464).
  - Strike ends with 62% banked.

The game-designer reviewer's counters (blast beats germ on the Drifter 30/32, germ beats blast on the Sporer 30/32) are not in my checkout. If they hold, the model already gives counters when tools act on its own terms.

**Keep, extend or replace:**

| Option | What it gets | Cost |
|---|---|---|
| Keep as is | Leaves "reads as one organism" and "shows intent" unsolved | 0 |
| **Extend (recommended)** | (a) Crowd-gated motion (`lone_step`, built on CPU and GPU, off): fronts mass before they surge, which shows intent a few seconds ahead. (b) A coarse body field: energy carried along the trail with a limited reach, for one supply, damage by mass and visible flow. | About 0.3-0.8 ms a frame on the GTX 1070 (my guess, to measure); 1-2 weeks |
| Replace with a Creeper World-style fluid | Readable and cheap to run | Loses the spread-out mesh (L2) and nearly everything built: the CPU/GPU twins, crater, alarm and scar fields, squad frame, nine tiers, toolkit, buildings, campaign tuning, the player's own mould |

## 2. What our tests missed

- **Score v2 puts wins (above 1) and stalemates (-0.5 to 0.5) on one scale.** "Best plan by mean" then mostly asks "can this plan ever win?", and only the core walk can. "Beats idle and random" is a weak gate when idle stalemates and random throws its squad away (wiped 41 of 48).
- **Measures that disagree with the goals:**
  - Lone step was scored as a harm because it slows the spread about 25%. P1 asks for exactly that: fronts move about 15 dish cells/s, soldiers 10.
  - Food sharing + hunger ("H") was dropped because "rush stops working". The real harm is the cliff: at matched size, random starves the mould 9 of 16. One shared energy pool is one health bar.
- **Every balance table is at 3-5% density; L1 wants 15-20%.** Drain is per walker (1 hp per walker per second), so a 4x denser mould melts soldiers 4x faster. Re-balance after `walker_mass` (P3) and the density choice.
- **The plans are a fixed scripted roster, with no search for a counter-plan** (`Reports/rts-research-2026-11-10.md` §7). Part of "strike is dominant" may come from the roster itself.
- **Maps flip the best plan more than tiers do.** On the Nest a turret ring was best and the killzone squad died 8 of 8; on the Fork every hand tied. Tiers were tested on the race world only.

## 3. Tensions between goals, and fixes

- **One supply + one body (B2 + L3) makes siege a cliff.** A pooled body dies all at once.
  - Fix: supply that has a reach and travels along the tubes.
  - The mesh's loops (L2) then become redundancy: how many cuts a siege needs depends on how many loops there are.
  - Tero et al. 2010 frame Physarum networks as a trade-off of "cost, transport efficiency, and fault tolerance" ([doi:10.1126/science.1177894](https://doi.org/10.1126/science.1177894), abstract via PubMed). That makes a natural tier dial: a tree-like Trunk falls to siege; a meshy kind resists siege but is thin enough to strike.
- **A dense body (L1) makes the strike weak.** Land P3 (damage by mass) first.
- **Slow fronts (P1) against "comes for soldiers" (B4) and "idle loses" (P2).**
  - A front slower than soldiers can only threaten things that can't move: the larder, harvesters, buildings, a squad that stands still or gets flanked.
  - That's the right RTS asymmetry: the player has mobility, the mould has a slow edge and fast trunks inside its body.
- **Core defence (B5) on top of a king rule rewards decapitation.** Fixed by direction 1 below.
- **Recoil (B4) failed only because random then held an unguarded core** (the condition is recorded in `dead-ends.md`). Without a king it can come back.

## 4. Other games (pages opened)

- **Creeper World**
  - One quantity per cell; emitters stop emitting at a cap; thin creeper evaporates ([blog](https://knucklecracker.com/blog/?p=182)).
  - The creeper cuts the player's collectors off their network, so the supply lines are the player's ([Wikipedia](https://en.wikipedia.org/wiki/Creeper_World)).
  - Its developer worried that invisible fields would make creeper unpredictable.
- **Factorio**
  - Pollution spreads from chunk to chunk; spawners absorb it and send attacks ([Pollution](https://wiki.factorio.com/Pollution)).
  - Evolution rises with time, pollution and nests killed ([Evolution](https://wiki.factorio.com/Evolution)).
  - The pressure comes from what the player does, and it shows on a map overlay.
- **They Are Billions:** a building's defence runs out, the infected get in, and one leak cascades ([wiki](https://theyarebillions.gamepedia.com/Zombies)).
- **StarCraft creep:** new tumours can only go on existing creep, and units are faster on creep ([wiki](https://starcraft.fandom.com/wiki/creep_tumor)).
- **Tooth and Tail:** each species tells you its role (`research/sources/tooth-and-tail.md`).

What these share:
1. The enemy's state is one visible quantity.
2. It has distinct targets to attack.
3. Its growth is known or signalled in advance.
4. Its local rule is predictable.
5. A passive player loses.

Jones already gives 1 and 4 at map scale. 2, 3 and 5 belong to the game layer and are missing.

## 5. Goals ranked by play value

1. B4, reacts to soldiers
2. P2, idle loses
3. B2, one supply
4. P1, pace
5. L4 and B6, intent shows and learnable
6. B5, defends
7. P4 and L6, the tier ladder
8. B3, cut pieces (mostly feedback)
9. P3, damage by mass (an enabler)
10. L2, the network (play value only as siege redundancy)
11. L1 and L3, continuous body and one organism (fantasy and readability; the owner's rulings stand)

The most decisions a minute come from:
- pushes you can read and get ahead of,
- the window between a recoil and the flood back,
- trunk cuts.

Spectacle only:
- speck-free edges,
- a mesh that lasts but adds no redundancy,
- a cut piece groping about.

## 6. Ranked directions

1. **Remove the king.**
   - Mechanism: the headless core by default; the core becomes the best-fed food, defended. A win is a wipe or a starve. Retry recoil.
   - Why: the core walk is the only decisive route today.
   - Risk: more stalemates until direction 2 lands.
   - Cost: 0.
   - Experiment: a `core=headless` key in `larderworld` (about 2 lines), race seeds 5-20, with farm, wall, strike, loud and plague.
   - Deciding measure: decisive-end rate per plan, and a strike-minus-farm sign test.
2. **Escalation, so idle loses.**
   - Mechanism: turn `Rules::overrun` on (it already exists); scale deposit and drain with walker energy, so a fed mould pushes harder.
   - Risk: turtles die and only attacking pays.
   - Cost: about 0 (one multiply per walker).
   - Deciding measure: idle loses before 10:00 on at least 14 of 16 seeds, and farm still beats idle and random under Holm.
3. **Crowd-gated fronts** (`lone_step` 3, floor 0.5).
   - Mechanism: walkers alone at the edge move slowly; a crowd moves at full speed.
   - Gains: slow, massing edges (P1); a surge you can see a few seconds ahead (L4); hunks that hold together (0.72 against 0.46).
   - Risk: a smaller mould (5,609 against 7,658 walkers at 300 s); specks at the leading edge stay.
   - Cost: about 0.2-0.4 ms at 100k walkers on the 1070 (my guess).
   - Deciding measure: the 90th-percentile frontier speed over 2 s windows is under 10 dish cells/s (soldier speed), plus forecast skill at 10 s (§7).
4. **Supply along the tubes.**
   - Mechanism: energy flows along the trail on a coarse grid and loses a little each cell.
   - Risk: a new field to twin-test, and possibly another cliff.
   - Cost: about 0.2-0.5 ms (256×160 grid, about 10 passes).
   - Deciding measure: far-side mass at 30 s after a trunk cut, against a control cut and no cut. The trunk cut must cost at least 1.5x the control's loss on 14 of 16 seeds.
5. **Make food bind for the player.**
   - Mechanism: running costs (already planned) and smaller starting banks.
   - Deciding measure: at most 25% of the harvest unspent, and +2,000 starting food raises the score on at least 12 of 16 seeds.

## 7. Measures to add

- **Decisive-end rate** per plan per tier. Today 0 of 144 for farm, wall and fort.
- **Idle loses before 10:00.** Today 2 of 48.
- **Unspent share of harvest.** Today 0.83-0.85.
- **Siege payoff:** a trunk cut against a control cut.
- **Forecast skill:** from one frame, predict the mould at +5 s and +10 s by growing it along the scent gradient. Score the overlap gain over plain even growth, per tier. Add a "click where it will be in 10 s" test to the blind A/B page.
- **Strike timing curve:** start the strike at 60, 180, 300 and 480 s. A best time in the middle means timing is a real decision.
- **Counter matrix under Holm**, plus a search for counter-plans. The best plan should win 50-80% of its tier's seeds.
- **P1 as tip speed against soldier speed**, not crossing time. Crossing already takes 3-5 minutes, set by growth, while the tips still outrun soldiers.

## Correctness concerns

- The core's global birth stop is a game rule, not part of the model, and the censuses read it as "strike is strong".
- Score v2's mean is taken over a two-peaked distribution (wins and stalemates). Report win rate and the stalemate median separately.