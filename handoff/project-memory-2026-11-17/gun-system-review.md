---
name: gun-system-review
description: 11-16/17 gun/card review: A-C, D families done (Mind, readers, route, Torch, Tinder; force = shove cards; cut count -> no Cut card, PR #435); next Provoke/Still cards, then E device slot; branch vx9t3p
metadata:
  type: project
  modified: 2026-11-17T15:45:00.000Z
---

Thread "Rethink the gun card system" (cmsg_01PcwPdssrGxiXYvyCG8WaUXCpsFRiVSrGm8cVBDxSCd73), owner 2026-11-16 02:34Z, branch claude/project-thread-vx9t3p. Plan: Reports/gun-system-review-2026-11-16/. Steps: A shapes; B trigger/expire/multicast + read-back batching; C seek + aim; D families (Mind, readers, route done; next element, force; Cut last); E device slot per squad on a cooldown; F luck that changes rules.

Owner 03:37Z 11-16 (wiki/decisions-game.md): aim cards only if beneficial; combos in-gun and across squads; devices on cooldown; rule-changing relics yes. Owner 11-17 03:07Z: "Continue without my guesses. I am busy".

Merged: A #376, B #378/#383/#387, C #399 (seek), D Mind #403 (quiz PLHGQc5kKxXi5qwNmqQqyb, key in Reports/gun-mind-2026-11-16/README.md), readers #408 (Shatter, Detonate), card table #410 (wiki/world/gun-cards.md), route #412 (Flare; Road dropped; germ first retune). Judge card effects by PAIRED 10 s windows, hit % beside kills.

Germ (11-17): GPU middle-seeding bug fixed (#416; pre-fix germ numbers read high); per-tier germ_resist/germ_upkeep; germ clips re-filmed #417; Detonate kept ('let it spread first', #418). Details in Reports/gun-route-2026-11-17/ and gun-readers-2026-11-17/.

Element (11-17): Torch built and merged (#423/#424; Card::Torch slot 7, Burst::light -> BigDish::ignite, torch_r 6, cost 2; more kills than plain 9/9; report gun-element-2026-11-17). fire_weight being deleted (word 860 free again). Torch cost unchanged until the balance pass. Sim-use thread built MIND_DEAF 4 (Provoke) and MIND_STILL 5 (Still) for our cards (Reports/sim-use-2026-11-17/b-provoke-still.md).

Tinder (11-17): Card::Tinder flight trail, TRAILS 5; BigDish::lay_fuel / GPU IGNITE lay mode (word 49 byte 1); plants over cap rot by plant_rot (GPU plant header word 62). Brush kept trail_fuel_gap 6 dish cells clear of LIVING SOLDIERS (rounds::keep_clear), not along the leg: bullets fly in per-frame legs shorter than 6, so an along-leg gap laid nothing; from the muzzle it lost 23/80 soldiers. Windows: torch+tinder > torch 9/9 (p 0.004), barrens 159 vs 123; adds nothing to scorch. Crossing test (planet 1:3, line 162.6,190.8-191.4,163.2, headless trail=16,LINE,tinder,lit + cross=LINE,30): none 214, w0 10, w-5 57 (waited, crossed char), fire_alarm 50 (sim-use C4) 31 -> fire_weight DELETED (follow-up PR after #428; fire_alarm is the one fire-shyness switch, agreed with sim-use + tech lead 12:05Z). Report Reports/gun-tinder-2026-11-17/, page DMHMAsve7qyjLV8JXsK54D.

Force + cut (11-17, designer 12:23Z): force counts as built (shove cards, no new card); Heave/Corral merge is an owner pick in the integration thread's playtest pack (#433). Cut count (`mould::necks`, key `necks=T,...` writes a cut map): run planets 1-8 idle, largest part one cut frees <2% of slime in 21/22 counts -> NO Cut card (PR #435, report Reports/gun-cut-count-2026-11-17/, page 7h8b6QmZCrwtEJvaNJUzfu, dead-ends entry with re-test condition). Lesson: count shapes on a pinhole-filled mask (speckle read as necks: Sleeper 56% -> 1%). Headless censuses: big planets (5-8) are slow on llvmpipe (p8 to 180 s ~23 min alone); background Bash runs default to a 30 min limit, pass a longer timeout.

STOPPED 11-17 15:43Z (owner, usage): Provoke/Still cards built + unit-tested on branch claude/project-thread-vx9t3p-wip (9294ed83, WIP, on top of the cut-count commit; not a PR; clippy not run; windows partial, film not made; README draft has PAGE/STATUS/PLAY placeholders). Resume: after #435 merges, rebase that commit onto main, finish windows (raw/windows.sh) + film (raw/film.sh), PR.

Card tuning notes go STRAIGHT to the game designer's thread cmsg_01PcwPdssrGxiXYvyCG8WaUX2dsdkBBhuYywD19AnT6kXG (look thread asked 11-17 not to be in the loop).

Film tricks: `boom=T,X,Y[,CARDS]` casts; `watch=X,Y,R` counts walkers round a point; tier=X sets a planet's slime.

Rules learned: a PR touching a report must touch every wiki page citing it (lint; it reads COMMITTED changes against origin/main, so fetch main and lint after committing). Every H preset fits default budget 10. Armory tab must fit one page at 200% UI on 1080 lines. GPU dish Settings defaults (seed_at etc.) can leak into the Larder world: check them when a GPU number disagrees with the CPU.

Hooks: aim `Game::boss_aim(k)`; devices (step E) fill `Model::throw` (PR #379). Related: [[gun-rounds]], [[gun-look]], [[mini-boss]].
