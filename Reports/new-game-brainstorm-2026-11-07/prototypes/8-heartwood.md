# Probe 8: Heartwood, your deck is your tree's genes

Read `common.md` and `plant-engine-rules.md` beside this file first.
**Source:** `explorer-e-plants.md` §2, Heartwood (the full design: ten cards, the Fork-Fork-Lean-Leaf-Bud combo, three seasons of light share, the 160x120 valley, a 4-day prototype). The two facts at the top of that file apply too.

**Pitch.** Two trees, one valley, one sun. You tap a growing tip and play a card on it, and you win by shading the rival tree out. Fork splits a tip and sends the next card to both halves; Bud holds a card until its branch breaks.

**The question.** Does playing cards onto tips feel like steering a living tree, or like a slow way of drawing one? Can a player say, before playing a card, what the tree will look like afterwards? This is the one plant probe built on cards, because here the order of plays is the fun.

**Engine hook (confirmed by the explorer):** a tip picks its next cell by weighing `[continuation, light, wind, upward]` in `plant.rs` around line 6663. A card overrides those weights for that tip's next N cells. Re-check the line on current `main`, because the file moves.

**Probe scope (about 4 days):**
- **The tree:** the shipped tree species at about 6x speed, for your game only.
- **Five cards first:** Grow Up, Lean to Sun, Fork (a modifier and a multicast: the next card goes to both halves), Leaf Out, and Bud (a trigger: it plays its held card when the branch is cut or breaks). Add Thicken, Deep Root and When Shaded only if the five work.
- **Sugar:** cards cost sugar, which comes from what your leaves earn. A tip with no card does not grow, so doing nothing loses.
- **The break is the engine's own:** a heavy crown on a thin trunk fails where it is weakest, and falls as a log and broken wood. Fork-Fork-Lean-Leaf-Bud should be able to show a planned break, then a new leader rising from the stub.
- **The rival:** a shipped tree with no cards, started earlier. Show the light-share number on screen all the time. Score light share at the end of each season, with stars at 40 / 55 / 70%.
- **Readability:** the screen shows which tip is selected, what the next card will do in plain words, and a faint outline of where those 10 cells will go.
- **Leave out:** the dry third season, seedlings and Drop Seed, and the run. Name them on the page.

**Measure:**
- doing nothing loses; random plays (Forks everywhere, leaves in your own shade) lose to a plan over paired seeds;
- light share per season, limbs lost, sugar spent, beside each picture;
- the prediction test: before a card, ask "what will the tree look like?", then show the result. Put a few on the page;
- the `ascii` worst frame at peak tip count, and when a limb breaks.
