# Probe 2: Mould, herded with a flashlight

Read `common.md` beside this file first.
**Sources:**
- Mould in `explorer-b-growth.md` §6 (it absorbed Veins, §2)
- **Extensions:** `review-1-player.md` §4 Mould and `review-2-engine.md` §4 Mould
- **Grafts:** `review-3-synthesis.md` §2C and §4c

**Pitch.** A yellow slime mould pulses across a dish, hunting for food. You garden it with light, oats and scissors until it grows a network that is short, reaches every food pile, and survives cuts.

**The question.** Does herding it feel alive, or is it watching?

**Probe scope (3–4 days):**
- **The agent model of slime mould** (Jeff Jones, 2010): 20,000 to 30,000 tiny walkers. Each one senses the trail ahead of it at three angles, turns toward the strongest, moves, and deposits trail. The trail spreads and fades.
  - Run it on the engine's fine per-cell trail field (`src/sim/pheromone.rs`); its source notes say it was sized against a Jones follower. If that field doesn't fit, a private field inside your game is fine.
  - This makes thick tubes, pruned dead ends and shortest paths with no network maths.
- **Tools, no cards.** Stocks per dish, for example:
  - a **Lamp** you drag under your finger: the mould recoils from it at once, the instant answer the hand needs;
  - **Oats** (5): a steady trail deposit that attracts;
  - **Salt** (1 line): walkers can't step on it;
  - **Snip** (3): kills walkers in a stroke and clears the trail there.
- **Food reserve = the number of walkers.** Walkers far from food slowly die, so doing nothing spreads a thin sheet that runs out before reaching the far food.
- **End of dish:** a storm cuts three tubes. Three stars:
  - **reach:** food piles joined;
  - **thrift:** total network length against the shortest possible;
  - **toughness:** food still joined after the cuts.
- **Draw tube brightness on a fixed dark-to-bright ramp.** Never blend it with the dish colour.
- **Leave out for now:** the pulse beat, spores and fruiting, dye, lightning as the finale, a rival. Mention them on the page as next steps.

**Measure:**
- the time from placing Oats to a visible lean (target 3 seconds or less);
- do-nothing against planned stars over 5 seeds;
- the worst frame with the whole dish awake.

A dish never settles, so the dirty-rect skip won't help. Say what that costs.

**Note.** If this probe works, it becomes the mould half of "Seep" (`review-3-synthesis.md` §4a), the gas-eating mould. Keep the mould code self-contained so that game can reuse it.
