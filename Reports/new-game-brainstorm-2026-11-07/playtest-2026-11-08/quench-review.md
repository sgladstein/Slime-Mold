# Quench playtest (2026-11-08)

`cargo run --release --bin quench`. Played through the code with my own scripted players (`examples/zzreview_quench.rs`, since deleted), on seeds 1-4, paired. The engine rules were left as the game ships them: lid drip on, water through crust on (`C`).

## 1. The open question and my answer

**Does sealing a lid feel like a play you made, or a chore when the shovel works just as well? Neither: the shovel works better.** Two kinds of evidence.

**The numbers.** I dug the builder's pit and then added his two lids, which needed only 2 splashes. That saved all 4 houses for a score of 170. A bigger pit with no water and no lids (70 digs) also saved all 4 houses, for 166. The builder's lid-and-splash plan used 11 splashes, saved 3 of 4 houses and scored 145, the same as his 24-dig pit. **The lids add 4 points over the bare shovel.**

**What you see.** In the GIF the water sits under the lower lid as a pale band. The lava comes over and around that lid and mostly crusts in the open water past the lids. Only 70 of 417 quench events (17%) happened under a lid. Splash by splash, 0-16 cells turned to stone under the lid, against 3-103 overall. The drip glow is pale blue on a sky-blue background. At the end of a round, the stone you made looks exactly like the gorge's own rock, so the dam never reads as yours. **I would not read cause and effect off the screen.**

## 2. What I played and what happened

All four seeds gave identical results for every plan except prewet and random. The eruption does not vary with the seed, so read each row as one round.

| strategy | stars s1-s4 | score | houses safe | lava past the dam | splashes / lids / digs |
|---|---|---|---|---|---|
| do nothing | 1 1 1 1 | 120 | 2/4 (first lost at 41 s) | 198-223 | 0/0/0 |
| builder's plan (2 stacked lids, water under each) | 2 2 2 2 | 145 | 3/4 (59 s) | 71 | 11/2/0 |
| builder's pit (dig only) | 2 2 2 2 | 145 | 3/4 (77 s) | 68 | 0/0/24 |
| fresh player (splash the lava as it comes, then a lid at its tip, then one on the outline) | 0 0 0 0 | 3 | **0/4 (17 s)** | 308-319 | 5-6/2/0 |
| clever: water under the lids *before* the lava arrives | 2 2 2 2 | 140 | 3/4 (49 s) | 47-89 | 10/2/0 |
| clever: pit plus lids | 2 2 2 2 | **170** | **4/4** | 62 | 2/2/24 |
| cheese: a bigger pit, nothing else | 2 2 2 2 | 166 | **4/4** | 77 | 0/0/70 |
| random clicks | 0 0 0 0 | 15-69 | 0-1/4 | 366-479 | 9-12/1-4/0-2 |

The forge reached 66-70% in every plan that kept it fed, so the best possible score is about 170, below the 175 that 3 stars needs. **No plan can earn 3 stars**, and **doing nothing earns 1 star.**

## 3. Captions

1. `1-nothing.png`: doing nothing, end of the round. Forge 70%, 2 of 4 houses safe, 211 lava cells past the dam, score 120, 1 star.
2. `2-lid-and-splash.gif`: the builder's plan, dam site only, 48-64 s at real speed. 8 splashes fire in this window. Over the whole run: 417 quench events, 70 of them under a lid, 199 drips, 3 of 4 houses safe, score 145.
3. `3-fresh-player.png`: the fresh player at 20 s. Four splashes on the descending lava ran into the forge's pit (the blue, left). The forge is at 3% and the lava has skipped the pit. The first house burned at 17 s; doing nothing loses its first at 41 s. Final: 0 of 4 houses, score 3, 0 stars.
4. `4-shovel-only.png`: the bigger pit, end of the round. 70 digs, no water, no lids. 4 of 4 houses safe, forge 66%, score 166.

## 4. What feels good / what feels dead

Good:
- A splash on lava answers within a second: you see hissing and a crust forming.
- The forge-against-houses tension is real. Doing nothing burns houses, and quenching at the vent starves the forge.
- The lid's footprint on the cursor turns green or red before you click.

Dead:
- The lid's engine is invisible: the crust under the lid looks like the rock around it.
- The first thing a new player does (the held tool is SPLASH, so throw it at the lava) is far worse than doing nothing, and nothing says why.
- A lid on the slope is refused with "A LID NEEDS OPEN SPACE UNDER IT", with no hint of where it would fit.
- Every seed plays the same round.
- "F LID DRIP / C WATER THROUGH CRUST" on the player's screen is engine talk.

## 5. Improvements, ranked

1. **(Do first) Make the combo show as yours.** Give stone quenched under a lid its own lasting colour (a warm "your dam" brown). Pop "+N STONE" over the lid a second after each splash. Draw the drip as visible falling drops in a colour that contrasts with the sky. If the dam still does not read once it is visible, the combo is not the game.
2. **Make the shovel cost something.** Give digs a limited stock, slow them down, or let them cut only rubble. Today 70 free digs save every house, so no chemistry is needed.
3. **Protect the obvious first move.** Raise the forge pit's rim or roof it so splash water cannot drown it, or flash "WATER IN THE FORGE - IT IS GOING COLD". Start with LID held, and show the goal "BUILD THE DAM ON THE DOTTED LINE" first.
4. **Fix the stars.** The forge tops out at about 70%, so 3 stars is unreachable, while doing nothing earns 1. Lower `FORGE_FULL_SECONDS` to about 40 s, or move the thresholds so that nothing scores 0 stars and pit-plus-lid scores 3.
5. **Vary the eruption by seed**: its rate, a surge, or where the lava splits, so seeds mean something and a plan has to read the lava.
6. Take the F and C switches off the player's screen. Keep them on keys or in the harness.

## 6. Promise score: 2 / 5

The chemistry does build a dam in the numbers. But on screen it reads as lava crusting in a puddle, and the shovel is simpler and wins.
