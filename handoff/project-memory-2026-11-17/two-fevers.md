---
name: two-fevers
description: Two Fevers playable demo (2026-11-10): module, browser page, census, owner's verdict that the first build has no strategy
metadata:
  type: project
  modified: 2026-11-10T21:12:35.078Z
---

Two Fevers (contagion war over one neutral mould) was built 2026-11-10 in thread "Build a playable Two Fevers demo", branch `claude/project-thread-9zu236`.

- Code: `src/mould/fever.rs` (own fixed-population Jones host, 23 field piles that regrow, two strain fields, per-strain immunity, hands Idle/Random/Plan), probe `examples/fevers.rs` (look / front / census), browser front `fevers-web/` (wasm cdylib, `fv_*` exports, `page/index.html`; build with `fevers-web/build.sh`). Kept apart from the RTS thread's `web/` and `skirmish.rs`.
- Playable page: https://claude.ai/artifact/UMzPJmevoYJYmx8KRqG37a (published with files {"fevers.wasm": ...}).
- Defaults: catch 0.03, sick 4 s, immune 40 s, spore every 10 s (max 3), 5,000 walkers, 3:00 match. Front ~7 cells/s.
- Census (16 seeds, both colours, 3:00): plan beats random 16/16 seeds but only by a part of 0.536 median; plan beats idle 0.80.

Owner, 2026-11-10 21:11Z: "Not sure I follow what the strategy or challenge is in this game. I click and it send a wave through the network, a few seconds later the immunity moves through the network and eventually everything ends back at the initial state."

**Why:** nothing a click does persists; the host is an excitable medium that resets.
Owner's card 21:15Z: "Both together" -> version 2 built and republished at the same link: owned ground (walker keeps colour of last fever), 15 s hold, Hush (carriers lay no trail). Census v2: plan beats random 16/16 by 0.537; no hold -> random wins 14/16; Hush-when-behind 10/16 (noise). PR #28.

**How to apply:** v2 awaits the owner's hand. Next levers named in the report: smaller fronts, aimed Hush, other traits, native build. Related: [[rts-direction]].

Owner, 2026-11-10 21:52Z, after v2: "still feel overly simple with 1-2 tools that lead to mostly 1 strategy ... (although, I didn't fully understand how hush worked) ... The slime mold seems more like background in this game and not the integral part". Response: v3 design `Reports/two-fevers-2026-11-10/v3-design.md` (branch claude/project-thread-9zu236): owned walkers follow their owner's Call; fever travels only through tube (trunk = highway); income from owned walkers on piles; Pinch (local, paid) replaces Hush; cut-off pieces starve. Core tension: efficient network = exposed network. Hoped cycle Trunk > Web > Raider > Trunk. Three cheap probes first (fever by tube, call reshapes, pinch cuts).

Owner's card 22:06Z "Build it after the checks" -> v3 built, PR #32, same page. Rules::v3(): calls, fever by tube, forage 0.005 (cap 12/pile), starve 12 s, free pinch every 5 s, resist 0.1 (fed owned walkers). Census: ladder trunk > web > raider (only trunk>raider survives Holm); raider = random (7/16); mirror 1:30 leader kept lead 32/32 (snowball). Next levers: income that falls with body size; a raid on starving pieces. v3 awaits the owner's hand.

RTS-research thread's review of v3 (23:21Z, Reports/rts-research-2026-11-10.md Part 5), proposals to test: sick walkers lay no trail (a fever riding a trunk thins it and starves what's behind); income falling with body size; score from contested ground or weighting the last minute; rival calls visible only where your walkers sense them; search a parameterised exploiter against Trunk. Not owner rulings; wait for owner's play first.
