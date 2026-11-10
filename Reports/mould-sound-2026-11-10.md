# Sound for the game (2026-11-10)

The owner, 2026-11-10 19:22Z: **"yes to sound."** (HANDOFF decision 8; the
Pixel_Physics review's item 2.) The window was silent. It now plays the
dish: listening page
[Mould Sound](https://claude.ai/artifact/6GXBj1swj93DzdytbUjqvA), source and
recordings in [`mould-sound-2026-11-10/`](mould-sound-2026-11-10/).

## What was built

Pixel_Physics's `src/audio/` architecture, read at `c3588b3`, with the
mould's own vocabulary, in `src/sound/`:

- `listen.rs`, `Listener::hear(&Dish, dt, hushed)`: reads the dish once a
  drawn frame and emits `SoundCmd`s. It takes `&Dish` only and draws no
  random numbers, so it cannot change the game.
- `synth.rs`, `VoiceBank`: Pixel_Physics's voice bank (a 64-grain pool,
  Poisson textures, bursts, tones, room tone, a small reverb, a soft
  limiter, habituation, `MAKEUP` 4, `DEFAULT_MASTER` 0.5) with its three
  styles removed (one sound until the owner asks for a choice) and ten
  grain recipes of the mould's own. A pure function of seed and commands.
- `device.rs`: cpal, gated as the organ's is (macOS, Windows; Linux with
  `--features audio-alsa`). `Output` is the empty shell elsewhere.
- `bin/mould.rs`: on by default; S turns it off and on (named in the title
  bar). Paused, briefing or menu: the textures fade, one-shots still sound.
- `examples/mouldsound.rs`: renders a dish played by a hand to WAV with the
  listener's and voice bank's counters. Hands `idle`, `random`, `plan`, and
  `tour` (the plan plus every tool once on the mould).

**Everything heard is a count off the dish**, the picture's rule held for
sound:

| Sound | Driven by |
|---|---|
| growth texture (low wet ticks) | walkers born per wall second, smoothed 0.35 s |
| feeding texture (a rough scrape) | walkers on piles, ours and the wild mould's; panned to them |
| starving texture (dry pops) | walkers starved, hungry or died asleep per second |
| oats thunk | `counts.oats_placed` moved |
| salt pour | `counts.salt_cells` moved |
| knife knocks | new `snip_marks`; a crackle when `snipped + salted` moved |
| drip | `counts.water_drops` moved |
| lamp hum | `Dish::lamp` is `Some`, panned to it |
| sizzle | `counts.burned` moved (silent today: `Params::burn` is 0) |
| grit at the seam | `counts.fought`, gathered a quarter second a burst |
| knock | a pile turned up (`Dish::arrived`) |
| marimba note, climbing | a join (`Dish::snaps`); pitch from `counts.snaps` |
| rising air, thud | storm aimed (`storm_line`), storm struck (`Phase::Storm`) |
| bell triad / falling bells | `Phase::Over` with walkers / the last walker died |

Levels follow Pixel_Physics's loudness law (`density`: rate to a 220/s
ceiling, amplitude as count^0.4). Level references are first guesses for the
ear to correct.

## Measured

Deciding measures, named before the runs: the dish hash is unchanged with
the listener on; the listener's cost per frame; and each sound's counter is
non-zero on a run that should fire it.

- **Same game, bit for bit.** `listen::tests::listening_never_changes_the_game`:
  free dish 4, 1,800 frames with an oats placement, listened to every frame
  and not, equal `state_hash`; its control asserts the listener heard the
  oats. And the Wild One run printed the same hash (`c8865388140e5e06`)
  before and after a listener change (gathering fights), as it must.
- **Cost.** Mean 0.6-1.6 us a frame over seven 102 s runs (6,120 frames
  each, worst single frame 13-194 us), against a 5.12 ms frame
  (`mouldprobe seeds=1 players=plan`, this build). Under 0.05%. The voice
  bank runs on the audio thread, not the frame.
- **Counters** (one run each, seed 1, `mouldsound`):

| Run | Tools heard | Joins | Grains (growth / feeding / starving / tools) | Bursts | Tones | Level |
|---|---|---|---|---|---|---|
| free, tour | oats 8, salt cells 137, knife marks 31, water 1 | 8 | 3,320 / 8,238 / 2,415 / 44 | 2 | 9 | -27.8 dBFS |
| free, idle | none | 4 | 1,383 / 6,629 / 584 / 4 | 0 | 5 | -30.5 dBFS |
| free, plan | oats 8 | 7 | 2,845 / 9,048 / 1,444 / 12 | 0 | 8 | -28.2 dBFS |
| free, random | oats 10, knife marks 65 | 5 | 2,304 / 5,602 / 1,404 / 79 | 0 | 6 | -29.4 dBFS |
| Wild One, random | oats 10, knife 65, fights 1,520 | 20 | 3,964 / 12,819 / 1,904 / 75 | 255 | 20 | -27.0 dBFS |
| Running Dry, random | oats 11; died | 1 | 1,339 / 2,186 / 1,782 / 14 | 0 | 2 | -29.4 dBFS |
| free, plan, fast | oats 10 | 8 | 2,634 / 2,152 / 1,818 / 14 (25.5 s) | 0 | 9 | -23.2 dBFS |

No grain was stolen in any run (pool of 64 never full). Peaks 0.77-0.93
under the limiter.

**Changed on the way.** (1) The first listener counted new joins by the
length of `Dish::snaps`, which is pruned as flashes fade, so it
"rebaselined" 3-6 times a game and could miss joins; events are now found by
their step stamp (all stamped before `t` increments, so `>= last t` is
exact), and restarts are found by tallies falling. (2) The Wild One sent a
fight burst nearly every frame (1,251 bursts thrashing six slots); fights
are now gathered a quarter second (255 bursts). (3) The bed was turned down
before the recordings (growth 0.30 to 0.22, feeding 0.22 to 0.14 at their
references, feeding 0.25 to 0.12 grains per eater) and the lamp hum up (2.5
to 4 times the room), because the per-second loudness of the tour run sat
flat at -24 to -28 dBFS with the tools hardly showing; it is still flat-ish
(the page's traces), which is the first thing the owner is asked to judge.

**Not checked here:** the device itself. No sound device in the cloud
container; `device.rs` was compiled and linted for Windows
(`cargo clippy --target x86_64-pc-windows-msvc`) and CI checks it for macOS
and Windows. The first live listen is the owner's.

## Open

- The owner's ears: is the bed too loud for the tools, and is any sound
  wrong for what it is? (The page's three questions.)
- No keys-line hint for S: the bottom line only shows when it fits beside
  the hint, and a longer line would drop more often.
- The sizzle waits for a lamp that burns (`Params::burn` > 0).
