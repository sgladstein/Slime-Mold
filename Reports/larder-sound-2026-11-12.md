# Larder World sound: shots that weigh what they killed (2026-11-12)

**Why.** The owner's first Larder World playtest (2026-11-12 04:13Z):
"shooting the slime didn't feel satisfying. I know it was killing lots of
individual walkers but visually it didn't impact much." The Larder thread
proposed sound and hit feel driven by the game's own counts, and the owner
approved it (04:25Z, relayed by the coordinator). The graphics thread is
reworking how a hit looks (the wound, the hole the kills leave, on its own
branch); this report is the ear's half. Screen shake was left out: it would
be a drawn-only effect, which the owner's rule forbids.

Listening page: https://claude.ai/artifact/P4hk1fsSXQj3qXuwieffa7 (source in
`larder-sound-2026-11-12/`).

## What was built

`sound::world::WorldListener` reads `&Game<W>` (any `World`, CPU or GPU) and
emits the same `SoundCmd`s the dish's listener does (`sound::listen`), into
the same voice bank. Like that one it draws no random numbers and cannot
change a round: `listening_never_changes_the_round` plays 60 s of a rush on
the CPU world heard and unheard and compares the world hash, with the
positive control that every shot and kill the game counted was heard.

| Sound | Read from | How |
|---|---|---|
| Crack | each `Shot` (all of a frame's steps, `WorldListener::shots`) | at the soldier; up to `SHOTS_VOICED` (6) a frame, the heaviest first, spread over the frame |
| Impact | `Shot::hit`, `Shot::killed` | at the hit, a few hundredths of a second after the crack; `SoundCmd::Impact` shapes the new `Splat` grain by `kill_weight`: lower (pitch to 55%), longer (to 2.2 times), fuller |
| Kill crackle | sum of `Shot::killed` a second | one pop a walker (texture, capped at 220 a second), at the kills' centre |
| Pulse | the nearest `Soldier::aim` against `Rules::range`, or `touching` | two thumps a beat, 54 to 150 beats a minute and louder as the mould nears; silent with no mould in sight |
| On you | `Soldier::touching` | wet squelch texture at the touched soldiers |
| Hurt, fallen | `Soldier::hp`, `alive` | a groan each 8 health lost (0.35 s apart at least); two low bell notes |
| Larder | `Game::stolen` a second | slurp texture at the larder |
| Banked | `Game::harvested` | a marimba note, at most one a second, higher as the bank fills |
| Bought, wall, dry, core, end | soldiers added, `Game::walls`, a crystal's charge reaching 0, `core_down`, `end` | thunk; thunk and grit; knock; thud and bell; bell or the falling triad |

Everything is placed by an `Ear`, the camera's view: panned across the
screen, full level on it, and `1 / (1 + 2 x views away)` off it.

In the window (`src/bin/larder-world.rs`): on by default where the build has
a device (as `bin/mould.rs`), **O** toggles; the listener's time is printed
beside the frame time every 10 s. `larder-world look=DIR ... sound=FILE.wav`
records a headless round from an ear above the squad; `examples/shotsound.rs`
renders the shot ladder.

## Measurements

**Kills a shot** (fork, seed 1, `larder-world` headless, 120 s): rush, 43
shots: 10th centile 1, quartiles 5, 37 and 56, top 81. Farm, 95 shots:
median 0, top 4. The first guess at full weight (30 kills) put 30 of the
rush's 43 shots at full weight, so `FULL_KILL` is 80 (the rush's 90th
centile) and `kill_weight` is the square root of its share: 1 kill 0.11, 5
0.25, 37 0.68. The rush then splits 11 light, 10 mid, 22 heavy.

**The ladder** (`shotsound`, one shot a second, nothing else running):

| Killed | Peak | RMS, first 0.3 s | Share under 200 Hz |
|---|---|---|---|
| stopped by rock | -10.7 dB | -35.3 dB | 2% |
| 0 | -11.8 dB | -33.8 dB | 11% |
| 1 | -9.4 dB | -27.4 dB | 34% |
| 5 | -6.9 dB | -25.6 dB | 44% |
| 15 | -6.2 dB | -22.8 dB | 55% |
| 37 | -5.2 dB | -20.4 dB | 72% |
| 56 | -3.1 dB | -18.3 dB | 80% |
| 80 | -1.8 dB | -16.0 dB | 79% |

Monotone in loudness and low end; the sound past 1 kill rings about 0.9 s
because the kill crackle is smoothed over 0.3 s.

**Rounds** (fork, seed 1, ear on the squad):

| Hand, length | Shots | Killed | Impacts light / mid / heavy | Beats | Groans | Fallen | Banked chimes | Bought | Level |
|---|---|---|---|---|---|---|---|---|---|
| rush, 40 s | 43 | 1,494 | 11 / 10 / 22 | 28 | 42 | 4 | 0 | 0 | -21.9 dBFS, peak 0.93 |
| farm, 60 s | 9 | 2 | 7 / 0 / 0 | 4 | 0 | 0 | 46 | 7 | -29.5 dBFS, peak 0.74 |

Banking was first one chime a deposit: the farm hand banked 325 times in
two minutes (2.7 a second). Gathered to one a second (`BANK_GAP`): 66 in
two minutes.

**Cost.** The listener takes 2.1-3.5 us a drawn frame on the cloud CPU
(mean over four headless runs of 600-1,800 frames; worst 83 us), and adds
no GPU read-back: it reads `Game` fields the window already has
(`GpuWorld::crystals` is a cached copy). The voice bank runs on the audio
thread, as for the dish.

## After the reviews (same day)

**Density.** The tech lead review: `FULL_KILL` is a raw count, set at 5,000
walkers; the GPU world will run 50,000-100,000. The same rush (fork, seed 1)
at `walkers=50000`: 18 shots killed 1,192, 66 a shot against 35 at 5,000,
so kills grow 1.9 times for 10 times the walkers. With the raw count, the
50,000 rush would sound almost all heavy; scaled in proportion to density,
all 18 shots were light. `full_kill` now scales `FULL_KILL` by the starting
walkers a dish cell against `REF_DENSITY` to the power `DENSITY_LAW` = 0.3
(log 1.9 / log 10): the 50,000 rush splits 9 light, 3 mid, 6 heavy, and
5,000 is unchanged (11 / 10 / 22; the ladder renders sample for sample the
same). One seed pair; re-measure when the Larder thread's density field
lands, and write the scale from it.

**Off screen.** The design review: sounds that tell you where to look must
not fade with the camera. The larder robbed and a crystal run dry keep 70%
of their level off screen (`ALERT_FLOOR`); a soldier falling and the core
were at full level already.

**Proposed, not built** (design review; wait for the owner's ears on the
first cut): a voice for the mould as one body (a drone from its size, the
alarm field and hunger, a cut-off piece with its own fainter voice once the
one-body work lands); a cue when a shot clears its block, so late-game
shots into a thinned mould do not sound like failing; turrets and blasts
ducked under the shots the player orders.

## Open

- The owner's ears: does a big kill feel big, is the pulse help or noise,
  are the shots' levels right against the rest (page above).
- `FULL_KILL` depends on the strike (`Rules::strike`) and on how kills are
  counted; the graphics thread's per-walker wound and the Larder thread's
  free shots may move the distribution. Re-measure with
  `larder-world look=DIR hand=rush at=120 sound=x.wav` and the kill centiles.
- No hand builds walls, so the wall sound has only its unit path checked.
- The squad toolkit's area effects (blasts, mines, siren; `Frame.bursts`,
  planned) should get sounds from the same listener when they land.
