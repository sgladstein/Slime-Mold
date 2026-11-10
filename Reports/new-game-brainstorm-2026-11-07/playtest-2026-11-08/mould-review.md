# Mould: playtest review (2026-11-08)

`cargo run --release --bin mould`. Played through the code with `examples/zzreview_mould.rs` (since deleted), on 5 paired seeds, with all pictures drawn by the game's own frame function. My do-nothing and plan runs give exactly the builder's numbers (2 and 6 stars), so the reproduction holds.

## 1. The open question and my answer

**Does herding the mould feel alive, or is it watching? Answer: partly. The first 20 to 30 seconds are alive. The last 60 seconds are watching.**

- The oats really do work as a verb. Within a second the mould leans toward them, and a line of oats becomes a string of beads that the tube threads through (picture 4). It looks like a real slime mould.
- Then the dish freezes. In 28 of 35 runs, the food-joined count at 30 s is the same as at 75 s. The population sits at exactly 300 per joined pile (600, 900, 1200, 1500). A fresh player has spent all five oats by about 20 s. After that the screen counts down for a minute with nothing left to decide.
- **The lamp is an eraser, not a herding tool.** Used alone to steer the front, it split the mould into islands: on seed 2 at 20 s only 1 of 5 piles was joined (picture 3). The builder's own lamp logic with no oats scored **1 star over 5 seeds, worse than doing nothing (2)**. The one verb you can use forever is the one that can't herd.
- The storm costs a pile in 30 of 35 runs, whatever you did. Toughness came in only 5 of 35 runs, and the builder's plan earned it on 0 of 5 seeds.

## 2. What I played and what happened (stars per seed 1-5, then piles joined before the storm)

| Strategy | Stars s1-s5 | Total | Joined s1-s5 |
|---|---|---|---|
| Do nothing | 0 1 1 0 0 | 2 | 2 3 3 2 2 |
| Builder's plan | 0 2 1 1 2 | 6 | 3 5 4 4 5 |
| Fresh player (lamp on the blob for 4 s; 5 oats in a line by 18 s; salt line; 3 snips on strays) | 1 1 1 2 2 | **7** | 4 4 4 4 4 |
| Clever: lamp-only shepherd (lights strays beside the front) | 1 1 0 1 1 | 4 | 3 3 2 2 3 |
| Builder's lamp logic, no oats | 0 0 1 0 0 | 1 | 2 2 2 2 2 |
| Cheese: dump all 5 oats at 1 s | 1 1 1 1 1 | 5 | 3 3 3 3 3 |
| Cheese: plan plus a decoy oat spur at 72 s to draw the storm | 0 1 1 1 2 | 5 | 3 4 4 3 4 |

The fresh player's dumb line of oats beat the plan. The builder's random hand also made 6. So the bar "random loses to a plan" is not met on stars. The decoy didn't fool the storm.

## 3. Captions

1. `1-nothing-frozen.png`: do nothing, seed 2, at 20 s, 45 s and 89 s, then the score. Joined 2, 3, 3. The thrift star comes free. 1 star.
2. `2-plan.gif`: the builder's plan, seed 2, about 5x speed. Joined 5/5 by 45 s, then the storm cuts home off (after-storm 4/5). 2 stars, 5 oats, lamp held 88 s.
3. `3-lamp-herding.png`: lamp-only shepherd, seed 2. At 20 s the lamp has broken the network (joined 1/5). It ends at 1 star.
4. `4-fresh-oats-line.png`: fresh player, seed 2. Five oats in a line give a beaded tube and 4/5 joined at 20 s; 1 star at the end (3 of 4 still joined after the storm).

## 4. What feels good / what feels dead

Good:
- Oats get an answer in under a second, and the beaded tube is lovely.
- The look is clear: glowing tubes on dark agar, piles ringed when reached, the storm cut drawn in red.
- The words on screen are plain, and the hint line always says what a click does.

Dead:
- Nothing happens from 30 s to 90 s.
- The lamp only subtracts.
- Salt and Snip made no visible difference to my fresh player's result.
- Thrift is almost free: 23 of the 25 runs that joined 3 or more piles got it.
- Toughness is a lottery.
- The score panel covers the top of the dish, where the top pile and often the storm scar are. After 8 s the picture shows the tube grown back round the scar, while the panel says the pile was lost.

## 5. Improvements, ranked

1. **(Do first) Make the lamp push, not erase.** Light should add a repulsion term to the sensors without wiping the trail under it, so holding the lamp beside a front bends it. On screen the tube should visibly swing away from the light while staying whole. That turns the one unlimited tool into the herding verb the game is named for.
2. **Fill the middle minute.** Oats should come back over time (one every 15 s, say), and the far piles should drift, spoil or appear partway through. The network then always has a next job.
3. **Make the storm readable and fair.** Telegraph it 5 s ahead: a shadow line over the tube it will hit. The player then has time to build a bypass or lamp the mould off that line. Strike the tube whose loss would cost the most piles (the builder's own suggestion), and score toughness after the full 8 s of regrowth, so a healed scar counts.
4. **Make thrift cost something:** a bar of 1.15 rather than 1.5, or score network length that is not on the shortest tree. Pruning with Snip then earns a star.
5. Move the score panel off the dish (below the HUD), or fade it after 3 s.

## 6. Promise score: 3 / 5

The organism looks and answers like a real slime mould, which is rare and worth keeping. But the player has about 20 seconds of real decisions, and the lamp has to become a herding tool before this is a game rather than a screensaver.
