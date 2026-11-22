# The music bed and the planet's air (2026-11-22)

Owner, 2026-11-22 03:43Z: "We need some like background, music or more
ambient sound or something but it sounds weird with just like mostly
gunshots". Built as `src/sound/score.rs` (`Score`, `MOODS`), sent by the
listener each frame as `SoundCmd::Score`, on by default, with an Options
row, Music and air (`music=` in settings.txt).

Page: https://claude.ai/artifact/WnCC7JYYUsqLKQVxk2gxgH (`index.html` here,
with its clips).

## Levels

Run 1, `mode=squads hand=squads at=120 sound=F.wav`, mix meter
(`SynthStats::mix`, each source's energy against the whole mix):

| Planet (biome) | score, first cut 0.055 | score at 0.12 | limiter |
|---|---|---|---|
| 1 (meadow) | -17.6 dB | -11.1 dB | 0.11% of samples, as before |
| 2 (barrens) | -19.4 dB | -12.9 dB | 0.20% (0.19%) |
| 3 (fen) | | -11.8 dB | 0.06% |

The whole mix ran -20 and -18 dBFS on planets 1 and 2 and its quietest
tenth of 0.4 s windows -24 and -21 dBFS, since the squad fights almost from
landing. At the first cut the score sat 13 dB under even those quiet
windows; at 0.12 it sits about 6 dB under them. No command refused and no
recovery in any recording.

Each mood alone (`examples/scoreplay.rs`, 36 s, tension 0 then 1 then 0):
-30.3 to -32.3 dBFS, so the moods are within 2 dB of one another.

Cost: 18 ms of one core for a second of the busiest mood with tension
(`score::tests::a_second_of_score_is_cheap`, run alongside other tests), on
the audio thread; nothing on the game's frame but one command.

## The ruling it bends

"Everything heard is the simulation" (`src/sound/mod.rs`): a music bed is
not a count off the game. Its one game input is tension (the pulse's
danger, plus a little for a slime grown past its start).
