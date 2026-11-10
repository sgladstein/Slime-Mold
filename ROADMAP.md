# Roadmap: every active plan in one order

Written 2026-11-10 18:15Z against `main` at `865c802`. It puts the four
plans that are running at once into one sequence, shows what each step
waits on, and names which project thread owns it. It adds no new plan: the
detail, numbers and gates stay in each plan's own source, linked below.
[HANDOFF.md](HANDOFF.md) is still the front page, and the owner's open
decisions are listed only there (wiki rule 5); this page points to them by
number.

The goal behind every line: interesting simulations that can be turned into
a game, not a realistic mould (owner, 2026-11-10 17:01Z). Fun over fidelity,
everything on screen is the simulation, and an outcome is a spread
([`wiki/decisions.md`](wiki/decisions.md)).

## The four plans

| Track | Plan | Owning thread | Source |
| --- | --- | --- | --- |
| **G** Game fixes | Keep It Alive's luck, The Wild One's middle, the snip, star bars, the forest floor | *Triage the previous session's hand-off* | Hand-off triage doc (https://claude.ai/artifact/XWm6u7y1UR3cfp2XxkhC7w); [HANDOFF.md, *Next steps*](HANDOFF.md) |
| **E** Engine checks and ports from Pixel_Physics | Dish hash, profiler, parallel draw, licence gates, keyed random draws, sound | *Review Pixel_Physics for reusable code* | Pixel_Physics review doc (https://claude.ai/artifact/Pe83NVdRMXhPvxeYoKASRu) |
| **S** Scale (GPU) | Six gated phases, from CPU foundations to new play at scale | *Research Sage Jenson's physarum work* | [`Reports/gpu-plan-2026-11-10.md`](Reports/gpu-plan-2026-11-10.md), [the mould at scale](wiki/model/scale.md) |
| **R** RTS (Bloomfront) | Five gated stages, from two keepers on today's dish to online play | *Explore turning the simulation into an RTS* | [`Reports/rts-design-2026-11-10.md`](Reports/rts-design-2026-11-10.md) |

Thread names are the project's thread titles on claude.ai. When a thread is
closed, its track is unowned until a new thread takes it; say so in its row.

## The order

Four bands. Work in a band can run in parallel; a band starts when what it
depends on is done, not when the band before it is finished.

### Now: no owner input needed

| # | Step | Owner | Waits on | State |
| --- | --- | --- | --- | --- |
| E1 | Dish hash and determinism test | Pixel_Physics thread | nothing | done: PR #14 (`Dish::state_hash`, `every_dish_replays_bit_for_bit`, `mouldprobe` prints `hash=`) |
| S1 | GPU phase 1: frame profiler (also the review's E2), "GPU semantics" CPU arm, delayed-census arm, new rules off by default | GPU thread | E1 for the hash; owner go-ahead given 17:50Z | profiler done (PR #16: diffusion is 62% of the simulation, moving walkers 24%); the two arms next, on E1's hash |
| E4 | Release profile re-measured on `mouldprobe` and one CI run | Pixel_Physics thread | E1 | done: `codegen-units = 1` kept (about 0.45 ms a frame), thin LTO dropped (no gain); hashes unchanged |
| E5 | Licence and notices gates, the JetBrains Mono OFL notice, `crispfont.py` | Pixel_Physics thread | nothing; must land before any build leaves the owner's machine | done: `scripts/licensecheck.sh`, `scripts/notices.sh` + `THIRD-PARTY-NOTICES.txt`, `notices.yml`, `scripts/crispfont.py` |
| R- | Bloomfront research (four inspiration agents, then a red team) | RTS thread | nothing; design only | running |

### Next: waits on the owner

| # | Step | Owner | Waits on |
| --- | --- | --- | --- |
| P | **Play Keep It Alive and The Wild One by hand** | Scotttt | nothing. This is the one step that unblocks most of the rest |
| G1 | Keep It Alive: trace single games to find what sends one into the short or long hump. Measures only; the game does not change | Triage thread | **done 11-10**; owner picked "Trace it now" (card, 18:56Z): the first pile on mostly-asleep mould decides it ([report](Reports/mould-keep-alive-luck-2026-11-10.md)). Next: a 20 s landing preview, a game change |
| G2 + R0 | **The Wild One's middle (seconds 30 to 150), done as Bloomfront Stage 0.** First separate which of lamp, snip and oats needs the aim; then give the wild mould a scripted hand on a mirrored dish | one thread, to be chosen | P for G2; the owner's say for R0 (HANDOFF decision 7, "wait or replace") |
| G3 | The snip: make a cut-off piece visibly starve, or take it off The Wild One | Triage thread | P |
| E6 | Keyed random draws and the dead-dish slowdown | Pixel_Physics thread | timing only: see *One census reset*, below |
| E7 | Sound for the game, from Pixel_Physics's `src/audio/` | Pixel_Physics thread | the owner wants it at all (HANDOFF decision 8) |

### Then: the scale fork

| # | Step | Owner | Waits on |
| --- | --- | --- | --- |
| S2 | GPU phase 2: offline 1024 x 640 with about 100k walkers, economy on, paired seeds; GPU timing test | GPU thread | S1's gate (flags off leaves the dish hash unchanged; dishes hold under a late census); **the owner at Scott-Desktop with the Claude app open**, for the timing run on the GTX 1070 through Remote Control |
| **Gate** | **The owner picks a target scale and look, or stops** | Scotttt | S2's page |

The two branches after the gate:

| | If the GPU goes ahead | If it stops |
| --- | --- | --- |
| Drawing | E3 (multi-core renderer) is dropped; the GPU draws | E3: port the Pixel_Physics multi-core row renderer |
| Engine | S3 sandbox (read Bleuje, Lague, PolyPhy first), S4 game rules on the GPU (needs Jones 2011 uploaded), S5 port the dishes that survive | stay on the CPU |
| Forest floor (G5) | built after S3, as a large world | built small on the CPU |
| Bloomfront Stage 3 (three or four keepers, big map) | on the GPU; the rival becomes a second whole dish in S4 | the plate-and-colonies split on the CPU, frame under 10 ms on the owner's machine |

### Last

| # | Step | Owner | Waits on |
| --- | --- | --- | --- |
| G4 | Star bars re-checked on 16 or more seeds | Triage thread | G1-G3; and S5 if the GPU goes ahead, because S5 re-derives the bars anyway |
| R1-R2 | Bloomfront Stage 1 (spend to act; needs a currency first, since the food store sits at a median of 0, [brainstorm](Reports/rts-design-2026-11-10/brainstorm/README.md)) and Stage 2 (grow and sever) | RTS thread | R0's gate (equal hands 45-55%, sharp beats clumsy on 16 paired seeds); P (Stage 1's gate needs the owner's play); G3 (Stage 2's sever is the snip that bites) |
| R3-R4 | Bloomfront Stages 3 and 4 | RTS thread | the scale fork; Stage 4's lockstep needs the integer simulation (HANDOFF decision 6) |
| S6 | New play that only scale allows | GPU thread | S5 |
| G6 | Later depth, in the owner's order: melting salt, spores, a rival the player steers, same-kind moulds joining | unowned | G1-G4 playing well |

## Dependencies

```mermaid
flowchart LR
  P([Owner plays both dishes]):::owner
  E1[E1 dish hash] --> S1[S1 GPU phase 1]
  E1 --> E4[E4 release profile]
  S1 --> S2[S2 GPU phase 2]
  D([Owner at Scott-Desktop]):::owner --> S2
  S2 --> F{{Scale gate: owner}}:::owner
  F -- go --> S3[S3 sandbox] --> S4[S4 rules on GPU] --> S5[S5 port dishes] --> S6[S6 new play]
  J([Jones 2011 upload]):::owner --> S4
  F -- stop --> E3[E3 multi-core draw]
  S3 --> G5[G5 forest floor]
  F -- stop --> G5
  K([Card: trace now?]):::owner --> G1[G1 Keep It Alive luck]
  P --> G2[G2 + R0 Wild One middle / Stage 0]
  W([Bloomfront: wait or replace?]):::owner --> G2
  P --> G3[G3 snip that bites]
  G1 --> G4[G4 star bars]
  G2 --> G4
  G3 --> G4
  S5 -.-> G4
  G2 --> R1[R1 spend to act]
  P --> R1
  R1 --> R2[R2 grow and sever]
  G3 --> R2
  R2 --> R3[R3 three or four keepers]
  F --> R3
  S4 -.-> R3
  R3 --> R4[R4 species, online]
  I([Integer or float]):::owner --> R4
  S1 --> I
  classDef owner fill:#fde68a,stroke:#b45309,color:#000
```

Yellow is the owner. A dotted arrow is "if the GPU goes ahead".

## Where the plans touch each other

These are the places two threads could step on each other or redo work.
Each has a rule.

- **One census reset, not three.** Three changes would each make every old
  census number stop reproducing: the keyed random draws (E6), the
  all-integer simulation (S1's decision), and the GPU path itself (S5). They
  are the same idea: a random draw keyed on seed, walker and step is exactly
  what the GPU plan's integer design uses. So build E6 as part of S1, in the
  form the GPU plan needs, and land it in one reset together with the
  integer switch if the owner agrees to it. Land it between reports, never
  while G1 or G2 are mid-measurement, and re-run every dish's baseline once
  in the same PR.
- **Parallel draw is on hold.** The Pixel_Physics review puts the
  multi-core renderer third; the GPU plan holds it until the scale gate,
  because GPU drawing would replace it. Hold it (HANDOFF decision 6); it
  becomes the fallback if the owner stops at the gate.
- **The profiler is built once.** E2 and S1's profiler are the same tool;
  the GPU thread builds it.
- **The Wild One has one owner at a time.** G2, G3 and Bloomfront Stage 0
  all change The Wild One. The RTS report's own advice is that Stage 0
  answers G2's question, so do them as one piece of work in one thread.
  Which thread is the owner's call with decision 7.
- **`src/mould/mod.rs` is busy.** E1, S1 and E6 all edit it. Land E1 first;
  S1 builds on it.
- **The snip matters twice.** G3 is a game fix and Bloomfront's Stage 2
  depends on it (supply raids). Its report should say whether a sever now
  starves the far side, measured both ways.
- **Determinism is shared.** E1's dish hash is S1's first gate and
  Bloomfront Stage 4's lockstep check. All three use the same hash.
- **The forest floor needs two engine fixes** whichever branch is taken:
  stepping only near the mould (one open 256 x 160 floor costs about 1 ms a
  step), and `spread` still skipping the border row
  ([`Reports/mould-real-world-2026-11-09.md`](Reports/mould-real-world-2026-11-09.md), §5;
  hand-off triage). No thread owns it yet.

## What waits on the owner

In the order that unblocks the most. The numbers are HANDOFF's open
decisions.

1. **Play Keep It Alive and The Wild One.** Blocks G2, G3, G4, Bloomfront
   Stage 1's gate, and most of decision 1.
2. ~~Answer the trace card~~ (decision 1): answered 18:56Z, "Trace it now";
   done. What follows from it, a 20 s landing preview, changes the game and
   waits on the owner's go.
3. **Bloomfront's three questions** (decision 7): wait for the queued work or
   take over The Wild One's next steps; fog of war drawn from a held
   quantity; two keepers or three-to-four. Recommended: do G2 as Stage 0.
4. **Hold the multi-core renderer** until the scale gate (part of decision
   6). Recommended: yes. Needed now, because it is third in the Pixel_Physics
   thread's order.
5. **Sound for the game** (decision 8): wanted or not.
6. **Be at Scott-Desktop with the Claude app open** when S2's timing test is
   ready; the GPU thread will ask.
7. **Integer or float** (decision 6), after S1 has measured it. Recommended:
   integer.
8. **Upload Jones 2011** before S4.
9. **The scale gate** after S2.
10. Low urgency, best answered after playing: retire F7 and grains or
    glints (decisions 2 and 3), food flow (4), the Organ and the Wanderer (5).

## Keeping this page true

- A thread that passes a gate, opens or merges a PR for a step, or changes
  a step's owner updates its row here in the same PR.
- An owner answer moves into [`wiki/decisions.md`](wiki/decisions.md) as
  usual; then update the row it unblocks here.
- The GPU go-ahead (17:50Z) and the owner's GPU (GTX 1070 on Scott-Desktop,
  17:54Z) are being moved from HANDOFF decision 6 into `decisions.md` by the
  GPU thread's first phase 1 PR.
- Each thread merges its own PRs once both CI jobs are green on the exact
  head, with a merge commit (owner, 17:47Z). Any new report must be cited
  from a wiki page in the same PR; `python3 wiki/lint.py` fails otherwise.
