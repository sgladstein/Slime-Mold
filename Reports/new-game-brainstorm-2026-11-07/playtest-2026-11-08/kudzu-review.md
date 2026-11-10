# Kudzu playtest (2026-11-08)

## 1. The open question and my answer

**Does dropping a whole sheet of vine feel good, and could a player tell where to cut before the preview lit up?**

**Feels good: yes.** A cut at the foot of a wall drops the whole wrap in about a third of a second. It slides down the stone like a curtain and heaps at the foot (picture 2). It is the best moment in the demo.

**Could a player tell where to cut: yes, too easily.** And the preview points away from the best cut.
- The obvious guess, "cut where it leaves the ground", is right. On seed 1 that one spot lights 569 cells.
- The strongest move is to cut the vine off from its roots at the crown. The preview shows that cut as **0 cells** ("NOTHING ELSE LETS GO"). Yet the whole vine wilts orange and pours off both walls within about 10 s (picture 3).
- That "cut at the roots" player got **3 stars on 8 of 8 seeds** with 4 to 12 cuts. It beat the builder's preview-maximising plan on 4 seeds and tied the other 4.
- So the deduction a player would naturally make is correct, but the game's bet (the preview) tells them it is worthless.

## 2. What I played and what happened

8 paired seeds. Walls clear at dawn (stars); start = walls clear when play begins.

| seed | start | nothing | builder plan | random | fresh player | clever: cut at roots | cheese: dig only | cheese: wait, cut last 25 s |
|---|---|---|---|---|---|---|---|---|
| 1 | 28% | 32 (0) | 93 (2) | 100 (3) | 98 (3) | 100 (3) | 33 (0) | 94 (2) |
| 2 | 52% | 55 (1) | 100 (3) | 85 (2) | 97 (3) | 100 (3) | 37 (0) | 99 (3) |
| 3 | 77% | 77 (2) | 97 (3) | 100 (3) | 75 (1) | 100 (3) | 77 (2) | 95 (3) |
| 4 | 20% | 27 (0) | 96 (3) | 85 (2) | 83 (2) | 100 (3) | 21 (0) | 94 (2) |
| 5 | 56% | 57 (1) | 100 (3) | 92 (2) | 92 (2) | 100 (3) | 100 (3) | 93 (2) |
| 6 | 72% | 75 (2) | 100 (3) | 99 (3) | 100 (3) | 100 (3) | 75 (2) | 96 (3) |
| 7 | 30% | 33 (0) | 97 (3) | 86 (2) | 93 (2) | 100 (3) | 13 (0) | 99 (3) |
| 8 | 31% | 30 (0) | 100 (3) | 87 (2) | 86 (2) | 100 (3) | 27 (0) | 92 (2) |
| **mean / stars** | | **48% / 6** | **98% / 23** | **92% / 19** | **91% / 18** | **100% / 24** | **48% / 7** | **95% / 20** |

How each player played:
- **Fresh player:** picks a patch of green, wiggles the pointer over 6 spots near it, and clicks whichever lights most, about once a second. Its cuts drop a median of 2 cells, which is 9 to 30% of the best cut available. It still averages 2+ stars.
- **Cut at roots:** cuts the vine just above the soil, nearest the crown first.
- **Dig only:** holds dig on the crown at the start and does nothing else.
- **Wait:** does nothing until 0:25 is left, then makes 6 of the builder's planned cuts.

My plan, random and nothing columns reproduce the builder's report.

## 3. Captions

1. `1-where-would-you-cut.png` (seed 1, start of play): the still frame, then the same frame with the pointer at the left wall's foot. Walls 28% clear; the preview lights 569 cells.
2. `2-first-cut-pours.gif` (builder's plan, seed 1, first 2 s): 3 cuts, 819 cells dropped, walls 28% to 93% clear. Every cut after this only nibbles at the heap.
3. `3-cut-at-the-roots.png` (cut at roots, seed 1, first 12 s): 4 cuts, each lighting 0 to 6 cells. The vine turns orange and slides off both walls: 1,967 cells dropped, walls 100% clear at 2:48, VINE DEAD by dawn.
4. `4-dawn-four-ways.png` (seed 1 at dawn), in reading order:
   - nothing: 32%, 0 stars, 0 cuts;
   - builder plan: 93%, 2 stars, 25 cuts, 4,203 dropped, 4,358 regrown;
   - fresh player: 98%, 3 stars, 25 cuts;
   - cut at roots: 100%, 3 stars, 4 cuts.

## 4. What feels good / what feels dead

**Good**
- The pour: there is an answer within a second, it is graded (some cuts drop 30 cells, some drop 600), and the heap stays on the ground as a mark.
- The wilt after a root cut: the whole vine goes orange, then slumps. It is dramatic and easy to read.
- The bar's "CLICK: 569 CELLS OF VINE COME DOWN" says exactly what the next click does.

**Dead**
- **The vine is not a threat during play.** Doing nothing moves the walls 28% to 32% over 3 minutes. The "wait" cheese (do nothing for 2:35, then 6 cuts) scores 95%, against 98% for the builder's 21 cuts.
- **Regrowth stays on the ground.** In the planned run 4,358 cells regrew, but almost all as a bush on the litter heap, not back up the walls.
- **Dig lies.** The screen says "PULL THESE ROOTS, THE WHOLE VINE DIES". Dig alone left the walls worse than doing nothing on 4 of 8 seeds (seed 7: 33% to 13%) and killed the vine on 1. I have not traced why. My guess, from a comment on `sprout_climber_root`, is that stems lying on the soil between the walls still drink, so a vine with no roots lives on.
- **The mood line contradicts the picture.** It reads "VINE THRIVING" in warning orange while the cut-off vine wilts, because it reports the rooted stump. It also overlaps "WALLS CLEAR" in the bar.
- **The game is decided in the first second:** 3 cuts take seed 1 from 28% to 93%.

## 5. Improvements (ranked)

1. **(Do first) Preview what starves as well as what falls.** Colour the cells a cut would part from the roots a dull orange ("WILTS"), next to the pale "FALLS", and say "0 FALL NOW, 1,365 WILT IN 10 S". This is a flood fill from the root cells through the vine's own tissue, run beside `severed_if_cut`. Without it the best move in the game is invisible, and the screen calls it worthless.
2. **Several crowns, with arms that cross.** That is the village's three crowns, one in a cellar. With one crown, one root cut ends the night, so there is nothing to deduce. With several, "which stem feeds which arm" becomes the puzzle, and the root preview is how you solve it.
3. **Make waiting cost.** Score walls clear averaged over the night instead of at dawn, and steer regrowth back onto the walls, aiming new runners at the nearest bare face. Then the "wait" cheese loses and early cuts matter.
4. **Make dig do what it says, or say what it does.** First trace why a dug vine lives on. Then either kill it the way a root cut does (the dug shoot loses its hold on hunger and its stems on the soil stop drinking), or change the line to say what dig really does.
5. **Fewer cuts.** Change "3 charges, one back every 8 s" (25 cuts a night) to about 6 for the whole night. Random snipping then stops earning 2 to 3 stars.
6. **Polish:** a count that pops ("−569") on the heap, and a rustle sound. Make the mood line report the largest living piece. Stop the bar text overlapping.

## 6. Promise score: 3 / 5

The verb lands and the fall looks right. The puzzle the pitch promises does not exist yet: one crown and a preview that hides the root cut leave no deduction to make. Both fixes are concrete.
