# Plan: a slime that shapes its world

2026-11-17. Draft 2, for review (draft 1 plus the owner's answers, 01:13Z). Thread: *Review how the game uses its
simulation*. Owner's steer (2026-11-17 00:45Z): replacing scripted behaviour
with fields "would be ideal, but I don't want to put a bunch of effort into
making the game worse"; ideas may go outside the run; **E (the slime builds
terrain) first, then C (fields that react) and B (faking its signals)**;
"first we write a detailed plan. Have it reviewed then start to implement."

Nothing here is built. Every number is a first guess, to be set by looking.

## In one paragraph

Today the player changes the world (walls, pumps, craters, fire) and the
slime only moves across it. This plan lets the slime change the world back,
out of its own model: where it lives long it leaves **crust**, ground that
remembers it, carries it faster and burns; where it lives long *and is being
shot*, the crust hardens into **bulwarks** that stop rifle fire. Then the
fields start to **react**: fed slime is wet and will not burn, starving slime
dries and does; burnt ground is barren; the slime flees fire; gas ignites. Then two **signal** tools work on the food news that runs
through its body: one that makes a part panic on your timing, one that
calms a charge. Last, the scripted charges and specks are moved onto
fields where a side-by-side film shows the field version is no worse.

## What this builds on (checked on main d024480d)

| Part of the model | Where | Used in play today |
|---|---|---|
| Walls with health; walkers bite a wall they bump; at 0 it is rubble (wet agar) | `engine.wgsl` `walls` (2092), `build_wall` (2203); `big.rs` `build_wall_hp` (2595), `bite_walls` (2698) | player walls, doors, Thicken, sapper mini-boss |
| Ground word: crystal bits 0-7, wet 8, wall 9, scent-wall 10, rock 11, soil 12-15, **16-23 kept for mould-made ground**, byte 3 kept for height | `engine.wgsl` 558-600; `Reports/larder-maps-2026-11-11.md` row 8 ("crust", planned, never built) | soil kinds on maps |
| Soil upkeep: barren ground costs `barren_up` x upkeep | `engine.wgsl` `life` (1243) | maps only; the player can't make barren ground |
| Plants, fire, char, moisture (moisture fixed from the map) | `plants.rs`; `engine.wgsl` `fire` (3812), `firekill` (3864), `ignite` (3888) | scorch rounds and blasts light plants; fire kills walkers on hot cells; trail >= `fire_damp` 3 doesn't burn |
| Food news through the body: a feeding walker hears now, cells spread the latest | `engine.wgsl` `ngather` (1335), `nspread`, `nhear`; `life` (1212) withers on old news | wither and panic of cut-off parts |
| Alarm field (laid by shots and strikes) | `Burst::alarm` (`squad.rs` 200) | flinch on planets 1-3, tiers 4-8 |
| Soldier smell laid into the food scent | `Rules::smell`, `smell_r` (`rules.rs` 231) | the slime reaches for soldiers |
| Targets written into the model: panic, surge, raid, roam, lump | `engine.wgsl` `panic_to` (743), `surge_to` (835), `raid_to` (869) | all the run's pressure; raids already walk a distance field |
| Statuses on walkers (Blind, Sour, Spin) | `BigDish::mind` (`big.rs` 5248), `mindlay` (3914); gun thread | Mind bullets from planet 2 |

The gun thread's plan ([gun-system review](../gun-system-review-2026-11-16/README.md),
*The six families*) already has bullets that **write** fields: Route (road,
false feast, decoy), Element (wet, dry, igniter, seeder), Body (mark, drain).
This plan is the other half: **rules in the model that make those writes
matter and that the slime itself acts on**. Bullets stay the gun thread's.

## Rules every step follows

- **A field, not a goal.** Each rule reads and writes cells. Nothing picks a
  target cell. Where a step can't avoid one, it says so.
- **The picture is the data.** Each new field is drawn from its own value
  through one ramp, with no threshold edge. A look choice goes up as a blind
  A/B.
- **Both engines.** CPU `BigDish` and GPU `Engine`, with a twin test (one
  tick to the bit on llvmpipe, card slack only where atomics race). New
  per-cell state packs into existing buffers (the `news` words, as plants and
  Mind do), not new bindings.
- **A key, then a default.** Each rule has a `gpu::set`/`BigParams` key, so
  `larder-world set=KEY=V` can film it. It goes on by default once its film
  is on a page and nothing measured says it harms play (the owner's "defaults
  on"). The steps in Part S are the exception: they replace something that
  works, so they need a film the owner has seen.
- **Look first, then one named measure.** Each step names its deciding look
  and measure before the run, on paired seeds, with counts beside the film.
  No census unless a step says why, and then under an hour.
- **Smoke checks:** idle doesn't win, a planet doesn't end in under a minute,
  nothing soft-locks (every new obstacle can be broken by something the
  player has from planet 1).
- **Frame cost** quoted on the GTX 1070 through the PC runner for anything
  that runs every step.

## Part E: the slime builds

### E1. Crust: ground that remembers the slime

**Rule.** One value a fine cell, `crust` from 0 to 1, updated once a game
second (`crust_every`):

- rises by `crust_rise` x min(walkers on the cell / `crust_occ`, 1) where
  the trail is at least `crust_trail` (roughly a minute of steady living
  fills it: `crust_rise` 1/60);
- falls by x `crust_keep` a second where no walker stands (half-life about
  two minutes: 0.994);
- is wiped by a crater, and becomes char where it burns (C2).

**What crust does** (each its own key, so each is filmed alone):

1. **Memory roads** (`crust_mem`): trail fades slower on crust, keep becomes
   keep + (1 - keep) x `crust_mem` x crust. A route the slime used keeps a
   faint trail after the slime is shot off it, so it comes back along the
   same roads instead of re-exploring. The map remembers the fight.
2. **Fast ground** (`crust_fast`): a walker steps x (1 + `crust_fast` x
   crust) on crust. Its old roads become its highways.
3. **Hard going for soldiers** (`Rules::crust_wade`): soldiers walk slower
   on crust, as they wade in slime but weaker, so ground the slime held
   stays hostile after it leaves.
4. **Fuel** (C2): crust burns like a thin plant cover, so fire can run
   along the slime's old roads.

**What the player sees.** A dull glaze on the ground under and behind the
slime, brightest on its oldest roads, fading over minutes after it leaves.
The glaze is the field. Crust and char and rubble must read apart at zoom
6 and 14 (the art direction page's palette; blind A/B of two glaze ramps).

**Play.** Clearing ground is no longer the same as killing what's on it.
Burning a road or blasting it cuts the slime's highway; leaving it lets the
slime flow back fast. It gives fire and blasts a job beyond kills, and it
gives the player a map of where the slime has been.

**Code.** CPU: `BigDish` gets `crust: Vec<f32>` and a `crust_tick` after
`life`; trail keep and step read it. GPU: a `crust` pass a thread a cell
every `crust_every` ticks, the value packed as a word in `news` after the
plant words; `diffuse` and `walk` read it. Squad: `squads.rs` speed reads it.
View: the slime-look thread draws it (their lines in `view.wgsl`).

**Deciding look and measure.** Planet 3, seed 1, idle hand, 0-5:00, zoom 6:
is there a readable map of its roads by 3:00? Then a blast kills a road's
middle at 3:00: seconds until walkers cross it again and walkers on it at
+30 s, `crust_mem` 0 against 0.6, 4 paired seeds.

**Cost.** One cheap pass a second (guess under 0.02 ms a frame on the 1070,
averaged), and one read in diffusion and walk every step (guess 0.02 ms).

**Risk.** Memory roads that never fade freeze the network in place (the
Reid et al. 2012 idea is the opposite, avoiding old slime). If the slime
stops exploring, flip the sign as an option (`crust_mem` < 0: old ground
repels, a nomad slime) and film both.

### E2. Bulwarks: the slime hardens where it is shot

**Rule.** A second value, `hard`, from 0 to 1, on the same slow tick:

- rises by `bulwark_rise` where crust >= `bulwark_crust` (0.6), at least
  `bulwark_occ` walkers stand, and the alarm is at least `bulwark_alarm`
  (it is under fire); and rises faster beside cells already hard
  (x (1 + `bulwark_join` x mean hard of the 8 around), the termite-pillar
  feedback, so it builds ridges, not a sheet);
- costs the walkers on the cell `bulwark_cost` energy a second, so a slime
  that builds is a slime that doesn't grow;
- at 1 the cell is a **bulwark**: a wall cell owned by the slime;
- falls back to crust over `bulwark_fade` seconds with no walker on it.

**What a bulwark does** (owner, 01:13Z: bulwarks block **both** shots and
soldiers; the slime walks through its own, as soldiers through doors):

- Stops soldiers' rifle shots like a wall, and takes damage from them as a
  wall does (shots chip `hard`), so it is cover, not a lock.
- Turret shells and lobbed devices pass over it (the owner's wall ruling,
  2026-11-16 19:50Z, applies unchanged), so turrets and throws are the
  answer to a dug-in slime.
- The slime walks through its own bulwarks at a slow step, as soldiers walk
  through their own doors (the door ruling, 2026-11-16 14:24Z, mirrored).
- Fire burns it back to crust (C2), a blast breaks it at once.

- Soldiers can't walk through it. Built in two PRs inside the step: cover
  first (E2a), then blocking soldiers (E2b), which needs the walking maps
  to go stale where a bulwark rises or falls (`paths.rs`, the levelgen
  thread's lock; maps rebuild every `PATHS_EVERY` 30 s today, so a fresh
  bulwark must mark its cells' maps stale) and the walls thread's door and
  side rules to know a wall can be the slime's. Until a map catches up,
  `steer.rs` must stop a soldier at the bulwark, never inside it.

**What the player sees.** Under sustained fire from one place, the slime's
face toward the squad thickens into dark ridges within about 30-60 s; shots
start hitting ridges instead of walkers; the kill count falls. Move the squad
and the new face is soft. Stop shooting and the ridges sag back to glaze.

**Play.** The slime answers a static firing line, out of the model: alarm
plus crust plus walkers. A squad parked in one place wins less over time;
flanking, turrets, throws, fire and blasts each have a reason. It is
pressure that grows with how you play, not with a timer, and it costs the
slime food, so pressing it from two sides makes it spend. It also makes
turrets matter more against a slime that has had time to dig in.

**Deciding look and measure.** Planet 3, squads hand standing at one
crystal's edge for 90 s, then the same hand moving 40 cells round at 45 s:
bulwark cells and the share of shots stopped by them over time, kills a
minute in each half, 4 paired seeds. The step works if the standing hand's
kills fall and the moving hand's recover.

**Smoke.** Idle: no bulwarks (no alarm). No soft-lock: rifles still chip
them; a planet never has a crystal the slime can wall off from every
direction the player can reach (check on planets 1-8, seed 1).

**Cost.** The slow tick plus a wall test in `shots` (already reads the
ground word). Guess under 0.03 ms.

**Risk.** A turtle slime is the run's old stalemate turned round. The fade,
the energy cost and the chip are the three dials; film with each.

### E3 and E4, later

- **E3 tunnels.** Hungry walkers pressed against soft rock wear it like a
  wall, so over a long planet the slime opens new ways through the map
  toward smelled food. Needs the walking maps to follow ground changes
  (E2b's work), and the level generator's fairness check to allow it.
- **E4 old ground regrows.** Spores rest in thick crust and wake near food,
  so a burned-off area refills unless the crust is burned too. The core that
  grew back was dropped by the owner's no-king ruling (`Reports/dead-ends.md`,
  `CoreKind::Regrow`); this is no core, but it must not become one.

## Part C: fields that react

Today the fields barely touch each other (wet ground and reeds stop fire,
and that is about all). Each rule below joins two fields the player already
writes. Together they make a table the gun thread's Element bullets and the
devices can play on.

| | crust | moisture | fire | char | plants | gas | alarm |
|---|---|---|---|---|---|---|---|
| **fed slime** | lays (E1) | wets (C1) | resists (C1) | pays x3 to cross (C2) | ignores (ruled) | sleeps (today) | flinches (today) |
| **starving slime** | lays less | dries (C1) | burns (C1) | pays x3 (C2) | ignores | sleeps | flinches |
| **fire** | burns to char (C2) | dries (C1) | | leaves (today) | eats (today) | ignites (C5) | lays (C4) |

### C1. Wet slime, dry slime

**Rule.** Moisture becomes a field that changes: each cell's moisture moves
toward its map value over about a minute, a fed walker on a cell raises it
by `slime_wet` x its energy share, and fire dries it. Fire's damp test reads
moisture instead of trail (`fire_damp` becomes a moisture level).

**Play.** A fed body is fireproof. A part cut off from food, withering,
dries and burns. So the cut and the fire become one plan: cut, wait for it to
dull, then light it. The gun thread's Wet and Dry bullets get a meaning on
the slime itself, not only on the ground.

**What the player sees.** The slime's sheen (#263) is driven by moisture:
glossy where fed, dull where starving. That is the cue to burn.

**Measure.** A cut-off part at 20 s, lit: walkers killed by fire with and
without C1, against the uncut body lit at the same time.

### C2. Burnt ground is barren, and crust burns

- Char above `char_barren` counts as barren for upkeep (x `barren_up`, 3)
  while it lasts, so a burnt strip is a firebreak the slime pays to cross.
  (The 11-15 review's finding 15.)
- Crust is fuel: fire runs along the slime's old roads, and crust burns to
  char.
- A bulwark burns back to crust.

**Play.** Fire becomes a way to shape where the slime can live, and a
counter to crust and bulwarks. **Measure:** crossings of a scorched line
between the slime's home and a pump in 2 minutes, with and without.

### C3. Not doing: the slime eating plants

Dropped. The owner ruled on 2026-11-14 17:29Z that the slime does not eat
plants (the plants thread's rulings), and on 2026-11-17 01:13Z was "not sure
the benefits [are worth] the cost". Draft 1 misread the 11-14 line as a
complaint. Plants still matter through fire (C2, C5) and as fuel.

### C4. The slime flees fire

Fire's heat lays alarm (a share of the heat), so a fire front pushes the
slime, and the flinch it already has does the rest. Fire becomes herding.
Check against `Reports/dead-ends.md`'s flee from gunfire (11-12): that one
lost a census gate, which no longer gates (build first, balance later).

### C5. Gas ignites

A cell whose gas is above `gas_burn` lights when fire or a blast reaches it,
using its gas up: sleep gas then a scorch round is a flash that kills what it
put to sleep. Device and bullet combos come from this with no new card rule.

## Part B: faking its signals

The slime's food news is the one signal that is only this game's. The gun
thread's Cut and Route bullets cover blocking energy and faking scent; these
two work on the news itself. Built as engine primitives on `Burst` (as
`Burst::mind` is), with one test device here; cards are the gun thread's to
make if they want them.

- **B1. Provoke.** Walkers in the area are stamped as having heard no food
  for the whole wither lag and lean: they panic at once, charging the
  nearest soldier in reach or running for a crystal. You choose when a part
  of the slime charges, so you can pull it off a crystal into turrets. One
  store a walker in the strike pass.
- **B2. Calm.** Walkers in the area are stamped fresh and ignore surges for
  `calm_secs`: a charge stops where it is. The answer to a surge you didn't
  choose. Needs `surge_to` to read the stamp (one test).
- **B3. Hush, after a count.** A timed band where cells don't pass news on:
  a cut that kills nothing, so the far side withers and panics. It is only
  worth building if run planets often have parts worth cutting, which the
  owner doubts (11-16 03:44Z). First count how often (the gun plan's own
  check before Cut), then decide.

**What the player sees.** Provoke: a part of the slime lurches toward the
squad 0.5-1.5 s after the hit. Calm: a charging front slows and spreads.
Both are movement, so they need no new drawing.

## Part S: scripted pressure onto fields, only where it is no worse

The owner's guard: no effort spent making the game worse. So each of these
is filmed side by side against today on planets 1-3, with the pressure
counts (seconds to first contact, walkers reaching the squad in a surge, pump
health at 3:00), and kept only if the counts are no lower and the owner
can't tell them apart or prefers the new one. Mini-boss errands stay as they
are: they are characters, and the owner shaped them.

- **S1. Surges follow the soldiers' smell.** A surging walker's sensors read
  the scent field the soldiers lay (`Rules::smell`) with its haste, instead
  of a straight line to the nearest unit. Walls and rock bend the charge
  naturally, and a soldier who masks their smell isn't charged. Check first
  whether soldier smell needs its own channel apart from food scent (today
  it is mixed in).
- **S2. Specks follow a far smell.** A coarse, slow-fading second scent
  channel that only hungry walkers read (the 11-15 review's findings 4 and
  13), so specks find fresh crystals by smell instead of by a picked target,
  and the main body reaches when hungry. Bigger: a new field.
- **S3. Panic runs on the same two smells** once S1 and S2 are in.

## Order and what the owner sees

| # | Step | Size | The owner sees |
|---|---|---|---|
| 1 | E1 crust: field, memory roads, fast ground, look | M | a page: crust building over 5 minutes, a blasted road refilling with and without memory; glaze A/B |
| 2 | E2 bulwarks: cover (E2a), then blocking soldiers (E2b) | M + M | a page: a standing squad walled off, a moving one not; a playable build |
| 3 | C1 + C2 wet/dry slime, barren char, crust burns | M | a page: cut, dull, burn; a firebreak holding |
| 4 | B1 provoke + B2 calm, one device | S | films of a provoked charge into turrets and a calmed surge |
| 5 | C4 fire lays alarm, C5 gas ignites | S each | a film each |
| 6 | S1, then S2 | S, M | blind side-by-side films |
| later | E3 tunnels, E4 regrowth, B3 hush | | after a count or the owner's ask |

Each step is its own PR with tests, a page under
`Reports/sim-use-2026-11-17/`, the wiki page it changes, and the 1070's
timing booked through the PC runner.

## Who else it touches

- **Slime-enemy rules** (engine rules): the slime-enemy thread is done; this
  thread takes these rules unless the coordinator says otherwise.
- **Walls thread**: bulwarks are walls the slime owns; door and side rules.
- **Levelgen thread**: walking maps going stale where ground changes (E2b,
  E3); fairness check.
- **Plants thread** (done): moisture becomes a field (C1), crust as fuel.
- **Gun thread**: Element and Route bullets read these rules; B's
  primitives for their cards; no new cards here.
- **Slime-look thread**: crust glaze, bulwark ridges, sheen from moisture.
- **Squad movement**: `crust_wade` on soldier speed.

## Outside the run (Q2: "you can expand outside the run")

Not in this plan's order, each with a first experiment if the owner wants it:

- **A. An evolving slime.** Children inherit sensing, speed and toughness
  with small changes; what the player kills least survives, drawn as a shift
  in tint. First: per-walker `size_spread` made heritable on the CPU big
  dish, one shooting hand, 10 minutes, does the mean shift?
- **D. The slime as the economy.** Dead walkers spill a little food where
  they die; the player's pumps can take it, and the slime streams back to
  eat it. Farming against wiping out. First: the spill into the plant or
  grass field, film a kill box for 60 s.
- **F. Rival slimes.** Two species on a deep planet that convert each
  other; the player baits one into the other. First: `species` 2 on planet 4,
  one minute, do they meet?
- **G. The player's own mould** (parked, D12).
- **A terrarium mode**: no soldiers, the player only writes fields (wet,
  fire, food, crust) to grow a slime into a shape or keep two in balance; a
  puzzle mode that shows off E and C and needs no new engine work after them.

## The owner's answers (2026-11-17 01:13Z)

| Question | Answer |
|---|---|
| Bulwarks block soldiers too, or only shots? | "Both" |
| The slime walks through its own bulwarks? | "agreed" |
| Fed slime fireproof, starving slime burns? | "sure" |
| The slime eats plants? | "not sure the benefits it cost of this": dropped (C3) |
| B's tools as devices, gun cards, or both? | "agreed": both, the gun thread makes the cards |
