# Your own gas and blasts hurt you (2026-11-18)

Squad and arsenal work, asked by Direction after the owner's playtest of
2026-11-18 and the Noita-rule audit in
[sleep-wears-off](../sleep-wears-off-2026-11-18/README.md). The owner,
22:33Z: "Any single spell that is incredibly powerful is also really
dangerous." Page: https://claude.ai/artifact/Dxcv2mU4NjQNMMKbtdqZgb
(source and clips in `page/`).

## The rules (switch `own_goal`, off; see "Why off")

- **Gas's cloud puts your soldiers to sleep** (`Rules::doze`, 1.0). A
  soldier standing where the sleep cloud is at the level that puts the slime
  to sleep (`KitRules::gas_sleep`) neither fires nor walks until it thins
  (`Game::dozing`, `Soldier::doze`). The cloud at each soldier is read in
  the calm pass the game already runs while gas lasts (engine.wgsl's `calm`
  writes it into each point's spare fourth word; `Engine::calm` reads it
  back in the same read-back, `World::clouds`). GPU only; the CPU world reads
  no cloud, so its soldiers never doze. Default 0; the switch sets 1.
- **A blast hurts the soldiers in its kill disc** (`RoundRules::blast_hurt`,
  40 of a soldier's 100 health; `Rounds::fire`). Shots stop at the first
  slime, so this happens when a blast goes off in slime on top of the squad,
  or a turret's blast lands among soldiers. Counted in `Rounds::blast_hits`
  and `blast_kills`. A soldier hurt by a blast is not hurt by another for
  2 s (`blast_gap`). Default 0; the switch sets 40.

## Film

Planet 1, run seed 1, `hand=greedy` (15 soldiers draining a crystal at
226.6, 52.1), one round cast on the squad's spot at 40 s
(`boom=40,226.6,52.1,gas` or `blast`), frames every 0.5 s from 38 to 56 s
at zoom 6 and 12 (`raw/f.sh`, `raw/jobs`; counts in `raw/*-counts.txt`).

| | shots 40-46 s | shots 46-52 s | mean health at 41 s | at 56 s |
|---|---|---|---|---|
| nothing cast | 86 | 93 | 99 | 98 |
| gas on the squad | 18 | 89 | 99 | 96 |
| blast on the squad | 98 | 95 | 81 | 84 |

The gassed squad fired a fifth of its shots for six seconds, then woke as the
cloud thinned. The blast took 40 off the soldiers in its disc (mean 99 to 81
across 15) and killed none.

Tests: `a_soldier_sleeps_in_its_own_gas_cloud` (GPU: a 19.9-level cloud on a
soldier; it sleeps, moves 0.00 fine cells while ordered to walk, and wakes at
14.7 s; with `doze` 0 it walks) and `a_blast_hurts_the_soldiers_in_its_disc`
(exactly the soldiers in the disc lose 40; off hurts none).

## Why off: the squad's own fire

Soldiers fire on their own at the walkers nearest them, so once the slime
reaches the squad they put their gas and blasts at their own feet. Planet 3,
`hand=greedy`, the squad's deck loaded at 0 s, raw logs not kept (the
container's `/tmp`); deciding measure soldiers standing:

| gun | danger | run 1795038793 | run 1 |
|---|---|---|---|
| plain + gas | off | 10 soldiers, 3,000 kills at 60 s; wiped at 114 s | 21 soldiers, health 75, 8,027 kills at 120 s |
| plain + gas | doze | 482 kills at 60 s; wiped | 1 soldier, 290 kills at 120 s |
| plain + gas | doze, gas held within 14 cells of a soldier | wiped at 107 s | 1 soldier, 290 kills |
| gas or blast | blast_hurt 5, 10, 20, 40 (no gap) | wiped | |
| plain + blast | 40 with the blast held within 7 cells | wiped | |
| plain + blast | 15, 2 s gap | wiped | wiped; planet 1 seed 1: 29 soldiers, health 57 |
| plain + blast | 5, 2 s gap | wiped by 120 s (9 soldiers at hurt 0) | 9 soldiers against 22 |

Holding a danger shot whose aim is near a soldier (tried in `turn.rs`,
dropped) all but stopped the danger cards (14 gas casts against 312) and the
squad still fell. On by default this makes Gas and Blast worse than a plain
gun for a squad that does not aim, which is a trap, not the owner's "really
dangerous". One switch (`own_goal`, Experiments tab) keeps it for him to try;
the choice of on, a gentler rule (a danger only for your own volley aim), or
deleting it is Direction's and the owner's.

## Not done

A dozing soldier has no look of its own beyond standing still in the violet
haze; the Look lane may want one drawn from `Soldier::doze`.
