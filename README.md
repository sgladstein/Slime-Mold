# Slime Mould

Herd a slime mould across a dish with a lamp, oats, salt, a knife and water.
The mould is a real simulation (Jeff Jones's walker model of *Physarum*): a
few thousand walkers lay and follow a trail, and the tubes, loops and dead
ends you see are that trail. Everything on screen is the simulation, nothing
is drawn on top of it.

```
cargo run --release --bin mould            # the game: lesson, challenge dishes, free dishes, the endless dish
cargo run --release --bin mould -- 3       # free dish seed 3   (-- 0 the lesson, -- d2 challenge dish 2, -- wander the Wanderer)
cargo run --release --bin organ            # side prototype: the network plays a rhythm
```

**Picking this up after the 2026-11-10 account move? Start at
[HANDOFF.md](HANDOFF.md):** where the game stands, the owner's open
decisions, and the next steps.

Keys are on screen. F2-F6 cycle the looks, T opens the tuning panel (saved
to `mould-tune.txt`), M the dish picker, H the briefing card.

## Where it came from

The game was built inside [Pixel_Physics](https://github.com/sgladstein/Pixel_Physics)
as one of eight prototypes from the 2026-11-07 new-game brainstorm, picked by
the owner on 2026-11-08 as the most promising, and moved to this repository
on 2026-11-10. **The git history of every file that came across is kept**,
back to the first probe commit of 2026-11-07, so `git log --follow` works
here as it did there. It never used the falling-sand engine: the only two
engine pieces it drew with, the bitmap font (`src/hud.rs`, `src/hud_crisp.rs`)
and the WAV writer (`src/wav.rs`), were copied in, so the two repos share no
code at build time.

## Layout

| Path | Holds |
|---|---|
| `src/mould/mod.rs` | the model: walkers, trail, food, lamp, salt, snips, water and sleep, the wild rival (`Dish`, `Params`) |
| `src/mould/level.rs` | the challenge dishes, stars, the endless dish (Keep It Alive) and The Wild One |
| `src/mould/lesson.rs` | the opening lesson, one tool at a time |
| `src/mould/look.rs`, `render.rs` | drawing: the game's look, the looks pass (walkers, food ramp, smoothing, stain, lamp light) |
| `src/mould/wander.rs` | The Wanderer prototype |
| `src/mould/organ.rs` | the Mould Organ prototype |
| `src/mould/tune.rs` | the T tuning panel |
| `src/bin/mould.rs`, `src/bin/organ.rs` | the windows (winit + pixels only) |
| `src/hud.rs`, `src/hud_crisp.rs`, `src/wav.rs` | copied from Pixel_Physics: font and WAV writer |
| `examples/` | the tools, below |
| `Reports/` | every report, review, brainstorm, page source and raw census output: [Reports/README.md](Reports/README.md) |
| `research/engine-sketches/` | code that only runs inside Pixel_Physics, kept for reading |
| `research/sources/` | one card per outside source (papers, Jenson, other implementations): how fully read, local copy, licence, key facts: [research/sources/README.md](research/sources/README.md) |

## Tools

| Tool | Answers |
|---|---|
| `cargo run --release --example mouldprobe` | scripted hands (idle, random, plan and named hands) over paired seeds on any dish: counts, stars, GIFs and stills of the window's own frame, frame cost. The census instrument behind every report. |
| `cargo run --release --example mouldlooks` | the same dish at the same instant in each look, for before/after pictures and draw cost |
| `cargo run --release --example mouldhand` | plays a dish from a written hand (`T lamp X Y DUR`, `T oats X Y`, ...) and writes frames; hands in `Reports/mould-review-2026-11-10/` |
| `cargo run --release --example organprobe` | the organ: timing wobble, lamp bends, offline WAV and frames |
| `Reports/mould-scratch-2026-11-10/tune/run.sh`, `run5.sh`, `w5.py` | census runners (4-wide) and the paired sign-test summariser used for Keep It Alive and The Wild One |

## Checks

```
cargo test --release
cargo clippy --all-targets --release --locked -- -D warnings
```

CI runs both on every pull request and every push to main, plus a build of
the organ's sound device for macOS and Windows (`.github/workflows/ci.yml`).
A pull request that changes only `Reports/`, `handoff/`, `research/` or
`*.md` skips both build jobs. Sound on Linux needs
`--features audio-alsa` and `libasound2-dev`.
