# Seep: playtest review (2026-11-08)

`cargo run --release --bin seep`. Played through the code with `examples/zzreview_seep.rs` (since deleted), 4 minutes per run, on paired seeds 1-5 (seeds 1-6 for the two late-added fresh-player variants). All pictures are the game's own window frame. My do-nothing, planned and gasonly runs give the builder's numbers.

## 1. The open question and my answer

**After the first minute, would a player reach for Damp or Cut? Is the mould used, or decoration? Answer: for a fresh player it is decoration. For a player who already knows the level it is a third-star puzzle.**

- **Sparking alone holds the mine.** My fresh player only sparks the biggest pink pocket every 2 s and tries each other tool once. It holds both lamps and hauls 96 carts on 5 of 5 seeds, the same as the builder's plan.
- **Damping after the first minute buys nothing you can see.** A fresh player who keeps damping ahead of the mould (about 1,300 late Damps) gets the mould to cap crack 1 at 60 s instead of 131 s. Stars and carts don't move: 2 and 96 on 6 of 6 seeds. The third star needs the mould led up the drip chimney to crack 2 before it opens at 90 s, and nothing on screen points there.
- **Cut only hurts.** One curious snip at 20 s, on the first tube by the sump, killed the mould for the whole level on 5 of 5 seeds: it ate about 30 units of gas, against 2,100 to 2,260 without the snip. Nothing on screen says the mould has died. No scripted player, the builder's included, ever needed Cut.
- **Damp's best use isn't the mould at all.** Damp rings round both lamps, and nothing else, held both lamps with 0 gas burnt on 5 of 5 seeds (2 stars). Damp works as a firebreak.
- The mould alone can't save a lamp. The plan with Blow and Spark removed loses lamp A at about 40 s on 5 of 5 seeds.

By the brief's own stop rule, the honest result on this level is **Firedamp alone, plus a mould that is scenery unless you know the solution.**

## 2. What I played and what happened (stars per seed 1-5)

| Strategy | Stars s1-s5 | Carts | Cracks capped | Gas eaten by mould |
|---|---|---|---|---|
| Do nothing | 1 1 0 0 0 | 32-53 | 0-1 | 3-1,652 |
| Builder's plan | 3 2 3 3 3 | 96 | 2 2 (s2: 1) | 1,944-3,293 |
| Plan without Damp or Cut (gasonly) | 2 2 2 2 2 | 96 | 0-1 | 27-1,444 |
| Fresh player (spark biggest pink every 2 s, one try of each tool) | 2 2 2 2 2 | 96 | 0 | 23-38 |
| Fresh, no snip, damps ahead of the mould after 1 min | 2 x 6 seeds | 96 | 1 (at 60 s) | 2,130-2,260 |
| Clever: mould tools only (plan minus Blow/Spark) | 1 1 1 1 1 | 55-56 | 1-2 | 2,465-3,637 |
| Cheese: Damp rings round the lamps, nothing else | 2 2 2 2 2 | 64 | 1 | ~2,050 |

- Blow was used 0 times by every player in these runs.
- Doing nothing still reaches 53 carts on seeds 1-2, so the "CARTS x/30" goal on screen is always beaten.

## 3. Captions

1. `1-nothing.gif`: do nothing, seed 1, the full 4 minutes at about 15x speed. Lamp A is lost at 26 s; 1 star, 53 carts, 1 boom.
2. `2-plan.gif`: the builder's plan, seed 1. The mould is damped along the floor and led up the chimney; crack 1 is capped at 40 s and crack 2 at 111 s. 3 stars, 96 carts, 91 sparks, 0 lamps lost.
3. `3-one-snip-kills-mould.png`: the same fresh player at 60 s, seed 1, with one Cut at 20 s (top: the mould ate 27) and without it (bottom: ate 2,144). Both get 2 stars.
4. `4-damp-as-firebreak.png`: Damp rings only, seed 1, at 120 s. The mine is full of gas and lamp A is choked ("GAS!"), but 0 gas burns and no lamp is lost. 2 stars, 64 carts.

## 4. What feels good / what feels dead

Good:
- The three-colour gas is instantly readable.
- Floating words ("TOO THICK", "SINGED", "BLOWN!") teach the rules.
- Sparking a pink pocket is a satisfying verb with a quick answer.
- Doing nothing loses fast and visibly: lamp A goes at 26 s.

Dead:
- The mould is a 1 to 2 px yellow line along the rock, easy to miss beside the gas. Whether it is fed, starving or dead is not readable.
- Cut has no good use.
- Damp's reward lands minutes later as a star rule nobody is told about.
- The carts goal means nothing.
- Blow is never needed.

## 5. Improvements, ranked

1. **(Do first) Make the level need the mould.** Put a pocket that sparking can't touch near a lamp: too thick, under a damp roof, or sitting right on a lamp so a spark would blow it. Only mould eating it down, or mould capping its crack, saves that lamp. Then Damp is the answer to a problem the player can see, not a bonus star.
2. **Draw the mould 3 cells wide, with its state in colour:** bright yellow and pulsing when fed and wet, pale straw when starving, grey when dead. Cut and the burn-like-a-fuse rule only make sense once you can see a dry tube.
3. **Give Cut a job and make it safe.** A dry tube touching a pocket should visibly carry flame toward a lamp, so snipping the fuse is a rescue. Don't let one snip next to the core end the mould: respawn tips from the core, or make the core uncuttable.
4. **Show crack 2 before it opens** (a hairline that hisses from 60 s) so leading the mould there is a plan you can see.
5. Score carts against a real goal (about 80), or drop the carts goal, so the HUD number means something.

## 6. Promise score: 2 / 5

The gas and fire half is good and could stand alone as Firedamp. The mould half adds nothing a fresh player would feel until a level makes it necessary and the screen makes it visible.

## Which mould reads better

**The swarm (probe 2) reads far better on screen.** It is thick, glowing and organic, and you can see it reach, bead and starve. Seep's graph is a thin yellow pen line that looks like a painted road.

The graph has the better *rules* for a combined game: tubes that eat gas, wet when fed, dry and burnable when starving, flow-thickened. It also works in side view, where the swarm's forward sensors fail on walls (seep's own dead end).

My recommendation for a combined game: **keep seep's graph as the model and draw it the way the swarm looks**, with 3-5 cell tubes whose width follows the flow, a glow, a pale colour for starving and a pulse. Then add the swarm's one real verb, a bait (oats) that pulls a tip within a second.
