# Review of 11-24 to 11-27 and the plan to finish Milestone 2 (2026-11-27)

The owner's asks, 2026-11-27: "Review all work over the past few days since
your work stopped" (23:04Z), then "your main goal is evaluate the current
state, the general roadmap, and you need to suggest next steps, how many
there, how we split up the work" (23:34Z). His pick on the plan's card
(23:55Z): **"Five lanes, no freeze"**, and yes to keeping the plan in the
repo (11-28 00:08Z), since the new account sees only this repository.

The claude.ai pages (on the old account): review
https://claude.ai/artifact/6dmCXQ5GC3H1w8LN4ZWGrE, plan
https://claude.ai/artifact/57RBPNiFdCkUk4DZwN28nG. This file is their text.
Reviewed on main `7a88a5a` and `5cd97ab`; read with
[`handoff/account-move-2026-11-27.md`](../../handoff/account-move-2026-11-27.md),
which lists what landed.

## The review: what landed 11-24 16:00Z to 11-27

62 PRs merged from 11-24 16:00Z to 11-27 20:29Z (#815 to #875), then #876
to #881. From 11-25 on, every game change followed the owner's own chat
rulings in his other project, quoted in
[`wiki/decisions-game.md`](../../wiki/decisions-game.md): the star map
(#856, #860, #869), hearts as a crystal mini-boss (#858), toughness and
levels by planet (#862, #864), mini-boss ward and stalking (#865), a faster
early climb (#868), Hard +2.5 and unguarded crystals kept (#870), harder
from planet 2 (#873), turrets that fire their loadout's gun and are bought
like guns (#871, #875, #876, #880), the slime bar and fewer alerts (#872,
#874). The Lab's hunt (#861) is a report; its prototype was taken back out.

**Quality.** Commits quote his words with times, give before and after
numbers and what was rejected, and add tests; what-is-new lines are there;
the wiki lint has 0 errors; the queue kept main green. #875 came from two
adversarial reviews of #871 and closed a free-turret-deck hole.

**Checked here** (cloud, software GPU, main `7a88a5a`):

- Lib tests for route, run, ceiling, origin, heart, levels, blows, bosses,
  turrets: 82 pass, 1 ignored. Window tests for star, alert, gauge, fit,
  save, resume: 9 pass.
- The level's GPU bits (12 to 14 of a walker's dense word, `GRADE_BITS` in
  `engine.wgsl`) collide with no other field, and every `set_dense` keeps
  them, so slime keeps its level through lumps, specks and breakaways.
- `levels.rs` takes crystal 0 as the home crystal; `Game::core` is
  `crystals()[0]`, so it holds.
- Saves: a save from before the star map plays node 0 (Fair) at each step;
  #876 keeps an old save's turrets. A save from before 11-26 resumes at the
  same step number, now an easier place on a twelve-step curve.
- No new off-by-default rule; `src/build/switches.rs` is unchanged.
- **Bot check, the run bot plan (`scripts/run-bot-plan.txt`) on step 3
  Fair, seeds 1 to 4**, paired with `../review-2026-11-26/raw/`
  (`8a863aa`): before 4 of 4 won in 3:01 to 3:49; on `7a88a5a` 3 of 4 won,
  in 4:30, 5:47 and 10:27, seed 3 lost at 9:04. Worse on 4 of 4 (sign
  test p 0.125). Step 3 now meets the Trunk (#868), not the Sprawler.

**Risks, most important first.**

1. Six difficulty raises in about 30 hours (#862, #864, #865, #868, #870,
   #873), each on one or two games, never measured together. The 11-26
   review's step 6 Fair went from 6 of 6 to 0 of 6 before the last three.
   He asked for each, so nothing is reverted.
2. #873's own smoke note: the greedy bot with a bare start is wiped in 0:23
   to 0:36 on step 2 Hard and step 6 Hard, before and after it.
3. The bots don't play today's game: the run bot takes only node 0 (Fair),
   builds turrets by kind with no design, and `strat.rs`'s siege still aims
   at a live heart. The nightly Run bot was red 11-25 and 11-27 (two of
   four seeds).
4. The PC strategy league (branch `claude/project-thread-8grg6d`) still
   pushes results every few minutes from a build of 11-22 (`33fa1db4`), so
   its numbers describe the old game.
5. No frame cost was quoted for the levels' per-step reads (`sense`,
   `born`, `dcount`); the last GTX 1070 timing was 11-24.
6. Small: alerts expire on wall-clock time, so one can go while paused; the
   heart-broken alert says "drain" where the rule is thin or drain.

## Milestone 2's tests on 11-27

| Test | State | Why |
|---|---|---|
| 1. Slime wiped out, the bot wins every step | Not measurable | The bot is behind (risk 3) |
| 2. A run of two hours | Close | His 11-27 run: about 55 min for steps 1 to 6 |
| 3. The curve page | Not measurable | Last pass 11-24, eight planets |
| 4. Save and resume | Holds, recheck | Not yet checked on his PC with a star-map run |
| 5. 60 fps at 4K on the 1070 | Held 11-24, recheck | Levels, bigger raids and Hard home-world sizes since |
| 6. Everything on | Holds | |

## The plan

Six steps over about a week; done around 12-05 if one or two of his runs
settle the difficulty. Long runs go to GitHub's checks (his 11-18 card).

1. **Re-anchor** (Direction). NOW.md for twelve steps (done in #881), the
   lane handoffs current, the PC league stopped or rebuilt on main.
2. **Retune after each of his runs** (Direction with Enemy). His card says
   no freeze: lanes keep raising difficulty on his notes, and each run's
   log gets one retune pass, measured on the bot from step 3.
3. **The bots up to today's game** (Engine and tools, with Squad; one to
   two days). The run bot picks Safe, Fair or Hard by a dial, buys and
   builds turret designs, drops the heart siege, and gets the builder arm
   (pumps plus a turret each, the step-B lure plan). Then the nightly is
   green or names a real wall, and the curve reruns on twelve steps.
4. **Retime on the GTX 1070** (Engine and tools, half a day): a Hard step 9
   and the home world at his 4K screen; a mid-planet save and resume of a
   star-map run.
5. **Pressure from the slime itself** (Enemy, the Lab's rig once). The
   11-25 review's top finding stands: the slime can't reach a squad that
   moves. Rerun the hunt (`../lab-hunt-2026-11-26/`) on today's tougher
   slime, under an hour; if a 2- to 3-hit tongue costs soldiers, build it
   in, else the Lab's "mass, then come in wide".
6. **Give energy a job** (Squad and arsenal, Direction for prices). He
   banked 5,000 to 8,000 and built no turret in his last two runs. Check
   whether #876's designs change that in his next log; then the
   turret-against-soldier value test and a rising soldier price.

Look work goes on beside these in his A to D order, the star map and the
slime colours checked against `wiki/looks/art-direction.md`.

## The split

Five lanes, five threads at once, the Lab paused until step 5 needs its rig
(his card, 11-27 23:55Z).

| Lane | Its steps | Model, effort |
|---|---|---|
| Direction | 1, 2; records every ask he makes in any thread | Opus 5.5, medium |
| Enemy | 2, 5 | Opus 5.5, high |
| Squad and arsenal | 3 (turret designs in the bot), 6 | Opus 5.5, high |
| Engine and tools | 3, 4, the PC league | Opus 5.5, high |
| Look and interface | look work; one thread, not two | Opus 5.5, medium |
| Lab | the hunt rerun only | Opus 5.5, medium |

What would have kept the 11-25 gap from losing context:

- **One home.** Two projects changed this repo for three days and one saw
  none of it. The new account's project is the home.
- **A handoff in every PR.** A lint check that fails a PR changing `src/`
  without touching a `handoff/*.md` file, like the what-is-new check
  (Engine and tools).
- **His asks reach Direction.** A thread he asks directly tells Direction,
  which records the ruling and updates NOW.md the same day.
- **Usage.** Runners on Haiku, docs on Sonnet, no census over an hour
  without his OK.

**Waiting on him:** the PC league (a `PAUSE` file in the league clone holds
it), "Automatically delete head branches", and D8 (the CPU slime engine).
