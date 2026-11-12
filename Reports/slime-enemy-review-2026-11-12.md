# The ideal slime enemy: deep review, and the one-walker rule (2026-11-12)

The owner asked (16:26Z) for a design doc on the ideal slime enemy, then "a
deep review/thought on how we expect to achieve that", and (16:28Z): "consider
is the jones mechanics a good backend for what we want or will we never achieve
it with this approach?" The design doc and its plan are the Claude Doc
[The ideal slime enemy](https://claude.ai/artifact/6Jb8uKgtnHrgP7vy7cqKyZ)
(tab "How we get there"). Five reviewers worked from one brief
([`brief.md`](slime-enemy-review-2026-11-12/brief.md)); their reports are kept
unedited in [`slime-enemy-review-2026-11-12/`](slime-enemy-review-2026-11-12/):

| Area | Report | Answer |
|---|---|---|
| Look and density (L1, L5, L6, P3) | [`review-look.md`](slime-enemy-review-2026-11-12/review-look.md) | Keep; fix the drawing, pack the front |
| The lasting network (L2, B1, P1) | [`review-network.md`](slime-enemy-review-2026-11-12/review-network.md) | Keep; found the one-walker bug; change the angles |
| Pieces and behaviour (L3, L4, B2-B5) | [`review-pieces.md`](slime-enemy-review-2026-11-12/review-pieces.md) | Keep, add a piece layer and creep |
| Play and balance (P2, P4, P5) | [`review-play.md`](slime-enemy-review-2026-11-12/review-play.md) | Keep; the game rules, not the walkers, block play |
| Other backends | [`review-backend.md`](slime-enemy-review-2026-11-12/review-backend.md) | Extend with a coarse Tero-style supply grid |

## The answer

All five say keep Jones's walkers and add layers under them; none says
replace them. Rejected as replacements: a Tero tube graph alone (trunks, but
no body and no exploring), a Creeper World style fluid (loses the mesh and
the CPU/GPU twins, fields, tiers and toolkit), Gray-Scott, Potts.

## The one-walker-a-cell rule was off in every dense run

Found by the network reviewer, checked in the code. `Engine::made`
(`src/gpu/mod.rs`) drops the crowd limit when `max(walkers, capacity)` is
over half the agar cells, and `race_params` (`src/mould/larder_world.rs`)
sets `capacity` to 10 x walkers. So every race-world GPU run above about
7,000 walkers ran without one walker a cell: the 25,000-walker films on the
[one-body page](one-body-2026-11-12.md) (section "Density, lossy sharing,
and the specks"), the 1070 bench of density A below, and possibly the 100k
scale runs. On `map=noise`, `capacity=80000` still drops it and `50000`
keeps it. The CPU `BigDish` has no such check, so at large capacity the CPU
census and the GPU game differ. The technical lead confirmed it (17:10Z) and
asked the GPU thread to fix the check and make it loud; until then, pass
`capacity` at most half the agar.

**Test 1, first look** (one seed, Fork, idle squad, the game's own look on
llvmpipe; raw [`raw/t1/`](slime-enemy-review-2026-11-12/raw/t1/), contact
sheets `img/t1-*.jpg`, films on the page):

| Run | Rule | Mould | Look |
|---|---|---|---|
| `walkers=25000 start=spread food=0 upkeep=0.00001` | off (slots 250k) | 24,701 at 0:30 | mesh at 0:04, a few ropes by 0:30 |
| same, `capacity=50000` | on | 30,402 at 0:30; squad wiped 0:47 | a mesh that coarsens slowly, still a mesh at 0:46 |
| `walkers=25000 upkeep=0.0002` | off | 30,575 at 1:00; squad dead 1:18 | thick ropes along the walls |
| same, `capacity=50000` | on | 28,768 at 1:00, 30,244 at 1:20, 12,290 at 1:30; squad wiped 1:56 | one continuous sheet with trunks to the crystals, no specks; after 1:25 a network of continuous tubes |

The fall at about 1:25 is the starting walkers' food running out together
(0.5 energy at 0.0002 a step lasts 2,500 steps, 83 s). One seed; the
8-seed measure (loops and tube width at 30 s and 60 s, thin share of the
body) is next.

## What else the reviews found

- **The angles we ship shrink a mesh.** Jones 2010 calls sensor 45° / turn
  45° the "minimising" regime. CPU dish, 25k walkers, limit on, 2 seeds
  (`raw/network/q-*.txt`): sensor 22.5° with sensor offset 5 holds 51 loops
  flat from 30 s to 60 s against 12-13 today; tubes 7.0 cells wide against
  17.6. ([wiki: sensors](../wiki/model/sensors.md) records why 22.5° was
  dropped on the small dishes.)
- **Specks live in the thin parts.** They hold about 0.11 walkers a cell at
  5k, 10k and 25k walkers (CPU, 3 seeds, `raw/look/results.txt`). More
  walkers cut the thin share of the body's area from 60% to 22% by piling
  into lumps. The drawn body (trail x walkers in a 5x5 window) leaves gaps in
  43% of thin tube cores against 3% for the trail alone; the 25k film drew
  its thin parts at 8% of their trail against 33% today, because the view
  scales `BODY_N` by the starting count.
- **A cut hunk bursts within a second** (largest part 1.00 to 0.42 at 1 s;
  pieces review), so the 20 s measure scored the wrong thing.
- **Our pace matches Jones's**: his figures at 670, 2,364 and 13,160 steps
  fall at 22 s, 79 s and 7.3 min at 30 steps a second.
- **Play is held back by game rules.** In the 48-seed core census
  (`Reports/larder-world-roadmap-2026-11-11/core/`) farm, wall and fort reach
  the 15-minute cap in 144 of 144 games; only the core walk ends a game
  (`check_core` then `stop_births`); idle survives 46 of 48; farm, wall and
  fort end with 83-85% of their harvest unspent (`raw/play/tally-core.txt`).
  Sent to the integration thread, which owns the rules.

## 1070 cost of density (GPU thread, 16:47Z)

[`gpu-bench-1070.md`](slime-enemy-review-2026-11-12/gpu-bench-1070.md): a
step costs 0.417 ms today, 0.483 ms at `walkers=25000 upkeep=0.0002`
(+16%, not visible in the frame), 0.117 ms at `scale=1`. Run A had the
one-walker rule off; to re-time with `capacity=50000`.

## The plan

Layer by layer, with estimated 1070 costs, in the doc's "How we get there"
tab: the crowd fix and new angles (L2, 0 ms); the drawing from a 9x9 window
with a per-mass threshold (L1, +0.1-0.2 ms a frame); a slow front with
births gated by local density as in Jones, Mayne and Adamatzky 2015 (L1, P1,
B1, +0.05 ms); a coarse supply grid with Tero-style tubes (B2, B3,
+0.2-0.4 ms); a piece layer with creep (B3, under 0.05 ms); alarm graded by
hunger, overrun on, `walker_mass` (B4, P2, P3). Tests in that order: (1)
the one-walker rule at density, (2) the angles, (3) the drawing, (4) a
packed slow front, (5) pieces, (6) the supply grid.

The owner, 17:00Z, in the testing-review thread: "We were developing bad
looking slimes because more runners killed the player too easily (instead
of rebalancing)." So the look is judged by the owner on films and in the
window; game numbers are rebalanced to fit it, and a census only checks
that nothing is broken.
