# Pixel_Physics review, and the dish hash (2026-11-10)

The owner's three questions (2026-11-10 16:47Z): is there engine code in
Pixel_Physics worth bringing into this game, are there tools worth bringing,
and is anything that did come across wrong for the mould. The full answer,
with tables, is the Claude Doc
[Pixel_Physics review for the mould game](https://claude.ai/artifact/Pe83NVdRMXhPvxeYoKASRu);
this report keeps its findings in the repo. The owner agreed to its
suggested order (17:49Z: "Agreed"). Step 1, the dish hash, is built here.

Read at Pixel_Physics `c3588b3` (main, 2026-11-10) and Slime-Mold `b8efc14`.

## The frame, split

`mouldprobe seeds=1 players=plan`, the cost line (free dish seed 1, the plan
hand, window frame at scale 4, 5,880 frames, peak 1,797 walkers, 4-core
container, one run): **mean 4.93 ms = simulation 1.10 + drawing 3.83**;
worst 14.94 ms (worst simulation alone 6.84). A second and third run on
this branch read 4.77 (1.08 + 3.69) and 4.97 (1.13 + 3.84). Drawing is about
78% of the frame, so engine work on the trail field moves census speed more
than play.

## Findings

**Engine code worth taking**, best first:

1. Drawing on all cores (`src/render.rs` about lines 5346-5400 there):
   rows on `rayon`, with a test that the parallel frame is byte-identical
   to the serial one. Aims at the 3.83 ms. `look.rs`'s five `Mutex` caches
   would have to be filled before the split. Not measured here.
2. Sound from the simulation (`src/audio/`: a read-only listener, a
   pure-function synth, one gated device file). The game window is silent.
   A new feature, so the owner's call.
3. Keyed random draws (`src/sim/rng.rs` `stream`). The dish's one `Rng` is
   drawn at 24 sites, so one added draw reshuffles every later walker and
   paired arms differ by luck as well as by the change. Changes every
   seed's outcome once.
4. Skipping empty trail tiles and rows (`src/sim/pheromone.rs`). Could fix
   the dead-dish slowdown (`mould-probe-2026-11-07.md`: a dish whose mould
   has died steps about 3x slower, "probably subnormal floats",
   unverified).

Not worth taking: the `u16` trail (it exists for memory at 8192x2560), the
chunked CA sweep, `fxhash` (two hash containers here), the RON/notify
tunables (`tune.rs` already does it), and everything about sand, plants,
ants and worlds.

**Tools worth taking:** a determinism test and a hash line (built below);
a per-phase frame profiler (`examples/frame_profile.rs`; the GPU-plan thread
builds it); the licence and notices gates (`scripts/licensecheck.sh`,
`scripts/notices.sh`, `notices.yml`); the font generator
(`scripts/crispfont.py`, which `hud_crisp.rs` names but which did not come
across); `scripts/docscheck.sh`; one shared, tested paired summariser in
the shape of `labpair.py` instead of one-off scripts such as `w5.py`. Not:
the review queue (retired by the owner 2026-10-21), `batch.yml` (this repo
is private, so runner minutes are billed), the index and read-guard scripts
built for 100k-token reports.

**What came across that does not fit:** the font is capitals only, a
scoping made for the lab's HUD; the JetBrains Mono (SIL OFL 1.1) notice is
in neither repo; the release profile (`lto = "thin"`, `codegen-units = 1`)
was measured on the sand engine, where LTO gave nothing, and was never
measured here, while its CI cost is now billed; `hud.rs` and `mould/mod.rs`
comments still describe the lab and the shared engine. `wav.rs` fits.

## Step 1: the dish hash

`Dish::state_hash` is FNV-1a over the clock, the random stream's state,
every field (trail, scent, light, carried, walkers seen, sap, wall, wet,
ground, fed), every walker and pile, the store, the stock, the tallies, the
score, and the wild mould's dish. Floats go in by their bits. FNV rather
than std's `DefaultHasher`, whose algorithm Rust does not promise to keep.
It runs once per run, never in the window, so it costs the game nothing.

`mouldprobe` prints `hash=` on each challenge dish line, one line per hand
on the free dish, and on the frame-cost line. Two processes, same build,
`seeds=1,2 players=idle,plan`: identical hashes both times (idle
`cc49bd5a724f4922,c54b1a72907a7826`, plan
`9b6233757f064155,4f71ce7b608004b0`, cost line `d50963c63fa89d11`).

Guards, both in `mould/mod.rs`'s tests:

- `every_dish_replays_bit_for_bit`: the free dish, the five challenge dishes
  and the Wanderer, under the plan and the random hand (oats, lamp, water,
  snips), 10 s of play each, two runs must agree; seeds 2 and 3 must not.
  About 33 s in release.
- `the_hash_sees_the_smallest_change`: a one-ulp change in one trail cell,
  a one-bit change in a walker's heading, and one extra random draw each
  change the hash.

**Red on purpose:** with one step re-seeding the stream from the system
clock at step 600, `every_dish_replays_bit_for_bit` failed on the free dish
in 1.3 s; reverted.

**Changed on the way:** the first control asked that all 14 runs give 14
hashes, and got 12. On two dishes neither hand had acted yet, so plan and
random played the same game, which is right and not a collision. The
control is now another seed.

## Step 2: the release profile, measured on the mould

Deciding measure, named before the runs: mean frame (sim + draw) on the
`mouldprobe seeds=1 players=plan` cost line, with every arm's dish hashes
equal. Four builds from `afc2e64` with the profile set by environment
(`CARGO_PROFILE_RELEASE_LTO`, `..._CODEGEN_UNITS`), each in its own target
directory, run alternately, three rounds, one 4-core container, nothing else
running.

| Build | Mean frame, ms (rounds 1-3) | Draw, ms | Sim, ms | Rebuild after touching `mould/mod.rs` |
|---|---|---|---|---|
| shipped: thin LTO + `codegen-units = 1` | 5.00, 4.88, 4.95 | 3.78-3.87 | 1.10-1.13 | 13.7 s |
| `codegen-units = 1` alone | 4.86, 4.99, 4.95 | 3.76-3.86 | 1.10-1.13 | 13.4 s |
| thin LTO alone | 5.44, 5.47, 5.38 | 4.28-4.35 | 1.10-1.12 | 9.5 s |
| cargo's defaults | 5.39, 5.59, 5.39 | 4.28-4.44 | 1.11-1.15 | 7.6 s |

Same answer as Pixel_Physics got on the sand engine: `codegen-units = 1` is
the whole gain, about 0.45 ms a frame (9%), all of it in drawing, for about
+6 s a rebuild. Thin LTO adds nothing on top (shipped against cgu 1 alone:
one round each way and one tie). Every arm printed the same hashes (plan
`9b6233757f064155`, cost line `d50963c63fa89d11`), so no arm changed the
game. Three rounds is a direction, not a sign test: the cgu-1 arms beat the
default arms in all 12 same-round pairings, by 0.39-0.71 ms, against round-to-round spread
of at most 0.2 ms within an arm. Cold builds (50-77 s) were too noisy here to
rank. **Changed: `lto = "thin"` dropped, `codegen-units = 1` kept**
(`Cargo.toml` carries the table). PR #14's `test + clippy` CI job, with thin
LTO, took 289 s; the next CI run is the after.

## Step 3: licence gates, the font notice, the font generator

- `scripts/licensecheck.sh` (from Pixel_Physics, package name changed):
  286 third-party packages, all permissive; `r-efi` offers LGPL only as one
  option of three.
- `scripts/notices.sh`, `about.toml`, `about.hbs` (cargo-about 0.9.2) write
  `THIRD-PARTY-NOTICES.txt`; `--check` proves it is not stale. Added here: a
  FONTS section. `src/hud_crisp.rs` is rasterised from JetBrains Mono
  SemiBold 2.304, whose name table reads "Copyright 2020 The JetBrains Mono
  Project Authors" and "licensed under the SIL Open Font License, Version
  1.1"; its text is in `licenses/JetBrainsMono-OFL.txt`. (Debian's package
  copyright file still says Apache-2.0, from before upstream changed licence;
  the font's own name table is what was read.)
- `.github/workflows/notices.yml` runs both, only when `Cargo.toml`,
  `Cargo.lock`, `about.*`, `licenses/` or the two scripts change.
- `scripts/crispfont.py` came across; run here it reproduces
  `src/hud_crisp.rs` byte for byte, so glyphs (lowercase, say) can now be
  added.
