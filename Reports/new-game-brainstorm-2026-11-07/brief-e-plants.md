# Brainstorm brief, explorer E: games built on the plant growth engine (2026-11-07)

## What the owner asked for (Scott, 16:37Z, verbatim)
"Start one more agent to explore game ideas that incorporate our plant growth engine or or at least are inspired by our plant growth engine which can be heavily modified."

This lifts the earlier "no plants" rule for you alone. Your subject is the **plant growth engine**: growing tips, branching, roots, the energy budget, light and water, seeds, senescence and rot. You may use it as shipped, heavily modify it, or take only its ideas (tips that extend and fork, a body that has to pay to stay alive). **The game does not have to look like gardening.** A growth engine can drive a city, a coral reef, a circuit, a cancer, a fungus, a rival empire. Cards are **optional**: use them only where the order of plays is the fun. Falling sand is allowed.

Read the main brief `brief.md` in this folder for the goal (a challenge or puzzle game), the lessons from the failed "satchel" card prototype (section "What this project already learned the hard way"), and the deliverable format. Everything there applies to you, except the "not plants" line and the "cards required" line.

## Don't repeat what is already being built
Twenty-one ideas came out of the first round, and six are now being prototyped:
- lightning that leaves glass scars;
- a slime mould herded with a flashlight;
- Firedamp, a gas-filled mine;
- Seep, gas plus a mould that eats it;
- Quench, a lava dam built with steam;
- Fault, cracking an ice wall.

The rest were Floodgate, Tincture, Saltworks, Veins, Brine, Delta, High Tide, Fulgurite, Landslide, Lode and Etch. Your five should be **different games**, and at least three should put the plant engine at the centre, not just add a plant.

## The engine you are designing for
Start here, and read only what your ideas lean on:
- `wiki/plants.md`: what the plants do, in plain words. **Read this one.**
- `wiki/world-cycles.md`: light, moisture and the day. Read it only if you need it.
- The source, to confirm a specific claim only: `src/sim/plant.rs`, `src/sim/organism.rs`, species data in `assets/species/`. Don't read whole files (they are large). Grep the mechanism.
- `CLAUDE.md` "The ethos": the plant line's own lessons. Death is graded (a starving tree is marked senescent and rots, not vanishes), and "the verb works, and what it produces is pieces" (felling a tree yields pieces).

What this project has learned about plants, which an idea should respect or knowingly break:
- Growth is slow on screen. The ant lab's complaint was "I'm just watching." An idea must give the player something to do every few seconds while growth plays out, or speed growth up a lot.
- The owner judges by eye at play scale. A plant game must read on a phone.
- The ants in the lab dig under roots and kill shrubs; plants and creatures interacting is rich but hard to read.

Do not read `README.md`, `PLAN.md`, the other explorers' files or `Reports/` (they are huge or are other people's ideas).

## What to deliver
Follow `brief.md` "What to deliver": five concepts, each with the full set of parts (pitch, 30 seconds, simulation exists vs new, verb (cards or tools), one surprising combo explained step by step, challenge with doing-nothing-loses and random-loses-to-plan, graded outcome, sample level, risk, smallest prototype), then your ranking and 5 to 10 prior-art games with URLs. Under about 3,500 words, plain words.

Write it to `/mnt/project-files/game-design/new-game-brainstorm-2026-11-07/explorer-e-plants.md`.

## Rules for the shared folder
`/mnt/project-files` is a network mount. Put a trailing slash on directory paths, never search the whole folder, write only your own file, and re-read it about ten seconds after writing to confirm it landed.
