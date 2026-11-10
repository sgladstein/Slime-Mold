---
name: mould-game-brainstorm-2026-11-08
description: Mould demo 11-08: Scott's play -> brainstorm -> overnight build (dishes, Wanderer, Organ, night look, 2 reviews) merged as PR #1054; verdicts pending
metadata:
  type: project
  modified: 2026-11-08T18:18:33.191Z
---
Follows [[mould-probe-2026-11-07]]; thread cmsg_016bYH3Ybq4z6xkGqHujo4HWKXbN7RGrsGS3sjZN1gbMSb.

SCOTT'S PLAY 11-08 05:37Z: "most promise so far"; needs instruction, clear options, a challenge. Loves toy feel; NOT bringing the Pixel Physics world/plants/ants in (06:44Z). Wants "little bridges where there wasn't even food".

BRAINSTORM folder /mnt/project-files/game-design/mould-game-brainstorm-2026-11-08/ (context.md + theme, out-there, near-term-challenges, toy-feel, research, bug-check). Synthesis page https://claude.ai/artifact/16qnDdYhzg3Q5kL3LnApYE. Lesson (PR #1052 merged) page https://claude.ai/artifact/PoyyzzGuRyqaEn4G8RidPK.

SCOTT'S OVERNIGHT RULINGS: picked Wanderer + Mould Organ prototypes (07:23Z); each piece eats its own food, on by default (07:43Z); wants to wake up with the look implemented: if thread + interface agent agree, build it (redo if he disagrees), else a third agent picks (07:48Z); bug checks/adversarial reviews over ALL demo work (07:49Z), batched into two combined reviews (08:13Z).

WHAT SHIPPED (PR #1054 MERGED 11-08 18:17Z as 14621b6b): pulse, open wound 4 s, join flash, per-piece food (`Params::pieces`); 3 challenge dishes (Behind the Wall, Beat the Ruler, Running Dry; src/mould/level.rs) + picker M; stars a LADDER (thrift/tough only when all food joined); Wanderer (W, wander.rs; lead 60% arrive vs careless 29%; page https://claude.ai/artifact/RrEJruyrAveXMGo6c7q5f4); Organ (`--bin organ`; wobble 0; lamp bends note +79..245 ms, springs back; page https://claude.ai/artifact/Np5fa71YVHe8gnZq7oVZNW); look Style::Night = blind option B glow + C's clickable tool tiles + storm timeline (key in prototypes/interface/key.md), F2 cycles, ~+1.3 ms/frame vs Classic. Review 1: 13 fixed; review 2: 12 fixed (review-1.md, review-2.md); `Params::oats_grace` 8 s lets oats rescue just-starved mould; "oats exempt from leash" REJECTED (broke star bar). Star bar 8 seeds: idle 0, random <=1 except Running Dry seed 7 (3 stars, open), intended 3.

Morning page https://claude.ai/artifact/TnWALC9vhyrkFQ2o52iWgY (source scratchpad morning/mould-night.html).

PENDING SCOTT: verdicts on the look, Organ, Wanderer. Wave 2 design agent not yet run.

**Why:** Scott wanted info first, then a plan, then work overnight. **How to apply:** brief any next round from the 5 brainstorm files + his verdicts; don't re-run wave 1.
Facts: sessions pause while idle (an agent stalled until Scott wrote); probe `mouldprobe level=all` is the ship-bar check.
GREY 11-08 18:30Z Scott: "the gray doesn't look great". Built look::STARVE_LOOK (F3, mouldprobe grey=N; render-only, graded by fed_at over 6 s, +0.3 ms): 0 grey (default), 1 embers, 2 dried slime. Commit aadbf8af on claude/project-thread-cek3ly (restarted from main), no PR yet. Blind page https://claude.ai/artifact/GaadFXKa1YEzEE6EdnrfCP, KEY: A embers, B grey, C dried. On his pick: make it default, PR to desk.
