# Red-team review: the RTS directions (2026-11-10)

**Status: review and probes, no change to the game.** A fresh agent, asked on
2026-11-10 after the brainstorm merged (owner: "red team after the merge makes
sense"), attacked the directions in
[`../brainstorm/README.md`](../brainstorm/README.md) and the first draft
[`../../rts-design-2026-11-10.md`](../../rts-design-2026-11-10.md). It ran its
probes from a scratch harness; the RTS thread then copied the harness, its raw
output and its stills into this folder unchanged, apart from the paths below.
The harness still builds and reproduces the runs on the main branch of
2026-11-10 (checked on `taut 1 22.5 1`).

Reviewed: `Reports/rts-design-2026-11-10.md` (Bloomfront), the synthesis
`Reports/rts-design-2026-11-10/brainstorm/README.md`, and its six notes.
Checked against `src/mould/mod.rs`, `src/mould/level.rs`, the review 2 and
Wild One reports, `dead-ends.md`, the GPU plan, the wiki and `HANDOFF.md`.

The agent edited nothing in the repository; `git status` was clean at the end.

**Marks.** [measured] is a run made for this review, with the command below.
[code] is checked by reading the code. [doc] is checked in a report or note.
[judgement] is my opinion and is not checked.

## How the runs were made

Two kinds of run, all on The Wild One (dish 5), 22.5 degrees unless said.

1. `./target/release/examples/mouldprobe level=5 players=idle flow=1 seeds=1,2`,
   the repo's own probe, built with `cargo build --release --example mouldprobe`.
2. A harness that links the library by path and changes no game file:
   [`harness/src/main.rs`](harness/src/main.rs). Build and run it from the
   repo root with
   `cargo run --release --manifest-path Reports/rts-design-2026-11-10/red-team/harness/Cargo.toml -- taut 1 22.5 1`
   (modes: `grid`, `taut`, `wave`, `molt`, `speed`, `look`; `PILE=3` picks
   the squeezed pile, `STILL=DIR` writes stills). The commands below write
   `redteam` for that binary. It adds four things the probe lacks.
   - **Paired clones.** Step a dish to 60 s, `clone()` it, change one thing in
     one copy only, and step both. `Dish` is `Clone` and carries its rng, so
     the copies differ only by the change. This is a perfect pairing.
   - **A symmetric two-mould dish.** The Wild One's piles, 400 walkers each
     side, each side with its own `Params`, both `foe_repel` 1.0. Each pairing
     is played with sides swapped, because the player mould always steps
     first.
   - **Walker tags**, to measure how far walkers really travel.
   - **Stills** through the game's own `render::draw`.

Raw output is in [`raw/`](raw/): `grid.txt` and `grid2.txt` (the pattern
grid, with the job lists `jobs.txt` and `jobs2.txt`), `taut1.txt` and
`taut2.txt`, `wave.txt`, `molt.txt` and `speed.txt`. Stills are in
[`stills/`](stills/). The two composites there, `taut-trio.png` and
`grain-grid.png`, were made afterwards for the Brainstorm tab of the Claude Doc
(https://claude.ai/artifact/EDygsdTpEg8vK7HZzTXhdp), from these stills with the
counts from `raw/`. Every run was under 5 minutes.

Seed counts are small: 2 to 4 seeds, or 8 games with sides swapped. Each
result is a direction with its count, not a proof. With 10 grid pairings,
no 8-0 (p 0.008) survives a strict Holm correction (bar 0.005).

---

## The findings, ranked by how much they should change the plan

| # | Finding | Kind | What it changes |
|---|---|---|---|
| 1 | Taut's lever collapses the network onto its piles in 2-5 s; it never straightens a road. Turned down, the mould leaves its piles and dies within 30 s. | measured | Drop Taut as written. It repeats a recorded dead end. |
| 2 | Four of the five directions score share of food eaten. Each pile pays at most 40 walkers' worth, split by head count. So the score counts heads on piles, and many verbs cannot move it. | code + measured | Every first test needs a check that the verb moves the score at all. |
| 3 | A Squeeze from home never reaches the seam. 1,440 food made 134-181 extra walkers at home and changed food share by 0.000 in 12 s. From the seam pile it moved share by +0.001 to +0.004. | measured | Squeeze's timing test is premature; first show a squeeze moves anything. |
| 4 | In mirror games the likelier failure is a dead draw, not a snowball. At 45 degrees both sides end at exactly 0.500 on every seed: the moulds never touch. | measured | Anti-snowball rules may make the real risk worse. Test stalemate too. |
| 5 | The pattern grid at today's scale is a straight ladder, with no cycle in 164 games. Hunter (drawn to enemy trail) > mesh > runner > cord > maze > bead. Maze and bead cannot leave their home pile. | measured | Grain and Grafts fail at today's scale, but the test cannot speak for GPU scale. |
| 6 | Food flow on two moulds costs 2.6 times the step time; the frame mean rises from 7.42 to 10.24 ms. Squeeze guessed "a few tenths of a millisecond". | measured | Squeeze's cost is under-stated tenfold; flow is also an open owner decision. |
| 7 | The "contagion war" joins two opposite dynamics. Tincture's majority rule feeds the leader; Two Fevers' immunity feeds the trailer. | doc + judgement | Pick one before testing; they need different tests. |
| 8 | Walkers move far slower than the notes assume: median 2-3 cells in 1 s, 90th percentile 11-20, not 120. | measured | Contagion front speed is unknown, not "too fast"; regions are sticky. |
| 9 | Bloomfront's Stage 0 gates would pass without testing its top risk. | doc + judgement | Replace Stage 0's gates, or drop Stage 0 as the first test. |
| 10 | Player count, session length, onboarding, replay value and solo-or-versus are missing. The dish runs out of food at about 2:20. | doc | Each direction needs a food supply and a match length before tests mean much. |

---

## 1. Bloomfront

**What the player does.** At minute 2, a keeper holds the lamp over rival
mould on their piles and drops oats as each regrows. The warden holds the
lamp for 102-150 s of 150 [doc: review 2]. Aim within about 2 cells decides
stars 2 and 3 [doc]. That is a chore of aim, not a choice. At minute 4, The
Wild One has no food left to fight for: four of seven piles run dry at
2:10-2:20 [doc]. Bloomfront's answer is wells and blooms, which do not exist.

**Most likely way to fail.** It stays a lamp-holding aim test with two
players instead of one [judgement]. Both keepers' lamps push both moulds,
since today the light field is shared (`r.lamp = self.lamp`) [code]. Two
lamps fighting over one seam doubles the attention problem.

**Does "the opening does not decide it" work?** Its four answers are
untested. Spending from the store has nothing to spend: the median is 0
[doc: storetrace]. A birth costs 1 food whenever a piece holds 1
[code: `economy_by_piece`]. Wells that refill, under the table cap, give a
fixed income per pile held. That points to a frozen split, not a mid-game
(finding 4) [judgement, supported by the 45-degree runs].

**Stage 0's gates test the wrong thing.**
- "Equal hands split food 45-55%." Idle already splits 0.494-0.497 on
  today's lopsided dish (600 v 400 walkers, repel 0.6 v 1.0) [doc]. Two
  equal scripts on a mirror will pass by symmetry. It says nothing about
  the opening.
- "A sharp hand beats a clumsy one." Review 2 already shows this for aim
  [doc]. Passing it confirms the aim test that the research says to remove.
- The mirror gate also depends on the moulds' numbers, not just the layout.
  In my symmetric mirrors the self side's share ran 0.48-0.52 for mesh, but
  0.41-0.69 for runner and 0.34-0.66 for maze [measured]. Seed luck can
  swing a fair dish by 30 points.

**Hidden costs.**
- The currency (correction already noted) [doc].
- Two keepers need two lamps on one light field and per-side tool stocks
  [doc]. Each rule added to `Dish` is paid twice, since the rival is a
  whole cloned `Dish` [code].
- Three or four keepers need the plate-and-colonies split: Large [doc].
- Cross-platform lockstep needs fixed-point headings: 14 float trig calls
  [doc].

**Owner's rulings.** Bloomfront is built on the demo's five tools. The
owner ruled at 18:15 that brainstorms must not anchor on them
[doc: `wiki/decisions.md`]. HANDOFF item 7 still offers Stage 0 as the first
probe. Running it first would re-anchor the plan [judgement].

**Verdict.** Weak as a direction, useful as a harness. Stage 0's two-keeper
plumbing helps any direction that keeps two moulds [judgement].

---

## 2. Squeeze

**What the player does.** At minute 2, tap a full heart, or hold one to
charge. Watch for the rival's tubes dimming. At minute 4, piles have run dry
on today's numbers, so hearts fall silent one by one. Fewer hearts means
fewer decisions, until the endgame is idle [judgement, from the 2:20 crash].

**Can the model do it?** Partly, and less than claimed.
- Flow, sap, the stream, the pulse and walls exist, but only in the single-
  mould free dish. `flow` is off by default, untuned on The Wild One, and has
  no verdict from the owner [code; doc: HANDOFF item 4].
- The pulse is one global sine wave. Both moulds share it in phase, since the
  rival is a clone with the same clock [code: `Dish::beat`].
- The fight ignores load. It reads only `fed[i] != u16::MAX` [code]. With flow
  on, `fed` is any tube joined to any eaten pile, at any distance. So a wave
  cannot tip a seam today.
- The note's own build list (stores, squeeze, hold, pinch, load-scaled fight)
  is five new rules. The synthesis table's "all exist" overstates it.

**Measured: where does a squeeze go?** Flow on, The Wild One, idle, 3 seeds.
At 60 s one clone gets 10 s of a pile's output (923-1,440 food) as sap at
the pile's rim. Command: `redteam wave SEED 10` (home) and
`PILE=3 redteam wave SEED 10` (the middle pile, which is the seam).

| Squeezed from | Where the food is at +2 s | Extra walkers at +12 s | Food share at +12 s |
|---|---|---|---|
| Home (x 40) | Mostly x 16-64; 82-131 food at x 64-80; nothing past x 112 | +172, +134, +181, mostly x 32-96 | 0.526 v 0.526, 0.542 v 0.542, 0.525 v 0.525 |
| Middle pile (x 128) | At +5 s: x 80-144 | +141, +99, +56, mostly x 80-144 | +0.001, +0.004, +0.001 |

The food front reaches about 25-40 cells from the rim in 2 s, then is
absorbed into births. A home squeeze fattens home and moves the score by
nothing. A seam squeeze adds walkers at the seam and moves the cumulative
share by a hair. These shares include 60 s of history. Over the 12 s window
alone, +0.001 is roughly +0.006.

**Does "the opening does not decide it" work?** "Waves thin with distance"
is true, and too strong. Only hearts at the seam matter. Whoever holds the
seam piles first owns the only useful hearts. That is The Wild One's late-
seam-piles dead end in a new form: decided by who is nearer [judgement].

**Dominant strategy risk.** If only seam hearts matter, "squeeze the seam
heart whenever it is full" is the whole game. That is the note's
"Metronome" hand [judgement]. The Reader-v-Random test can pass and still
leave a metronome game.

**Hidden costs.** Flow on both moulds [measured]:

```
./target/release/examples/mouldprobe level=5 players=idle flow=1 seeds=1,2
seed 1 idle: ... share 0.537 (49.9 s)
seed 2 idle: ... share 0.527 (48.7 s)
frame cost, dish 5 intended hand: 9000 frames, mean 10.24 ms, worst 24.90 ms
```

Without flow the same game takes 18.5-20.7 s headless [doc: RTS design].
Its frame mean is 7.42 ms, worst 20.09 ms. So flow costs 2.6 times the
step time.
Note also: both flow seeds went to the player's side (0.53, 0.54), against
0.494-0.497 with flow off. Two seeds only, on a dish that is not symmetric
(the wild mould starts at 600 with repel 0.6), so the cause is unknown.

**Owner's rulings.** The wave is real sap, so it meets "the picture is the
data". But Squeeze needs `flow` on, which is an open owner decision.

**Verdict.** Not dead, but its premise is untested and its first probe is
not small. The cheaper first question is "can a squeeze move a seam at all",
which needs only a load-scaled fight.

---

## 3. The contagion war (Tincture and Two Fevers)

**They are two different games.** The synthesis merges them [doc].
- Tincture converts by margin: "if a rival dye's trail beats its own by a
  margin, it converts". Denser flow converts more. That is a majority rule,
  which feeds the leader [judgement].
- Two Fevers is a spread-and-recover model with immunity per strain. Ground
  you held turns immune to you. That feeds the trailer [doc].
- The synthesis table says "Immunity" answers the opening for both. Only Two
  Fevers has immunity [doc].

**What the player does.** Two Fevers at minute 2: one seed per 6 s, at most
3 held. Pick a tap point, then watch. At minute 4: the same loop, unchanged.
Spores are a flat income, so both players get the same seeds. The game is
placement only [doc]. The "Choke" rule ("narrowest host tube; the rival's
immune wake") is a fixed heuristic. A script that plays it well may be
near-optimal, so the human skill ceiling is low [judgement].

**Does the comeback rule work, or overwork?** Per-strain immunity is a
rubber band. A strong rubber band pulls every game toward 50/50 whatever
the players do. The note's guard checks that leaders sometimes lose. It does
not check that skill survives the band [judgement]. Tincture has the
opposite risk: a snowball the note names itself [doc].

**Can the model do it?** Not built. The GPU plan's contagion is one mark bit
and one short-lived signal field [doc: GPU plan lines 127, 169]. That is one
strain, used as a pulse or alarm, with no immunity and no traits. So "on
the GPU plan" covers perhaps a fifth of it [judgement]. Traits need
per-walker `Params`, which today live on the `Dish` [code: `move_walkers`
reads `self.params`].

**Measured: how fast do walkers really move?** The notes say a front runs at
"walker speed" [doc]. Tincture says "120 cells a second", crossing the dish
"in seconds" [doc]. Tagged walkers at 60 s, `redteam speed SEED ANGLE`:

| Seed, angle | Median displacement after 1 / 5 / 10 s | 90th percentile | Most |
|---|---|---|---|
| 1, 22.5 | 3 / 7 / 11 cells | 20 / 40 / 47 | 53 / 104 / 88 |
| 2, 22.5 | 2 / 5 / 9 | 11 / 32 / 49 | 44 / 77 / 91 |
| 1, 45 | 1 / 3 / 5 | 9 / 20 / 30 | 34 / 44 / 52 |

Walkers never share a cell. A blocked walker stays put and turns at random
[code: `move_walkers`]. So tubes are jammed, and most walkers barely move.
A contact front would ride the fast tenth: roughly 10-40 cells a second at
first, if infection were instant [judgement from these numbers].

So the risk is not "too fast". The real risks are these [judgement]:
- The front may be a slow, diffuse stain, not a crisp travelling edge.
- Walkers stay within about 10-50 cells over 10 s, so held regions are
  sticky. Where you seed early may decide a lot.

**Does a front stop at a gap?** Untested. A wound heals in 4 s
(`Params::wound`) [code]. The 22.5-degree host sprouts across gaps [doc].
So a firebreak buys about 4 s. Both notes concede this.

**Hidden costs.** Two strains, two trail channels, per-walker state and
timers, six trait overrides, a neutral host with regrowing piles, and an AI.
At GPU scale, a 4x finer grid makes walkers and scent 4x slower in dish
terms [doc: GPU plan]. Every speed claim then changes again.

**Other.** The neutral host removes "your mould is your body" [judgement].
That body is what the demo was built to test: "feels alive or feels like
watching" [code: module doc]. Plague Inc. has a two-player versus mode [doc].

**Verdict.** The most original direction, and the least checked. Two Fevers
is the stronger half: its verb does not fight the model, and its score is
host share, not heads on piles [judgement].

---

## 4. Taut

**The premise does not hold in this model.** [measured] Taut's pull is a
spring's scent strength. In this model that is `fed_scent`, the share of
smell a pile keeps once eaten. Each mould has its own scent field, so this
is per player [code]. At 60 s I changed it on one clone of the player's
mould, 4 seeds, 22.5 degrees. Command: `redteam taut SEED 22.5 SCENT`.
Control is the shipped 0.15. It acts on all of that mould's piles at once.
Taut wants one dial per spring, so this is the strongest case of the lever.

| Pull | Network length at +5 s | Walkers within 4 cells of a pile | Piles joined in the best piece | Food share since 60 s, at +30 s |
|---|---|---|---|---|
| 0.15 (control) | 142-225 | 0.71-0.75 | 4 | 0.494-0.508 |
| 1.0 ("taut") | 60-80 | 0.92-0.98 | 1-2 | lower on 3 of 4 (0.449-0.509) |
| 0.5 | 108-121 | 0.85-0.89 | 3 on 4 of 4 | lower on 3 of 4 (0.444-0.516) |
| 0.3 | 145-194 | 0.80-0.82 | 4 (3 on one seed by +30 s) | higher on 3 of 4 (0.489-0.532) |
| 0 ("slack") | 323-434 | 0.20-0.35 | 4-6 | 0.108-0.229; mould dead (0-63 walkers) by +30 s |

At 45 degrees, a pull of 1.0 cut length from 98-102 to 66-70 within 5 s.
Joins fell from 3 to 1, on 4 of 4 seeds.

**Looked at.** Stills at +5 s, seed 1 (`stills/taut-s1-*`). At 1.0 the gold
mould becomes three blobs on its piles and hands the middle pile to the
rival. At 0.3 the change is mild and reads as thinning, not straightening.

**What this means.**
- Tightening does not straighten roads. It pulls the mould off them.
- Slackening does not bow roads. The mould walks off its food and starves.
- The response is fast (2 s), so "acts within 5 s" passes. It acts the wrong
  way.
- This is a recorded dead end: "a pile at full smell after the mould reached
  it pulled every walker back onto it" (`dead-ends.md`, `deposit_food`).
  The phenomena note grepped `dead-ends.md` but missed this entry [doc].
- Jones's tightening raised every node's weight tenfold at once, at 45/45,
  with no food economy, and also removed agents (2010, Fig. 20) [doc]. No
  per-node pull is shown there.

**What the player does.** The dial has one safe band near today's value and
two traps [measured]. The best play is "never touch it". That is neither a
decision nor a chore; it is a dead control [judgement].

**Verdict.** Weakest direction. A mild pull (0.3) may be a small lever worth
keeping inside another direction [judgement].

---

## 5. Pattern counters (Grain and Grafts)

**Measured: the grid at today's scale.** Symmetric Wild One, 4 seeds, each
pairing played with sides swapped (8 games), 150 s, food share.
Command: `redteam grid SELF RIVAL SEED`. 100 games.

| Grain | Settings (SA / RA / reach) |
|---|---|
| mesh | 22.5 / 45 / 9 (The Wild One's own) |
| cord | 45 / 45 / 9 |
| maze | 67.5 / 67.5 / 9 |
| bead | 112.5 / 112.5 / 9 |
| runner | 22.5 / 45 / 15 |

| Pairing | First grain's wins of 8 | Mean share | Range |
|---|---|---|---|
| mesh v cord | 6 | 0.513 | 0.486-0.534 |
| mesh v maze | 8 | 0.721 | 0.598-0.833 |
| mesh v bead | 8 | 0.833 | 0.818-0.856 |
| mesh v runner | 7 | 0.564 | 0.498-0.634 |
| cord v maze | 8 | 0.651 | 0.597-0.740 |
| cord v bead | 8 | 0.740 | 0.740 every game |
| cord v runner | 1 | 0.470 | 0.427-0.553 |
| maze v bead | 5 (3 ties) | 0.599 | 0.500-0.658 |
| maze v runner | 0 | 0.351 | 0.229-0.449 |
| bead v runner | 0 | 0.154 | 0.129-0.191 |

Mirror games (self side's share): mesh 0.48-0.52; cord 0.500 on all 4;
bead 0.500 on all 4; maze 0.34-0.66; runner 0.41-0.69.

**A second lever: stance toward the rival.** Grafts' "Hunter" is drawn to
enemy trail. I added `hunter` (mesh with `foe_repel` -1) and `shy` (mesh
with `foe_repel` 3). 64 more games, same method.

| Pairing | First's wins of 8 | Mean share | Range |
|---|---|---|---|
| hunter v mesh | 7 | 0.544 | 0.498-0.581 |
| hunter v cord | 8 | 0.558 | 0.539-0.569 |
| hunter v runner | 7 | 0.529 | 0.495-0.556 |
| hunter v shy | 6 | 0.601 | 0.452-0.698 |
| shy v mesh | 0 | 0.411 | 0.333-0.451 |
| shy v cord | 3 (1 tie) | 0.492 | 0.429-0.518 |
| shy v runner | 4 | 0.505 | 0.328-0.613 |

Hunter beats everything it met. The top of the ladder just moved up one
rung. Seven grains and stances, 164 games, and no cycle anywhere.

**What this shows.**
- A straight ladder, no cycle: hunter > mesh > runner > cord > maze > bead.
- Among the five angle grains, mesh, the default, is never beaten on a
  majority of games.
- Looked at (`stills/grain-*-s1-60s.png`): maze and bead never leave home.
  Cord holds its own three piles and never touches the middle. These are not
  textures that counter each other. They are degrees of mobility.
- Jones's labyrinths and islands come from 15% of cells filled, no food,
  500 x 500 [doc: Jones 2010 Fig. 13]. Our two moulds hold about 2,000
  walkers on 41,000 cells, about 5% [measured]. The patterns do not form
  here.

**So the cheap test cannot do what the synthesis says.** It says: "if one
pattern beats all the others, two directions fall at once." At today's
density the grid kills Grain and Grafts, but only for today's dish. It
cannot speak for GPU scale, where the patterns might form [judgement]. A
cheap follow-up would rerun it at Jones's density, by raising `table` or
the inoculum.

**A mid-match molt is slow and small.** [measured] `redteam molt SEED cord`
switches the player's mould from mesh to cord at 60 s. The rival stays mesh.
Network length at +5 s: 226 v 225 on seed 1, so nothing shows in 5 s. By
+15 s: 182 v 210. Share since 60 s at +90 s: molt ahead on 4 of 4, by
+0.003 to +0.037 (p 0.125). Grain's "molt reorganises over a few seconds"
is closer to 10-15 s.

**Other.** Grafts' "Lace: wide angle, short reach ... finds food fast"
breaks the sensor-reach dead end: shorter reach smells food from less far
[doc]. The phenomena agent applied that dead end; the player agent did not.
Per-walker rules, a third population and fusion make Grafts Large [doc].

**Verdict.** Dead at today's scale on this evidence. Untested at scale.

---

## 6. The card layer (the Rack)

- **A fixed rack is solvable offline.** Ten cards in a known order with a
  tray of three is a sequence a census can optimise [judgement]. The owner
  allows shuffling, which fixes this.
- **Two snowballs stack.** The reserve is income from eating, and cards are
  playable only near your own fed tube. The leader gets more income and more
  places to act [doc; judgement on the stacking].
- **Gate 5 will cut most cards.** "Each removal moves share." If share
  counts heads on piles (finding 2), many cards will move nothing [judgement].
- **It multiplies verbs before one verb is shown to work.** The base game has
  no proven mid-game [judgement].
- Example cards lean on the old tools [doc: synthesis concedes].

---

## 7. Cross-cutting issues

### 7.1 The score is heads on piles (finding 2)

`Pile::bite` gives at most `table` x `eat` = 40 x 0.03 = 1.2 food a step
[code]. Both moulds split it by head count. So food share is the time-
weighted share of heads on piles. Network shape, waves, road tension and
texture count only if they change heads on piles. The squeeze test shows a
large visible effect (+134 to +181 walkers) with a share change of 0.000
[measured]. Bloomfront, Squeeze, Taut and the pattern counters use this
score. The contagion war scores host share instead. None of the notes says
what the food score really counts.

### 7.2 The research agrees for a shared reason

- **"The opening decides" was measured against a handless rival.** Review 2's
  wild mould has no hands [doc]. In a two-handed mirror the risk may be the
  opposite: a dead-even freeze. Cord v cord and bead v bead ended at 0.500
  on every seed [measured]. The catch-up rules the notes recommend would
  deepen a freeze [judgement].
- **Both clean-slate agents found contagion and pattern counters.** The
  synthesis presents this as independent agreement. Both read the Jenson
  report, which names exactly those two next changes: contagion, and
  parameters set per walker (36 Points) [doc]. Shared input, not two
  confirmations.
- **Jones's figures were read as levers.** Patterns at 15% density and
  tightening by a tenfold global weight did not transfer to this dish
  [measured].
- **"It shines at GPU scale."** No speed or cost is measured there. The GPU
  plan says a finer grid slows everything 4x in dish terms [doc].
- **"Beats random on 13 of 16" as the gate.** That proves a heuristic helps.
  It does not prove the choices are interesting. A game can pass it and be
  solved by one fixed policy [judgement]. Rts-craft's "opening triangle
  census" tests for a dominant policy; the synthesis dropped it.

### 7.3 Claims the notes or code do not support

| Claim (synthesis unless said) | What is true |
|---|---|
| Squeeze "builds on ... (all exist)" | Exist separately; flow off, never run with a rival; pulse global; fight ignores load [code] |
| Squeeze frame cost "a few tenths of a millisecond" (note) | Flow on two moulds: 2.6x step time, +2.8 ms mean frame [measured] |
| Contagion "on the GPU plan" | One strain, one bit, a pulse; no immunity or traits [doc] |
| Contagion "immunity" answers the opening | Only Two Fevers has immunity [doc] |
| Fronts at "120 cells a second" (Tincture) | Median walker 2-3 cells a second [measured] |
| Taut "runs on today's dish"; "slack roads bow, taut ones straighten" | Taut collapses, slack starves [measured] |
| Pattern grid: "if one beats all, two directions fall" | True only at today's density, where the patterns do not form [measured] |
| Grain "molt reorganises over a few seconds" | Nothing visible at 5 s; about 10-15 s [measured] |
| "None of the six slime games is a real-time contest" | Two found titles were never opened (Sprawl Grid, Slimepolis) [doc] |
| Bloomfront Stage 0 is "the decision in front of the owner" | Its gates would pass without testing its top risk [judgement] |

The store correction is right. A birth costs 1 food and fires whenever a
piece holds 1, so the store sits near 0 [code]. It is cheap to fix with a
capped reserve; it is a currency question, not a blocker [judgement].

### 7.4 What is missing entirely

- **Player count.** Bloomfront says 2-4, Tincture 2-4, the rest 1 v 1. Three
  or more needs the Large plate-and-colonies split [doc].
- **Session length and food supply.** Proposals say 4-6 minutes. The Wild
  One's piles run dry at 2:10-2:20 [doc]. No direction has designed or
  costed food that lasts. Regrowing food under the table cap gives a fixed
  income per pile, which tends to a freeze (7.2) [judgement].
- **Onboarding.** No direction has a lesson. The 2026-11-08 playtest said "I
  didn't really know what I was doing" [code: `render.rs` comment].
  Indirect control with emergent results is hard to learn [judgement].
- **Why play twice.** One dish. Drafts and bans (Grain, Strain) are the only
  replay hooks. They rest on counters that failed at today's scale
  [measured].
- **Solo or versus.** The RTS doc says AI first, online last [doc]. But the
  skill in Squeeze, Grain and Two Fevers is reading a rival. Scripted hands
  read exact state (the review hands read `r.walkers` directly) [code]. No
  note says who the second human is: same screen, or network.
- **No human has played The Wild One** [doc: HANDOFF]. Every claim about fun
  here rests on scripts.
- **Competing work.** The owner's decided next game is the forest floor from
  above [doc: decisions]. The RTS plan does not say how it relates.

---

## 8. Is the "cheapest questions" order right?

1. **Pattern grid.** Cheap, as claimed: about 40 minutes to write a harness
   and about 20 minutes of compute for 164 games here. But it cannot kill
   Grain or Grafts at the scale they need. At today's density its answer is
   a ladder topped by whichever setting moves and attacks most.
2. **Squeeze probe.** Not small. It needs stores, squeeze, hold, pinch, a
   load-scaled fight, and flow on both moulds at 2.6x cost. My slug test
   shows timing cannot matter until a squeeze moves the seam.
3. **Contagion.** Rightly the dearest. But it is the most novel idea, and
   the only direction no run here could test [judgement].

The order misses the test that killed Taut in two minutes of compute: apply
one verb to one of two identical clones and compare. Rts-craft ranked a
"response census for every verb" first. The synthesis left it out.

---

## 9. Recommended first test: the paired-clone verb census

**What.** Step The Wild One to 60 s and clone it. Apply one direction's core
verb in one copy only. Step both 30 s. No model change is needed unless
the verb does not exist yet. Then build the minimum rule and nothing more.

| Direction | Verb in the clone | Status |
|---|---|---|
| Taut | Pull up or down (`fed_scent`) | Done here: fails |
| Squeeze | Sap slug at a pile's rim | Done here: home moves 0.000; seam +0.001-0.004 |
| Grain / Temper | Switch angle or reach | Done here: slow, small |
| Squeeze, fixed | Same slug, with a fight scaled by load | Needs one rule |
| Contagion | Seed one strain (one bit, one field) | Needs the GPU plan's phase-1 rule on CPU |
| Bloomfront | Lamp, oats, salt, cut, each once | Tools exist; needs only the clone harness |

**Deciding measure, named before the run.** Food share from the action to
+30 s, verb copy minus control, on 16 seeds. For contagion, use host share.
Also record seconds to the first visible change, read from stills.

**Pass.**
- A visible change by +5 s on 16 of 16 seeds (the owner's "every action
  shows something").
- Share moved by at least 0.02, in the intended direction, on 13 of 16
  (sign test p 0.02). 0.02 is the mirror noise measured here: mesh v mesh
  landed at 0.48-0.52.
- The same verb at a random place and time moves share less, on 13 of 16.
- Holm across the verbs tested; say which survive.

**Why first.** A verb that cannot move the score beyond mirror noise cannot
carry a strategy game, whatever its timing or counters. Each verb costs
minutes of compute. It needs no new rules for three of the five directions.
It is the cheapest test that can kill a direction outright, as it did Taut.

**Then.** Build the one-strain contagion rule and look: front speed, and
whether a front stops at a wound, on 16 seeds. Only if fronts travel at a
readable 10-40 cells a second and stop at gaps, add the second strain and
immunity. In parallel, rerun the pattern grid at Jones's density.

**Strongest direction** [judgement]: Two Fevers. Its verb rides the model,
its score is not heads on piles, and nothing here weakened it. Its risk is a
comeback rule so strong that skill washes out, and it is wholly unbuilt.

**Weakest direction** [measured]: Taut. Its lever does the opposite of its
premise, fast, and repeats a recorded dead end.
