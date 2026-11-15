# Slime enemy review (2026-11-15)

The owner (11-15 08:03Z): "I want to start up a thread that works on the
slime enemy. It should start with a deep review of the current state and
then ask me questions." Page: https://claude.ai/artifact/3wF8KBUfBUp7oBTntvPBJi
(source in `page/`, open `index.html`). Code on main at 17f2e5b; every count is
the game window's own (`larder-world look=DIR`) on llvmpipe, seed 1, one run
each, so a first look, not a census.

## Findings

1. **The slime is a tide, not an army.** It follows food scent, covers
   crystals and bites what stands where it grows. Only soldier scent, the
   home pull, the panic charge and tier specials point it at the player. An
   idle squad on the Fork is gone at 16 s (`raw/fork-idle.txt`). The player
   AI's take hand (best on the CPU census, #242) is wiped at 69 s in the
   window (`raw/fork-take.txt`). The scripted squad-mode hand firing from the
   edge loses no one, but the slime regrows past its start under the fire:
   24,985 -> 21,923 at 21 s -> 25,416 at 60 s (`raw/fork-squads.txt`).
2. **The run plays the old race slime, not shape C.** `run_args` and the
   `run=` path both skip `default_shape`, so a planet plays the race keys
   (upkeep 0.001, slots x10, no energy_flow, wither or panic) plus its tier.
   Planet 5, idle: 18,034 -> 4,545 at 30 s -> 5,907 at 2:00 today. With C's keys
   added (capacity=50000 upkeep=0.0002 energy_flow=0.05 wither=0.01
   panic=2): 18,034 -> 18,399 -> 29,739. Planet 1 today: 3,088 -> 1,397 -> 1,479.
   With C: 3,088 -> 3,203 -> 6,801. Core world today: 67,442 -> 7,750 -> 9,273.
   With C: 67,442 -> 67,414 -> 54,152 (`raw/planets-5-8.txt`, `raw/planet5-*.txt`).
   This is the run thread's "sheds nine tenths in the first minute". The
   run thread took the fix (11-15 08:24Z).
3. **One slot ceiling on every planet.** C's 50,000 slots stay the same at
   every depth. The core world lands 67,442 walkers and can only shrink. The
   run thread will scale slots by level.
4. **No pressure curve.** Its size follows held crystals (about 9,400 walkers
   a crystal, `examples/foodprobe.rs`), so it peaks early and then only shrinks.
5. **It ignores the player's economy.** `BuildRules::pump_smell` is 0, and
   pumps are the main income since #322.
6. **Its best behaviours are off or missing from the run.** Condensing and
   breakaway are off. The ladder (`run::LADDER`) has neither plain C nor the
   Grudge, and the Grudge is CPU only.

## Commands

```
larder-world look=DIR hand=idle seed=1 at=1,4,..,148 screen=960x600
larder-world look=DIR mode=squads hand=squads seed=1 at=1..60 screen=960x600
larder-world look=DIR hand=take seed=1 at=5,11,..,359 screen=960x600
larder-world look=DIR run=1:5 grow=0 hand=idle at=1,5,..,121 screen=960x600 [C keys]
larder-world look=DIR run=1:D grow=0 hand=idle at=1,30,60,120 [C keys]   # D = 1, 5, 8
```

GIFs made with ffmpeg from the `-all.png` and `-near.png` frames.

## Questions put to the owner

Main threat (ground, hunting or both by tier); how they win; whether a slime
left alone should keep getting stronger through a level; condensing and
breakaway on, and where; a run planet's slime size.
