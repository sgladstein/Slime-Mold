# Capture kit for the whole-game review (2026-11-15)

How a reviewer films Larder World on the cloud machine, which has no GPU:
one proven command for each kind of picture, what it shows, the flags that
change it, and how long it takes. Every command below ran once on
2026-11-15, on llvmpipe with 4 cores, into `/tmp/wgr/kit/`. One proof still
for each command is in `kit/`, a JPEG of 1,280 x 800 at quality 80, under
200 KB each. `kit.sh` re-runs them all and logs the wall time of each.

## Setup (once a machine)

```
apt-get install -y mesa-vulkan-drivers vulkan-tools   # use sudo if you are not root
vulkaninfo --summary | grep deviceName                 # llvmpipe (LLVM 20.1.2, 256 bits)
cd /home/user/Slime-Mold
cargo build --release --locked --bin larder-world      # about 2 min 40 s from clean
```

The binary is `/home/user/Slime-Mold/target/release/larder-world`. Run it
from the repo root, because map files load from there. The first line it
prints, `larder-world: llvmpipe (...) (Vulkan)`, means software Vulkan is in
use. None of the captures below needs an example binary.

## How a capture works

`look=DIR` plays the round with no window. At each game second listed in
`at=` it saves the window's whole picture (the world, the HUD and the
marks) twice:

- `…-NNNs-all.png`: the whole map, fitted to the screen.
- `…-NNNs-near.png`: the near view at zoom 6. It centres on the squad, or
  on the controlled squad in squad mode. With no squad it centres on the
  larder; `centre=` overrides it.

A time with a fraction is named in milliseconds (`…-06300ms-near.png`).
Files are named `MAP-HAND-TIME-VIEW.png`, for example `fork-squads-020s-near.png`.
A run's frames all play one round, so several `at=` values form a clip.

| Flag | What it does |
|---|---|
| `at=T1,T2,..` | Game seconds to save (default `5,30,60`). Fractions are allowed, as in `6,6.1,6.2`. |
| `screen=WxH` | Picture size (default `1280x800`). Earlier reports used `960x600` and `1600x900`. |
| `centre=X,Y[,Z]` | Holds the near view at world point X,Y with zoom Z (default 6; 24 is the maximum). Use the same world units as the `squad at` part of the t line. A held camera turns a frame run into a still-camera clip. |
| `seed=N` | Seed for the map and mould (default 1). |
| `map=` | `fork` (default), `rim`, `flats`, `nest`, `passes`, `noise`, or `planet:DEPTH[:SEED]`, a generated planet without the run. |
| `hand=` | Who plays (default `rush`). See *Hands* below. |
| `mode=squads` | Squad mode (`mode=rts` is the default). |
| `run=SEED[:DEPTH]` | The roguelike run, landing on planet DEPTH (1-8). |
| `mission=NAME` | A campaign mission: `1-landfall`, `2-first-contact`, `3-the-gaps`, `4-cut-the-cord`, `5-kill-zone`, `6-the-nest`, `7-spore-season`, `8-two-moulds`. |
| `range=SEED` | The gun range: everything is free and the bank holds 100,000. |
| `menu=PAGE` | Draws a menu page over the round: `main`, `campaign`, `options`, `keys`, `shop`, `shop-mid`, `shop-build`, `shop-tools`. |
| `drag=FROM>TO/..[/go]` | With `menu=shop..`, drags cards in the shop across the frames. `go` closes the range's armory. |
| `hover=X,Y` | Puts the cursor at screen pixels X,Y, so tooltips show. `select=0` selects nobody. |
| `turret=X,Y` `pump=X,Y` `lure=` `outpost=` `siren=` | Puts a whole building there from the start, paid for. Repeat a flag for more of that kind. |
| `boom=T,X,Y` | Fires the squad's round at X,Y at game second T, for free. Use it with `rounds=bomb`, `blast`, `gas` or `germ`. |
| `rounds=NAME` `gun=` `ammo=` | The squad's loadout. |
| `bank=N` | Starting food. |
| `own=1` | Your own mould's garden: your green species against the enemy. |
| `condense=300` `roam=300` | Condensing and breakaway. Both are off by default. Breakaway needs condense. |
| `shape=a..e` | The enemy mould's shape (C is the default). |
| `plants=0`, or any engine key | Engine keys pass through. |
| `cut=T,X,Y,R[,HOLD]` `cutsquad=` | Cuts a ring out of the ground. |
| `sound=FILE.wav` | Also records the round's sound. |
| `holes=1` | Times how long shot holes take to refill. This is slow. |

**Hands.** These are scripted: `rush` (the default; everyone attacks the
core), `farm` (harvest the crystal nearest the larder, buying soldiers from
1,200 food), `idle`, `stand`, `volley`, `edge`, `hold[@X,Y]`, `push`, `far`,
`build`, `buildq`, `siren`, `pumps`, `greedy`, `sow` (own mould),
`squads` and `squadwork` (squad mode). The player AI's styles also drive
the round through `larder_world_ai`, as the census does: `boom`, `claim`,
`cut`, `fort`, `late`, `strike`, `take`, `take_boom`, `take_claim`,
`take_fort`, `take_strike`.

## The proven commands

Wall time is the whole run, including about 2-3 s of start-up and the
picture saves, divided by the last game second captured. Prefix every
command with `./target/release/larder-world look=/tmp/wgr/kit/NAME`.

| # | Capture | Command (after `look=DIR`) | What it shows | Wall | s per game s | Still |
|---|---|---|---|---|---|---|
| 1 | Squad mode on the Fork | `mode=squads map=fork seed=1 hand=squads at=1,20` | Two squads of 4 with the squad panel (bottom left: `AUTO` and `YOU`), following the controlled squad | 16.5 s | 0.82 | `kit/squads.jpg` |
| 2 | Campaign mission | `mission=1-landfall hand=claim at=1,20` | Landfall on the Flats with the mission's goal line. Any mission name works, with `hand=` any AI style. | 12.8 s | 0.64 | `kit/mission.jpg` |
| 3 | Run, first planet | `run=1 hand=farm at=1,20` | Planet 1 of run seed 1 at the level the menu's Run gives | 9.5 s | 0.48 | `kit/run1.jpg` |
| 4 | Run, planet 4 | `run=1:4 hand=idle at=1,20` | Planet 4's level (a bigger map) | 22.8 s | 1.14 | `kit/run-d4.jpg` |
| 5 | Run, planet 8 | `run=1:8 hand=idle at=1,20` | Planet 8, the largest map, many crystals and star-shaped colonies. **Slow**: 224 s for 20 game s. | 223.8 s | 11.2 | `kit/run-d8.jpg` |
| 6 | Run shop | `run=1 menu=shop at=1` | The shop after planet 1, won with two stars | 3.1 s | (still) | `kit/run-shop.jpg` |
| 6b | Run shop, three planets in | `run=1 menu=shop-mid at=1` | Cards in the gun and bag, a second gun, points left. Use `shop-build` or `shop-tools` for a folder open. | 3.1 s | (still) | `kit/run-shopmid.jpg` |
| 7 | Turret and pump standing | `map=fork seed=1 hand=idle turret=135,208 pump=55,224 centre=95,210,6 at=1,10` | A pump on its crystal (it snaps to 60,225) and a turret, both whole. The log lists each building's hp and `working`, and `pumped N food`. | 9.3 s | 0.93 | `kit/buildings.jpg` |
| 8 | Plants burning | `map=fork seed=1 hand=idle rounds=bomb boom=3,200,105 centre=200,105,8 at=6,12` | A bomb into the forest west of the larder at 3 s. A burning ring with char inside at 12 s. | 12.2 s | 1.01 | `kit/fire.jpg` |
| 9 | Your own mould | `own=1 hand=sow at=1,20` | The garden's race world: your green mould and the enemy's. The log adds `own mould N (green), enemy M`. | 34.2 s | 1.71 | `kit/own.jpg` |
| 10 | Condensing and breakaway | `map=fork seed=1 hand=farm condense=300 roam=300 at=1,40` | The body round the crystals after it has condensed (how it looks: `Reports/condense-2026-11-14/README.md`). Condensing starts after 10 s ripe; look from about 20 s on. | 33.4 s | 0.83 | `kit/condense.jpg` |
| 11 | Gun range | `range=7 hand=stand at=1,10` | The range round: the squad standing and firing, with the goal line "Gun range: H opens the armory..." | 9.6 s | 0.96 | `kit/range.jpg` |
| 11b | Gun range armory | `range=7 menu=shop hand=stand at=1` | The armory with every gun and bullet card. Add `drag=Carbine>Carbine/Standard blast>1/go at=1,1.05,..` to build a gun, then fire it. | 4.1 s | (still) | `kit/range-shop.jpg` |
| 12 | Player AI's best style | `map=fork seed=1 hand=take_claim at=1,40` | `take_claim`, the plan matrix's best (cut, claim, 16 turrets, 4 farmers) | 29.7 s | 0.74 | `kit/ai-take.jpg` |
| 13 | Close-zoom still | `map=fork seed=1 hand=farm centre=262,126,24 at=20` | Zoom 24 (the maximum) on the farmers at the larder's crystal: soldiers, muzzle flashes and slime texture | 15.5 s | 0.77 | `kit/zoom24.jpg` |
| 14 | Burst, 0.1 s apart | `map=fork seed=1 hand=rush at=6,6.1,6.2,6.3,6.4,6.5` | Six frames of the rush into the body. Add `centre=X,Y,Z` for a still camera. | 10.7 s | 1.65 | `kit/burst.jpg` (6.3 s) |

Notes from the proof runs:

- **Burst time.** The first burst try at 20-20.5 s found all of the rush's
  soldiers dead (`soldiers 0`). Pick a time when the squad is alive; check
  the t line's `soldiers`.
- **Fire placement.** The first fire try, the fire report's
  `map=planet:2:1 boom=3,60,240`, now lands on bare barrens, as did
  `boom=3,180,190`. Both gave only a scorch. The planets have been
  regenerated since that report. Aim a bomb at plants you can see in an
  earlier still.
- **Condense control.** At 40 s on the same seed and hand, the mould count
  is 16,864 with condense on and 24,870 with it off. A condensed walker
  counts once, so the count roughly halves. The kit prints no separate
  count of condensed walkers; the picture is the evidence.
- **Planet 8** costs about 11 s of wall time per game second here, and the
  ground field alone takes 19.5 ms. Keep its clips short, or use
  `screen=960x600` and few frames.
- **Start-up.** A one-still run (`menu=shop at=1`) takes 3-4 s, which is
  about the fixed cost of start-up plus one picture pair. Most of a longer
  run's time is the game seconds simulated between frames: about 0.5-1.7 s
  of wall time per game second on the maps above, and 11 s on planet 8.

## How to read the printed counts

After each saved time the run prints the two picture paths, then:

```
  CPU: ground field 3.59 ms (once a game second), model and HUD 0.162 ms a frame
  t 20 s: mould 21920, soldiers 8, bank 423, harvested 0, shots 231, killed 3027, took 0%, crystals left 0.998; squad at 277.0,134.9
  squad: touching 0.2 mean, 1 most; health 71 mean
    turret at (135, 208): hp 299 / 300, built 1.0, standing
    pumped 75 food
```

- **CPU**: what the window would add each frame on the CPU. It is a cost
  note, not part of the game.
- **t S s**: the game second.
- **mould**: the enemy's living walkers. A condensed walker counts once.
- **soldiers**: your soldiers alive.
- **bank**: food in hand.
- **harvested**: food your soldiers carried home this round.
- **shots / killed**: shots fired by the squad, and walkers they killed.
- **took**: the share of the crystals' charge that you took, against the
  mould's take (score v2's stalemate measure). It reads 0% before anything
  is taken, as a rush does.
- **crystals left**: the charge left in all crystals, as a share of the
  start.
- **ended: …**: appears when the round has ended: `mould wiped`, `squad
  wiped`, `larder overrun` or `cap`. A mission's verdict also stops the
  steps.
- **squad at X,Y**: the squad's middle in world units. These are the units
  `centre=`, `turret=`, `boom=` and `cut=` take, so a printed position can
  be pasted back as a camera or building spot.
- **squad: touching / health**: walkers touching each living soldier (mean
  and most), and their mean health out of 100.
- **Building lines** (`turret at (x, y): hp / max, built, standing,
  working`) and `pumped N food`: printed when buildings stand.
- **own mould N (green), enemy M**: printed with `own=1`.
- **rounds t S s: … casts (blast, gas, germ, bomb [..]), N walkers killed,
  N flung**: printed when the loadout has rounds. `boom t … : N killed, N
  flung` is printed for each staged boom.
- With `drag=`, `drag: frame N pointer X,Y down|up` and `drag: the gun
  fires …`. With `sound=`, `heard: …` and `voice t …` lines.

The near view's camera centre is the t line's `squad at`, so a reviewer can
tell where the near picture is looking.
