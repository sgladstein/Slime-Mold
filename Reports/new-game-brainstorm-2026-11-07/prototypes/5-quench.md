# Probe 5: Quench, aim the roof, not the water

Read `common.md` beside this file first.
**Sources:**
- C's Quench, `explorer-c-open.md` §2: the lid-and-drip engine, the gorge level, the 2-day prototype.
- D's QUENCH, `explorer-d-cards.md` §4: the forge that needs *some* lava.
- A's Floodgate, `explorer-a-flows.md` §3: boiled water comes back as rain somewhere else.
- **Extensions:** `review-1-player.md` §2 D and §3 item 5; `review-2-engine.md` §3 item 3 and §5 items 6 and 9.

**Pitch.** Lava pours down a gorge toward a village, and a forge at the bottom needs a trickle of it to work. Throw water and place stone lids. Steam under a lid drips straight back onto the lava, so one splash becomes a quench engine that builds the dam where you put the roof.

**The question.** Does the engine's own chemistry make combos fun? Lid then Splash should read as a clever play you made, not as nothing happening under a rock. This is the cheapest probe of the six, because almost all of it is shipped.

**Check two engine facts first (half a day, before building the level):**
- **Time one Lid plus one Splash** over a lava pool: how many seconds the drip keeps quenching, and how many rubble cells it makes. Then render it as a GIF and look at it. If the engine runs for under 2 seconds or over 30, change the lid or the splash size before going on.
- **Crust:** C's design needs the crust to crumble and sink (the rubble is the dam's footing); D's needs it to stay put. The shipped engine crumbles (`wiki/liquids-and-gases.md`). Keep the shipped rule. Do not add a second crust material in this probe.

**Probe scope (2–3 days):**
- **The level:** C's 256x200 gorge, trimmed to about 256x160 if it reads. A lava vent at the top left, a rock overhang, a dotted dam outline, the village row past the dam, and the forge in a hollow at the bottom right.
- **The goal has a middle (D's rule):** the forge scores while lava reaches it; the houses must stay dry of lava and, at the flood, dry of water. Blocking everything loses the forge. Letting everything through loses the houses.
- **The flood** comes at about 100 seconds and tests the dam you built.
- **Tools, no cards unless order turns out to be the fun:** Splash (a ball of water), Lid (a stone slab placed as a roof), Channel (dig). A 400-cell water tank that refills a little from steam caught in a cistern.
- **Make the engine legible:** the steam under a lid, and the drip, must be visible at phone size. If it reads as nothing happening, a faint glow on the quenching cells is fair.
- **Floodgate's rule is optional:** where steam that escapes outside comes back as rain. Try it only if day 2 has time. Review 2 warns that if the rain lands everywhere, the lesson reads as random weather; check where it lands before building on it.

**Measure:**
- doing nothing loses (lava buries the houses), and blocking everything loses (the forge goes cold);
- planned against random over paired seeds;
- the share of the dam outline filled, forge output, houses kept dry;
- the time from a Splash to a visible result (target under 3 seconds);
- the `ascii` worst frame while the quench engine runs.
