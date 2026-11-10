# Brainstorm brief: a new puzzle/challenge game on the Pixel Physics engine (2026-11-07)

## What the owner asked for (Scott, verbatim)
"I want to brainstorm a totally different game. I am thinking challenge or puzzle based game that is based on pixel physics simulation but not plant or creature related, not falling sand based. I am feeling more inspired by flows, pheromone type mechanics (not as a smell-based signal but more the gas diffusion aspect), while I'm not necessarily interested in plants, I like the concept of the growth, growing tip branching networking. ... I want it to be inspired by the noita wand system, card game mechanics."

So the target is a **challenge or puzzle game** whose world is a **pixel/cell simulation** built from:
- **flows**: things that move through space under pressure or gradients (liquids, currents, charge, heat);
- **diffusion fields**: a quantity that spreads cell to cell and decays, like gas or pheromone *as physics* (concentration, gradients, saturation, evaporation), not as a smell signal for animals;
- **growth**: growing tips that extend, branch, join into networks and compete for space, as an abstract process (crystals, lightning, rivers, mycelium-like veins, circuits, cracks), not as plants;
- and a player verb modelled on **Noita's wands / deck-building**: an ordered set of cards, where modifiers change the next card, multicasts play several at once, and triggers carry a payload until a condition fires. The surprise comes from how cards combine and from how the simulation answers them.

Not wanted: plants, creatures, or ants as the subject. Explorers A and B also exclude falling-sand physics (granular piles, powders, gravity-driven sand). Explorer C may use it.

## What this project already learned the hard way (read before inventing)
1. **The Noita research** is at `/mnt/project-files/game-design/noita-wands/noita-wands.html` (a long HTML page; skim the sections on how the wand evaluates, what makes it flexible, and the hard caps). Also read `review.md` and `notch-up.md` beside it, which are short. Key points: the wand is a container with stats and the spells are cards; draw order is deck, hand and discard; modifiers draw a replacement card; triggers carry the next card as payload; each spell is a small script on shared cast state; hard caps (a recursion limit) stand in for pairwise balancing; and **the output lands in a simulated world that multiplies the outcomes**.
2. **The first attempt, the "satchel" seat (2026-10-31), failed in playtest.** It was cards (FIRE, SOAK, WIDE, TWIN, WAKE) played into the ant lab, against spiders. What the owner said:
   - "I have no idea what's going on... what's favor? ... 'NEXT WAKE+FIRE R4 FOR 10', what does that mean?" **Jargon kills it.** Every card must be readable in world words, and the screen must say what the next play will do.
   - "Randomly clicking around and I still won." "If I do nothing I win." **Doing nothing must lose**, and a random hand must do worse than a planned one. The threat has to be something the system does not beat by itself.
   - "There's soak card which has no visual effect." **Every card must have a visible consequence.**
   - "All you can do is sometimes light your plant material or ants on fire." **Two effects cannot produce surprise.** Noita's fun is many effects that interact.
   - "I don't see how adding explosions to ant plant simulation makes sense." **Cards must pull the world's own levers**, not be generic destruction bolted on.
   - "After I win the game just kind of turns into me watching." **There must always be a next thing to do.**
3. **The repo's two laws** (`CLAUDE.md`, "it has to feel satisfying"):
   - An outcome is a distribution, not a binary: there should be a middle between total success and failure.
   - There must be a player verb, and it must deliver something visible.
4. **Fun over fidelity.** Real physics is inspiration, not the bar.

## The engine you are designing for (brief; read only if you need the detail)
Pixel Physics is a Rust cellular simulation: a grid of cells (the test world is 512x320 and is meant to grow), materials with per-material rules, a chunked sweep with a dirty-rect render skip, and a frame budget treated as a hard constraint. Shipped pieces include:
- liquids that flow and level, and gases that rise and spread (`wiki/liquids-and-gases.md`);
- fire and heat (`wiki/fire-and-heat.md`);
- explosions with fracture (`wiki/explosions.md`);
- structural collapse of rigid bodies (`wiki/structural-collapse.md`);
- weather (`wiki/weather.md`);
- per-cell scalar fields that diffuse and decay, such as pheromone, light and moisture (`wiki/world-cycles.md`);
- growing-tip plant code, whose tip-extension and branching *mechanics* could be reused abstractly (`wiki/plants.md`; skim it for mechanism only).

The wiki pages are under `/home/user/Pixel_Physics/wiki/`. Read **at most two** of them, and only for something your idea depends on. Do not read `README.md`, `PLAN.md` or `Reports/` (they are huge). The source is in `src/sim/`: do not read it except to confirm one specific claim.

You may search the web for **prior art**: puzzle and challenge games built on flows, diffusion, growth or networks, and deck-builders and spell-crafting systems. Name the games, say in one line what each one teaches, and cite a URL for each. Prior art is for learning what works; do not propose a recreation of an existing game.

## What to deliver
Write one Markdown file to the path your role names (below). Contents:

1. **Five distinct game concepts**, each with:
   - a one-line pitch;
   - the core loop: what the player does in 30 seconds and what changes on screen;
   - the simulation underneath: which flow, field or growth rules it uses, and which already exist in this engine and which would be new;
   - **the card system**: list 6 to 10 starting cards in plain words, and show how modifiers, multicasts and triggers work here. Give **one concrete combo** whose result surprises, and say step by step why the sim produces it;
   - **the challenge**: what you are trying to achieve, what makes doing nothing lose, and why random play loses to a plan;
   - **the graded outcome**: stars, a score or levels of success, not just win or lose;
   - **one sample level**, described concretely enough to build (grid layout, starting state, goal, hand);
   - **the risk**: the most likely way it ends up dull or unreadable, and how a prototype would find out;
   - **the smallest prototype that tests whether it's fun** (a few days of work, in this engine).
2. **Your ranking** of the five, with one paragraph on why your top pick will be the most fun to play, not the most elegant to build.
3. **Prior art**: 5 to 10 games with what each teaches, and URLs.

Keep it under about 3,500 words. Plain words. Write for an owner who reads on a phone and judges ideas by "would this be fun".

## Rules for the shared folder
`/mnt/project-files` is a network mount. Put a trailing slash on directory paths, never search the whole folder, write only your own file, and re-read your file about ten seconds after writing it to confirm it landed. Do not read the other explorers' files: the three of you are meant to be independent.
