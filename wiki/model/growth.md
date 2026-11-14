---
title: Growth and starving on the big dish
kind: mechanism
status: open
updated: 2026-11-14
sources:
  - Reports/condense-2026-11-14/README.md
  - Reports/fed-mould-2026-11-13/README.md
  - Reports/food-tubes-2026-11-11.md
  - Reports/growth-2026-11-11.md
  - Reports/larder-world-roadmap-2026-11-11.md
  - Reports/mould-shape-2026-11-11.md
  - Reports/one-body-2026-11-12.md
  - Reports/slime-enemy-review-2026-11-12.md
code:
  - BigDish::alive
  - BigDish::dense
  - BigDish::energy
  - BigParams::birth_cost
  - BigParams::birth_energy
  - BigParams::capacity
  - BigParams::condense
  - BigParams::condense_crowd
  - BigParams::condense_r
  - BigParams::energy_flow
  - BigParams::fed_crowd
  - BigParams::flow_loss
  - BigParams::hunger_scent
  - BigParams::lone_step
  - BigParams::lone_upkeep
  - BigParams::start_energy
  - BigParams::starve
  - BigParams::tether
  - BigParams::upkeep
  - CLAIM_BIRTH
  - DRAW_BIRTH
  - DRAW_STARVE
---

# Growth and starving on the big dish

## Now

The big dish's walker count can follow its food (Larder World W0;
`BigParams::upkeep` above 0, default 0, off). Each walker holds energy,
eats its equal share of what its cell loses (grass, and on Larder's
crystals its take `took`), pays upkeep, starves at zero with chance
`starve` a step, and divides at `birth_energy` into a free agar cell
beside it. Walkers live in `capacity` fixed slots, a child taking its
parent's slot xor one keyed mask a step, so a GPU thread a slot can run
it. On grass a fed mould grows and a starved one dies (8 of 8 seeds), and
its size is nearly linear in the regrowth
([growth, *Fed against starved*](../../Reports/growth-2026-11-11.md)).
Growth off changes nothing. On the GPU it is three passes that match the
CPU in a one-tick twin test and add 0.16 ms to a 1.82 ms GTX 1070 frame at
100,000 walkers ([growth, *On the GPU*](../../Reports/growth-2026-11-11.md)).
A fix after the merge: the GPU cleared its grass eater counts only when
grazing steered, so with growth on and grazing's weight 0 it over-ate the
grass; the twin test now runs the GPU a few ticks first, which catches it
(grass L1 0.12 before, 0 after).
On crystals the size stops near 20,000 whatever the charge: the crystals'
area sets the intake and the charge only how long it lasts; grass under
the crystals swamps them
([mould shape](../../Reports/mould-shape-2026-11-11.md), *On crystals*).
From a small blob on the home crystal (the race start) the network with
growth explores, then joins crystals with tubes: a median 19 of 22 on its
main body at 60 s against 2 for the food-reach droplets, 16 of 16 seeds
([food tubes](../../Reports/food-tubes-2026-11-11.md)).

**Condensing** (11-14, `BigParams::condense`, off by default; the
`condense=` key and the start menu's Condense option): a walker is ripe on
a step when it took crystal or the 5 x 5 square round it holds at least
`condense_crowd` (20) walkers; after `condense` steps ripe (300 on the
menu, 10 s) half the ripe walkers are absorbed and half condense
(`BigDish::dense`), taking the absorbed one's energy. A condensed walker
pays twice the upkeep, bites and holds a leg as two, and has two lives (a
shot, blast, withering or starving breaks it first); births stop while the
mass (a condensed walker counting two) fills the slots, so the freed
ground does not refill. Both engines, twins exact. In the window (Fork,
seed 1, farm) C has 26,904 walkers at 5:00 against 44,806 off and holds
its crystals in copper lobes instead of covering the map; at a crowd of 16
the tubes between crystals broke into beads
([condense](../../Reports/condense-2026-11-14/README.md)).

## Owner's word

- 2026-11-11 14:06Z: "Crystal should run out which forces you to expand."
  (the Larder World ruling that needs growth; [decisions](../decisions.md)).
- 2026-11-11 14:54Z (Larder thread), on crystals that run out: "If you do nothing, the
  slime should get much stronger than you and kill you."
- 2026-11-14 08:07Z: "if the slime gets to a high enough density,
  concentration in an area or it stays near an energy crystal long enough,
  it reabsorbs itself and reduces its density/coverage by half but gets
  twice as strong and healthy. This makes it more difficult and stops the
  slime from just filling the map." 08:34Z, on its look: "especially during
  the transformation process. It has to look cool" (condensing, above).

## Tried and rejected

- Pairing slots by cycling single bits: a small mould bred about one step
  in five ([`dead-ends.md`](../../Reports/dead-ends.md)).

## Open

- Run-dry crystals: `crystal_flow` makes the whole disc reachable (one
  seed); for a mould left alone to keep growing, a crystal's yield has to
  rise with its charge (the Larder thread's rule).
- `BigParams::energy_flow` (off; on the GPU too since 11-12, `esum` and
  `egain`, twin-tested): with `BigParams::hunger_scent`
  8 it makes the race-world mould one body on more seeds (largest part
  higher 8/8) and 2.5x its size; a cut gap refills in 1-2 s, so a cut-off
  piece only starves while the gap is held
  ([one body](../../Reports/one-body-2026-11-12.md)). On by default awaits
  the owner's look. On the designed maps a cut-off hunk alone still finds
  a crystal in about 2 s but arrives as a stream; aligning headings did not
  hold it together (dead end). In the Larder census (race 16 seeds) H keeps the
  planned gate (farm, wall beat idle and random) but rush stops winning and
  every hand scores lower; kept off by default for the owner's call.
  **On in the window's shape C since 2026-11-13** (0.05 alone, no hunger;
  the race world's default stays off): the food a dense mould eats on its
  crystals reaches walkers along the network, which breed where there is
  room, so the body visibly swells round the crystals it holds (Fork seed
  1, 5:00: 17.1% of the screen against 7.0%;
  [fed mould](../../Reports/fed-mould-2026-11-13/README.md)). The owner
  picked it in the blind A/B (11-13 02:39Z: "Definitely film 1").
  `BigParams::fed_crowd` (a fed walker packs more to a cell; both engines,
  off) added walkers but not picture. At the peak its 50,000 slots fill
  at most 34.8% of a map's open cells at scale 2 (a test holds it to
  half); inside the body 66-83% of walkers are boxed in (59-66% before).
  **Cut off from every crystal** (11-13), a part of it stays whole for
  75 s or more and then dies at once, as its shared energy runs down
  together (held ring cuts: 10 of 12 whole at 2:00 on the CPU; +75 s in
  the window). `BigParams::flow_loss` 0.05 (food fades as it travels; on
  the GPU too since 11-13, `eloss`) kills it in 35-50 s, still at once,
  and leaves C about 30% smaller; 0.2 shrinks C to a fifth. **Parked** (designer and integration, 11-14 00:35Z, cloud plan D2): off, kept in the code on both engines, not deleted; withering is the owner's pick for a cut-off piece. It would come back only if withering reads too slowly in play.
  **`BigParams::wither`** (11-13, both engines, off in `BigParams`; **on
  at 0.01 in the window's shape C since 11-13 15:53Z**) makes a cut-off
  part thin steadily instead: news of food travels through the
  body two cells a step and each walker carries the last it heard; one
  with none for 5 s dies with chance 0.02 twice a second. A held ring cut
  leaves 81% of the piece at 10 s, 54% at 20 s and 9% at 60 s (never more
  than 21% gone in 5 s), and uncut C is the same to the walker on 4 of 4
  seeds. Withering on food income (C fell to 15-29k) and news kept only in
  cells (C 2-16% smaller) were rejected. Of a blind A/B/C of the three
  the owner picked withering, "still too fast", and asked for half the
  speed: at 0.01 a cut piece keeps 74% at 20 s, 41% at 45 s and 15% at
  90 s, and one cut of 12 lost 34% in 5 s as its pooled energy ran out
  ([fed mould, *A cut-off part withers*](../../Reports/fed-mould-2026-11-13/README.md)).
  `larder-world -- wither=0` plays C without it.
  **`BigParams::panic`** (11-13, both engines, off in `BigParams`; **on at
  2 in the window's shape C**, the owner's "act more like a scared animal
  ... It should be more reckless", no stronger attack): a walker with no
  news of food for the lag heads straight for the nearest squad unit
  within `panic_r` (20 dish cells), else the nearest crystal with food,
  creeping at `panic_creep` (0.5) of its pace for `panic_lean` (1.5 s,
  the tell) and then at twice it; it calms once it hears news. The creep
  came from the game designer's review of the last stand: a walker
  heading straight already moves 15 cells a second to a soldier's 10, so
  a lean at its own pace was a charge; at half pace the piece bunches and
  slides for about 3 s before it lands, against about 1.5 s, and uncut C
  is back within 0.13% of today on 4 of 4 seeds. A charge at a unit gives
  up after `panic_dash` (15 steps, 0.5 s) and creeps on; a run for a
  crystal keeps its bolt. On the retreat probe a unit that pulls back half
  a second late from 24 cells gets clear on 5 of 6 seeds (0 of 6 when the
  charge never gives up), and one that stands is caught on 5 of 6 either
  way. In the window four soldiers 34 cells
  from a cut piece were charged 5 s after the cut and shot it down,
  losing one; with none near, the piece pours into a crystal's lobe.
  Smoke scores unchanged; the window's farm squad has 30 soldiers at 2:30
  against 33 (bits its shots knock off charge it, and are shot). Those
  bits are slivers: on the CPU world (Fork, farm, seeds 1-2, 5:00) at
  most 16-24 walkers are scared at once, mostly alone, they make 4-6% of
  the squad's bites, and it loses 26 and 24 soldiers against 25 and 25,
  so no size floor. A bigger cut piece (163 walkers) charging four
  soldiers kills all four in 2 s (the last-stand film). A lean starting before the lag was rejected (uncut C 6% smaller on
  one seed); at the lag uncut C is within 2.9% on 4 of 4 seeds but has
  eaten from one crystal fewer on 3 of them ([fed mould, *A cut-off part panics*](../../Reports/fed-mould-2026-11-13/README.md);
  [films](https://claude.ai/artifact/SRxAj8nri2iGAsvE4DFGbP)).
  `larder-world -- panic=0` plays C without it.
- **Size barely follows food** (11-14, parked by the owner's 01:08Z
  ruling that they beat the game easily): at its cap C eats three times
  its upkeep, a crystal feeds about 9,400 walkers, so with all but one
  crystal emptied it holds 50,000 for two minutes and falls to 10,000 by
  six; with four kept it is at 47,000 at six minutes (seed 1,
  `examples/foodprobe.rs`; [fed mould, *Does its size follow*](../../Reports/fed-mould-2026-11-13/README.md)).
  The one-body page's 25,000-walker films ran with one walker a cell off
  (corrected 11-12 17:10Z); the 5,000-walker game and the CPU numbers here
  are unaffected ([slime enemy review](../../Reports/slime-enemy-review-2026-11-12.md)).
- Growth's numbers are per fine step, so the race start holds at the
  game's scale 2 but not on the 2048 x 1280 GPU world (scale 8): there it
  stays a knot at home, and with trail sensors 36 fine cells out it touches
  20 of 22 crystals in 12 game minutes but joins few with thick tubes (one
  seed; [food tubes](../../Reports/food-tubes-2026-11-11.md), *For the GPU
  world*). Without growth the small start still makes tubes (3 seeds).
- With [the life cycle](life-cycle.md) on, only ordinary walkers pay
  upkeep, starve, divide and share energy; a spore or a dormant walker
  keeps its energy until it lands or wakes, and a struck walker frees its
  slot for a birth. Should sleeping cost upkeep? Untested in play.

## Sources

- [growth](../../Reports/growth-2026-11-11.md), all sections.
- [mould shape](../../Reports/mould-shape-2026-11-11.md), grass and crystals, size on crystals.
- [Larder World roadmap](../../Reports/larder-world-roadmap-2026-11-11.md), W0.
- [food tubes](../../Reports/food-tubes-2026-11-11.md): the race start joins crystals with tubes.
