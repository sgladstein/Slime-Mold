# Probe 7: Kudzu, a vine is eating the village

Read `common.md` and `plant-engine-rules.md` beside this file first.
**Source:** `explorer-e-plants.md` §1, Kudzu (the full design: verbs, the gutter-torch combo, the "dawn" challenge, the 256x160 hill village, a 3-day prototype). The two facts at the top of that file apply too.

**Pitch.** Overnight a vine pours over a hill village. You have shears, a torch and the village's water. One well-placed cut peels a house-sized sheet of green off the bell tower and drops it in the street.

**The question.** Is a cut that drops a whole sheet satisfying, and can a player read which stem feeds which arm well enough that cutting is a deduction, not a guess? The hover preview (everything that would fall lights up pale) is the whole bet.

**Probe scope (about 3 days, in this order):**
1. **The vine.** The shipped creeper, or your own species derived from it, at about 6x growth speed, for your game only. Check first whether its tips hold onto a stone wall and climb it. That is unconfirmed, and the game needs it. If it does not, add clinging for your species only.
2. **Snip and the preview.** Use the existing cut. The hover preview reuses the same "is this still connected to the ground through its own tissue" walk that the engine runs after a cut. What falls must come down as litter clumps with a sound or a flash.
3. **Regrowth is the threat.** A snipped vine regrows from its store. Dig out a crown (hold 3 s) removes the store.
4. **One economy, made visible.** Show the vine going pale when its budget is negative. The gutter lever (water to one side or the other) and the Torch let the player starve one arm by hurting another.
5. **The level:** a box with two stone walls and one crown first, then the hill village from §1 (five houses, the bell tower, three crowns with one in a cellar, the gutter and well) if time allows.
- **Tools, no cards:** Snip (3 charges, one back every 8 s), Dig out a crown, the gutter lever, Salt, Torch. Scott asked whether Kudzu has cards: it doesn't. Between-level perks (pick one of three, such as "snips also burn") are a fine next step to name on the page, but are out of scope.

**Measure:**
- doing nothing loses (the vine covers most of the village by dawn);
- planned against random snipping over paired seeds (random should lose, because leaves regrow);
- **the readability test:** a still frame with "where would you cut?", then the same frame with the preview. Put both on the page;
- beside each picture: cuts made, green dropped (cells), regrowths, roofs lost, % of wall clear;
- the `ascii` worst frame at peak vine size, and while a sheet is falling.
