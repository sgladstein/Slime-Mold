# Clean-slate brainstorm: phenomena first (2026-11-10)

Brainstorm only. Nothing was built, run or measured. The five-tool toolbox was
treated as not existing. `level.rs`, `lesson.rs`, `play.rs`, the dish reports
and `Reports/rts-design-2026-11-10.md` were not opened while the concepts were
made. The RTS doc was read last, for section 5 only.

**Read:** `wiki/index.md`; `wiki/model/` (scale, sensors, water and sleep);
`Reports/jenson-physarum-2026-11-10.md` and its three contact sheets
(network_2, explosion, ef_9); Jones 2010 in full; Jones 2015 sections 1-10,
13 and 14; the module notes and `Params` in `src/mould/mod.rs`. After the
concepts were drafted, `Reports/dead-ends.md` was grepped for each mechanism
used (method rule 5).

**Rules every idea must pass** (owner, `CLAUDE.md`): fun over fidelity;
everything on screen is the simulation and looks like it; every action
delivers something you can see; an outcome is a spread. Interface off the
play area is allowed.

**Scale assumed:** the GPU plan's 10-100x walkers, so 20k-200k walkers and
tubes 1-2% of the dish wide, as in Jenson's network_2. Each concept says what
it needs at today's 256 x 160 dish.

---

## 1. Phenomena the model shows, or could show with a rule change

"Now" means our model does it today. "Rule" means it needs a rule change.

| # | Phenomenon | What it looks like on screen | Source | Status |
|---|---|---|---|---|
| P1 | **Network from noise.** Scattered walkers condense into a web of tubes. | Speckle, then feathery fronts, then a mesh. | Jones 2010 Fig. 4; network_2 | Now |
| P2 | **Shortest path and Steiner tree.** Between food points the web pulls down to a tree. Joints meet in threes near 120 degrees. | Loops vanish; a taut tree with Y-shaped joints remains. | Jones 2010 §4, Fig. 19 | Now (45 degrees) |
| P3 | **Tension: tight against slack.** A strong pull from the food points straightens the tree into a spanning tree. A weak pull lets it sag and close loops. A ring of points can hold a loop open. | Roads straighten like plucked strings, or bow outward. | Jones 2010 §4.1, Fig. 20 | Rule: per-point pull |
| P4 | **Loop collapse and the surge.** Small holes shrink and vanish. Flow surges at the moment one closes. | Holes closing; a bright pulse where two streams merge. | Jones 2010 Fig. 6 | Now |
| P5 | **Sprouting across gaps.** At 22.5 degrees a branch grows across a hole. On contact, flow surges in from the far side. | A tendril, then sudden brightening along it. | Jones 2010 §3; wiki sensors page | Now (22.5 degrees) |
| P6 | **Trunks over capillaries.** At scale, old thick rings persist under a new fine mesh. | Veins with fine branching between them. | network_2 end frames | Rule: scale |
| P7 | **Shrink-wrap.** A ring of mould shrinks onto the outer points. Posts that smell only when touched mark the hull. A shrinking sheet traces a tour. | A band tightening and snagging on posts. | Jones 2015 §7.1, §10.2 | Rule: touch-only scent |
| P8 | **Growth that stops itself.** Seeded on a skeleton, the mould inflates and halts at a concave outline. | A stick figure fattening into a blob that stops. | Jones 2015 Fig. 10 | Rule: growth bias |
| P9 | **Shrink to the centre.** A mass thinned slowly ends on its own centroid. | A shape contracting to a ball. | Jones 2015 §13 | Rule: slow culling |
| P10 | **Voronoi seams.** Separate colonies grow until they meet. Borders sit midway, shifted by source strength. | A cell pattern with dense or dark seams. | Jones 2015 §8 | Now (rival) |
| P11 | **Fusion and rejection.** Kin fuse into one body. Strangers keep a gap. | Two webs merging, or a dark seam between them. | Jones 2015 §1; `Params::foe_repel` | Now (rival) |
| P12 | **Pattern families.** Sensor and turn angles swing the look. Attraction gives mesh, labyrinth or islands. Repulsion gives spots and stripes. Some settings lock to the grid. | Wholly different textures from one rule. | Jones 2010 Figs. 13-16; ef_9 lattice | Rule: per-walker angles |
| P13 | **Domain takeover.** In repulsive patterns one stripe direction spreads and converts its neighbours. | One grain of stripes eating another. | Jones 2010 Fig. 16, §5 | Rule: repulsion |
| P14 | **Repair, two ways.** Repulsive patterns heal back exactly. Attractive webs heal with new fingers and a new shape. | Fingers creeping into a wound and knitting it. | Jones 2010 Figs. 22-23 | Now (attractive) |
| P15 | **Shuttle streaming and the pulse.** Each walker jitters back and forth. Tube contents surge out and back. | Brightness breathing along tubes. | Jones 2010 Fig. 7; `pulse_seconds` | Now (flow) |
| P16 | **Contagion.** A state passes walker to walker through its own trail. It travels along tubes only. A cut stops it; a loop brings it from two sides. If it changes behaviour, the web rewires as it passes. | A coloured front sweeping through the web. | Jenson report; explosion.gif | Rule: planned, off |
| P17 | **Tube memory.** Busy routes grow lasting walls. The mould regrows along them after a cut. Idle ones fade. | Thick trunks and faint ghost roads. | `Params::wall_grow` | Now (flow level 3) |
| P18 | **Sleep and waking.** Dry ground stops walkers but keeps their tube. Wet ground wakes them. | Matte crust, then motion. | wiki water-and-sleep | Now (wet dishes) |
| P19 | **Graded starvation.** Walkers far from food die first. | Tips thinning and fading backward. | `mod.rs` model notes | Now |
| P20 | **Behaviour set by density.** Walkers set reach and angle from the trail under them. | Fine wanderers and slow bundles in one frame. | Jenson report, 36 Points | Rule: per-walker params |
| P21 | **Full coverage over time.** The 22.5-degree web eventually walks every cell. | A mesh that never stops moving. | Jones 2010 Fig. 8 | Now |

Three things here are rare in games. A network **builds and prunes itself**
(P1-P5). A body's **texture is its behaviour** (P12-P13). A **state travels
only where the network reaches** (P16). Those three carry the developed
concepts.

---

## 2. From phenomena to games

The model's levers, with the old toolbox set aside, are these:

- **Who a walker belongs to.** Contagion makes allegiance a travelling state.
- **How hard a point pulls.** Tension decides whether a road is short and thin or loopy and wide.
- **What pattern a body makes.** Angles and reach set the texture and how it fights.
- **Whether two bodies fuse.** Kin merge; strangers wall each other off.
- **Which chemical a walker follows.** A walker could ride a rival's roads.
- **When a body breathes.** The pulse and its timing.

Structures considered: territory, routes and blocking, tug-of-war, races,
drafts and bans of rules, asymmetric sides, prediction, short rounds and long
matches.

### The concepts (18)

| # | Name | Phenomena | Player verbs | Structure and length | Main risk | Verdict |
|---|---|---|---|---|---|---|
| 1 | **Tincture** | P16, P12, P20, P5 | Inject your dye into a shared mould; pick its trait (what it does to carriers); cut a firebreak | 2-4 players, territory of allegiance, 4 min | Snowball; fronts too fast to read | **Developed (A)** |
| 2 | **Taut** | P2, P3, P10, P11, P4 | Set each spring's pull: slack, even, taut; throw sprouts briefly; let go | 2 players, rival road networks that cannot cross, 6 min | Tension acts too slowly to feel | **Developed (B)** |
| 3 | **Grain** | P12, P13, P14, P10 | Pick the pattern of each separate piece of your mould; molt to a new one; bud a piece | 2 players, best of 3 rounds of 3 min, bans between rounds | One grain may beat all others | **Developed (C)** |
| 4 | Hoop | P7, P3 | Feed your ring to swell, starve it to shrink; plant posts it snags on | 2 players, enclosed area, 3 min | Posts are lures in disguise; rings collapse to blobs | Rejected |
| 5 | Ebb | P7, P9, P19 | Grip or release cities as your sheet shrinks on a clock | 2 players, retreat game, 90 s | Jones's shrinking sheets end the same way every run | Rejected (most surprising) |
| 6 | Tempo | P15, P16 with a rest period | Place your pacemaker; set its beat | 2 players, wave duel, 2 min | The faster beat entrains everything; one number decides | Rejected |
| 7 | Graft | P11, P19 | Fuse with a rival to share food; pinch off to betray; keep your lineage at the food | 3-6 players, diplomacy, 10 min | Needs humans; scripted hands cannot test trust | Later mode |
| 8 | Dialect | P16, P11 | Switch which chemical your walkers follow, to ride the rival's tubes to its food | 2 players, raid and counter-raid | Interleaved colours unreadable; a switch is a full reset | Rejected |
| 9 | Desire Lines | P17, P6 | Drive bursts of traffic to lay lasting walls that both moulds follow | 2 players, 8 min | Half-life 30 s; weak link from act to result | Folded into Taut as a rule |
| 10 | Blight | P16, P2, P5, P19 | Builder sets tension and firebreaks; Blight seeds infection and picks its trait | Asymmetric 1v1 or against the machine, 5 min | Side balance | Variant of Tincture |
| 11 | Strain | P12, P20 | Draft physics cards for your species: reach, angle, self-repulsion, memory | Draft, then real time | The draft decides the match | Folded into Grain as bans |
| 12 | Omen | P2, P10 | Bet where joints and seams will form; spend a few nudges | 2-6 players, prediction, 2 min | A bet shows nothing in the dish; skill is reading, not doing | Rejected (owner's rule) |
| 13 | Seedfall | P10, P8, P11 | Drop spores; colonies grow to weighted seams | 2 players, land grab, 3 min | Decided by the first drops | Rejected (opening) |
| 14 | Tide | P15, P11 | Time pulses in a shared channel so surges push the seam | 2 players, tug-of-war, 60 s | One-dimensional; a rhythm test | Rejected |
| 15 | Dormancy | P18, P17 | Prepare where your mould will sleep before a drought | 2 players, season clock, 6 min | Seasons are a clock, not a game | Kept as a clock for any concept |
| 16 | Mend | P14 | One side wounds; the other heals | Asymmetric, short | The healer has little to decide | Rejected |
| 17 | Junction | P2, P3 | Take turns placing nodes; the mould grows the tree; score nodes it uses | Turn-based, 10 turns | The mould only referees | Rejected; a good tutorial |
| 18 | Sweep | P21, P16 | Score cells your dyed walkers have touched | 2-4 players | Too close to known ink-coverage games | Rejected (originality) |

**Why the top three.** Each makes one phenomenon *the decision*. Tincture
turns allegiance into territory, and the network's shape decides where
allegiance can travel. Taut turns tension into strategy: a road's shape is the
choice. Grain turns pattern families into counters you can see. Each scores
a share, so the outcome is a spread. Each has a cheap first test.

**The most surprising rejected idea: Ebb.** A territory game where everyone
only loses ground. Each sheet shrinks on a clock and clings to the cities it
still touches. Your choice is what to let go of. It is fresh and tense. It
fails on Jones 2010 §4.2: shrinking sheets "show identical network
configurations" across runs. The end shape follows from the geometry, so
play may wash out and the result becomes a yes/no. *Re-test if* a grip moves
the final shape on paired seeds.

**Dead-ends grep.** Two entries bear on these concepts. Shorter sensor reach
made the mould smell food from less far and broke every planned hand. So no
trait or grain below shortens reach under 9 at today's grid. The Wild One's
late seam piles were decided by whichever mould stood nearer. So every late
food here comes with a visible warning, and every concept is tested against a
hand that stops playing early.

---

## 3. Three concepts developed

### A. Tincture: a war of allegiance inside one living network

**Pitch.** One neutral mould fills the dish and feeds itself. Nobody owns it.
Each player owns a dye. A dye passes from walker to walker through its own
trail. Each dye also changes how its carriers move. So the network rewires as
a colour takes it, and the new shape decides where the colour can go next.

**On screen.** The mould is drawn from its walkers and trails, as now. Each
dye is its own trail channel, drawn as hue. How strongly a walker holds its
dye is drawn as saturation. A front is a real colour boundary moving through
the web, as in explosion.gif. No line is painted.

**Verbs.**

1. **Inject.** Tap the mould. Walkers in a small disc take your dye. You see a bloom that runs off along the tubes.
2. **Trait.** Before injecting, pick what the dye does to its carriers. Draft three of five before the match:
   - **Run:** longer reach (12-15) and a longer step. Fast, coarse cords; your front races along trunks.
   - **Knit:** sensor angle 22.5. Sprouts and loops; your colour wraps pockets and arrives from two sides.
   - **Clench:** 45/45. Prunes to taut cords; few contact points, so your region is hard to enter.
   - **Shun:** carriers turn away from other dyes' trail. A dark seam opens at your border, a living firebreak.
   - **Swell:** carriers divide more and pay more upkeep. Thickens a region fast but drains food.
3. **Firebreak.** A short wound across a tube, a few charges per match. It stops a front dead. It also stops food beyond it, so it is a sacrifice.

**The conversion rule (starting guess).** Every few steps a walker compares
the dye trails at its cell. If a rival dye's trail beats its own by a margin,
it converts with a small chance. Neutral walkers convert at a lower bar. A
carrier's hold fades unless it walks on its own dye's trail. So dense flows
beat thin ones, trunks carry fronts, and cut-off pockets turn neutral.

**Minute-to-minute loop.**

- 0:00-0:20. The neutral web forms from noise (P1). Players watch where trunks form and which reach food.
- Every 10-20 s, a choice. Inject on a trunk to race to food with Run. Inject behind a rival front to cut it off. Pick Knit to wrap a pocket. Pick Shun or Clench to hold a border.
- A rival front comes up a trunk at you. Firebreak it and lose the food beyond, or counter-inject at the joint.
- Your pockets fade when cut off. Rescue one with a Knit bloom, or let it go.

**Spend and earn.** Pigment. You earn it while your dyed walkers stand on
food. Inject and firebreak cost pigment. Food piles deplete. New ones grow
in with a rising scent halo 20 s before they can be eaten, so you can
prepare.

**How a match is won.** Your score is the share of all walkers carrying your
dye, sampled each second and averaged over four minutes. A typical result
might read 41% to 33%, with 26% neutral.

**Why the opening does not decide it.**

- Hold fades when a region loses contact with its own dye's flow, so an early bloom must stay connected.
- The neutral web is in the 22.5-degree regime and keeps rewiring. Walls you relied on drift.
- A bigger region has a longer border, so it offers more targets.
- Clench cords are chokepoints: one rival injection on a trunk can turn a whole branch.
- Pigment comes from food, and food moves through the match.

**Model rules it bends.** Contagion, already planned as an off-by-default rule
in GPU phase 1. One dye id and one hold value per walker. One trail channel per
player. Walker parameters set by dye (the 36 Points idea). A neutral owner.
The firebreak reuses the existing wound cells.

**Build cost: medium.** A CPU probe on today's dish is small: a byte per
walker and one extra 3x3 pass per dye channel, each about the trail blur's
cost (to be measured). The look wants the GPU plan's 1024 x 640 dish. An
opponent script and the interface make it medium.

**Risks.** Walkers step a cell per step, 120 cells a second, so a raw front
could cross today's dish in seconds. The conversion chance must slow it to
10-30 s per dish. A dominant dye may snowball. Readers may not see why a
front moved. Today's wound heals in 4 s, so a firebreak only buys time; how
long it lasts is a tuning lever.

**The single measurement.** Look first: on 16 paired seeds, does a front stop
at a firebreak every time, and can you see it at 4x? Then the deciding
measure: **mean dye share over the match**. There are four hands. A *trunk
injector* injects where walker flow is highest near its border, and picks its
trait by rule. The others are a *random injector*, *idle*, and the trunk
injector that *walks away at 60 s*. All four play the same reference opponent
on 16 paired seeds. It works if the trunk
injector beats random on at least 13 of 16 (sign test p 0.02). It must also
beat its walk-away copy on at least 13 of 16. Both survive a Bonferroni
correction for two tests (p < 0.025).

---

### B. Taut: two road networks that cannot cross

**Pitch.** Two moulds build roads between the same springs. Strangers repel,
so neither can cross the other's road. You never draw a road. The mould finds
the shortest one itself. You only set how hard each spring pulls on your
network.

**On screen.** Two moulds in two hues. Each spring's scent halo is drawn from
the scent field, which is data. The dial settings sit off the play area. When
you tighten a spring, its roads visibly straighten. When you slacken it, they
bow out.

**Verbs.**

1. **Pull**, per spring your mould touches: slack, even or taut. Jones showed that tighter pulls turn a Steiner tree into a minimum spanning tree and straighten roads (2010 §4.1). Slack lets roads sag, close loops and cover more ground.
2. **Reach.** For 5 s a stretch of your mould switches to 22.5 degrees and throws sprouts across a gap. It costs food and recharges.
3. **Let go.** A spring's pull drops to zero. Its branch retracts like a rubber band, and those walkers rejoin the rest.

**Minute-to-minute loop.**

- Your mould grows from home along scent toward the springs (P1, P2).
- Every few seconds, a tension call. Tighten a safe spring to free walkers. Slacken one near the rival so your road bows across their path. Let go of a spring that has dried up.
- Reach to loop round a rival's thin road, or to touch a new spring first.
- At the seams, fed roads hold and long, thin, starving roads are eaten into (the existing fight rule).

**Spend and earn.** Food. A spring pays you while your tube joins it to your
home. Food moves along the tube by the existing flow rule, so a long road
delivers less. Every walker pays upkeep, so a slack, loopy network costs more.
Reach costs food. Tightening is free, but a taut road is thin and exposed.

**How a match is won.** Your share of all food delivered home over six minutes.
A typical result might read 58% to 42%.

**Why the opening does not decide it.**

- Springs dry up on a schedule. New ones grow in behind a rising scent halo 20 s ahead.
- The minimising mould pulls back any branch nothing pulls, so early ground is not kept for free.
- A lead built on taut roads is brittle; a lead built on slack roads is costly.
- Tube memory stays off, or short, so the first roads do not lock in.

**Model rules it bends.** A scent channel per player, with each spring's
weight in it set by that player's dial (Jones's tightening and loosening). A
timed per-walker angle override for Reach. The rival, the fight at the seam,
the feeding census and food flow already exist.

**Build cost: medium.** It runs at today's scale: roads about 12 cells wide,
with ten springs. Scale makes the roads finer and allows more springs. New
work: the second scent channel, the dial, the override and an opponent script.

**Risks.** Jones's tension effects take thousands of steps (his Figs. 18-20
run 2,000-30,000). At 120 steps a second that is 15 s to four minutes. If a
road does not straighten within about 5 s of a pull, the verb will feel
dead. Is the pull an old lure in disguise? No: it sets the shape of roads you
already have, not where the mould goes next.

**The single measurement.** Look first: does a road visibly straighten within
5 s of going taut, on 8 paired seeds? Then the deciding measure: **share of
food delivered**. There are four hands. A *tension planner* goes taut on
uncontested springs and slack on springs that border the rival. It lets go
when upkeep beats delivery. The others are *always even*, a *random dial*, and
the planner *walking away at 90 s*. All four play one reference opponent on 16
paired seeds. It works if the planner beats always-even on at least 13 of 16.
It must also beat its walk-away copy on at least 13 of 16. Each is p 0.02, and
both survive a Bonferroni correction for two tests.

---

### C. Grain: pattern families as counters you can see

**Pitch.** Each player's mould is a species with a grain: a sensor angle,
turn angle and reach that fix its pattern. Grains fight at borders by
physics. Each separate piece of your mould has one grain, and you choose it.
You read the rival's grain from its texture.

**Grains (starting guesses from Jones 2010, Figs. 13, 15 and 16).**

| Grain | Settings | Look | Hoped-for role |
|---|---|---|---|
| Mesh | SA 22.5, RA 45 | Reticular web, always sprouting | Floods open ground fast; thin |
| Cord | SA 45, RA 45 | Few thick taut tubes | Pushes through lanes; leaves gaps |
| Maze | SA and RA about 67.5-90 | Winding labyrinth walls | Hard to pass through; slow to grow |
| Bead | SA and RA about 112-135 | Isolated dense islands | Holds ground in place; moves food badly |
| Comb | Repulsion, SA 112.5, RA 67.5 | Regular stripes | Heals back exactly (P14); converts neighbouring stripes (P13) |

Reach stays at 9 or above at today's grid, per the dead end.

**Verbs.**

1. **Molt.** Tap one of your pieces and pick a grain. The piece reorganises over a few seconds, visibly churning. It is weak while it churns. Each piece has a cooldown.
2. **Bud.** Spend food to start a new piece at a tip, in any grain.
3. **Pinch.** Let a stretch go thin until your mould splits in two. Two pieces can carry two grains, but each eats only its own food (the existing `pieces` rule). Kin pieces that touch fuse again.

**Minute-to-minute loop.**

- Grow toward food from home. Meet the rival and read its grain from the texture.
- Molt the piece on that border to the counter, or bud a counter piece behind it.
- Pinch your mould so different borders can carry different grains.
- The rival molts back. Timing molts against cooldowns and churn windows is the skill.

**Spend and earn.** Food from the piles your pieces hold. Buds cost food.
Molts cost time and a weak window, not food.

**How a match is won.** Best of three rounds of three minutes. A round scores
each player's share of the food eaten. Between rounds, each player bans one
grain for the other, and the round's loser picks first. That is the draft of
rules.

**Why the opening does not decide it.** If grains have counters, any lead can
be answered by a molt. Bans change the menu between rounds. Churn windows
punish a molt made at the wrong time.

**Model rules it bends.** Walker parameters per piece (per-walker, as in 36
Points). Repulsion as an option for Comb. Two species with their own trails
and the existing seam rules.

**Build cost: large.** Per-walker parameters are cheap. But at today's tube
width of 12 cells the grains will barely read apart, so it needs the GPU
dish. Tuning five grains into distinct, balanced counters is the long part.
The first test is cheap, though. The rival is already a second `Dish` with
its own `Params`, so a grain-against-grain census needs almost no new code.
Only Comb's repulsion is new.

**Risks.** Grain stands or falls on whether counters exist. One grain may beat
all the others. Textures may not read at speed.

**The single measurement.** A **matchup matrix**, with no player input. Each
ordered pair of grains meets on a symmetric dish, over 16 paired seeds. Each
game is scored by share of food eaten at three minutes. It works if every grain
loses to at least one other on at least 13 of 16 seeds. Only then is it worth
testing a counter-picking hand against a fixed-grain hand and a random
molter.

---

## 4. What happened to the old five tools

- **The knife survived, in Tincture only, as the Firebreak.** It wins on merit there. A cut stops a dye front dead, and you see it at once. It costs you the food beyond, which makes it a real choice. In the old herding game it moved nothing on the rival dish; contagion is what gives it a job. Tincture's Shun trait may do the same job as simulation alone, so test both.
- **Oats did not survive** as placed food. Taut's Pull shares their lever, the scent field. But it sets the tension of existing roads rather than leading the mould somewhere new.
- **Salt did not survive.** In Taut and Grain the rival's living mould is the wall, and it can be broken through.
- **The lamp did not survive.** No concept needed a push.
- **Water did not survive** as a tool. Sleep and waking appear only as a possible season clock (Dormancy, concept 15).

---

## 5. Compared with the RTS doc

Read last. `Reports/rts-design-2026-11-10.md` proposes **Bloomfront**: an
indirect-control RTS. Each keeper steers their own mould with the five old
tools, paid from the food store. It adds spores, a global mood switch
("Temper": Forage, Hold, Fight) and fixed species presets. It wins on share
of food eaten, and gates every stage on paired-seed censuses.

| This brainstorm | Overlap with Bloomfront | What is new here |
|---|---|---|
| **A. Tincture** | Bloomfront leaves an empty slot for contagion: "contact can convert a foe's walker rather than kill it", as a fifth species' rule. | Nearly all of it. One neutral mould that nobody owns. Allegiance is the territory. Dye traits rewire the web, so conquest changes the map. Hold fades when cut off. The knife gets a job (stopping a front) without needing starvation. |
| **B. Taut** | Both use two repelling moulds, borders that form by themselves, tube as supply line, and share-of-food scoring. | Tension per spring as the main verb. Road shape and route length as the economy. Planar blocking: the rival's road is the wall, so no salt. Let go as a deliberate retreat. |
| **C. Grain** | Close to Bloomfront's Temper switch with a cooldown, and to its species presets (Lattice at 45 degrees, Runner at 22.5). | Partly new. A grain per separate piece, not one per mould. Grains beyond 22.5 and 45: labyrinths, islands, repulsive stripes. Counters read from texture. Bans between rounds. |
| Section 4 | Bloomfront keeps all five tools and adds two. | Only the knife survives here. |
| Method | Bloomfront already gates on "letting go at 3:00 loses to playing on", and on mirror matches near 50/50. | The same walk-away control is used here. Each concept also has a look-first gate on one phenomenon. |

Name clash: Bloomfront's "Temper" is a global mood switch. Tincture's verb was
renamed "Trait" to avoid confusion.

**In one line:** Bloomfront is the existing toolbox made competitive. Tincture
and Taut are different games, because the player's verb acts on a phenomenon
(allegiance, tension) rather than on mould position. Grain is the nearest to
Bloomfront: its Temper and species ideas, carried much further.

---

## 6. Recommendation

Prototype **Tincture's core** on today's CPU dish before any GPU work. Use
two dyes, the five traits and the firebreak, with scripted hands on 16 paired
seeds. Contagion is already a planned rule, so the probe is small. It answers
the deciding question cheaply: can a planned injector beat a random one in a
network that rewires itself? If it cannot, fall back to Taut. Taut runs at
today's scale and reuses the rival, the fight and food flow.

---

## Sources

- Jones, J. (2010). Characteristics of pattern formation and evolution in approximations of Physarum transport networks. *Artificial Life* 16(2). Repo copy: `Reports/mould-game-2026-11-08/depth-2026-11-09/papers/jones-2010-artl-physarum-transport-networks.txt`.
- Jones, J. (2015). Applications of multi-agent slime mould computing. arXiv:1511.05774. Repo copy: same folder, `jones-2015-applications-multi-agent-slime-mould-computing.txt`.
- `Reports/jenson-physarum-2026-11-10.md` and its contact sheets.
- Two web searches, titles only, no pages opened. They found sandbox simulators, for example [PHYSARUM: Slime Mold Simulator](https://www.dekudeals.com/items/physarum-slime-mold-simulator), [Slime Mold Simulator](https://lordmisterman.itch.io/slime-mold-simulator) and [Spore Valley](https://lystrosaurus-studio.itch.io/spore-valley). No result title showed a competitive game built on these network phenomena. That is a weak check, not proof of originality.
- Known games kept clear of, from memory: map-based plague spreaders, cell-linking tentacle tug-of-war games, ink-coverage shooters, Go and Hex. Sweep (concept 18) was cut for being too close to ink coverage.
