# Fault (`--bin fault`): playtest review, 2026-11-08

## 1. The open question and my answer
**Is a running crack satisfying to cause, and can you guess roughly where the next one will go?**

**Partly, on both counts.** The crack itself answers in under a second: a white line runs about 20-30 cell edges with a flash. But it's short and hooks around the grain, and it rarely crosses the 30-cell-wide wall. **The satisfying moment comes 40 s later, not when you tap.** On seed 6 the plan cut a 1,122-cell piece at 2 s, and nothing moved until the lake reached it at 48 s. Then the slab shoved off and the water roared through, which looks good.

On guessing: I tapped 9 spots 4 cells apart on one wall. Every crack stayed within about 10 cells of the click ("roughly where": yes). Moving 4 cells flipped the crack from vertical to horizontal or sent it the other way ("which way": no). The builder's finding holds: the same tap on the same wall gives the identical crack.

## 2. What I played and what happened (stars per wall, seeds 1-10)
| Strategy | Stars per wall | Total /30 | Bursts |
|---|---|---|---|
| Do nothing | 0 on every wall | 0 | 10 |
| Random (builder) | 0 2 0 0 1 1 1 0 2 0 | 7 | 5 |
| Builder's planner | 1 0 1 0 1 1 0 1 0 0 | 5 | 5 |
| Fresh player (spends taps in the first 10 s, then drill, Meltwater+Frost, last taps on the crest; ±4 cells jitter) | 0 0 0 0 0 0 0 1 0 0 | 1 | 9 |
| Clever: tap the strained upstream face low down, under water | 0 1 0 0 0 0 0 0 2 0 | 3 | 8 |
| Cheese: drill only (chip the crest, or a late slot of 5 holes) | 0 on every wall | 0 | 10 |

The drill cheese fails because the wall is 30 cells thick, so holes alone never cut through. That's good. The fresh player loses badly: taps spent early make cracks the lake can't use yet, and nothing tells them so.

**Why the plan doesn't beat random.** I instrumented a copy of the probe to log the lake's highest row (the crest is row 40).
- **The pitch's "controlled spillway" scores as a failure.** On walls 3, 5 and 8 the planner opened a notch as the lake arrived. The lake was held steady at rows 48-61, the flow was gentle (about 68 cells/s), and the mill stayed dry. "Drained" is measured against the lake's own peak, so that scores 0% and earns 1 star, the same as a messy random survival.
- **Stars 2 and 3 fight each other on this level.** Draining half the lake means dropping about 50 rows through a 16-cell gorge 70 cells upstream of the mill. Every run in any strategy that drained 50% or more without bursting hit or flooded the mill (4 of 4: planner wall 6, 50% drained, 130 cells of flood; random walls 2 and 9; my deep-tap run on wall 9). No run reached 3 stars.
- **Random's wins are deep accidents.** Random taps at all depths. A cut that happens to be low gets reached by the lake early and drains 76-80% (walls 2 and 9). The planner deliberately avoids deep cuts.
- Re-scoring alone doesn't fix it. Grading "the lake stayed 15 or more rows below the crest" plus "the mill untouched" moves the planner to 11 and random to 10: a tie, not a win. The level has to change too (see 5.1).

## 3. Captions
- `1-nothing-burst.png`: do nothing, wall 6, 100 s. "THE WALL BURST": the lake tops the crest and the face crazes, but 0 slabs fall. 0 stars. The worst outcome looks mild here.
- `2-plan-cut-at-2s-still-waiting-at-10s.png`: planner, wall 6. One tap at 2 s cut a 1,122-cell piece, and it sits there unmoved for 46 s. No on-screen sign that it is cut free.
- `3-plan-release-47s-to-73s.gif`: the same wall. The lake reaches the cut and shoves it off, then a second notch goes. 5 pieces released, 281 slabs, 50% drained, lake ends at row 109. The debris dams the gorge, the flood reaches the mill ("MILL HIT"): 1 star.
- `4-nine-taps-4-cells-apart.png`: wall 1, taps on a 3x3 grid 4 cells apart (red circles). Each crack runs 20-30 edges with 0-3 forks. Neighbouring taps often run opposite ways.

## 4. What feels good / what feels dead
Good: a visible white crack within 0.3 s of a click. The same tap always gives the same crack. The slab shove plus the water rush is the best moment in the demo. Doing nothing loses on 10 of 10 walls.

Dead: the tap's payoff is delayed 30-60 s with no signal. The direction of a crack can't be read from the ice. The burst sometimes just overflows and nothing falls. You can't get 3 stars. The HUD says "LAKE 111/110 HIGH", which is a number, not a line on the wall.

## 5. Improvements (ranked)
1. **(Do first) Grade the spillway the pitch describes, on a level where it is possible.** Paint a **safe line** on the ice, about 15 rows below the crest. Stars: no burst; the lake stayed below the safe line after it first reached it; the mill is dry and untouched. Give the gorge room to carry it: widen it, or put the mill on a terrace above the outflow, so a staged notch can be mill-safe. Then make deep cuts dangerous: a crack started below the waterline runs on, so a random deep tap splits the wall low and floods the mill. The planner already avoids that.
2. **Show where the tap will run before you click** (Storm's dotted line is what made that demo honest). Hover shows the two joint lines from `tap_corner`/`tap_dirs`, and a faint run from `trial()` + `settle_cracks()`, the planner's own lookahead. That answers "can you guess" by design.
3. **Say when a piece is cut free.** Outline it when it closes, with "CUT FREE - THE LAKE WILL PUSH IT AT THIS LINE", and a tick on the wall at the row where it will go.
4. **Holes draw cracks in.** A tip within about 4 cells of a hole bends to it and stops. Then "drill two, tap between" (stitch-drilling) becomes a reliable plan that random can't copy.
5. **Make the burst a burst.** Release the whole upstream face as slabs. On some walls it now only overflows.

## 6. Promise: 2/5
The release moment is real, but the core tap is small and its direction unpredictable. As graded, the level punishes the plan the pitch is about, so it needs a level and scoring redesign before the "fun to cause" question can really be tested.
