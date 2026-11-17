# Handoff: gun cards (the gun-card rethink), 2026-11-17

The thread "Rethink the gun card system" (owner, 11-16 02:34Z: "five
different categories of bullets and two of them are just two different
sized explosions"). The plan, steps A-F, is
`Reports/gun-system-review-2026-11-16/README.md`. The wiki pages are
`wiki/world/guns.md` and `wiki/world/gun-cards.md` (each card, its planet,
what you see). The ROADMAP row is N10.

## State

- **Done and merged**:
  - A: set shapes, blast and bomb apart, germ sold.
  - B: trigger, expire, multicast, with the GPU's kill read-backs batched.
  - C: seek (homing in the shot trace). Aim cards wait.
  - D, all families:
    - Mind: blind, sour, spin.
    - Readers: shatter, detonate.
    - Route: the Flare, plus the germ retuned per planet.
    - Element: the Torch and Tinder. `fire_weight` was deleted; `fire_alarm` is the one fire-shyness switch.
    - Force: counts as built (the shove cards).
    - Cut: no Cut card. The cut count, #435: one thin cut frees under 2% of the slime in 21 of 22 counts (`Reports/gun-cut-count-2026-11-17/`).
- Each step's report is in `Reports/gun-*-2026-11-1x/`. Its page source is in that folder's `page/`. The old claude.ai page links will not open on the new account, so open `page/index.html`.

## Open branch (not a PR)

- `claude/project-thread-vx9t3p-wip`, commit 9294ed83: **Provoke and Still as Mind cards**. These are the simulation-use thread's `MIND_DEAF` and `MIND_STILL` statuses, laid for the devices' 12 s and 10 s through `BurstRules::mind_steps_for` on both engines.
  - Done: shells sold from planet 2, icons, faces, H decks. Unit tests pass.
  - Not done: clippy not run, not filmed, windows partial (`Reports/gun-provoke-still-2026-11-17/raw/windows-partial/`). The README is a draft with PAGE, STATUS and PLAY placeholders.
  - To resume: rebase that commit onto main, run `raw/windows.sh` and `raw/film.sh` (planet 1's raid at 59 s is the scene), fill in the report, then open a PR and label it `queue`.

## Next

1. Finish Provoke and Still (above).
2. Step E: one device slot per squad on a cooldown. The hook is `Model::throw` (#379's throw aim).
3. Step F: luck cards that change rules.

## Waiting on the owner

- The Mind cards' blind quiz. The key is in `Reports/gun-mind-2026-11-16/README.md`.
- Whether Heave and Corral merge into one card. The pick is in the 11-17 playtest pack (#433).
- The Torch's and Tinder's costs (2) wait for the balance pass. The game designer called it.
- Timing the Mind statuses on the GTX 1070.

## Handed to others

- The run shop should offer Tinder before the fire-shy Sprawler's planet (game designer, 15:26Z). This went to the run-progression thread as a `Run::restock` rule. This thread offered to do it.

## Working rules this thread used

- **Judge a card by paired windows from one game.** Load the deck at T with `load=T,CARDS` and compare 10-20 s windows. Put the hit % beside the kills.
- **Card tuning notes go straight to the game designer**, not to the look thread.
- **A PR that touches a report must touch every wiki page citing it.** Run the wiki lint after committing.
- **Plan for slow headless runs.** Runs of planets 5-8 on llvmpipe take 20-30 min each. Give background runs a long timeout.
