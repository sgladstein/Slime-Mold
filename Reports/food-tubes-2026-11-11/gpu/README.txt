The race start in the graphics thread's 2048 x 1280 GPU world (scale 8),
examples/worldspread.rs on the GTX 1070, run through Remote Control from
a worktree of this branch (2026-11-12). Each folder: run.txt (stdout) and,
for seed 1, spread.mp4. worldspread writes spread.gif; rounds 1-2's GIFs
(8.7-9.9 MB, committed from the PC in 94c64bd and f1c1e1c) were converted
to spread.mp4 here and removed from the folder. Round 3 uses down=4
(512 x 320 pictures) to keep its GIFs small.

The clock. A worldspread second is 120 engine ticks whatever `ticks` is
(`ticks` only batches them into frames). At scale 8 that is one game
second (8 engine ticks a game tick, 15 game ticks a second), while a
bigdish second on the scale-2 race world is 8 game seconds. So the first
two rounds (seconds=90, 00:35-00:47Z) cover 90 game seconds: what the CPU
census shows at about 11 s, not at 90 s. They were meant to match the CPU
clock and do not; they are kept as early pictures. Round 3 runs
seconds=720 with every=8 (one picture every 8 s, 90 pictures).

Common keys: seed=S walkers=80000 capacity=800000 crystal=512 piles=23
rect=1 smell=144 rock=0.2 wet=0.2 wall=0 ticks=16.
- race8:      laid=4 upkeep=0.001 crystal_eat=0.005 (race_params's numbers; reach 9, keep 0.9)
- race8reach: race8 + reach=36 (4.5 dish cells, as at scale 2)
- race8keep:  race8reach + keep=0.974 (0.9 ** 0.25: trail fades per dish cell as at scale 2)
- race8all:   race8keep with laid=1 upkeep=0.00025 crystal_eat=0.00125 (per-step costs / 4)
Rounds 1-2 (seconds=90, every=1; pictures every 8 frames, 1.07 s):
race8 and race8reach on seeds 1-2, race8keep and race8all on seed 1.
../pics/gpu-*-s1.jpg are pictures 5, 28 and 84 (5.3, 29.9, 89.6 s).
Round 3 (seconds=720 every=8 down=4, 00:55-01:09Z, 184-200 s a run on the
1070): long8, long8reach, long8keep and long8all on seed 1, the same keys
as race8, race8reach, race8keep and race8all. Pictures every 64 frames
(8.53 s); ../pics/gpu-long8*-s1.jpg are pictures 5, 28 and 84 (43, 239
and 717 s, the census's 5, 30 and 90 s). The GIFs (3.1-3.5 MB, committed
from the PC in 444ca5e) were converted to spread.mp4 here (3 pictures a
second) and removed from the folder.
