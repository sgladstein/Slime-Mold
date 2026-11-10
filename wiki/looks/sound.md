---
title: Sound
kind: look
status: current
updated: 2026-11-10
sources: [Reports/mould-sound-2026-11-10.md]
code: [Listener::hear, VoiceBank, density, DEFAULT_MASTER, TEXTURE_RATE_CAP, LOUDNESS_LAW]
---

# Sound

## Now

The game is heard as well as seen, on by default where the build has a
device (macOS, Windows; Linux with `--features audio-alsa`); S turns it off
and on. Every sound is a count read off the dish: growth is walkers born per
second, feeding is walkers on food, starving is deaths per second, and each
tool, join, pile arrival, storm and ending sounds when its tally moves
(table in the [report](../../Reports/mould-sound-2026-11-10.md), *What was
built*). The listener only reads the dish, so sound on and off play the same
game bit for bit. Levels are first guesses; the owner has not listened yet.

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| dish hash, listened every frame vs not | equal | free dish 4, 1,800 frames, one test | n/a | [report, *Measured*](../../Reports/mould-sound-2026-11-10.md) |
| listener cost | mean 0.6-1.6 us a frame, worst 194 us, against a 5.12 ms frame | seven runs, seed 1 | n/a | [report, *Measured*](../../Reports/mould-sound-2026-11-10.md) |
| loudness, whole run | -30.5 dBFS (idle) to -27.0 (Wild One, random); fast forward -23.2 | seven runs, seed 1 | n/a | [report, *Measured*](../../Reports/mould-sound-2026-11-10.md) |

## Owner's word

- 2026-11-10 19:22Z: "yes to sound" (this thread).

## Tried and rejected

- Counting joins by the length of `Dish::snaps`: the list is pruned, so it
  rebaselined 3-6 times a game. Events are found by their step stamp.
- One fight burst a frame on The Wild One (1,251 bursts): gathered a quarter
  second.

## Open

- The owner's ears on the [Mould Sound page](https://claude.ai/artifact/6GXBj1swj93DzdytbUjqvA):
  is the bed too loud for the tools; does any sound feel wrong.
- The lamp's sizzle is silent while `Params::burn` is 0.

## Sources

- [mould-sound-2026-11-10.md](../../Reports/mould-sound-2026-11-10.md); page source in `Reports/mould-sound-2026-11-10/`.
