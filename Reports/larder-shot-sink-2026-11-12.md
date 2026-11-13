# Shots that sink into the slime (`Rules::sink`), 2026-11-12

The Larder thread. A shot's blast is now centred one strike radius inside the slime instead of on its edge. Earlier Larder World work is in the [roadmap report](larder-world-roadmap-2026-11-11.md); raw files, the page's source and its clips are in [larder-shot-sink-2026-11-12/](larder-shot-sink-2026-11-12/).

**Why.** The owner, playing shape D (22:06Z): "if I keep my group of soliders in one spot and have them fire on a large group of mold it barely seems to do damage, but if I move them into the mold, they destroy it easily. I think it has to do with the craters." The owner had free fire on (23:10Z).

- A shot walks its line and stops at the first cell that reads as slime (or a walker in a crater). Before this change its blast was centred there, on the body's near edge, so about half the disc struck open ground.
- A squad inside the body has slime at its feet, so its blasts strike slime all round.
- The craters were not the cause: a crater keeps struck cells under slime, so the next shots fly on and the hole deepens. But each blast still sat on the hole's edge.
- Asked "A possible fix is to centre the blast one blast-width inside the slime, so shots from outside take a full bite. Want me to try it?", the owner said "Yes" (23:14Z).

**The pre-test** (CPU, shape D, a squad standing 9 cells off the core, seeds 1-2, on the shapes branch with an environment-variable hack, not committed). It used a cruder rule: the furthest agar cell within one strike radius.
- Kills a shot went from 10.8 and 13.3 to 12.2 and 15.5 (+13-17%).
- Hits on fresh slime, as opposed to walkers in a crater, killed about twice as many: 8.1 to 16.0 and 12.2 to 20.2.
- On shape A's thinner arms it made no difference.

**The rule** (`squad::sink`, `Frame::sink`, `Rules::sink` 1). It was first built as `bite` (and the clips and smoke runs below used the key `bite=`); it is named sink because the mould's bite is its damage to soldiers (`Rules::drain`, which #174 doubled).
- From the first cell that stops the shot, look on along the line, cell by cell. Keep going while the cells are on the agar and within the sink's reach of that first cell.
- The reach is floor(sink² × the strike's r²) in whole cells: 25 cells² for the default strike of 2.5 dish cells at scale 2.
- Centre the blast on the last cell in reach that would itself stop a shot (slime, or a walker in a crater).
- So against a thick body the blast lands one radius in. Against a strand it stays on the strand, because a blast on the empty ground past a strand takes less of it than one on its edge. The cell first met always lies inside the new blast, since the reach equals the radius.
- Everything else follows the moved hit: kills, the cleared trail, the crater, the alarm, the wound in the look, and the shot's tracer end.
- On the GPU the `shots` pass does the same from `Params::shot_sink` (sink², f32), the spare word after the tend row (no new row, no layout change). It uses the same integer distances and the same f32 product, so the two engines agree to the bit.
- The garden's shots (`own_mould`) sink into the enemy's slime the same way.
- Turrets' shots sink too: their shots go through the same frame (one rule for every shot, the buildings thread's call).
- Keys: `sink=` on the window, the census and the look. `sink=0` is the old edge hit.
- New look hand `hand=edge`: volley's walk to the thick body, then stand still on free fire where volley burns a line.

**Clips** (the page: at 60557ab, before #174 made C the window's default and doubled the mould's bite on soldiers; GPU on software Vulkan, shape C, the Fork, seed 1, paired by hand: same seed, sink 0 and 1; raw t lines in [`raw/`](larder-shot-sink-2026-11-12/raw/)). The deciding measure, named before the runs, was kills a shot while the squad stands still on free fire.

| hand, window | sink 0, kills a shot | sink 1 | mould at the window's end, 0 / 1 |
| --- | --- | --- | --- |
| edge: standing still on free fire, 26-34 s (72 shots each) | 12.6 | **20.3** | 23,516 / 22,911 |
| edge, from 22 s (walking the last of the way, then standing) | 21.0 | 25.6 | |
| rush: walking in, 10-20 s (40 shots each) | 42.0 | 47.9 | 23,263 / 22,853 |
| volley: burning a line into the core's ring, 26-34 s (72 shots) | 19.5 | 20.6 | 24,056 / 24,003 |
| stand: firing at thin wisps by the larder, 22-40 s (72 shots) | 5.5 | 5.2 | 26,594 / 26,467 |

- **The owner's case improves most: +61% a shot while standing still.** The gap to walking in narrows from 3.3 times (42.0 against 12.6) to 2.4 times (47.9 against 20.3).
- Walking in is still deadlier a shot. A squad standing still empties the slime within its reach and the craters keep it empty. A squad walking in keeps meeting fresh slime, and pays for it in wading and the mould's bite. If the owner wants standing fire stronger still, `sink=1.5` or `sink=2` sinks the blast deeper.
- Volley barely changes: its burn line already drives each shot deeper as the craters open the way.
- The stand hand's wisps don't change, as the strand rule intends.
- One seed and one map: a smoke check and a picture, not a census (the owner's 17:26Z testing rule).
- The volley and edge games end at 36 s by the larder overrun (`Rules::overrun`, on since PR 4), because these test hands spend the whole bank on soldiers in the first second. The clips stop there.

**Twins.**
- `the_squad_matches_the_cpu_for_one_frame` has two new cases: sink 1 on the plain world with craters, and sink 1.5 with cone craters. One more turns it on for the Swarmer's alarm case.
  - Every output, walker, trail cell and crater cell agrees between the engines on llvmpipe.
  - In the same frame the sink takes the kills from 19 to 28 (30 at 1.5).
  - The test also checks that the sink moves at least one hit in each sink case and none without it.
- A CPU test, `a_hit_sinks_its_blast_to_the_last_slime_within_reach`, covers:
  - the strand;
  - slime two and four cells on, at sink 1 and 1.5;
  - a wall between, which the sink doesn't cross;
  - a thick body, where the blast lands one radius in.
- **The core-pull game twin** (`a_squad_at_the_core_draws_the_mould_on_the_engine_as_on_the_cpu`, seed 3, 30 s) parted 10% at 25 s with the sink on: 1,388 walkers on the GPU against 1,260 on the CPU, past its 8% bound.
  - Its over-seeds chaos check (`twin_gap_over_seeds_is_chaos`, seeds 3-10, default rules, so with the sink) puts the GPU's mean gap at 30 s at 104 walkers (pull 8) and 84 (pull 0). The CPU with one walker more is further from itself, at 142 and 126, and the GPU is above on 4 of 8 at each pull. On seed 3 itself the one-walker CPU sits 35 walkers off at 25 s; on seed 10 it sits 334 off.
  - So the drift is chaos, not a rule the engines differ on. The core-pull twin holds the sink off, as it already holds solid crystals off for the same reason.
- The other game twins (craters, flee, buildings, maps) pass with the sink on.
- 1070 twin: pending. It is booked on the PC behind the three timing jobs.

**Smoke games** (CPU, plain mould, race seeds 5-8, five hands, about 4 min each with the sink on and off at 60557ab; raw [`smoke-sink1-race-5-8.txt`](larder-shot-sink-2026-11-12/smoke-sink1-race-5-8.txt), [`smoke-sink0-race-5-8.txt`](larder-shot-sink-2026-11-12/smoke-sink0-race-5-8.txt)):
- Idle starves on all 4, at 570-595 s (522-597 off).
- Random loses all 4, at 100-375 s (103-258 off).
- Farm reaches the cap on all 4 with its 8 soldiers. It scores higher on all 4 with the sink (+0.21 to +0.33, against +0.15 to +0.28): it ends with a smaller mould (2,159-4,720 walkers, against 3,031-6,775), banks more, and kills fewer walkers in all (13,186-30,451, against 28,001-49,715), because fewer come.
- Strike caps 2 and is wiped on 2 (767 and 612 s). Off, it capped 2, was overrun on 1 and wiped on 1. Its 30-soldier army is the player AI's, and it starves on thin seeds either way.
- The rush is wiped on all 4 (60-819 s); off, on 3, with seed 8 sitting at 0 soldiers to the cap. The hand walks its squad into the body.
- The shortest game is the rush on seed 5 at 60 s (58 s off). Farm and wall never stall.
- Farm and strike beat idle and random 4-0 on both arms. Four seeds can't pass Holm, and nothing here asks them to.

**Cost.** A hit that sinks reads at most about ten more path cells on one GPU thread (shots run one a thread, 256 at most), and the CPU does the same in its loop. That is too small to show in a frame. No 1070 timing: the owner's timing jobs hold the PC.
