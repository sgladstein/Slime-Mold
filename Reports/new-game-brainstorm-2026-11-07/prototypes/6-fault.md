# Probe 6: Fault, crack the ice wall before the lake breaks it

Read `common.md` beside this file first.
**Sources:**
- Fault, `explorer-b-growth.md` §3: the tension field, crack tips, Meltwater then Frost, "The Mill".
- **Extensions:** `review-1-player.md` §3 item 4 and `review-2-engine.md` §3 item 5 and §5 item 8.
- **The breakage it hands over to:** `wiki/structural-collapse.md`, then README's `M17 status` (via its By topic table). Grep `Reports/dead-ends.md` for the mechanism you touch (for example `fracture`, `rigid`, `load`) before building on it.

**Pitch.** A meltwater lake swells behind an ice wall above a mill. Tap the ice and a white crack runs, branches and stops at a hole you drilled. Pour water into the cracks, then freeze it, and every wet crack jumps at once. Cut a controlled spillway before the lake bursts the whole wall.

**The question.** Is a running crack satisfying to *cause*, and can a player predict roughly where it will go? Two reviewers called the tap the best single touch-and-sound verb of the 21 ideas. Its risk is that the tension underneath is invisible and cracks look arbitrary.

**Probe scope (3–4 days):**
- **The tension field (new, inside your game):** a diffusing number over the ice. The lake's push on the wall and the weight of ice above feed it.
- **Crack tips (new):** a tip steps to the neighbour with the most tension, forks when tension is high, and stops dead at a drilled hole or at the edge of a gap. Draw the crack bright white, 2 cells wide, and give it a sound or a flash as it runs.
- **Breaking loose (shipped):** when cracks close a loop, hand the piece to the existing fracture and structural-collapse code, so it falls as blocks, cobbles and grit. Water flows through the gap with the shipped liquids.
- **Budget a day for "does the same cut break the same way twice"** (review 2). Structural collapse is the subsystem that has surprised this project most. If one tap brings the whole wall down unpredictably, the puzzle is dead; report that.
- **Tools:** Tap, Drill and Frost first. Add Meltwater (pour into cracks) once those work, because Meltwater then Frost is the moment the design is built around.
- **The shimmer A/B (B's own test):** play with and without a faint tension shimmer on the ice. Ask whether players can say where a crack will run before they tap. Put both on the page as a blind A/B.
- **Level:** "The Mill", scaled to about 256x160. The lake rises on a clock. If it reaches the top of the wall, the wall fails all at once, which is the worst outcome.
- **Graded:** spillway flow kept below a mark, slabs that hit the mill, and the share of the lake drained before the timer. Stars.

**Measure:**
- doing nothing loses (the wall fails all at once), and random tapping also loses (it cracks the wall top to bottom);
- planned against random over paired seeds;
- repeatability: the same tap on the same seed gives the same slabs; the same tap on 10 seeds gives a spread you can describe;
- slabs freed, cracks run and cracks stopped by holes, beside each picture;
- the `ascii` worst frame when a slab breaks loose.
