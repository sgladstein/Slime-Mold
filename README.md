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

**Picking this up? Start at [HANDOFF.md](HANDOFF.md):** where the game
stands (one line per subject, linked to its [wiki](wiki/index.md) page), the
owner's open decisions, and the next steps. [ROADMAP.md](ROADMAP.md) puts
every active plan (game fixes, GPU, RTS, Pixel_Physics ports) in one order.

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
| `src/sound/` | the game's sound: `listen.rs` reads the dish, `synth.rs` the voice bank, `device.rs` the only cpal file (S toggles in the window) |
| `src/mould/level.rs` | the challenge dishes, stars, the endless dish (Keep It Alive) and The Wild One |
| `src/mould/lesson.rs` | the opening lesson, one tool at a time |
| `src/mould/look.rs`, `render.rs` | drawing: the game's look, the looks pass (walkers, food ramp, smoothing, stain, lamp light) |
| `src/mould/wander.rs` | The Wanderer prototype |
| `src/mould/organ.rs` | the Mould Organ prototype |
| `src/mould/tune.rs` | the T tuning panel |
| `src/gpu/` | the big dish on the GPU: `engine.wgsl` and `Engine`, shared by the sandbox and the Larder world (wgpu through pixels) |
| `src/bin/mould.rs`, `src/bin/organ.rs` | the windows (winit + pixels only) |
| `src/bin/mouldgpu.rs`, `src/bin/larder.rs` | the GPU sandbox's window, panel and presets; the Larder window |
| `src/hud.rs`, `src/hud_crisp.rs`, `src/wav.rs` | copied from Pixel_Physics: font and WAV writer |
| `examples/` | the tools, below |
| `wiki/` | what is true now, one page per dish, tool and mechanism, each citing its reports: [wiki/index.md](wiki/index.md) |
| `Reports/` | every report, review, brainstorm, page source and raw census output: [Reports/README.md](Reports/README.md) |
| `research/engine-sketches/` | code that only runs inside Pixel_Physics, kept for reading |
| `research/sources/` | one card per outside source (papers, Jenson, other implementations): how fully read, local copy, licence, key facts: [research/sources/README.md](research/sources/README.md) |

## Tools

| Tool | Answers |
|---|---|
| `cargo run --release --example mouldprobe` | scripted hands (idle, random, plan and named hands) over paired seeds on any dish: counts, stars, GIFs and stills of the window's own frame, frame cost. The census instrument behind every report. |
| `cargo run --release --example mouldlooks` | the same dish at the same instant in each look, for before/after pictures and draw cost |
| `cargo run --release --example mouldhand` | plays a dish from a written hand (`T lamp X Y DUR`, `T oats X Y`, ...) and writes frames; hands in `Reports/mould-review-2026-11-10/` |
| `cargo run --release --example mouldsound` | hear a dish offline: plays a dish with a hand (idle, random, plan, tour) and writes what the window would play to a WAV, with the sound counters and the listener's cost |
| `cargo run --release --example organprobe` | the organ: timing wobble, lamp bends, offline WAV and frames |
| `scripts/crispfont.py` | regenerates `src/hud_crisp.rs` from JetBrains Mono; add a glyph here, never by hand |
| `Reports/mould-scratch-2026-11-10/tune/run.sh`, `run5.sh`, `w5.py` | census runners (4-wide) and the paired sign-test summariser used for Keep It Alive and The Wild One |

## Checks

```
cargo test --release
cargo clippy --all-targets --release --locked -- -D warnings
```

CI runs both on every pull request and every push to main, plus a build of
the organ's sound device for macOS and Windows (`.github/workflows/ci.yml`).
A pull request that changes only `Reports/`, `handoff/`, `research/`, `wiki/`
or `*.md` skips both build jobs. Sound on Linux needs
`--features audio-alsa` and `libasound2-dev`.

**Licensing** (`.github/workflows/notices.yml`, runs only when `Cargo.toml`,
`Cargo.lock`, `about.*`, `licenses/` or the scripts change):
`bash scripts/licensecheck.sh` fails on a dependency with no permissive
licence, and `bash scripts/notices.sh --check` fails when
`THIRD-PARTY-NOTICES.txt` no longer matches `Cargo.lock`. After a dependency
change, run `bash scripts/notices.sh` (needs `cargo install cargo-about
--version 0.9.2 --locked --features cli`) and commit the file. The crisp
font's table is generated: `python3 scripts/crispfont.py` (Pillow, and
JetBrains Mono at the path in the script).
