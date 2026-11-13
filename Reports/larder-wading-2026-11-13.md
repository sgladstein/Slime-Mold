# Wading counts every walker (`Rules::wade`), 2026-11-13

The Larder thread. Soldiers in thick slime now slow down on shape C: the slowdown counts every walker on a soldier whole, as the bite already did. Raw files, the page's source and its clips are in [larder-wading-2026-11-13/](larder-wading-2026-11-13/); its [raw README](larder-wading-2026-11-13/raw/README.md) names each file. Clips page: https://claude.ai/artifact/DKoWuDZja5GGeMTUuJFLMR.

**Why.** The owner, 15:55Z, after the sink page's clips: "I have said this before, but 1) this is really hard to judge when the soliders are just running into the slime, 2) why are they not way slowing down. I thought we implemented that, 3) Running into the thickest part of the enemy should be very deadly". Then (15:56Z): "why do you measure it instead of checking the code to see if it was implemented. I was shown a video at one point where it worked".

**What the code said.** Wading (PR 3b, #143) is on: a soldier walks at `speed / (1 + x²)`, `x = touching × mass / wade`. `Rules::mass` (#164) is 5,000 over the start walkers at scale 2, so 0.2 for shape C's 25,000. Half speed took 30 walkers on a soldier, not 6. The video the owner remembered was the 5,000-walker mould, mass 1. The bite (`Rules::drain`) stopped using mass in #174; wading kept it.

**The change.** `x = touching / wade`. Every walker counts whole at any mass; `wade` 6 stays.

| Walkers on a soldier | 2 | 6 | 12 | 20 | 26 |
|---|---|---|---|---|---|
| Speed before on C (counted at 0.2) | 100% | 96% | 86% | 69% | 57% |
| Speed now | 90% | 50% | 20% | 8% | 5% |

At mass 1 (the 5,000-walker mould, the census's default) nothing changes.

## What was wrong with the last clips

The sink page's rush clip (PR #181) showed four soldiers walking into C's body at 6-14 s and standing there to 20 s with none lost. That binary was built before #174 merged into the branch, so the bite was still 1 × mass, 0.2 health a second a walker. On today's main the same hand (`hand=rush`, Fork, seed 1, llvmpipe) loses all four by 10 s with the old wading, with 16 to 28 walkers on each soldier from 6 s ([raw](larder-wading-2026-11-13/raw/preview-r-a0-w30-b45.txt)). With the new wading it loses them by 12 s, slowed short of the core ([raw](larder-wading-2026-11-13/raw/preview-r-w6.txt)). So point 3 was already true on main. The owner was shown stale clips.

## The clips (shape C, the Fork, seed 1, llvmpipe)

Two new look hands, both buying the squad to eight at once with 400 food kept: `hand=push` walks the eight into the body from 3 s (`Order::Attack`, the rush's order). `hand=hold@X,Y` stands them at (X, Y) on free fire.

**Walking in** (`hand=push`, `wade=30` for the old rule on C, against the default):

| | 8 s | 10 s | 12 s | Wiped |
|---|---|---|---|---|
| Before (walkers at 0.2) | 8 soldiers, health 92, 3.5 on each | 8, 65, 12.9 | 3, 24, 19.0 | 13 s |
| Now (walkers whole) | 8, 93, 2.4 | 8, 75, 7.5 | 8, 46, 17.0 | 15 s |

- Deadly either way: both squads die within 5-7 s of the first walkers reaching them.
- Slowed: by 12 s the squad has walked 104 world units from the larder, against 128. That is why it lives 2 s longer: it never gets as deep.
- Raw: [`clip-push-before.txt`](larder-wading-2026-11-13/raw/clip-push-before.txt), [`clip-push-now.txt`](larder-wading-2026-11-13/raw/clip-push-now.txt); to the wipe, [`preview-pl-w30.txt`](larder-wading-2026-11-13/raw/preview-pl-w30.txt), [`preview-pl-w6.txt`](larder-wading-2026-11-13/raw/preview-pl-w6.txt).

**Standing at the edge** (`hand=hold@318,148`, the mouth of the right-hand channel, against `sink=0`). This answers the owner's point 1: a squad that never walks in, to judge the sink.

| 4-30 s | Shots | Killed | A shot | Health at 30 s |
|---|---|---|---|---|
| Blast on the first slime (`sink=0`) | 58 | 271 | 4.7 | 44 |
| Blast sunk one radius in (default) | 47 | 261 | 5.6 | 50 |

- One seed and one stand, a picture, not a census.
- **The squad fires rarely**: about 50 shots in 26 s from eight soldiers that could fire about 200. A soldier's aim is the densest block of walkers within its sight (`squad` frame, `UnitOut::target`, both engines), found with no wall check. `Game::target` then drops the shot when a wall or rock blocks the line (`clear_line`). The likely cause: the densest slime in sight is the lobe beyond the right-hand wall, so the slime in front goes unshot. Not yet checked by a probe. It is the next fix, and this stand barely tests the sink until then.

## Tried and dropped: a soldier's blast kept off its own touch circle

Built and measured, not merged ([patch](larder-wading-2026-11-13/raw/arming-not-merged.patch); `Rules::arming`). A soldier's bullet could not strike until it had flown past its own touch circle and one blast radius (4 dish cells). The first leg flew that much further, and the engines traced only the part past it, so the bullet was still drawn from the soldier.

- **Why tried.** From the stale clips I read that a soldier in the slime blasted its own feet clear, and its crater kept the walkers off.
- **Why dropped.** On today's main walkers sit on soldiers in the body anyway (16-28 each, with or without it). And the owner, 16:15Z: "will this make it so a solider can never kill a walker close enough to it. This could be a problem in a different situations where I solider is standing still and gets rushed by walkers".
- **What it measured.** These runs all had arming and new wading together, against neither, so its own share is not separated.
  - Walking in (`hand=push`): wiped at 14 s with it, 13 s with neither, 15 s with wading alone.
  - Four soldiers idle at the larder: dead by about 21 s with it, about 23 s with neither.
  - Eight standing at the larder on free fire: health 21 at 60 s with it, 19 with neither.
  - The CPU thick probe (`examples/thickprobe.rs`, race world, 240 s of growth, then the squad attack-moved into C's thickest part, seeds 1-4): 40 soldiers lost 18, 27, 0 and 15 with it, against 4, 14, 0 and 15 with neither; first deaths at about 2 s against 16-23 s. 10 soldiers lost 4, 3, 2 and 3 against 3, 2, 5 and 3. Raw in [`raw/thickprobe/`](larder-wading-2026-11-13/raw/thickprobe/) (`new` is arming with wading, `old` neither).
- The probe now has a `wade=` key, and no rations or overrun. A squad held at the larder for 240 s otherwise lost the round before the order on seeds 1, 2 and 4.

## Smoke games

Running at the time of the first commit (CPU, shape C's keys without withering, race seeds 5-8, five hands, `wade=6` against `wade=30`); results land here before the merge.

## Cost

None. One multiply fewer in the walk.
