# Sound pass one: the mix and the guns (2026-11-20)

**Why.** The owner, 2026-11-20 05:40Z: "I want a high effort opus that to
improve the sound design of the game, which is really bad right now". This
is the first batch: the sounds a fight already makes. The second batch
gives a sound to the events that make none (open, below).

Listening page: https://claude.ai/artifact/28QMA88oMoWq1mRjgZjdd9 (source,
clips and spectrogram in `sound-pass-2026-11-20/page/`).

## How it was measured

Three headless games, recorded with the game's own voice bank
(`larder-world look=DIR run=1:P mode=squads hand=H at=T sound=FILE.wav`, run
seed 1, llvmpipe): planet 1 with the greedy hand for 150 s (it buys 40
soldiers), planets 2 and 3 with the squads hand for 120 s (planet 2's squad
is wiped at 84 s). Sound only reads the game, so before and after are the
same games shot for shot.

New for this: the voice bank's **mix meter** (`SynthStats::energy`,
`SynthStats::mix`), the energy each source put in before the master,
printed under every recording as `mix:`; the limiter's share of samples
(`limiter:`); and the hits by walkers killed (`hits by kills`,
`WorldStats::kill_hist`). Loudness (BS.1770, LUFS) and the share of energy
under 100 Hz are from the WAV. Energy weighs bass heavily, so the drone and
the pulse count for more on the meter than a small speaker plays.

## What was wrong (before)

| Source's share of the energy | Planet 1 | Planet 2 | Planet 3 |
|---|---|---|---|
| Gun (the crack) | 9% | 4% | 7% |
| Reverb | 26% | 25% | 25% |
| Hits | 18% | 25% | 24% |
| Drone (the mould's voice) | 17% | 7% | 21% |
| Pulse | 21% | 25% | 19% |
| Under 100 Hz | 34% | 37% | 41% |
| Pulse beats | 236 | 193 | 149 |

- The whole mix went to the reverb at 0.35, the 48 Hz drone and 55 Hz
  pulse too, so the reverb was the loudest source and rang under every gap.
- The pulse beat whenever a soldier saw mould anywhere in range, or had one
  walker on it: most of every fight.
- The gun was one 3-5 ms noise snap at 2.5-3.8 kHz over a 160 Hz blip, the
  same for all four guns and the turret, one per bullet (a scattergun was
  four ticks).
- The hit's body was a 115-145 Hz sine: under a laptop speaker, and in the
  gun's way on big ones.
- Kills were the starving crackle, 2-5 kHz noise ticks at up to 220 a
  second: static.
- A banking chime once a second (117 in 150 s on planet 1).
- A grain over its kind's cap was cut mid-sound (a click).

The hit's weight was not the fault: with four soldiers its weights spread
(planet 2: 442 light, 148 mid, 17 heavy). Only the greedy hand's 40
soldiers split each kill thin (901 of 905 light).

## What was built

| Change | Where |
|---|---|
| A gunshot per gun (`SoundCmd::Gun`, `GunVoice`): click, crack, punch, report and, for the long gun and scattergun, a bolt or pump; numbers by kind in `gun_sound`, jittered in `gun_layers` | `sound/synth.rs` |
| One report a trigger pull: bullets leaving one gun in one frame are one shot; the voice is the squad's gun's kind or a turret's (`gun_voice`) | `sound/world.rs` |
| Distance dulls: `Ear::far` takes a far sound's low-pass toward `FAR_LP_HZ` (1.8 kHz), its crack down and its reverb up | both |
| Reverb by source (`Spec::verb`): a report 0.6, a crack 0.25, textures 0.1, pulse and drone none; its input high-passed at `VERB_HP_HZ` (280 Hz); a master high-pass at `MASTER_HP_HZ` (28 Hz) | `synth.rs` |
| The hit (`Splat`): a wet slap with its squelch at 220-300 Hz; a heavy hit still drops to half pitch and rings longer | `synth.rs` |
| Kill pops (`TextureKind::Kill`, `GrainKind::Pop`): a bubble's rising blip, 12-24 ms | both |
| The pulse: floor 0.7 (`PULSE_FLOOR`, was 0.05), walkers on a soldier count by how many (`TOUCH_FULL` 12), quieter, dry, at 60-66 Hz with a 160 Hz thump | both |
| The drone at `VOICE_LEVEL` 0.12 (was 0.18); banking chimes at most every 2.5 s (`BANK_GAP`, was 1) | `world.rs` |
| A grain over its cap fades in 4 ms (`FADE_SAMPLES`) instead of being cut; the pool 96 (was 64) | `synth.rs` |
| `examples/shotsound.rs guns=1`: every gun near, far and as a volley | example |

## After

| Source's share of the energy | Planet 1 | Planet 2 | Planet 3 |
|---|---|---|---|
| Gun (crack, punch, report, action) | 61% | 36% | 54% |
| Reverb | 9% | 6% | 7% |
| Hits | 14% | 29% | 21% |
| Drone | 8% | 5% | 11% |
| Pulse | 2% | 4% | 2% |
| Under 100 Hz | 11% | 10% | 13% |
| Pulse beats | 127 | 141 | 118 |
| Loudness, LUFS (before) | -16.2 (-18.2) | -15.4 (-15.7) | -16.8 (-18.2) |
| Limiter, samples pulled (before) | 0.05% (0.02%) | 0.18% (0.18%) | 0.04% (0.04%) |

The listener's time a frame is unchanged (12-22 us mean on llvmpipe).

**Rejected on the way.** The first cut of the guns had their punches 4 dB
up and cracks 2 dB down from the kept numbers: the punch became the mix's
biggest source and 100-300 Hz its loudest band. A pulse floor of 0.45 with
any touch as full danger still beat 246 times on planet 1.

## Pass two: what made no sound (2026-11-20, later)

| Event | Read from | Sound |
|---|---|---|
| A gun card's payload | `Rounds::pops` (`WorldListener::pops`, new each frame) | Blast and Bomb: thud, impact by kills, debris; Gas: a hiss (`GrainKind::Hiss`); Germ and Detonate: wet pops; Flare and Torch: whoosh and sizzle; Shatter: knock and crackle; the mind rounds: a soft shimmer. Up to `PAYLOADS_VOICED` (4) a frame, sized by the burst's radius |
| A raid starting | `Game::raids.sent` | a rising groan where it heads (`GrainKind::Swell`); the last stand louder, with a snarl |
| A surge | `Game::surges` | a groan at the slime's home |
| A mini-boss | `Game::bosses` | forming: groan and snarl; broken: a tear and a wither |
| A building | `Game::buildings` | set down: a clank (`GrainKind::Clank`) and a thunk; lost: a thud, debris and a clank |
| Menus and HUD | the window's picks and HUD events (`App::click_sound`), straight to the speaker | a tick (`GrainKind::Tick`); buying a chime; a card into a slot a knock; landing a whoosh |
| A planet won or lost | `Game::end` | a win's arpeggio climbs to the octave; a loss's falling bells have the slime's groan under them |

Recorded: planet 2 with a Blast, Gas, Germ and Flare gun heard 111
payloads in 30 s (before: none); planet 1 (greedy) heard its 3 surges,
planet 3 (squads) its 2 surges and a mini-boss. Each sound's level was set
against the rifle's shot in `examples/shotsound.rs events=1` (first cut:
the flare, the germ, the mind rounds and the landing whoosh were 40-48 dB
under a blast, too quiet to hear in a fight; raised).

## Volume (2026-11-20, later)

Options, Game tab, second row: Volume, 0% to 200% of the first-run level
(`settings::VOLUME_STEPS`, `synth::master_at`; the master squares it, so
50% is -12 dB and 200% +12 dB), taken at once with a tick at the new
level, saved in `settings.txt` beside the UI size, and kept by All to
defaults. O still mutes, now back to this volume
([still](sound-pass-2026-11-20/options-volume.png)).

## Open

- The owner's ears: he plays on his own time; the page has the clips.
