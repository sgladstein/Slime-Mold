---
title: Sound
kind: look
status: current
updated: 2026-11-12
sources: [Reports/mould-sound-2026-11-10.md, Reports/larder-sound-2026-11-12.md]
code: [Listener::hear, VoiceBank, density, DEFAULT_MASTER, TEXTURE_RATE_CAP, LOUDNESS_LAW, WorldListener, kill_weight, full_kill, FULL_KILL, SHOTS_VOICED]
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

**Larder World** has its own listener, `WorldListener`, on the same voice
bank (2026-11-12, after the owner's playtest found shooting unsatisfying).
A shot is a crack at the soldier and an impact at the hit whose weight is
the walkers it killed (`kill_weight`, full at `FULL_KILL` = 80 at 5,000 walkers, the
rush's 90th centile, scaled by density to the 0.3 power, `full_kill`), so a shot into a vein is a heavy low slap and a graze a tap;
kills crackle, a pulse quickens as the mould nears the squad, hurt soldiers
groan, and the economy answers (table and measurements in the
[report](../../Reports/larder-sound-2026-11-12.md)). Placed by the camera's
view, quieter off screen. O toggles it in `larder-world`; 2-3.5 us a frame,
no new GPU read-back.


## Owner's word

- 2026-11-10 19:22Z: "yes to sound" (this thread).
- 2026-11-12 04:13Z, Larder World playtest: "shooting the slime didn't feel
  satisfying. I know it was killing lots of individual walkers but visually
  it didn't impact much"; sound and hit feel approved 04:25Z.

## Tried and rejected

- Counting joins by the length of `Dish::snaps`: the list is pruned, so it
  rebaselined 3-6 times a game. Events are found by their step stamp.
- One fight burst a frame on The Wild One (1,251 bursts): gathered a quarter
  second.

## Open

- The owner's ears on the [Larder Sound page](https://claude.ai/artifact/P4hk1fsSXQj3qXuwieffa7):
  does a big kill feel big; is the pulse help or noise; shot levels.
- `FULL_KILL` follows the strike and the kill count; re-measure if either
  changes (the report says how).
- The owner's ears on the [Mould Sound page](https://claude.ai/artifact/6GXBj1swj93DzdytbUjqvA):
  is the bed too loud for the tools; does any sound feel wrong.
- The lamp's sizzle is silent while `Params::burn` is 0.

## Sources

- [mould-sound-2026-11-10.md](../../Reports/mould-sound-2026-11-10.md); page source in `Reports/mould-sound-2026-11-10/`.
- [larder-sound-2026-11-12.md](../../Reports/larder-sound-2026-11-12.md); page source in `Reports/larder-sound-2026-11-12/`.
