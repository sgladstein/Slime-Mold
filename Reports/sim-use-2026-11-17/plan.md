# Plan: a slime that shapes its world

2026-11-17. **Draft 3**: draft 2 (the owner's answers of 01:13Z) revised
after three independent reviews ([game designer](reviews/game-designer.md),
[artificial-life researcher](reviews/alife-researcher.md),
[readability](reviews/readability.md); briefs in
[reviews/prompts.md](reviews/prompts.md)). Thread: *Review how the game uses
its simulation*. Owner's steer (00:45Z): field-driven over scripted "would
be ideal, but I don't want to put a bunch of effort into making the game
worse"; ideas may go outside the run; **the slime building terrain first,
then reacting fields and faking its signals**; plan, review, then build.

Nothing here is built. Numbers are first guesses, set by looking.

## In one paragraph

The player changes the world (walls, pumps, craters, fire) and the slime
only moves across it. This plan lets the slime change the world back, out of
its own model. It gets a **memory of where it was hurt** (the Grudge's scar,
moved to the GPU) and a **memory of where it lived** (crust, which its
walkers read, so it returns to old ground). Where it remembers being hurt
and lives on, it **sets**: walkers there stiffen into a living wall that
soaks fire and blocks soldiers, and the slime flows through it. Then fields
**react**: slime cut off from food goes dry and burns where there is fuel,
crust is fuel, char is ground it can't breed on, fire scares it, gas
ignites. Then two **signal** tools act on its food news. Last, the scripted
charges move onto a distance field where a film shows that is no worse.

## What the reviews changed

All three reviews agreed the direction is right and found the same holes.

| Draft 2 | Problem the reviews found | Draft 3 |
|---|---|---|
| Crust acts on trail keep and step | Walkers never read it, so memory lasts about 3 s (toy: 2.8 s against 1.1 s); trail keep is self-made clay, which trapped the mould on 15 of 16 seeds | Walkers **sense** crust, as the Grudge senses its scar; its trail keep stays as it is |
| Crust shown as a glaze | The view hides ground under the slime (`view.wgsl:2318`); no free brightness level is left | A crisp satin **gloss** whose coverage is the crust value, seen where the slime has left; settled by a blind "name the ground" quiz |
| Bulwarks read the live alarm | The alarm halves in a few fine steps; once-a-second samples are mostly zero | Bulwarks read the **scar** (threat memory), ported to the GPU |
| Rifle craters wipe crust | E2 then can't form where it is shot (toy: kills don't fall) | Only blasts and fire wipe crust |
| A bulwark is a WALL cell | Walls are off the agar: walkers are evicted, news is cut, fade flickers | A bulwark is **walkers that set** (a living wall), with a ground bit only for soldiers' paths |
| Fade "with no walker on it" | The slime lives on its bulwarks, so nothing ever fades | Set walkers wake when the scar fades |
| Join term for ridges | Self-reinforcement with no inhibitor makes a slab | Dropped; the shape comes from where it was hurt |
| Energy cost | Doesn't bind at the 50,000-walker cap | Cost in bodies: set walkers neither move nor breed |
| C1 wets by energy over a minute | Energy pools through the body; dries as the part dies anyway; fire needs fuel | Dryness is the **news age** (no new state, dry 5 s after a cut), sheen drawn from it; burns only on fuel |
| Barren char stops crossings | Nothing reads it; walkers cross and die | Char forbids births; measured by fill, not crossings |
| B1 provoke is one stamp | `nhear`'s `atomicMax` (`engine.wgsl:1389`) restores fresh news next step | A timed **deaf** status (a Mind kind) |
| B2 calm stamps fresh news | `surge_to` never reads news; can delete the main threat | A timed Mind status with a cost; renamed **Still** (`calm` is a pass) |
| S1 surges follow soldier smell | That smell fades by e every 3 cells, under the crystals' | A **units distance field** (the raid's search, many sources) |
| `crust_wade` slows soldiers | Owner 11-14: slow ground "might be annoying" | Dropped |
| 4 paired seeds | Can't pass a sign test (smallest p 0.125) | 8 paired seeds for a deciding measure; 2 x 2 with the rule off |

Owner rulings the reviews said draft 2 pushed against: sustained fire
carves a growing hole that doesn't refill while under fire (11-12 04:39Z);
slow ground might be annoying (11-14 17:29Z); fire must not make wiping out
the slime easy (11-14). Each is designed around below; the first is a
question for the owner.

## What this builds on (main 85a1bd14)

| Part of the model | Where | Today |
|---|---|---|
| The scar: a slow per-cell memory of hurt, read by the sensors | `big.rs` 1192-1212, 3321-3325 (CPU only) | the Grudge tier, kept out of the run's `LADDER` |
| Armour: a lump member takes several hits | `engine.wgsl` `armoured` 426-434 | mini-bosses |
| Mind statuses: a timed word a walker | `big.rs` `mind` 5248; `engine.wgsl` `mindlay` 3914 | Blind, Sour, Spin bullets |
| Food news through the body | `ngather` 1335, `nspread`, `nhear` 1381-1394 | wither, panic, the ash-grey of cut parts (`view.wgsl` 1107) |
| The raid's distance field | `big.rs` `raid_field_for` 5492; `raid_dist` | raids walk round walls |
| Fire through fuel; char | `engine.wgsl` `fire` 3812, `firekill` 3864 | plants burn; slime with trail >= 3 doesn't |
| Barren soil: upkeep x 3 and no births | `big.rs` 1262-1274 | maps only |
| Ground word bits 16-23 kept for mould-made ground | `engine.wgsl` 558-600 | unused |
| Walking maps rebuilt every 30 s | `paths.rs` `PATHS_EVERY` | soldiers' paths |

The gun thread's bullet families (Route, Element, Body, Mind) write fields.
This plan adds the rules that make those writes matter. Cards stay theirs.

## Rules every step follows

- **A field, not a goal.** Rules read and write cells. Where a step keeps a
  picked target it says so.
- **The picture is the data,** crisp at 24 px a cell: a field's value sets a
  coverage or a material, not a soft glow. Each look gets a blind quiz
  (shuffled clips, key off the page) at zoom 1, 6, 12 and 24, with a grey
  frame.
- **Both engines,** twin-tested; new state packs into existing buffers.
- **A key, then a default,** after a page. Part S needs the owner's film
  first.
- **One named measure on 8 paired seeds,** with the rule-off arm, deaths by
  cause logged (shot, fire, starve, wither, wade).
- **Smoke:** idle doesn't win; a planet lasts over a minute; nothing
  soft-locks (every obstacle can be broken by something planet 1 sells).
- **Frame cost** on the GTX 1070 through the PC runner.

## Step 0. Threat memory on the GPU

Port the Grudge's scar (scar x `scar_keep` + `scar_gain` x alarm each fine
step, read by sensors at `scar_weight`) to the GPU, inside the alarm pass.
It is the input E2 needs, and it puts the Grudge back in the run's ladder at
planet 6 or later. Twin test; frame cost. **See:** the Grudge on planet 6
converging where it was shot. Size S-M.

## Part E: the slime builds

### E1. Crust: ground that remembers the slime

**E1a, the field and its look, no effects.** One value a fine cell, 0 to 1,
on a once-a-second tick: rises where walkers have stood (by occupancy, one
walker a cell, with trail over `crust_trail`), falls slowly where none stand
(half-life about two minutes), wiped by blasts and fire, never by rifle
craters. Drawn as a crisp satin gloss on the ground whose coverage is the
crust value, seen where the slime has left. Before building, compute crust
offline from logged walker positions and look at it beside the frame
(the readability review's first experiment). Then the blind "name the
ground" quiz: crust, char, rubble, damp bank, barren.

Expect a footprint: shape C's home is a packed sheet (the ALife probe: 53-60%
of crusted cells full by 1:30), so crust shows the slime's range and its
roads off the home plate, not a road map inside it. A hand that pushes the
slime back shows it; an idle hand won't.

**E1b, the effects:**

1. **Return to old ground** (`crust_weight`): sensors read crust beside the
   trail. Film +w and -w: + is territorial, it flows back to where it was;
   - is a nomad that moves on (close to the Wary tier, a dead end on a
   census gate, which no longer gates). Deciding: at 2:00 blast half the home
   away; overlap of cells re-held at +60 s with the old crust, paired
   against weight 0.
2. **Fast ground** (`crust_fast`), capped so a step never passes 1.5 (the
   Sprawler's): `walk` tests only the landing cell, so a longer step could
   hop a 2-cell wall.
3. **Fuel**: crust burns, so fire runs along abandoned roads (C2).

`crust_wade` is dropped (slow ground "might be annoying").

### E2. Setting: the slime hardens where it was hurt

**Rule.** A walker on crust where the scar is above `set_scar` sets: it
stops moving, stops breeding, and takes `set_armour` hits to kill (the lump's
armour count). It wakes when the scar under it falls below `set_wake`, with
a gap between the two (hysteresis), so it doesn't flicker. A cell holding at
least `set_block` set walkers gets a ground bit soldiers can't walk through.
Unset walkers flow through set ones at a slow step (the owner's door ruling,
mirrored). Nothing new is drawn: set walkers are drawn by their own state, a
material settled by blind A/B (the readability review: not "dark ridges",
which read as the player's walls; not bronze, which the owner turned down;
try translucent amber against the body raised and glossy).

**Blocking** (owner, 01:13Z: "Both"):

- Shots: set walkers soak hits, so a rifle line on a set rim kills slowly.
  Sight is not blocked, so soldiers keep firing and the rim always wears
  down: no soft-lock.
- Soldiers: the ground bit marks walking maps stale; `steer.rs` stops a
  soldier at a set cell, never inside one. Blocking soldiers is E2b, behind
  a key, after a check that no nest door (7-9 dish cells on planets 1-2) can
  be sealed with planet 1's rifles. Setting starts from planet 3 unless that
  check passes earlier.
- Turrets and lobbed throws: as today, they kill set walkers like any other.

**Why it plays.** A squad parked on one spot meets a rim that remembers it.
Flanking, turrets, throws, fire and blasts each have a reason. It costs the
slime real bodies (set walkers neither move nor breed), so pressing from two
sides makes it pay. The headline combo (game designer): set walkers count
as dry (C1), so they burn on the crust they stand on. Fire answers a dug-in
slime; the slime answers a static gun line.

**Interactions to film** (from the reviews): the flinch moves walkers off
the cells being shot (setting needs them there); a leech mini-boss shielded
by a set ring at a pump; Shove, Blind and Sour statuses on a set rim; mines
on old roads; turrets against a set slime.

**Deciding measure.** Planet 3, 8 paired seeds, 90 s: 2 x 2 (setting on or
off x squad stands or moves 40 cells at 45 s), shots fired logged. It works
if the interaction is right: kills a minute fall for the standing squad with
setting on, and recover when it moves. Watch for the toy's warning: without
the crater wipe a slab formed in about 20 s and kills fell five-fold, a fast
turtle. `set_scar`, `set_armour` and `set_wake` are the dials.

**The hole ruling.** The owner ruled (11-12 04:39Z) that sustained fire
carves a growing hole that doesn't refill while under fire. Setting keeps
the hole open (killed walkers don't refill it) but toughens its rim, so it
grows slower. Asked of the owner: toughen while under fire, or only after
fire stops (the slime fortifies where it was hit, so the next attack at the
same place meets a wall and a fresh angle carves as today).

### Later: E3 tunnels, E4 regrowth

- **E3 tunnels:** soft rock lets scent seep (`wall_scent` above 0), so hungry
  walkers press and wear it toward food they smell. First a CPU look: fingers
  toward the crystal, or the whole edge worn?
- **E4 regrowth:** spores rest in thick crust; must not become the dropped
  core (no-king ruling).

## Part C: fields that react

| | crust | fire | char | gas | alarm |
|---|---|---|---|---|---|
| **Fed slime** | lays, returns to it (E1) | doesn't burn (as today) | can't breed on it (C2) | sleeps (today) | flinches (today) |
| **Cut-off or set slime** | lays | burns where there is fuel (C1) | can't breed on it | sleeps | flinches |
| **Fire** | burns it to char (C2) | | leaves it | ignites it (C5) | lays it near the front (C4) |

### C1. Dry slime burns

**Rule.** Fire's damp test reads the news age: slime is wet if its trail is
over `fire_damp` *and* it heard of food within the wither lag, and a set
walker (E2) is always dry. Fire still
needs fuel under it (plants or crust). No new state.

**See.** The sheen follows the news age, so the player sees which parts are
cut off, which nothing on screen shows today, beside the ash-grey that
already marks starving parts. A plant crown is never resized by it (the
readability review: that would be drawn-only).

**Guard, the owner's fire ruling:** fire must not make wiping out the slime
easy. The fed body never burns; only cut-off and set parts do.

**Measure.** A cut part on grass, lit at +10 s: fire kills per walker
present, cut against joined, 8 seeds.

### C2. Crust burns, char is barren

Crust is fuel; it burns to char; char above `char_barren` counts as barren
(no births, x 3 upkeep) while it lasts. The slime crosses char but can't
fill it. Together with C1 this is the forest-fire front: fire runs along
abandoned roads into dry parts and stops at the fed body. **Measure:**
walkers on a scorched strip at +60 s, with and without.

### C3. Not doing: the slime eating plants

Dropped (owner 11-14 17:29Z, the slime doesn't eat plants; 11-17 01:13Z, not
sure it's worth the cost).

### C4. Fire scares it, a little

Fire lays alarm near its front. At alarm reach (about 3 fine cells) this
makes the slime step back from flames, not herd it; measure fire kills and
the body's centre at +30 s. Herding would need a longer-reach smoke field:
a later step only if the owner wants herding.

### C5. Gas ignites

Gas above `gas_burn` lights when fire or a blast reaches it and is used up:
sleep gas then a scorch round flashes across the cloud. Check a real cloud's
values first.

## Part B: its signals

Built as engine primitives on `Burst` beside `Burst::mind`, with one test
device here; the gun thread makes the cards (owner, 01:13Z).

- **B1. Provoke:** a timed deaf status (a Mind kind): the walker hears no
  news for the wither lag plus the lean, so it panics, charging the nearest
  soldier in reach (on the units field, S1, once it lands) or running for a
  crystal. It turns ash-grey as it panics (the cut-off look), which is
  honest: it believes it is cut off. Side effect to film: it may wither.
- **B2. Still:** a timed Mind status that `surge_to` and `panic_to` read: a
  surging or panicking walker stops. It costs: Still also stops the walker
  eating, so a held slime starves where it stands; and it must look apart
  from sleep gas.
- **B3. Hush:** only after counting how often a planet has a part worth
  cutting (the owner doubts cuts, 11-16 03:44Z).

## Part S: scripted pressure onto fields, only where no worse

Filmed side by side against today on planets 1-3 (seconds to first contact,
walkers reaching the squad a surge, pump health at 3:00), kept only if no
lower and the owner can't tell them apart or prefers the new one.
Mini-boss errands stay.

- **S1. A units distance field.** The raid's search (`raid_field_for`) with
  every soldier as a source, on 2-cell blocks, rebuilt once a game second.
  Surges and panic descend it instead of steering at a picked unit: walls
  bend the charge, and a masked soldier (the smother card) is left out of
  the sources.
- **S2. A far smell** that only hungry walkers read, for specks and for a
  hungry body to reach (the 11-15 review's findings 4 and 13).

## Order

| # | Step | Size | The owner sees |
|---|---|---|---|
| 0 | Threat memory (the scar) on the GPU; Grudge back in the ladder | S-M | the Grudge on planet 6 |
| 1 | E1a crust field and look (offline look first) | M | the "name the ground" quiz |
| 2 | E1b crust effects + C2 crust burns, char barren | M | return to old ground, +w and -w; fire along old roads |
| 3 | E2a setting (soaks fire), then E2b (blocks soldiers) behind a key | M + M | the 2 x 2 films; a playable build; the material A/B |
| 4 | C1 dry slime burns, sheen from news age | S | cut, dull, burn |
| 5 | B1 provoke, B2 still, one device | S | a provoked charge into turrets; a stilled surge |
| 6 | C4, C5 | S each | a film each |
| 7 | S1, then S2 | M, M | blind side-by-side films |
| later | E3, E4, B3 | | after a count or the owner's ask |

Each step is its own PR with tests, a page under
`Reports/sim-use-2026-11-17/`, its wiki page and the 1070's timing.

## Who else it touches

| Thread | What |
|---|---|
| Gun cards | Mind kinds for provoke and still; cards for them; Element and Route bullets on these rules |
| Mini-boss | armour reused by setting; the leech behind a set ring |
| Walls | set cells as the slime's door-like walls; side rules |
| Levelgen | walking maps going stale at set cells; nest doors check; E3 |
| Slime look | gloss for crust, the set material, the news-age sheen |
| Run | the Grudge in `LADDER`; setting from planet 3 |

The engine rules are this thread's (the slime-enemy thread has finished)
unless the coordinator says otherwise.

## Outside the run

- **A. An evolving slime, with a control, no code first:** `mix_share` 0.5
  with a faster kind against all-alike kinds, a shooting hand, 10 minutes;
  does the kind share move beyond the neutral arm's spread?
- **D. The slime as the economy:** kills spill food the slime comes back to
  eat; the game designer's fourth direction.
- **F. Rival slimes:** two converting species on planet 4.
- **G. The player's own mould** (parked, D12).
- **A terrarium:** no soldiers, the player writes fields; three cycling
  species make spiral waves (the ALife review's look-only film).

## The owner's answers

| When | Question | Answer |
|---|---|---|
| 01:13Z | Bulwarks block soldiers too, or only shots? | "Both" |
| 01:13Z | The slime walks through its own? | "agreed" |
| 01:13Z | Fed slime fireproof, starving slime burns? | "sure" |
| 01:13Z | The slime eats plants? | not sure it's worth the cost: dropped |
| 01:13Z | B's tools as devices, cards, or both? | "agreed": both |
| open | Setting toughens the rim while under fire, or only after fire stops? | |
