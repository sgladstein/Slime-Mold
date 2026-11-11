---
title: The owner's rulings and decided choices
kind: method
status: current
updated: 2026-11-11
sources: [CLAUDE.md, HANDOFF.md, Reports/mould-sound-2026-11-10.md, Reports/rts-design-2026-11-10.md, Reports/mould-review-thread-handoff-2026-11-10.md, Reports/mould-real-world-2026-11-09.md]
code: [Params::pieces]
---

# The owner's rulings and decided choices

Every ruling and every decided choice, with the owner's words and the time
they were said, newest first in each group. **Open decisions live in one
place only: [HANDOFF.md, *Decisions the owner has not made*](../HANDOFF.md).**
When the owner answers one, move it here with the quote and time, and update
the pages it changes in the same commit. Never write an answer the owner
did not give; a recommendation on a card is not his answer.

Times are UTC. "Card" means he tapped an option. Old-account thread ids and
artifact links are left out; the quotes are kept in the sources named.

## Standing rulings (how all work is judged and shown)

| When | Ruling | Owner's words | Source |
|---|---|---|---|
| 2026-11-11 00:06 | Tests may run on the owner's PC (Scott-Desktop, the GTX 1070) whenever it is reachable, as many as needed; threads may start sub-agents or more threads to explore the Jenson comparison and mould simulations | "you can use my local PC for as many tests as you want." ... "Feel free to start sub-agents or additional threads to explore this comparison further and/or more research asking mold simulations" | (GPU plan's thread), [scale](model/scale.md), [CLAUDE.md](../CLAUDE.md) |
| 2026-11-10 20:55 | The current demos (The Wild One and the other dishes) are unlikely to be the true games: don't spend effort tuning work to them; engine work such as the GPU plan is judged on the engine, and the project is now exploring RTS-type games | "I just think The Wild One and other current demos are unlikely to be the true games. So I don't want us to be wasting effort or get too far away from your original task which was not really about the game part of this project. (we are now exploring RTS type games)" | (GPU plan's thread), [scale](model/scale.md), [gpu plan](../Reports/gpu-plan-2026-11-10.md) |
| 2026-11-10 19:44 | Threads with long test runs may start up to 4 Haiku runner threads for extra compute; runners make no decisions, run only the commands given and send the output back | "Feel free to start some haiku runner threads just to get additional compute. If you want to do that, they make no decisions and just run the code you tell them and send the output back to you. You can run up to 4 threads if you want" | (project chat), [CLAUDE.md](../CLAUDE.md) |
| 2026-11-10 18:15 | A brainstorm for a new direction starts from the model, not from the demo's tools (lamp, oats, salt, knife, water), which were built for a herding puzzle | "it seems like all the brainstorm agents are just taking the current tools ... I want to make sure we are truly brainstorms how this should best work and not doing limited exploration on what exists in our current demo." | [RTS brainstorm](../Reports/rts-design-2026-11-10/brainstorm/README.md) |
| 2026-11-10 17:47 | Each project thread merges its own pull requests | "all threads handle their own PR mergers" | (project chat) |
| 2026-11-10 17:01 | Judge outside simulations by what is interesting to watch and could become play, not by how real they look | "our goal is not a realistic slime mold. It is interesting simulations that we can turn into a game." | [Jenson report](../Reports/jenson-physarum-2026-11-10.md), [scale](model/scale.md) |
| 2026-11-10 | Research, harnesses, raw census output and page sources live in this repo, not the project's shared folder | (recorded in `CLAUDE.md`, no quote kept) | [CLAUDE.md](../CLAUDE.md) |
| 2026-11-10 13:53, card 13:58 | Everything on screen is the simulation, and looks like it: no thresholded edges or colour lines; the picture is the data | "I want all the visuals to look like they're coming from a real simulation and not just graphics animations." | [review hand-off](../Reports/mould-review-thread-handoff-2026-11-10.md) |
| 2026-11-09 01:09 | No drawn-only effects (the painted pulse) | "Everything we're seeing should be based on real pixel simulations." | [project-memory note](../handoff/project-memory/mould-everything-simulated.md) |
| 2026-10-27 02:51 | Every difference as paired counts with a sign-test p and a multiple-comparison caveat | "Is 111 significantly different than 180 in a statistical sense?" | [note](../handoff/project-memory/report-significance.md), [census](method/census.md) |
| 2026-10-25 06:03 | Pictures on an Artifact page, never chat attachments | "I cannot see the images that you post" | [note](../handoff/project-memory/images-via-artifact.md) |
| 2026-10-16 18:59 | Fun over fidelity: biology is inspiration, not the bar | "the overall goal of this work isn't a biologically scientifically accurate ant simulation. It is a fun simulation based game" | [note](../handoff/project-memory/fun-over-fidelity.md) |
| (undated) | Defaults on unless there is a measured harm or a stated uncertainty | (recorded in `CLAUDE.md`) | [CLAUDE.md](../CLAUDE.md) |

## Decided choices about the game

| When | Decision | Owner's words | Pages |
|---|---|---|---|
| 2026-11-11 04:04 | Larder stalemates: both a late end to food drops and a 15-game-minute cap won on share of the map held | Card: "Both" (food stops arriving late in the round, and a cap at 15 minutes catches any stalemate left) | [Larder](rts/shooter.md) |
| 2026-11-11 03:38 | Larder squad: starts at 4 soldiers and can grow to 40; soldiers die, attacked by the mould; the round ends when every soldier or all the mould is dead | "Start with 4 and can grow to 40. They should definitely die and be attacked by the slime. The round ends when the soldiers or slime are all dead?" | [Larder](rts/shooter.md) |
| 2026-11-11 03:14 | Larder (mould-as-enemy shooter) is controlled RTS-style: orders, drawn fire lines and rings, not twin-stick aiming; next, a full build plan | "Rts-style. When you finish your current task, let's develop a full detailed plan of how to build the demo" | [Larder](rts/shooter.md) |
| 2026-11-11 01:50 | Shooting must cut a branch in seconds, and a held ring of fire must stop all mould reaching the food | "holding a ring of fire should stop all mold from getting to the food" | [Larder](rts/shooter.md) |
| 2026-11-11 00:30 | Larder: a squad, solo against the mould, tests first; no browser build for the full game | "Squad, start just you against the mold, tests first" | [Larder](rts/shooter.md) |
| 2026-11-10 22:32, card | The GPU mould aims for 100,000 walkers on 1024 x 640 (phase 2's gate), over 32,000 or stopping the GPU work | card ("100,000 walkers", on "Which scale should the GPU mould aim for?") | [scale](model/scale.md) |
| 2026-11-10 22:06, card | Two Fevers version 3 (calls, fever by tube, pinch, foraging, starvation; [design](../Reports/two-fevers-2026-11-10/v3-design.md)), built after three cheap checks, after the owner found version 2 "overly simple" with the mould as "background" (21:52) | card ("Build it after the checks (recommended)") | [Two Fevers](rts/two-fevers.md) |
| 2026-11-10 23:29 | The field skirmish's counterplay: explore changes to the model itself (the fed fight bonus, the head-count share, a cost of size) rather than more prices | On "The fix probably has to be in the model itself": "then lets explore this" | [field skirmish](rts/skirmish.md) |
| 2026-11-10 21:17 | The field skirmish's next version spends a bank: food into a bank, beacons, spores, costed habits; explore how it plays and whether one strategy wins | "This is an interesting direction. Brainstorm/explore how these tools/game plays out. What different strategies would different players use to win. How complex or interesting are the strategies? Does one strategy beat all the others?" | [field skirmish](rts/skirmish.md) |
| 2026-11-10 21:14 | The field skirmish's first version (habits, salt, the lamp) set aside after play | "Overall, user input seems limited and doesn't lead to any complex stratgy" | [field skirmish](rts/skirmish.md) |
| 2026-11-10 20:42, card | Build both RTS demos at once, in two threads: the field skirmish and Two Fevers | card ("Both, in two threads") | [RTS directions](rts/directions.md) |
| 2026-11-10 20:33 | After the cheap, parallel tests already planned, build playable demos rather than more harness testing | "You can run these tests, especially if they are cheap and/or can be paralellized. But then lets focus on actually building playable demos" | [RTS directions](rts/directions.md) |
| 2026-11-10 21:15, card | Two Fevers' next version: owned ground and one host-shaping trait together (after the owner found no strategy in version 1, 21:11) | card ("Both together") | [Two Fevers](rts/two-fevers.md) |
| 2026-11-10 19:22 | Build sound for the game (HANDOFF decision 8), from Pixel_Physics's `src/audio/` | "yes to sound" | [Sound](looks/sound.md) |
| 2026-11-10 18:56, card | Trace Keep It Alive's luck now rather than wait until the owner has played (HANDOFF decision 1; The Wild One's scoring half was already done in PR #6). The trace had been run meanwhile on that recommended option | card ("Trace it now") | [Keep It Alive](dishes/keep-it-alive.md), [luck report](../Reports/mould-keep-alive-luck-2026-11-10.md) |
| 2026-11-10 17:54 | GPU runs go on the owner's home desktop, a GeForce GTX 1070 (the GPU Jenson used), which is not the machine they usually work at | "my home machine has a geforce 1070. Not sure if we can access it right now." ... "It is not the computer that I can currently working on though." | [scale](model/scale.md) |
| 2026-11-10 17:52 | No on-screen readability probe for the RTS yet: still early planning | "readability can be solved, I think we are still in the early planning stages, so we don't need that yet." | [RTS brainstorm](../Reports/rts-design-2026-11-10/brainstorm/README.md) |
| 2026-11-10 17:51, card | Run the wiki's success check first, then add the next pages (oats, salt, the lesson and puzzle dishes, sensors, the look) and ingest review 1 | card ("Both, check first") | [plan report, *Success check*](../Reports/llm-wiki-plan-2026-11-10.md) |
| 2026-11-10 17:50 | Start the GPU plan; asked whether it needs more threads or whether to proceed alone (answer given: the plan's thread builds phases 1 and 2, the Pixel_Physics review thread the dish hash) | "How many threads do we need to start your plan or do you want to just proceed with the implementations yourself?" | [scale](model/scale.md) |
| 2026-11-10 17:26 | `HANDOFF.md` stays the hand-written front page; its status section becomes one linked line per subject, and it keeps the open decisions and next steps | "Sure" (to that recommendation) | [HANDOFF](../HANDOFF.md) |
| 2026-11-10 17:25 | The wiki lint stays in CI | "lint can stay in the CI for now." | [README](README.md) |
| 2026-11-10 17:24 | Plan agreed; build the wiki to grow with the repo | "I agree with the plan, but note that this repo is brand new so it will grow significantly." | [README, *Growth*](README.md) |
| 2026-11-10 16:49 | Build the wiki, starting with seven pages | "You can start with the first 7 pages, we can always revise once the review comes back" | [index](index.md) |
| 2026-11-10 16:04, card | The first out-of-the-dish game is the forest floor seen from above (not side-on) | card | [real-world report, §5](../Reports/mould-real-world-2026-11-09.md) |
| 2026-11-10 14:49 | The from-the-simulation look is the default; F7 keeps the tube look | "much better" | [HANDOFF](../HANDOFF.md) |
| 2026-11-10 03:06 | Order for depth: the endless dish, then water and sleep (tools that work on each other through damp ground), then a wild second mould; later, if those play well, melting salt, spores, a rival the player steers, same-kind moulds joining | "Agreed" | [Keep It Alive](dishes/keep-it-alive.md), [water and sleep](model/water-and-sleep.md), [The Wild One](dishes/the-wild-one.md) |
| 2026-11-10 03:00 | A rival mould that fights and steals | "Also wondering about having multiple molds that can fight or steal resources from each other" | [The Wild One](dishes/the-wild-one.md) |
| 2026-11-10 02:58 | More depth, not more tools | "We also need to add complexity to the game. The tools are quite one dimensionally right now" | [water and sleep](model/water-and-sleep.md) |
| 2026-11-10 02:57, card | The keep-it-alive endless dish first | card | [Keep It Alive](dishes/keep-it-alive.md) |
| 2026-11-09, card | Keep 22.5 degrees and find it new goals (after the 22.5 vs 45 page); dishes now set their own angle | card ("Keep 22.5, new goals") | [sensors](model/sensors.md) |
| 2026-11-08, card | Each piece of mould eats only the food it is joined to (`Params::pieces`) | "Yes, each piece eats its own food" | [the knife](tools/knife.md) |
| 2026-11-08 | The mould game is the most promising of the eight demos; it needs instruction and a challenge | "the most promise so far" | [probe report](../Reports/mould-probe-2026-11-07.md) |

## Sources

- [CLAUDE.md](../CLAUDE.md), *What the owner has ruled for this game*.
- [HANDOFF.md](../HANDOFF.md), *Where the game stands* and *Decisions the owner has not made*.
- [mould-review-thread-handoff-2026-11-10.md](../Reports/mould-review-thread-handoff-2026-11-10.md), *What the owner said in this thread*.
- [mould-real-world-2026-11-09.md](../Reports/mould-real-world-2026-11-09.md), §5.
- The `handoff/project-memory/` notes linked above (a frozen snapshot of the old project's memory; do not edit them).
