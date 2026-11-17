---
name: rts-skirmish
description: Field skirmish demo: v2 budget skirmish playable (opens on Strike and Fortify); brace (strike + fortify what is struck) now wins every pairing; food after 1:00 failed; next is Fortify's price and the exploiter search
metadata:
  type: project
  modified: 2026-11-11T04:02:50.555Z
---

Code `src/mould/skirmish.rs`, browser `web/`, report `Reports/rts-skirmish-2026-11-10.md`, wiki `wiki/rts/skirmish.md`, harness `duel SEED LEFT RIGHT` (TRACE, LOOK, RULES="name=value", set via `Skirmish::set`, which also sets model Params on both moulds) in `Reports/rts-design-2026-11-10/red-team/harness`. Analysis in the report folder: `analyse.py`, `screen.py`, `compare.py`, `held.py` (held score), `wiped.py` (wipe-out to survivor), `strikes.py`. Run job files with `xargs -P 4 -d '\n' -n 1 bash -c < jobs` (plain `xargs -I{} sh -c` strips the RULES quotes).

v1 set aside by owner 21:14Z. v2 page https://claude.ai/artifact/936ACBn8mJUkvFpb4iSnUS (version 5, 2026-11-11: default rules = Strike and Fortify; menu also has long match 4:00 + regrowth 1%, and the whole economy sketch, which was never measured together).

Findings to 2026-11-11 (PR #41, PR #50): soft/keep topped every rule set through round 7b's food-after-1:00 tests (4:00 match, regrowth, held-ground score: the 1:00 leader still wins 80-85%; regrowth 1% cuts wipe-outs 217 -> 132). Under Strike (5 s countdown) and Fortify (ring: own losses x0.25, attackers x4; defenders-only version failed), brace beat soft 15/16 and won all 7 pairings of an 8-plan round robin; greed does not beat brace.

Two bugs were fixed: spores were overcharged under spore_ramp (rounds 3-7 affected; soft v keep moved from 5/16 to 10/16); regrowth was scored to the rival in `Dish::step`.

**Why:** the owner's bar is several strategies that counter each other; one plan still wins.

**How to apply:** next is Fortify's price (150 v a strike's 1,000+; try 300-600, fortify_harm 2) and the sketch's exploiter search against brace; give greed a payoff (income a strike forfeits). Related: [[rts-direction]], [[web-build]], [[runner-threads]].
