---
title: The owner's rulings and decided choices
kind: method
status: current
updated: 2026-11-12
sources:
  - CLAUDE.md
  - HANDOFF.md
  - Reports/mould-real-world-2026-11-09.md
  - Reports/mould-review-thread-handoff-2026-11-10.md
  - Reports/mould-sound-2026-11-10.md
  - Reports/rts-design-2026-11-10.md
  - Reports/testing-practice-2026-11-12.md
code:
  - Params::pieces
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
| 2026-11-12 17:26, card | Build first, balance later: a feature or look lands when it works and can be seen; a census is not its gate. Balance never blocks looks or features (retune the numbers; never hold one area for balance in another). A census only answers a strategy question (player AI, a shared default, dominance before a milestone or playtest, mission difficulty, an owner question), named with its runner time, under about an hour without the owner's OK | "It seems like for every feature we build we run lots of tests for balance to see if it beats random. Sometimes this is hours of testing." ... "We were developing bad looking slimes because more runners killed the player too easily (instead of rebalancing)." (17:00); card "Adopt as written" | [review](../Reports/testing-practice-2026-11-12.md) |
| 2026-11-11 15:28, card | After the second success check tied the wiki with the reports (3-2, 33 ties), shrink the wiki to short pages with no Numbers tables: keep the index, decisions, lint and short Now pages that link the reports; the numbers stay in the reports | card ("Short pages, no numbers tables") | [plan report, *Success check, second run*](../Reports/llm-wiki-plan-2026-11-10.md) |
| 2026-11-11 15:16-15:17 | The wiki upkeep suggestions are all agreed: a `world/` folder for Larder World, a mould-tier page, split oversized pages, a `paused` status for the old dish and tool pages, a slimmer HANDOFF, one-sentence index lines. Page size is capped in bytes (12 KB a page, 2 KB a Now), not 300 lines, after the owner asked whether 300 was right. Paused pages may still be changed by a thread working on them directly | "agreed to everything, but first, is 300 lines a good limit or too small?"; "if an agent is directly working on these, then they can change them though" | [README](README.md) |
| 2026-11-11 04:11 | Lean on published knowledge of what these simulations' parameters do instead of rediscovering it by sweeps; judge a new rule by whether it makes a creature for play, not by whether it makes a new organic pattern | "There's a lot of information out there about these types of simulations and what the parameters do, so we don't necessarily need to ReDiscover everything ourselves. Also at some point this just becomes an organic pattern generator and not really slime mold. Not that the point of what we're doing is also to generate slime mold, but it generates some sort of creature for gameplay and not just random organic patterns" | [outside rules](../Reports/outside-rules-2026-11-11.md), [scale](model/scale.md) |
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

## Decided choices about the games

They are in [decisions-game.md](decisions-game.md) (split 2026-11-11 for
size). Add a new game decision there, newest first.

## Sources

- [CLAUDE.md](../CLAUDE.md), *What the owner has ruled for this game*.
- [HANDOFF.md](../HANDOFF.md), *Where the game stands* and *Decisions the owner has not made*.
- [mould-review-thread-handoff-2026-11-10.md](../Reports/mould-review-thread-handoff-2026-11-10.md), *What the owner said in this thread*.
- [mould-real-world-2026-11-09.md](../Reports/mould-real-world-2026-11-09.md), §5.
- The `handoff/project-memory/` notes linked above (a frozen snapshot of the old project's memory; do not edit them).
