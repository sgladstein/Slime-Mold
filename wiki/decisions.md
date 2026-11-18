---
title: The owner's rulings and decided choices
kind: method
status: current
updated: 2026-11-18
sources:
  - CLAUDE.md
  - Reports/continuation-plan-2026-11-17.md
  - Reports/handoff-2026-11-17.md
  - Reports/mould-real-world-2026-11-09.md
  - Reports/mould-review-thread-handoff-2026-11-10.md
  - Reports/mould-sound-2026-11-10.md
  - Reports/roadmap-2026-11-17.md
  - Reports/rts-design-2026-11-10.md
  - Reports/testing-practice-2026-11-12.md
  - Reports/whole-game-review-2026-11-15/README.md
code:
  - Params::pieces
---

# The owner's rulings and decided choices

Every ruling and every decided choice, with the owner's words and the time
they were said, newest first in each group. **Open decisions live in one
place only: [NOW.md, *What waits on the owner*](../NOW.md)** (since
2026-11-17; before that, the old HANDOFF.md's list). The Direction lane keeps these
pages. When the owner answers one, move it here with the quote and time, and
update the pages it changes in the same commit. Never write an answer the
owner did not give; a recommendation on a card is not his answer.

Times are UTC. "Card" means he tapped an option. Old-account thread ids and
artifact links are left out; the quotes are kept in the sources named.

## Standing rulings (how all work is judged and shown)

| When | Ruling | Owner's words | Source |
|---|---|---|---|
| 2026-11-17 19:21, card | New slime rules are written and tested on the GPU engine only (on the software GPU in the cloud and in CI): no new CPU twin and no new twin test. Today's CPU engine (`src/mould/big.rs`) stays as it is for the tests and the census that stand on it, and whether to remove it is decided at Milestone 2. This closes the whole-game review's D8 | "So what is the point of the CPU then. I thought we start with CPU because it is easier to write and test in the cloud threads. If we start in the GPU (which I am not against) is there any reason to even have the CPU twin?" (19:07, to "Build new slime rules on the GPU first, and write the CPU twin only when a rule becomes a default (D8)?"); card "GPU only for new rules, keep old CPU" (over "Retire the CPU engine now" and "Keep full twins as today") | [plan](../Reports/continuation-plan-2026-11-17.md), [CLAUDE.md](../CLAUDE.md), [scale](model/scale.md) |
| 2026-11-17 19:07 | Go on the continuation plan. One milestone at a time with exit tests, the first being "Three planets" (planets 1 to 3 as one 30-minute sitting); five standing lanes that each own files, plus one lab thread; `NOW.md` as the one front page in place of HANDOFF and ROADMAP; lighter records (a report only for a measurement others will cite, a dead end, or a page for the owner) | "1) Yes" (to "Start the five lanes on this plan?") | [plan](../Reports/continuation-plan-2026-11-17.md), [NOW.md](../NOW.md), [CLAUDE.md](../CLAUDE.md) |
| 2026-11-17 19:07 | Models: Fable for the main threads, Opus or Sonnet for simpler tasks, Haiku for pure runners, at medium to extra high effort by the task | "Fable for all of the main threads because we have a Fable usage reset on this account. Opus/Sonnet can be used for simplier tasks. Haiku for pure runners. Recommend Medium to Extra effort based on the complexity/importance of each task." (his answer 9, to question 10: which models should the lanes run) | [CLAUDE.md](../CLAUDE.md) |
| 2026-11-17 19:07 | Films and stills are kept on an archive branch of this repository, off main. Read as yes to the recommendation: the question was an either-or (an archive branch or a second repository) and the archive branch was recommended. He gave nine answers to ten questions, so the lab question (9: steering the slime as its first prototype) has no answer and goes ahead on the recommendation | "8) Yes" | [plan](../Reports/continuation-plan-2026-11-17.md) |
| 2026-11-17 19:07 | Old thread branches whose work is already on main are deleted, and each branch is deleted when its pull request merges; branches with unmerged commits stay. The repository switch "Automatically delete head branches" is his to turn on | "7)Yes" | [plan](../Reports/continuation-plan-2026-11-17.md) |
| 2026-11-17 19:07 | Never assign the owner to a pull request or request his review: the 11-14 rule is kept, and is in the new account's project instructions | "6)Yes" (to "Keep the rule 'never assign me to a pull request or request my review'?") | [CLAUDE.md](../CLAUDE.md) |
| 2026-11-17 17:39 | `play.cmd` may send the game's play logs to this public repository, so threads can see when he played, on which build and with what on | "\"play.cmd may send play logs to the public repository.\" - yes play logs is a great idea" | [plan](../Reports/continuation-plan-2026-11-17.md) |
| 2026-11-17 17:25 | The game is not unplayed, and he cannot playtest on a thread's schedule. So no thread asks for a playtest or ends its work on "awaits the owner's playtest": work lands on, says what he will see, and is done | "Note that I have playtested many times. the threads don't always know that and I often cannot playtest when a specific thread asks. But the game is not unplayed." | [plan](../Reports/continuation-plan-2026-11-17.md), [CLAUDE.md](../CLAUDE.md) |
| 2026-11-17 17:25 | Features that land switched off are a problem: he must be able to tell what is on, what is off and what needs testing. The plan's rule from it (approved 19:07): everything lands on by default; at most three switches are off at once, each declared in one list in code, shown on the game's Experiments tab and ended at his next play; the game lists what is new and up to five things to try. This tightens "Defaults on" below | "this is a problem. There is a lot of work going on in parallel and when I have time to playtest, I don't know what is on/off and what needs to be tested." (on the draft's "Features land switched off. 14 switches are built but off, each waiting for a pick.") | [plan](../Reports/continuation-plan-2026-11-17.md), [CLAUDE.md](../CLAUDE.md) |
| 2026-11-17 17:25 | Improving the visuals is an important factor, and a plan needs specific direction for them. The order of look work he set at 17:39 is in [decisions-game.md](decisions-game.md) | "I didn't see any specific directions about improving the visuals, but that is an important factor" | [plan](../Reports/continuation-plan-2026-11-17.md), [art direction](looks/art-direction.md) |
| 2026-11-17 17:12-17:13, four cards | The aim, the next phase, the number of threads and how choices reach him. Release work stays parked until he says. Threads make planets 1 to 3 fun and finished before adding systems, with one thread free to prototype new simulation ideas. Threads pick looks and tuning and he vetoes in play; he is asked only the big questions, at most five a session. So a blind A/B is kept only for a choice that sets the whole art direction | cards: "A game I love to play; release maybe"; "Slice first, plus one lab thread"; "About 5 standing lanes"; "Threads decide small, ask big" | [plan](../Reports/continuation-plan-2026-11-17.md) |
| 2026-11-15 08:35 | After a multi-agent review, anything every reviewer agrees on is built without waiting for the owner (sent to the owning thread, or a new thread); a decision without a clear answer waits for the owner, listed with a recommendation | "After the main work is in. Anything where everyone agrees, just start building. If there are decisions to be made without a clear solution that we can wait to discuss" | [whole-game review](../Reports/whole-game-review-2026-11-15/README.md) |
| 2026-11-14 03:52, card 03:55 | PRs merge through a merge queue instead of each thread re-merging main and re-running CI. GitHub's queue is for organization-owned repos only, so the queue is our own Actions workflow (`.github/workflows/merge-queue.yml`, label `queue`) rather than moving the repo to an org | "yes" (to setting up a merge queue); card "Build our own queue in Actions" | [CLAUDE.md](../CLAUDE.md) |
| 2026-11-12 23:14 | The project stays proprietary, all rights reserved, so it can be sold later: no open-source licence. [LICENSE](../LICENSE) grants nothing beyond reading; every dependency must offer a permissive licence (`scripts/licensecheck.sh`). The repo being public (since 2026-11-12 01:40, for Actions billing) does not change this | "Can we add a license to this project on GitHub in case I decide to want to monetize it later? I don't want open source" | [LICENSE](../LICENSE) |
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

- [CLAUDE.md](../CLAUDE.md), *What the owner has ruled*.
- [The continuation plan](../Reports/continuation-plan-2026-11-17.md) and its thread (2026-11-17): the 11-17 rows above; the quotes are the owner's messages there, the questions the planning thread's.
- [The old HANDOFF.md](../Reports/handoff-2026-11-17.md), *Where the game stands* and *Decisions the owner has not made*, and [the old ROADMAP.md](../Reports/roadmap-2026-11-17.md): the front pages until 2026-11-17, kept as history.
- [mould-review-thread-handoff-2026-11-10.md](../Reports/mould-review-thread-handoff-2026-11-10.md), *What the owner said in this thread*.
- [mould-real-world-2026-11-09.md](../Reports/mould-real-world-2026-11-09.md), §5.
- The `handoff/project-memory/` notes linked above (a frozen snapshot of the old project's memory; do not edit them).
