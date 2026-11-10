---
name: mould-probe-2026-11-07
description: Mould probe (new-game probe 2): --bin mould, slime mould herded by lamp; PR #1028 merged, playtest fixes PR #1042 merged, before/after page, open question for Scott
metadata:
  type: project
  modified: 2026-11-08T05:05:52.036Z
---
Probe 2 of [[new-game-brainstorm-2026-11-07]], thread cmsg_016bYH3Ybq4z6xkGqHujo4HWKXbN7RGrsGS3sjZN1gbMSb. Built 11-07: `cargo run --release --bin mould`, code `src/mould/` (own trail + scent fields, no shared engine edits), harness `examples/mouldprobe` (lean=1, verbs=DIR, bend=1 offset=N, gif=DIR, look=DIR, any Params field by name). PR #1028 merged 11-07 (2902c4f9). First page: https://claude.ai/artifact/R4Yjkt4bsozfZDV1CkpcDc. Report: Reports/mould-probe-2026-11-07.md.

PLAYTEST FIXES 11-08 (from [[demo-playtest-2026-11-08]], Scott 03:29Z "give all your findings to the threads that developed the demo"): PR #1042 MERGED to main 11-08 05:30Z (8770e012). Before/after page: https://claude.ai/artifact/52KtqyTGjyhKMb7L14Eqq6.
- Lamp pushes (Params::push 0.9, push_light) instead of sensor repulsion + fade + burn: join lost while held on a tube 5/5 -> 0/5. Sensor lamp at every setting cut the join (dead-ends other:174).
- Oat back every 15 s; far pile arrives 0:30, 6th pile 0:55.
- Storm: purple warning line 5 s ahead, aimed at the cut costing most piles, toughness read after 8 s.
- Thrift bar 1.15; score moved under the dish (HUD 5 lines).
- Same 7 hands, stars before -> after: idle 2->0, random 6->3, plan 6->8, fresh 6->9, shepherd 3->3, lamp-only 1->3, dump 1->2. Lamp alone still never joins all food.
- Frame worst 13.4 -> 16.7 ms (storm-aim frame).

**Why:** Scott judges by playing; "does herding feel alive" is his call. **How to apply:** wait for his verdict before more work (listed next steps: pulse, spores, dye, lightning, rival); compare with Seep's graph mould before keeping either.
