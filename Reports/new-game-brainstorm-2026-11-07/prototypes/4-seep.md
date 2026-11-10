# Probe 4: Seep, the garden that makes its own bomb

Read `common.md` beside this file first.
**Sources:**
- **The full design:** `review-3-synthesis.md` §4a. It has the first 60 seconds, the rules, the "Two Lamps" level at 256x160, and a staged 6-day plan with a stop rule. Build from it.
- **The halves:** Firedamp in `explorer-a-flows.md` §1, and Mould in `explorer-b-growth.md` §6.

**Pitch.** A mine leaks gas toward the miners' lamps. You blow it and spark it, and you grow a slime mould that eats it. But a mould that eats a pocket too thick to burn thins it until it *can* burn.

**The question.** Is the mould something players *use*, or decoration beside the gas? This probe is the only one that tests whether two layers make one game: you push the air, you grow the mould, and the gas and fire answer.

**Two sibling probes run at the same time.** Probe 2 (Mould) and probe 3 (Firedamp) each build half of this game. Don't wait for them, but don't build the same thing twice blind either.
- **The mould here is a graph, as §4a says:** junction nodes, with tubes as edges drawn into cells. Probe 2 builds an agent swarm instead. That makes the two moulds a real comparison, not a duplicate. Say on your page which one reads better.
- **The gas:** at the start of each day, look at the Firedamp lane's branch (`git branch -r | grep claude/`, then `git log` on the one whose files sit under `src/firedamp/`). If its gas field is usable, borrow it. Until it is, build the smallest gas of your own: rises, spreads, escapes at the air shaft, three colours.
- If you borrow code from a sibling branch, say which commit in your PR body.

**Probe scope (follow §4a's stages, trimmed to about 4 days):**
- **Days 1–2:** the gas on the fine grid, with the three-colour ramp (pale blue too thin, pink burnable, deep purple too thick), one crack and Blow. Blow pushes gas directly, not through the air grid. Lamps count exposure. Then Spark and the burn band: a front runs through pink and stops at blue, purple or wet cells.
- **Days 3–4:** the mould graph. Its tips climb toward richer gas along rock surfaces, fast on damp rock. Where it touches gas it eats it. Tubes that carry flow get thick and bright, and idle ones fade. **Fed tubes are wet and stop flame; starving tubes dry out and burn like a fuse.** Add Damp and Cut, then build "Two Lamps".
- **Tools, no cards:** Blow, Spark, Damp, Cut, each with a meter that refills, plus 2 Fuses.
- **Leave out:** the run, perks, fruiting, the second crack's escalation if time runs short.

**Measure:**
- doing nothing loses in about 45 seconds;
- planned beats random over paired seeds;
- gas eaten by mould against gas burnt, per run;
- cracks capped, lamps blown and face-seconds worked beside each picture;
- the `ascii` worst frame during a burn with the mould on screen (the mould never settles, so the dirty-rect skip is lost; quote the cost).

**Stop rule (from §4a).** If players never Damp or Cut after the first minute, the mould is decoration. Say so on the page, and say that Firedamp alone is the honest result.
