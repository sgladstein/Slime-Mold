# Mould tiers for the campaign: Drifter, Sprawler, Trunk, and a mould that remembers (2026-11-12)

The campaign's missions 1-4 ([design](larder-campaign-2026-11-12.md)) need three tiers that were only names on the ladder: the Drifter (slow, grazing), the Sprawler (fast, thin) and the Trunk (thick, heals). After them came habituation, a mould that learns from where it is hurt (review 2's second candidate, [the reviews](mould-tiers-review-2026-11-11.md)). Asked for by the coordinator at 05:10Z, with the owner asleep (04:48Z: threads settle questions together). Thread: *Build new mould tiers for the campaign*.

## Summary

- **Three shapes, each told apart at a glance, on rules both engines already run.** `mould::tiers::Tier::{Drifter, Sprawler, Trunk}` are settings, not new rules. The **Drifter** is droplets: round bodies on crystals, crawling between them on faint threads (sensors 90 degrees apart, trail kept 0.93, a packed walker lays its trail). The **Sprawler** is explorers that run from trail before they join the network: a thin mesh over the whole map within 15 s, with about 1,100-1,600 walkers in it (seed 1). The **Trunk** is long straight cords (sensors 22.5 degrees apart reading 18 fine cells out, trail kept 0.95) round a few big bodies. Picked by eye from 15 candidates on seed 1 (`look.sh`, `candidates-*.txt`).
- **The GPU runs them as the CPU does.** The race world's GPU settings with each tier give exactly the CPU's params (a test), and the Larder game on software Vulkan keeps each mould within 2% of the CPU's every 5 s over 20 s (`the_campaign_tiers_run_on_the_engine_as_on_the_cpu`). No new GPU row, no species slot.
- **Habituation is a scar field** (`BigParams::scar_keep`, `scar_gain`, `scar_weight`; CPU, off by default): a per-cell memory of the alarm a hit lays, kept about 40 game seconds, read at the sensors. Two tiers read it opposite ways. **Wary** (weight -0.5) keeps away from where it was shot again and again: 20 s after the last of 16 strikes, 17-78 walkers at the spot against 149-1,466 with the same alarm and no scar, fewer on 8 of 8 seeds (p 0.008, holds over 3). **Grudge** (weight +0.5, with the Swarmer's alarm) comes back there in force: more than the Swarmer without the scar on 7 of 8 seeds (p 0.07), then on 8 of 8 fresh seeds named before their run (919-1,516 against 192-1,115, p 0.008).
- **The census** (each tier against idle, random and seven plans, held-out seeds 201-216, at #106's rules: free shots, no flee, **before shot craters (#113), so these difficulty numbers are not final**). **Grudge is the hardest tier**: every plan scores lower against it than against the plain mould on 16 of 16 seeds (strike 14 of 16), and random loses its squad on 15 of 16 (9 on the plain mould). The **Trunk** is a little easier than plain under most plans and strike still wipes it (12 of 16). The **Sprawler** at 400 exploring steps was wiped by every plan and by random, so it now explores 200 (re-run in progress). Strike is the best plan on every tier, so the best plan does not change between tiers yet (the designer's bar fails). The toolkit's blast plan wins on all six tiers (8 of 8), and the scar does not answer it.

## The three shapes

World: the census world, `larder_world::race_params` at scale 2 (512 x 320, 23 crystals of charge 512 that run dry, rock and wet ground, 5,000 walkers on the home crystal in 50,000 slots, upkeep 0.001), run by `examples/bigdish.rs` (`look.sh`). Seed 1, 60 s. "Held" is crystals with 20 or more walkers on them; "bodies" is tube pieces of 4 dish cells or more.

| Tier | Keys | Walkers at 15 / 30 / 60 s | Held at 15 / 30 / 60 s | Bodies at 60 s (largest, dish cells) |
|---|---|---|---|---|
| plain | (race world) | 2,851 / 5,896 / 9,290 | 10 / 16 / 19 | 12 (5,053) |
| Drifter | `sensor_angle` 90, `keep` 0.93, `blocked_deposit` 1 | 1,534 / 3,602 / 4,701 | 4 / 6 / 9 | 13 (982) |
| Sprawler | `explore_steps` 400, `explore_spread` 1, `keep` 0.8, `step` 1.5, `sensor_offset` 18 | 1,595 / 1,258 / 1,144 | 16 / 14 / 14 | 50 (193) |
| Trunk | `sensor_angle` 22.5, `sensor_offset` 18, `keep` 0.95 | 1,706 / 2,113 / 3,342 | 9 / 17 / 17 | 8 (6,648) |

What was tried and left (seed 1, `candidates-1.txt` to `candidates-3.txt`):

- **Drifter.** The first try was the first-reach report's grazing droplets (`graze_weight` 10). With growth on, the grass is food: it filled all 50,000 slots by 15 s and covered a third of the map, a flood rather than a crawl. Grass that feeds little (`graze_eat` 0.0005) looked like plain droplets. A slower step (0.6) held 3 crystals at 60 s, too still to reach a squad. Shared food (`energy_flow` 0.05, the one body thread's rule, CPU only) made big crawling bodies (6,556 walkers at 60 s); a candidate once the GPU runs it.
- **Sprawler.** Explorers alone (`explore_steps` 600, keep 0.9, step 1) spread wide and then starved, to 253 walkers and no crystal held at 60 s: explorers walk away from trail, so they leave food. A longer step and reach keep the mesh on 14 crystals with about 1,100 walkers. A higher explorer deposit (0.3-0.5) starved faster.
- **Trunk.** Reach 27 with keep 0.97 drew the widest cords but stopped growing at about 1,900 walkers; no crowd limit (`crowd` 0) starved it to 1,000; two trail passes made the thickest cords and grew best (4,842 walkers at 60 s with `crystal_flow` 0.05), but the GPU has no `trail_passes`. The kept Trunk is the simplest that both engines run.

The **Trunk's counter is a cut that starves the far piece**, which needs the one body thread's shared food (`energy_flow`). It said (05:19Z) a cut 4 dish cells wide refills within 2 game seconds on the race world, and that an unfed piece held apart starves in 20-35 s. The Trunk will set `energy_flow` once the GPU runs it.

## Habituation: the scar

**The rule** (`BigParams::scar_keep`, `scar_gain`, `scar_weight`, `BigDish::scar`; off by default; the default hash is unchanged). With the alarm field on, each fine step every agar cell's scar becomes scar x `scar_keep` + `scar_gain` x alarm, after the alarm spreads. The scar never spreads, so it stays where the hurt was. A sensor adds `scar_weight` x scar beside trail, scent and alarm. The tiers use keep 0.9999 and gain 0.01: 16 strikes 2 s apart leave a peak of about 250 that halves in about 40 s. The alarm itself fades in about a second. The picture (`bigdish style=life`) draws the scar in dim blue, so the memory is on screen as a field, not a mark.

**Looks first** (`habit-look.txt`, seed 1): 16 strikes at the crystal nearest home, every 2 s from 10 s to 40 s. With the alarm read at -1 and no scar, the mould is back on the crystal by 55 s. Wary leaves it empty at 55 s and 70 s, under its scar. Grudge piles onto it, and each strike killed more than the last (16, then 50, 95 ... 306).

**Paired test** (`habit.sh`, `habit.py`, seeds 1-8, measures named in the script before the run, Holm over 3; raw `raw/habit-1-8/`):

| Test | Tier | Control | Paired | p |
|---|---|---|---|---|
| H1: walkers within 16 dish cells of the strike at 60 s | Wary 17-78 (median 45) | flee, no scar, 149-1,466 (807) | fewer on 8 of 8 | 0.008, holds |
| H2: the same count | Grudge 947-1,571 (1,208) | Swarmer's alarm, no scar, 272-1,282 (942) | more on 7 of 8 | 0.07 |
| H3: walkers killed by the last 8 strikes | the alarm term / (1 + scar) | flee | more on 7 of 8 | 0.07 |

H3 was habituation in the biologist's sense (Physarum stops answering a repeated harmless stimulus; Boisseau, Vogel and Dussutour 2016): a scarred mould stops fleeing fire. It was not significant and it only helps the player, so `scar_habit` was removed (`dead-ends.md`).

**Fresh seeds 9-16** (named before the run as H2's re-test, after seeing seeds 1-8; the same script, raw `raw/habit-9-16/`): Grudge 919-1,516 walkers at the spot (median 1,116) against the Swarmer's alarm without the scar at 192-1,115 (690), more on 8 of 8, p 0.008. H1 held again (Wary 20-109 against 140-1,213, fewer on 8 of 8). H3 went 5 against 3 (p 0.73), which confirms dropping it.

**Cost.** One multiply-add a fine cell a fine step, and one more read at each sensor, only when on. On the race world (`bigdish`, seed 1, 20 s, one thread, three runs each) the alarm alone costs 3.55-3.68 ms a tick and the alarm with the scar 4.02-4.12 ms, so the scar adds about 0.45 ms a tick (12%) on the CPU. The plain mould pays nothing: the field is not allocated.

**On the GPU** the scar is one f32 a fine cell and three Params floats. The integration thread ruled (06:29Z) that it shares the toolkit's per-cell damage buffer (the crater's countdown in channel 0, the scar in channel 1), on the binding after 46, with its floats in a Params row appended at merge. The Wary and Grudge tiers refuse the GPU (`Tier::settings` names `scar_keep`) until then.

## The census

**Rules.** The race world under #106 (free shots, soldiers' scent, the flee opt-in and off), held-out seeds 201-216, 900 s cap, hands idle, random and the seven plans (`census.sh`, Haiku runners at 62cd014; rows in `raw/census/`, tables from `census.py` in `raw/census/census-62cd014.txt`). **These numbers predate shot craters (#113, on by default once merged), which change how a squad digs in; the tiers' difficulty must be re-run on main after it, before the campaign treats them as final.** Measures named in `census.sh` before the run: C1 every plan beats idle and random, C2 the best plan by tier, C3 each tier against the plain mould, paired by seed, sign tests, Holm over all 117.

Mean score v2 (mould wiped / cap / squad wiped), 16 seeds a cell unless marked:

| Hand | plain | Sprawler (explore 400) | Trunk | Wary (8 seeds) | Grudge |
|---|---|---|---|---|---|
| idle | -0.32 (0/16/0) | -0.33 (0/13/3) | -0.27 (0/16/0) | -0.34 (0/8/0) | -0.33 (0/16/0) |
| random | -0.33 (5/2/9) | +1.68 (14/2/0) | +0.89 (9/5/2) | +1.66 (6/2/0) | **-1.61 (0/1/15)** |
| farm | +0.23 | +1.41 (14 wiped) | +0.35 | +0.34 | +0.11 |
| wall | +0.23 | +1.46 (15) | +0.36 | +0.43 | +0.09 |
| boom | +0.22 | +1.70 (12) | +0.49 | +0.20 | +0.05 |
| claim | +0.38 | +2.10 (14) | +0.55 | +0.56 | +0.08 |
| fort | +0.22 | +1.76 (13) | +0.37 | +0.22 | +0.04 |
| strike | **+1.24** (9 wiped) | **+2.14** (14) | **+1.65** (12) | +0.46 | **+0.28** |

- **C1.** Every plan beats idle on 16 of 16 on every tier. Against random it is mixed on the plain mould (11-12 of 16, n.s.), the Trunk (7-8 of 16 bar strike, 14 of 16) and the Sprawler (random beat farm 13 of 16, wiping the mould 14 times). On Grudge every plan beats random 16 of 16 (holds): random's scattered fire is what Grudge punishes, and it lost its squad on 15.
- **C2.** Strike is best by mean on every tier but Wary (claim, on 8 seeds), best on 10-15 of 16 seeds. The best plan does not change between tiers: the designer's bar fails here, as it does for the toolkit's loud plan.
- **C3.** Grudge is harder than plain for every plan: lower on 16 of 16 for farm, wall, boom, claim and fort, 14 of 16 for strike (all hold). The Trunk is easier under random (15 of 16), farm (14) and wall (13). The Sprawler at 400 was easier under every hand (15-16 of 16). An idle squad is never wiped by 900 s on any tier but the Sprawler (3 of 16), so **the Drifter's "reach an idle squad by 11:00" waits on its census rows**.
- **Sprawler, fixed.** Newborns start as explorers (age 0) and walk off food, so at 400 the mesh starved. At 200 (cf62e8e), on dev seeds 1-4 random lost its squad on 2 and capped behind on 1, wall and strike won 4 of 4 and wiped it once each; on held-out 201-204 random lost its squad on 2 and every plan capped above idle. Its held-out re-run, the Drifter and the rest of Wary come in the follow-up.

**The toolkit's blast plan** (`kitplans`, seeds 1-8, cap 600 s, tools carried without the toolkit's later carry limit; raw `raw/kit/kitplans-tiers-1-8.txt`): loud wipes the mould on 8 of 8 on all six tiers (+1.72 to +2.01), best against farm 8 of 8 each time. Grudge walks back into the blast spots and dies there; random's squad is wiped 8 of 8 against it. The toolkit is re-running all five new tiers with its carry rule.

## Commands

```
bash Reports/mould-tiers-campaign-2026-11-12/look.sh WORK                          # the three tiers and plain, seed 1
ARMS=Reports/mould-tiers-campaign-2026-11-12/candidates-1.txt bash .../look.sh WORK # the candidates
bash Reports/mould-tiers-campaign-2026-11-12/habit.sh OUT && python3 .../habit.py OUT
bash Reports/mould-tiers-campaign-2026-11-12/census.sh TIER OUT 201-204 4         # then census.py OUT
cargo run --release --example bigdish -- tier=sprawler ...                          # any tier by name
```

Page: *pending* (follow-up).
