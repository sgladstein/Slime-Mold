# A new economy and three actions for the skirmish: design sketch and tests (2026-11-11)

**Status: design, nothing built or run.** The owner asked at 00:09Z: "What
do you think about our resources/economy and options for the player to
control/interact with the game". I answered with the proposals below, and
at 00:16Z the owner said: "These all sound good". This note turns that
answer into one design for the budget skirmish
([report](../rts-skirmish-2026-11-10.md)) and lists the cheap tests to run
before building it. Each part is tied to the research in the
[parent report](../rts-research-2026-11-10.md) (Parts 4 and 5). The
skirmish's thread owns the build. Two Fevers can borrow the economy later.

## What is wrong today, in one paragraph

Food eaten is both your income and your score, so a lead in food is
already a lead in score, and that food buys more walkers. Every purchase
ends up as more of your walkers on a pile. The field is full by 0:10 and
the food runs out at about 1:50. So the game is decided by the first
concentrated hit (soft, 127 of 128), and the side ahead at 1:00 wins 416
of 473 games. The tithe is the one real tension: money now against body
now.

## The parts of the mould, each given its own job

| Part | Its job | How it already exists in the model |
|---|---|---|
| **Food** | Income. Each bite feeds the mould, and the tithe sends part of it to the bank | `Pile::bite`, `Dish::tithe` |
| **Walkers** | Army and workers at once. They eat, hold piles and fight at the seam | `Params::fight` |
| **Tube** | Infrastructure. It gives reach, its length is the cost, and it is where you are exposed | `Dish::fed`: the tube distance from each cell to the nearest food the mould touches |
| **Score** | Held ground over time, not food eaten | new |

### The size cost (Civilization IV's fix, in the mould's terms)

**Supply lines.** A walker counts as fed in a fight only within `supply`
tube cells of food it is joined to (`Dish::fed` below `supply`). Beyond
that it fights as unfed, at four times the fed death rate. A mould that
reaches across the field has a weak, starving front. A compact mould is
strong but holds less. The rule changes only the fight check, so it costs
almost nothing per step. A second option, if this one is too weak:
upkeep from the bank per pile fed beyond the first k, rising with each
pile.

### The score

**Points from held ground.** Every second, each pile where your walkers
outnumber the rival's two to one pays you a point. The first to a target
wins, or the higher total at the bell. A wipe-out still ends the game, but
the trailing side can win by holding ground, not only by killing. The match
runs 4:00, and the late food keeps landing until the end so piles regrow
(they stop at about 1:50 today). That leaves a midgame for counters to live
in.

### The one pull-back rule (to sweep)

**Neglected food regrows faster.** A pile nobody has eaten from for 20 s
regrows at twice the rate. The trailing side always has somewhere worth
reaching. The rule is in the world rather than a bonus for losing, so it
reads as the dish and not as a rubber band. The alternative to sweep is
starving walkers that fight harder (desperation).

## The three actions

All three replace the beacon, the habits and the instant spore.

| Action | Cost | What the mould does | Answer to it |
|---|---|---|---|
| **Call** (two at most, moved freely, 3 s cooldown) | free | Your mould craves that pile, as a beacon does today (`Dish::beacons`). This sets your income and your network's shape | The rival reads your tube growing toward it, about 10 s ahead |
| **Fortify** a pile or tube disc | bank (about a rally's 150) | Your walkers in the disc count as fed in fights for 15 s, even past `supply` | Strike somewhere else; the defender cannot fortify everywhere |
| **Strike** (a spore) | bank (1,000 today), price rising with use (`Rules::spore_ramp`) | 150 walkers land on a pile within reach of your tube after a **5 s countdown that both sides see** | Fortify the target during the countdown |

Why Fortify, and not the rally: round 3 found that a rally brings walkers
who arrive unfed into a fed crowd, so it does not change who wins the
fight. Fortify acts on that cause directly. The countdown makes the strike
telegraphed and answerable, like StarCraft's nuke. It differs from the
sleeping spores already tried: that was a wait, and this is a wait with a
specific answer the defender must pay for. The intended triangle: strike
beats greed (a mould that only calls), fortify beats strike, and greed
beats fortify (fortify spends the bank and buys no ground).

## The tests, cheapest first, with the deciding measure named

Use the same harness and round robin as the skirmish report (`duel SEED
LEFT RIGHT`, 8 seeds a side). Render and look at every new rule before
counting it.

0. **Side edge first.** Trace why the left mould wins 14 of 16 mirror
   games. Pass: mirror idle v idle within 9-7 either way on 16 games.
1. **Fortify answers a strike.** Soft's strike against a plan that
   fortifies the counted-down pile, on the same seeds as round 3's soft v
   keep. Pass: the defender holds the pile in at least 12 of 16 strikes,
   with paired counts against round 3's rally.
2. **Supply lines move the snowball.** Sweep `supply` (for example 20, 40,
   80 cells and off) with idle and soft hands. Deciding measure: how often
   the side ahead at 1:00 wins (today 416 of 473, 88%). Target: under 70%,
   with the plan still beating random 13 of 16.
3. **Points and regrowth.** The same sweep under points scoring and the
   4:00 match. Measures: wipe-outs (today 300 of 576) and the leader count
   again.
4. **The strategy map.** A round robin of idle, random, greed (calls
   only), soft-strike, fortify-keep and mixed plans. Then an **exploiter
   search**: random search over a parameterised plan (tithe, target rule,
   when to strike, when to fortify), 64 candidates against the round's best
   plan. Deciding measure: no plan wins every pairing, and a cycle whose
   links each survive Holm, with each plan in it beating random 13 of 16.
5. **Frame cost.** Quote it. The supply check is one comparison per fight
   check. The points census is one count per pile per second.

If test 1 or 2 fails, the design changes before the plans are written.
After test 4, the owner plays it.
