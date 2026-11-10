# Prototype lanes: what all eight share (2026-11-07)

Scott asked for three prototype demos, one per thread, from the new-game brainstorm, then expanded it to six (16:18Z). Probes 1-3 are lightning, Mould and Firedamp; 4-6 are Seep, Quench and Fault. At 17:38Z he added 7-8, the top two from the plant-engine explorer (`explorer-e-plants.md`): Kudzu and Heartwood. The page with all the ideas and the reviewers' verdict is https://claude.ai/artifact/2b3KnLZGmNCq5zmBReufvR. The source files are in `/mnt/project-files/game-design/new-game-brainstorm-2026-11-07/`.

## The goal of a probe
Answer one question about **fun in the hand** in about 2 to 4 days of work. This is not a game. Build the least that lets Scott, or a fresh player, feel the core verb and see the world answer it. If the answer is no, a fast no is a good result.

## The owner's bar
These come from the failed "satchel" card prototype; see `brief.md`, "What this project already learned".
- **Every action shows a visible consequence within a second.**
- **Use plain world words on screen.** No jargon or internal names. The screen says what your next action will do.
- **Doing nothing must lose**, and random play must lose to a plan.
- **Outcomes are graded**: stars or a score, never just win or lose.
- **There is always a next thing to do**: a restart, the next level, or play on.
- **The world answers with its own levers.** No generic explosions bolted on.
- Cards are optional. Use them only where the *order* of plays is the fun.

## How to build it
- **A new binary of its own**: `src/bin/<name>.rs`, with its code under a new folder `src/<name>/`. It must not be a lab seat. These are separate games on the same engine.
- **Touch shared engine files (`src/sim/**`) only when you must.** Keep each change small and additive, behind your game, and default-off for every other binary, so it cannot move the lab, outdoor or held-world results. Before editing a shared file, run `bash scripts/branchcheck.sh --who-touched <path>`. If you change anything the lab's ants or plants read, tell the Teacher lane first.
- **Fixed world size for the probe**: about 256x160, the size that reads on a phone. Draw thin things 2 to 3 cells wide.
- **A headless harness** (`examples/<name>probe.rs`, with a row in `Reports/instruments.md`) that:
  - runs scripted play, including a do-nothing control and paired seeds;
  - prints counts;
  - writes GIFs or PNGs of the real window;
  - reports the `ascii`-style worst and mean frame.
- **Run from the repository root.** The lab's `Scenario::load` is relative to the working directory, and the same trap may apply to you.
- **Follow `CLAUDE.md`.** In particular:
  - look before you measure;
  - put pictures on an Artifact page with the counts beside each one;
  - never attach pictures to chat;
  - commit messages carry the measurement;
  - never `git add -A`.

## Engine facts already checked (2026-11-07)
- `weather::strike` (`src/sim/weather.rs`) only draws a bolt; it changes no cells. Each strike frame, and the one after it, forces a full-screen redraw.
- Heat does **not** push the air grid. The air grid (`src/sim/field.rs`) works at `FIELD_SCALE = 16`, in 16x16-cell blocks, which is too coarse to steer anything down a narrow tunnel.
- The fine per-cell trail field (`src/sim/pheromone.rs`) has two general channels plus an alarm channel at single-cell resolution. You can deposit into it, sample it and scale it per cell. Nothing makes it drift with wind yet.
- Under a roof, steam drips back where it condensed. Lava crust crumbles and sinks (`wiki/liquids-and-gases.md`).

## How to land it and report
- Push to your own `claude/` branch, open a PR, and hand it to the Merge desk thread with the PR number and head SHA. Don't merge it and don't watch CI.
- Before pushing, run only your module's tests and clippy (`CLAUDE.md` Commands).
- Show Scott a page with:
  - the paired GIFs (do nothing against planned);
  - the counts beside them;
  - the frame cost;
  - the one question for him;
  - the exact `cargo run` line to play it.
- The probe is done when Scott can play it and the page answers the question. Leave the thread open for his verdict.
