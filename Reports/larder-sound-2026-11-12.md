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

**Proposed, not built** (design review): a cue when a shot clears its
block, so late-game shots into a thinned mould do not sound like failing;
turrets and blasts ducked under the shots the player orders.

## The mould's voice (same night)

The owner's first playtest note was that the mould "didn't feel like one
cohesive organism", and every sound so far was the squad's or an event's.
Built under the owner's overnight rule (04:48Z: "have the threads work
together to come up with the best answer and move forward with it"), on the
designer thread's call: one breathing drone for the whole body, never per
walker, from counts the game already has (no read-back).

| Input | Read from | Heard as |
|---|---|---|
| Size | `Game::mould` | level, log of the walkers (`0.18 x log10(1 + n/100) / log10(51)`) |
| Natural change | `Game::mould`'s change a game second with the walkers shot added back (births less starving), smoothed over 4 s | growing: pitch up to +12%; starving (`FULL_CHANGE` 2%/s): pitch down 22%, breath 40% slower, rough |
| Eating | the crystals' charge falling faster than the squad carries it (`FULL_EAT` 0.02%/s) | the noise band opens |
| Gunfire | kills a second as a share of the body over 0.4% | a sharp inhale (at most every 2.5 s), the breath up to 2.4 times faster while it settles over 3 s |
| Heavy hit | `SoundCmd::Impact`'s weight | the voice ducks up to 60%, back in 0.3 s |

It sits on the mould's home, half-panned, and does not fade off screen.
**K** turns it off and on in the window; `voice=0` starts without it. On by
default (the standing rule: defaults on unless a measured harm or a stated
uncertainty).

**What it reads** (fork, seed 1, every hand): the mould dies back from
5,000 to about 1,400-1,500 walkers between 15 and 45 s whatever the squad
does (9-11% a second at worst, hunger 1.0 for 15 s), then regrows at
1.2-1.6% a second (growth 0.6-0.8). So the voice's biggest event in a
round's first minute is the body's own die-back, which the first cut did
not sound at all. Eating ran 0.006-0.014% of the charge a second; at the
first guess of `FULL_EAT` (0.2%) the band never opened, so it is 0.02%.
The idle hand's few default-stance shots made 5 inhales in 90 s; the rush's
volleys 3 (the 2.5 s gap holds them).

**Level.** Alone in the first 10 s it is about -32 dBFS (the room is -59);
the rest of the mix runs -27 to -29. The rush with and without it: -22.9
and -23.6 dBFS. Listener 2.9-3.5 us a frame with it.

**For pieces**, once the one-body thread lands a piece id: a fainter, higher
copy a piece over about 200 walkers, a tear at the cut, at most 3-4 voices
(the designer's plan; not built).

## What is new: bullets, siren, blasts and the cut-off pieces (2026-11-14)

The cloud plan's item C7 ([cloud-plan-2026-11-14.md](cloud-plan-2026-11-14.md)):
sounds for what the game gained since the 11-12 build, from counts the game
already has, recorded for the owner since they cannot playtest now. All on
by default; the piece voices go with K.

| Sound | Read from | Heard as |
|---|---|---|
| Zip | each fired `Shot`, while `Rules::bullet_speed` > 0 (at most 4 a frame) | a short high whizz at the flight's midpoint, delayed by half the flight; a turret's shot (no soldier) at half the level of a soldier's |
| Siren | `Game::sirens` rising | a two-note wail (740 and 554 Hz, twice) panned to the working siren |
| Blast | `Kit::bangs` (mines, thrown charges), passed in by `WorldListener::bangs` | a thud, an impact weighted by its kills, debris |
| Tear | a piece settling (below) | a ripping noise where it is |
| Snarl | a heard piece breaking up within 20 dish cells of a soldier (a scared piece's charge), or a piece's centre closing on one faster than 3 dish cells a second | a rising growl |
| Withering | the walkers the pieces lose a second (`TextureKind::Wither`, full at 40 a second) | a dry fizz; a piece withering out is a soft puff |
| Rejoin | a healthy piece gone next to a big part of the body | a double squelch |
| Piece voice | up to three pieces (`SoundCmd::Body` slots 1-3), each from its walkers, its change and its approach | a fainter, higher copy of the body's drone, panned to the piece: strained and rough as it withers, quick and rising as it charges |

**The pieces** (`sound::pieces`) are the 4-connected groups of the mould
grid (`World::mould_grid`, one dish cell a square), of squares holding 2
walkers or more, followed from readout to readout by centre and size. A
group is a piece at 100 walkers (0.4% of the start) and under 10% of all
walkers; it is torn once it has lasted 2 s, and stays the same piece if it
drops off the grid for under 2.5 s. Each of those numbers came from the
first recordings (Fork, seed 1, farm hand, shape C, llvmpipe), which got
it wrong four ways:

- A floor of 4% of the start (the designer's 200 of 5,000) is 1,000 at
  shape C's 25,000: the staged ring's 220-walker part was never a piece.
- The Fork's mould lies in two or three parts of about 8,000 walkers each,
  apart on the grid; the smaller ones were "pieces", and a piece's match
  jumping between them read as a 4,700-walker-a-second wither and a
  60-cell-a-second charge (false snarls). Fixed by the 10% ceiling and by
  matching only a group a third to three times the piece's size.
- At twice the scale a square (the campaign's `BIN`), two squares side by
  side span 8 fine cells, so the 6-wide ring left the piece joined to the
  body; and the ring's cells kept 55-75 stranded walkers, one to a square,
  for ten seconds. Fixed by one dish cell a square and the floor of 2.
- A settle of 0.5 s heard 21 tears in 3 minutes, 16 of them back in the
  body 1-3 s later (the GPU world's grid is a game second old). At 2 s
  with the 2.5 s grace: 10 tears, 7 rejoins, 3 withered out.

The charge could not be heard from the piece's centre: in the staged last
stand (184 walkers in a held ring with four soldiers inside) the piece was
seen settled once, at 156 walkers, and at the next readout its walkers
were spread one to a square, running at the squad; its centre's approach
never passed 1 dish cell a second. So the snarl sounds as the piece breaks
up beside a soldier. It is the charge itself, not a warning before it; a
true warning needs the walkers' stale-news count, which only the GPU has
and no readout carries.

**Measured** (headless, `larder-world look=DIR ... sound=FILE`, counts from
the `heard (C7)` line):

| Recording | Zips | Torn | Rejoined | Withered out | Charges | Listener |
|---|---|---|---|---|---|---|
| Farm, held ring r22 at 2:30 (to 3:00) | 2,199 | 10 | 7 | 3 | 2 | 23.0 us mean, 0.54 ms worst |
| Farm, last stand r32 at 2:30 (to 2:55) | 2,219 | 10 | 7 | 4 | 3 | 21.4 us, 0.60 ms |
| Rush (wiped at 1:20) | 47 | 7 | 4 | 1 | 1 | 21.8 us, 0.60 ms |

The ring's tear comes 7 s after the cut (2:37), when the stranded walkers
die off; a cut made by a wall or a crater has no such walkers. The listener
went from about 6 us a frame (the same build with the pieces off) to
21-23: the pieces ask for the grid four times a game second (asking every
frame cost more on the CPU world, which sums its grid on each ask) and walk
its 41,000 squares. The worst frames (0.5-0.6 ms) are first asks, which
the GPU world reads back once. No new read-back after that: the GPU world
packs the asked square into its readout once a game second. In a mission
the window sets `WorldListener::piece_bin` to the campaign's `BIN`, so
both askers share one packed grid (at the cost of not hearing cuts under 8
fine cells wide); `World::walkers_in` (mines) asks one dish cell too.

**Sound board** (`examples/c7sounds.rs`): each new sound alone at the
listener's levels, 33 s; peak -1.1 dBFS (the siren), rejoin the quietest
at -44 dBFS rms, a piece's voice -33. The siren hand did not get its siren
built in 120 s on seed 1, so the siren is heard only on the board and in
`a_siren_wails_and_a_blast_booms_once`. WAVs for the owner in the shared
folder (`larder-sound-c7/`); MP3s on the page (version 3).

## Open

- The owner's ears: does a big kill feel big, is the pulse help or noise,
  are the shots' levels right against the rest, and does the mould's voice
  make it one creature (A/B on the page).
- `FULL_KILL` depends on the strike (`Rules::strike`) and on how kills are
  counted; the graphics thread's per-walker wound and the Larder thread's
  free shots may move the distribution. Re-measure with
  `larder-world look=DIR hand=rush at=120 sound=x.wav` and the kill centiles.
- No hand builds walls, so the wall sound has only its unit path checked.
- The owner's ears on the new sounds: is the snarl readable, are the zips
  too busy in a long fight, do the piece voices make a cut part a separate
  creature or clutter.
- A warning before a charge (the panic's 1.5 s lean) needs a count of
  stale-news walkers from the GPU readout; not built.
- The squad toolkit's GPU bursts (`Frame.bursts`) other than the kit's
  bangs are not heard yet.
