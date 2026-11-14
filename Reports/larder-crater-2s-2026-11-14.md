# Two-second craters (`Rules::crater` 2), 2026-11-14

The Larder thread. Each hit's crater now lasts 2 game seconds, not 4.5: the owner's pick. Raw files are in [larder-crater-2s-2026-11-14/raw/](larder-crater-2s-2026-11-14/raw/), and its [README](larder-crater-2s-2026-11-14/raw/README.md) names each one.

**Why.** The owner played shape C on 11-12 and said (22:18Z): "2) The craters last too long." On 11-14 the thread put a card to them, "Pick how shot craters should fade in the slime", with three choices:

- Shorter craters, 2 s. This was recommended.
- Shallower craters. A crater would keep half the slime trail under it, so the mould closes the hole sooner, and thick mould closes it fastest (`scorch_keep` 0.5).
- Keep 4.5 s.

The owner picked **"Shorter craters, 2 s"** at 01:05Z. The card's reason was that 2 s answers "too long" directly. Also, since aims see their slime (#198), a squad by a wall fires about four times as often (47 shots in 26 s before, 198 after), so a hole under steady fire is renewed before it can close.

**The change.** `Rules::crater` defaults to 2.0 (was 4.5 since the owner's 11-12 17:29Z pick, and 10 before that). It applies on both engines through `squad::Frame::crater`. `crater=4.5` (window, `larderworld`) plays the old time. The window's usage line still said 10 s, stale since 11-12; it now says 2.

## How fast a hole refills (shape C, the Fork, seed 1, llvmpipe)

The measure is the window's `holes=1` timer. For each hit it takes the drawn body (trail × living walkers, the view's `body`), averaged over the hit's strike disc. It counts the hole as refilled once that average is back to 80% of the tick before the hit. The game runs 15 ticks a second. Every scene below ran at the old 4.5 s (`crater=4.5`) and then at the new default, with the same binary.

| Hand | Hits (4.5 s / 2 s) | Median to refill | Quarter of holes slower than | Still open at the end | Kills | Kills a shot |
|---|---|---|---|---|---|---|
| `volley`'s walk, 9 soldiers, to 22 s | 170 / 167 | 3.93 s → 0.93 s | 8.2 s → 2.1 s | 34 → 28 | 1,297 → 1,196 | 7.6 → 7.2 |
| `stand`, the start squad on Defend | 90 / 75 | 1.27 s → 0.53 s | 2.8 s → 0.7 s | 0 → 0 | 800 → 650 | 8.7 → 8.6 |
| `rush` | 45 / 45 | 3.13 s → 0.53 s | 3.9 s → 2.5 s | 7 → 10 | 1,135 → 1,061 | 24.1 → 23.6 |

A hit's hole now refills in about half a second to a second, against one to four seconds before. A shot kills 2 to 6% fewer walkers, because walkers flow back into the hole sooner and the next shot meets a thinner patch. The `stand` squad fired 76 shots against 92 before it died, and both runs had no soldiers left at 30 s. That is one seed, so it is not a measure of how long squads live; the smoke games below are the check on that.

## Smoke games

This is the CPU census on the race world, with shape C's keys and no withering. It ran seeds 5-8 with a cap of 600 s and five hands, at `crater=4.5` against the default 2. It is the wading report's smoke command plus `crater=`:

```
larderworld census seeds=5-8 walkers=25000 upkeep=0.0002 keys=capacity:50000+energy_flow:0.05 hands=idle,random,farm,strike,rush cap=600 threads=4 crater=4.5|2
```

| Hand | 4.5 s | 2 s |
|---|---|---|
| idle | capped 3 of 4 at -0.333; wiped at 586 s | capped 2 of 4 at -0.333; wiped at 576 and 599 s |
| random | wiped 4 of 4 at 70-114 s | wiped 4 of 4 at 73-114 s |
| farm | capped 4 of 4 at +0.074 to +0.089; kills 23,661-33,107 | capped 4 of 4 at +0.068 to +0.092; kills 22,391-28,992, lower on 4 of 4 |
| strike | capped 4 of 4 at +0.066 to +0.094 | capped 4 of 4 at +0.051 to +0.091; kills lower on 3 of 4 |
| rush | wiped 3 of 4 at 122-215 s | wiped 3 of 4 at 119-250 s |

The smoke checks pass at 2 s: idle never wins, no game ends inside a minute (the shortest is random's 73 s), and nothing soft-locks.

Two pairs move by more than 0.015:

- **Idle, seed 5.** Its last starving soldier died at 599 s instead of living to the 600 s cap, so the score falls from -0.333 to -1.002. That is the wipe rule, not the crater.
- **Rush, seed 5.** It lasted 35 s longer (+0.058).

Farm's kills fall on all four seeds, by 5 to 23%, because shorter holes leave less to kill. Its harvest is higher on 2 seeds, the same on 1 and lower on 1. Each arm took 16 to 17 minutes on 4 threads, sharing the machine with the look runs. Raw: [`smoke-c-crater4.5.txt`](larder-crater-2s-2026-11-14/raw/smoke-c-crater4.5.txt), [`smoke-c-crater2.txt`](larder-crater-2s-2026-11-14/raw/smoke-c-crater2.txt).

## Every scripted squad dies by 30 s on C, and a burn line holds fire

All four look hands lose their whole squad on C by 30 s, at either crater time. `rush` and `stand` are walked or stood in reach of the body, as the owner asked (walking in "should be very deadly"). `volley` fails for another reason.

The `volley` hand was written for the 5,000-walker mould (11-12). It walks the squad from 15 s to a stand point short of the ring round the core crystal, and at 22 s it orders a burn line (`Order::Burn`) across the ring. On C, with wading counted whole (#197), the squad is still about 58 cells short of its stand point at 22 s (48 at 23.5 s), wading through slime with 8 to 17 walkers on each soldier ([raw](larder-crater-2s-2026-11-14/raw/squad-volley-4.5.txt)).

A burning soldier aims only at the line's five points. Those points were 66 to 76 cells away, out of its reach of 34, so no soldier fired at all from 22 s. The shot count stayed at 170 while the walkers on each soldier went from 1 to 17, and the squad was gone by 24 s. The `edge` hand stands the squad on free fire instead, and it fired 19 more shots and lived one second longer.

In the window the same thing happens to a player who orders a burn line far away: the soldiers walk to it and don't shoot the slime that reaches them on the way. That goes against the owner's 11-13 16:15Z ruling that a soldier rushed while standing still must be able to shoot what reaches it. The fix went in as the next PR, because the census's random hand orders burn lines and so needed smoke games of its own. The `volley` hand also needs its walk retuned for C before it can film the ring again.

## A burning soldier shoots what reaches it on the way

In the next PR, a burn point out of the soldier's reach (the shot's own test: range plus 4 fine cells) falls through to the soldier's own aim and stance (`Game::target`). A point in reach is fired at as before. On the default Defend stance, the soldier then shoots walkers that are on it or near it, and holds fire at slime that is far off. The test is `a_soldier_walking_to_a_burn_line_shoots_what_reaches_it`, and it fails without the fix.

**The volley squad at 2 s craters** (C, the Fork, seed 1; [before](larder-crater-2s-2026-11-14/raw/burn-volley-2-before.txt), [after](larder-crater-2s-2026-11-14/raw/burn-volley-2-after.txt)):

| | Shots after 22 s | Kills by the wipe | Wiped |
|---|---|---|---|
| Before | 0 | 1,235 | 24 s |
| After | 22 | 1,813 | 25 s, about 20 cells deeper |

The squad still dies. In this build it had been starving since 1 s (the hand spends the whole bank on soldiers, and rations were on) and was at health 39 when it reached the slime. Rations have been off by default since #220, from the owner's 11-14 01:49Z playtest.

**Smoke games.** Only the census's random hand orders burn lines, so only it was rerun. It ran on seeds 5-12 with C's keys and the 600 s cap, using one binary built from each side ([before](larder-crater-2s-2026-11-14/raw/smoke-random-before.txt), [after](larder-crater-2s-2026-11-14/raw/smoke-random-after.txt)):

- Random still loses all 8 games. They are wiped at 74-137 s, against 73-170 s before.
- Shots and kills are higher on 7 of 8 seeds.
- Scores are higher on 4 seeds, lower on 2 and tied on 2.
- Seed 10 is the one that fell, from -1.717 to -1.774: it was wiped at 136 s instead of 170 s.
- No game ends inside a minute.
