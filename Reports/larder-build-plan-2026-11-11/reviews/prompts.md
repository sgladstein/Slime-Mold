# Prompts given to the three independent reviewers (2026-11-11 04:07Z)

Each reviewer got the shared preamble, then its own brief. None saw the others' output or the author's opinion of the plan.

## Shared preamble

You are reviewing a build plan, independently and adversarially. Repo: /home/user/Slime-Mold (a Rust slime-mould simulation turned into games). Read CLAUDE.md first (the owner's rulings), then the plan: Reports/larder-build-plan-2026-11-11.md. Background as needed: Reports/mould-shooter-2026-11-11.md (the tests behind the plan), wiki/rts/shooter.md. Do not edit any file. Return a ranked list of findings (most important first), each with: the problem, the evidence (file:line or a number from a report), and the concrete change to the plan you recommend. End with the three changes you would make if you could make only three. At most 900 words.

## Reviewer 1: engineering

Judge whether the plan can be built as written on this codebase. Read src/mould/mod.rs (the Dish, Ground, stroke_dab, pieces, beacons, wounds, step), examples/larderprobe.rs (the harness), src/bin/mould.rs (the native window) and src/mould/skirmish.rs. Look for: APIs the plan assumes that don't exist; work that is much bigger than it looks (soldiers smelling like food, walkers attacking soldiers, dead soldiers becoming piles, timed drops, a 15-minute cap); frame cost of 40 soldiers each firing and sensing; determinism and replay; milestone ordering and what could be merged or cut; anything that breaks other dishes or tests.

## Reviewer 2: game design

Judge whether the demo described will be fun and give several strategies that counter each other, by the owner's bar in CLAUDE.md and the memory notes there (earlier demos were rejected as too simple, 1-2 tools, one strategy; the mould must be integral). Look for: whether the four plans really counter each other or collapse into one; whether 4 to 40 soldiers and RTS orders fit a 256x160 field; whether the mould's answers are readable to a player; the round end (wipe, late food stop, 15-minute cap); what an RTS player would expect that is missing; anything that is drawn-only rather than simulation. Compare with known games where it helps (Creeper World, They Are Billions, Pikmin).

## Reviewer 3: test method

Judge whether each milestone's gate is measurable, meaningful and reachable, by the method in CLAUDE.md (look first, name the deciding measure before the run, paired seeds, sign test, Holm correction, idle and random controls, frame cost quoted). Look for: gates that can pass while the game is still bad, or fail while it is good; scripted plans that are under-specified; whether a "best on no more than 12 of 16 seeds" plus a counter cycle is the right test for "plans counter each other" when the mould is not a player who picks; census run time (the priced-ring census took 528 s for 6 plans x 3 prices x 16 seeds); what the outcome spread (who was wiped, when, soldiers left) should be reduced to for a sign test.
