---
title: Sound
kind: look
status: current
updated: 2026-11-22
sources:
  - Reports/larder-sound-2026-11-12.md
  - Reports/mould-sound-2026-11-10.md
  - Reports/music-bed-2026-11-22/README.md
  - Reports/sound-pass-2026-11-20.md
code:
  - BANK_GAP
  - DEFAULT_MASTER
  - Ear::far
  - FAR_LP_HZ
  - FULL_KILL
  - GunVoice
  - Health
  - LOUDNESS_LAW
  - Listener::hear
  - MOODS
  - PIECE_MIN
  - PIECE_SHARE
  - PULSE_FLOOR
  - Pieces
  - SCORE_LEVEL
  - SETTLE
  - SHOTS_VOICED
  - Score
  - TEXTURE_RATE_CAP
  - TOUCH_FULL
  - VERB_HP_HZ
  - VOICE_LEVEL
  - VoiceBank
  - WorldListener
  - WorldListener::bangs
  - WorldListener::pops
  - density
  - full_kill
  - gun_layers
  - gun_sound
  - gun_voice
  - kill_weight
  - master_at
  - raise_priority
---

# Sound

## Now

On by default where the build has a device (macOS, Windows; Linux with
`--features audio-alsa`). Every sound is a count read off the game, and the
listener only reads, so sound on and off play the same game bit for bit.
The dish's sounds are in the [dish report](../../Reports/mould-sound-2026-11-10.md).

**Larder World** (`WorldListener`; Options' Volume, saved, via `master_at`;
O mutes, K the mould's voice) places
each sound by the camera's view (`Ear`), quieter and duller off screen
(`Ear::far`). Each gun has its own shot (`GunVoice`, `gun_sound`: crack,
punch, report, a bolt or pump), one a trigger pull, turrets under the
squad. A hit weighs the walkers it killed (`kill_weight`, `full_kill`);
kills pop; a pulse starts when mould is close or on a soldier
(`PULSE_FLOOR`, `TOUCH_FULL`); the mould has one breathing drone
(`VOICE_LEVEL`), and its cut-off pieces tear, wither and snarl
(`Pieces`). The economy, sirens, kit blasts and bullets in flight answer
too. Reverb is per sound and high-passed (`VERB_HP_HZ`). Tables:
[2026-11-12](../../Reports/larder-sound-2026-11-12.md),
[sound pass one](../../Reports/sound-pass-2026-11-20.md) (2026-11-20: the
gun 36-61% of the mix, was 4-9%; under 100 Hz 10-13%, was 34-41%). The
voice bank prints a mix meter under every recording (`SynthStats::mix`).
Since pass two the gun cards' payloads, raids, surges, mini-bosses,
buildings, the menus and the end of a planet sound too (`WorldListener::pops`,
`App::click_sound`).
The music bed and the planet's air: [The score](#the-score).

Keeping it playing: [below](#keeping-it-playing).

## The score

(2026-11-22, `Score`, `MOODS`): a music bed and the
planet's air under all of it, on by default, its level the Options row
Music and air (saved as `music=`; 0% off). One mood a biome and one for
the menus: a slow pad in the mood's key and mode (four open chords, one
every 14-22 s, crossfaded), soft bells from the chord in play, and the air
(wind, a river, insects, drips, a rumble). It is the one sound that is not
a count off the game, a bend of the rule above made because the game was
gunfire and silence; its one input is tension (the pulse's danger, plus a
little for a slime grown past its start), which opens the pad's filter,
brings a low drone and a faint high beating pair, and thins the bells.
`SCORE_LEVEL` puts it 11-13 dB under the mix on planets 1-3
([page](https://claude.ai/artifact/WnCC7JYYUsqLKQVxk2gxgH)). Hear a mood
alone with `cargo run --release --example scoreplay -- DIR`.

## Owner's word

- 2026-11-10 19:22Z: "yes to sound" (this thread).
- 2026-11-12 04:13Z, Larder World playtest: "shooting the slime didn't feel
  satisfying. I know it was killing lots of individual walkers but visually
  it didn't impact much"; sound and hit feel approved 04:25Z.
- 2026-11-20 05:40Z: "I want a high effort opus that to improve the sound
  design of the game, which is really bad right now".
- 2026-11-22 03:43Z: "We need some like background, music or more ambient
  sound or something but it sounds weird with just like mostly gunshots"
  (the score).

## Tried and rejected

- Counting joins by the length of `Dish::snaps`: the list is pruned, so it
  rebaselined 3-6 times a game. Events are found by their step stamp.
- One fight burst a frame on The Wild One (1,251 bursts): gathered a quarter
  second.
- Pieces on the campaign's square (4 fine cells at scale 2): a 6-wide cut
  never parted them; a 0.5 s settle: 21 tears in 3 minutes, 16 back in the
  body within 3 s; a 4% floor: no 220-walker piece at shape C's density
  ([report](../../Reports/larder-sound-2026-11-12.md), C7 section).

## Open

- The owner's ears on the [Larder Sound page](https://claude.ai/artifact/P4hk1fsSXQj3qXuwieffa7):
  does a big kill feel big; is the pulse help or noise; shot levels; does
  the mould's voice make it one creature.
- `FULL_KILL` follows the strike and the kill count; re-measure if either
  changes (the report says how).
- The owner's ears on the [Mould Sound page](https://claude.ai/artifact/6GXBj1swj93DzdytbUjqvA):
  is the bed too loud for the tools; does any sound feel wrong.
- The lamp's sizzle is silent while `Params::burn` is 0.
- The new sounds (C7) await the owner's ears on the Larder Sound page
  (version 3). A warning before a panic charge needs the GPU's stale-news
  count in the readout; the snarl is the charge itself.

## Sources

- [music-bed-2026-11-22](../../Reports/music-bed-2026-11-22/README.md); page source and clips in that folder.
- [sound-pass-2026-11-20.md](../../Reports/sound-pass-2026-11-20.md); page source in `Reports/sound-pass-2026-11-20/page/`.
- [mould-sound-2026-11-10.md](../../Reports/mould-sound-2026-11-10.md); page source in `Reports/mould-sound-2026-11-10/`.
- [larder-sound-2026-11-12.md](../../Reports/larder-sound-2026-11-12.md); page source in `Reports/larder-sound-2026-11-12/`.

## Keeping it playing

Sound that cut out for good (11-20) was a NaN that
stuck in the reverb and a dead stream: `VoiceBank` refuses non-finite
commands and clears itself on a bad sample, and `Output` reopens a stream
that errors or stops calling. Sound that kept dropping out (11-22, "still
an issue") was most likely the Windows callback thread at normal priority, starved
behind a 4K frame and the strategy league on every core: it now raises
itself to the "Pro Audio" class on its first call (`raise_priority`; cpal's
own `realtime` feature is MPL-2.0, which the licence gate rejects). The
callback keeps a `Health` count, and the play log prints a `sound` line
every 10 s (`prio`, `late` callbacks over 30 ms, the longest gap and
render, errors played through, dropped commands, reopens).
