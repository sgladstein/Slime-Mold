# Review 4: today's guns and cards against the owner's complaints

2026-11-16. Reviewer 4 of the gun-system review ([brief](../brief.md)). This is a
review of the code on main as of 1562ac2, not a build: no file but this one was
changed. Code claims cite `file:line`. Nothing was run, so every number comes
from the code or from an earlier report.

## The big picture

The owner is right. The 11-12 research's structure (a shot is a small program of
payloads, modifiers and triggers; [research](../../flashes-of-chaos-guns-2026-11-12.md),
*Principles*) was built one level deep. The budget, portable cards and caps
landed; the principles that make combos did not:

- **Chains** are missing: no cast releases another.
- **Order** matters only for a modifier placed before a payload; 8 of the 19
  fixed cards ignore their slot.
- **The world multiplies the cards** in two places only, both by accident: alarm
  wakes gassed slime, and blasts light dry plants.
- **Luck** goes into the size of numbers, not into which rules you find.

So the bullet axis (the gun range's five rows of four) is mostly one verb, kill a
disc, at three sizes, with four stat leans you can't see at a front. Combos stop
at "one modifier makes the next payload bigger".

The fix is not more cards of the same shape:

1. Make the four cards in a row four *shapes*, not four sizes.
2. Lift three cheap limits: split pieces that carry casts, an on-expire hook,
   and flight cards that care about their slot.
3. Build trigger and homing on top.

The one structural cost is on the GPU: every killing burst is a synchronous
read-back, so chains need burst batching first.

## 1. Why the categories feel samey

### What a bullet is in code

A bullet card is `Ammo { shell, set, depth, damage, speed, range, spread, cost }`
(`src/mould/bullets.rs:146-159`). Only four of those numbers reach the game:

- **Damage** becomes a radius. `Ammo::reach` is `sqrt(damage / 10)`
  (`bullets.rs:196-198`). It scales a plain strike (`src/mould/larder_world/turn.rs:139`)
  and a payload's burst (`src/mould/rounds.rs:537`).
- **Range and spread** multiply the gun's (`turn.rs:140-141`).
- **Speed** multiplies the bullet's flight (`src/mould/larder_world/shots.rs:41`).

The **set** (Standard, Slug, Needle, Buckshot) is only a lean on those four
numbers (`bullets.rs:133-140`). It adds no behaviour. So the four cards in a row
of the gun range (`src/mould/run.rs:850-858` lists `Shell::ALL x Set::ALL`)
differ in four ways:

- **Burst or strike radius**: Slug 1.26x, Needle 0.77x, Buckshot 0.89x, and
  about ±10% per one-sigma damage roll (`SIGMA_DAMAGE` 0.2, `bullets.rs:46`). A
  plain strike stops growing at `R_MAX` 7 fine cells (`src/mould/squad.rs:54`,
  `turn.rs:139`). That is 1.4x, or damage 19.6, so a deep slug's lucky damage
  roll on a plain bullet is invisible.
- **Speed**: invisible at a front. The rolled-bullets films found "Speed and
  range matter little when the mould is within a few cells", and the four plain
  bullets "are hard to tell apart" ([report](../../rolled-bullets-2026-11-14.md),
  *Films*).
- **Range**: invisible while the slime is inside the gun's reach. The soldier
  aims at the densest block it can see (`squad.rs:645-690`), so a long bullet
  just hits the same place.
- **Spread**: invisible on the rifle and the long gun. Their base spread is 0.5
  and 0.3 cells (`src/mould/guns.rs:128-130`), so even Buckshot's 2.5x lean is
  about one cell.

So within a category the player sees one difference: a ring about a quarter
wider or narrower. That is exactly the owner's "just a stronger explosion".

### Between categories

A burst's fields come from `rounds::bursts` (`rounds.rs:542-551`):

| shell | kill r | crater | alarm | other | colour (`draw.rs:1636-1646`) |
|---|---|---|---|---|---|
| plain | strike 2.5 cells | 2 s (`Rules::crater`) | the shot's | — | warm white |
| blast | 6 (`kit.rs:130`) | r 7, 20 s | yes | lights dry plants | orange `[255,120,40]` |
| bomb | 4 (`rounds.rs:424`) | r 4, 6 s | yes | flings the ring out to 10 cells | orange `[255,90,30]` |
| gas | — | — | — | sleep 20 s + cloud | violet |
| germ | — | — | — | contagion | green |

Plain, blast and bomb are one verb, "kill a disc and hold it open", at three
sizes. The bomb adds a throw, but the throw lands around the hole, and "the body
fills back in within about a second" ([explosions](../../explosions-2026-11-14/README.md),
*In the films*). It reads as a smaller blast, and in almost the same orange.

The germ, the one payload whose effect travels on its own, is **never sold in a
run**. `shells_at` and `card_power` leave it out because "the GPU world casts
none yet" (`run.rs:429-438`, `run.rs:455-460`, `src/bin/larder-world/draw.rs:1679-1680`).
That reason is stale. The GPU world has cast the germ since 11-14
(`src/gpu/larder_world.rs:750-754`; ROADMAP L-K, #216). Gas, whose 1070 twin is
equally pending, is sold.

So a run's shop offers two distinct bullet verbs before planet 4: kill, and
sleep. From planet 4 it adds the throw. The owner's "really just four" is
generous.

### Cards that are invisible at a front

- **Quiet** removes the alarm (`rounds.rs:540`), which only the Sleeper, Swarmer
  and Grudge read (`src/mould/tiers.rs:153,163,168`), plus the Sporer's
  fruiting. The run's first three planets (Drifter, Sprawler, Trunk; `run.rs:75`)
  read none, so Quiet does nothing visible there.
- **Thrift** changes only the budget (`rounds.rs:302-314`).
- **Range** does nothing while the slime is already in reach.
- **Smother** "barely shows over 3 s" ([gun budget](../../gun-budget-2026-11-14.md)).
- **Bait** at a front pulls slime that was already coming.
- The **speed, range and spread** of every rolled bullet (above).

### Counting the real verbs

Every field write a card makes today: kill a disc; hold ground open (crater);
throw live slime (fling); sleep (gas, mist); infect (germ, not sold); add food
scent (bait); wipe it (smother); alarm (implicit, read by half the run's tiers);
light dry plants (incidental, `squad.rs:273-284`). Pierce, split, scatter, wide,
delay and thick are six ways to deliver them; rate, range and thrift change only
numbers. A run has about seven verbs, four of them on trail cards. The bullet
categories the owner looks at hold three.

## 2. What limits combo depth

**One level deep, by construction.** `casts()` folds modifiers into the next
payload and resets them (`rounds.rs:458-477`, reset at `:472`). `Cast` holds one
payload and four flags (`rounds.rs:443-454`). Scatter and Quiet are bools, so a
second copy does nothing (`:464-465`), and a second trigger replaces the first
(`:466`).

**Casts fire only on a slime hit.** `after_step` keeps only shots with
`cast > 0` and a hit (`rounds.rs:736`). A bullet that runs out of range just
expires (`shots.rs:112-115`), and one that meets a wall ends with no hit
(`shots.rs:54-59`). There is no on-expire, on-wall, on-kill or on-land hook.

**Some bullets carry no cast.** Split pieces carry cast 0 and no pierce
(`shots.rs:107`), and turret shots carry no mods at all (`shots.rs:40`,
`turn.rs:205`). A scattergun's fan and a pierce do carry the cast
(`turn.rs:142-151`; "A piercing bullet casts at each strike", [no recharge](../../no-recharge-2026-11-14.md)),
but split, the widest fan, multiplies only plain strikes.

**Order mostly doesn't matter.** Flight cards are counted squad-wide, not read
in order. `flight()` and `trails()` count copies over the whole deck
(`rounds.rs:483-500`), and `casts()` skips them (`:468`). So pierce, split,
scorch, bait, mist, range, rate and smother apply to every bullet wherever they
sit. The deck is a bag with one ordered rule.

**The bullet has no behaviour state.** `Bullet` is position, heading, range left,
radius, owner, pierce, split, cast and speed (`src/mould/larder_world.rs:210-230`).
Flight is a straight leg per frame (`shots.rs:47-64`). There is no turn rate,
target, bounce count, depth or timer.

**Effects don't read what they hit.** A burst is the same whether it lands on
asleep, sick, dense or flung slime, so there are no multiplicative rules in the
code. The world gives two interactions:

- Alarm wakes gassed walkers, an anti-synergy that Quiet answers (`lull`).
- Blast and scorch light dry plants.

**The cost is on the GPU.** `GpuWorld::burst` does a `flush()` and then, for
every killing burst, a synchronous `struck()` read-back
(`src/gpu/larder_world.rs:660`, `:679-682`; `src/gpu/mod.rs:2738-2741`). Trail
bursts and burns skip it, but payload kills don't. A scattergun on blast is
already "the case to time" ([no recharge](../../no-recharge-2026-11-14.md),
*Frame cost*). Any chain multiplies those stalls. ROADMAP L-K already lists
"bursts batched into the frame's submit" as its next step.

### Cheap limits and structural ones

**Cheap** (game-side, the same on both engines): split pieces carrying the
parent's cast (one field at `shots.rs:107`, plus a burst cap); an on-expire cast
(`landed` pushes the expiry point to a list `after_step` reads); counts for
scatter and quiet; positional flight cards (`pierce` and `split` move into
`Cast` and onto the bullet from its cast); turret decks (a `Deck` on
`Building`); multicast (N casts a shot in `turn.rs:127-133`).

**Structural:** batching the GPU's kill counts, which every chain needs first;
effects that read walker state (asleep, sick, dense) in the engine's strike
pass; bullets that sense fields, since the game sees only `mould_grid`
(`world.rs:120-126`) and trail or scent would need a sample in the packed
readout; non-disc bursts on the GPU.

## 3. The trigger card: planned, never built

**The record.** The owner's 11-14 15:06Z notes list "trigger next card on hit"
([gun budget](../../gun-budget-2026-11-14.md); `wiki/decisions-game.md:87`). The
gun-budget report built range, rate, thrift and smother from the same list and
wrote that the trigger "waits on the owner's answer about what a bullet type
is". The owner answered at 16:46Z (bullets are rolled cards) and 18:05Z
(payloads are bullets). Both later reports then list "2. The trigger card" under
*Next* ([no recharge](../../no-recharge-2026-11-14.md), [rolled bullets](../../rolled-bullets-2026-11-14.md)).
`git log --oneline | grep -i trigger` finds nothing in the 290 commits on hand.
The code's `Kind::Trigger` names Delay and Thick (`rounds.rs:128`), which decide
*when* a payload goes off, not *what* comes next. Since 11-15 the HUD labels the
kinds Bullet, Next bullet and Always (`CardKind::label`), so "trigger" is no
longer on screen at all.

**Why it slipped** (inferred from the reports' order): rolled bullets took the
gun thread that evening, then it moved to the gun's look and feel (11-15). There
is also a code seam: pieces are made in `Game::landed`, which knows no casts,
while casts go off later in `Rounds::after_step`, so a trigger needs the cast
table inside `BulletMods`.

**How it would fit:**

- **`Card::Trigger`**, a modifier: "where the next bullet hits, it releases the
  bullet after it." In `casts()`, a Trigger marks the next payload as a carrier.
  The payload after that compiles as its child and is taken out of the firing
  cycle, as Noita's trigger draws its payload from the deck.
- **`Cast`** gains `child: u8` (0 none, else index + 1). A Trigger before the
  child nests once more.
- **`BulletMods`** gains `child: [u8; MAX_SLOTS]` and `outer: u8`. `outer` is
  the number of casts the cycle walks. Today `casts` counts every bullet
  (`rounds.rs:488-492`, `turn.rs:126-130`).
- **`Bullet`** gains `depth: u8`.
- **In `landed`**, on the bullet's final hit (and on expiry, when the on-expire
  hook exists), spawn the child's pieces the way split does (`shots.rs:99-109`),
  but with `cast = child`, the child's own `Ammo` radius, speed and range, and
  `by` kept so `casts_of` resolves.
- The child's set can choose the fan: Buckshot 4 pieces wide, Needle 1 piece
  straight on, Slug 1 piece that lodges as a Delay.
- **`after_step` is unchanged.** Each child that hits casts its own payload a
  frame or two later.
- **GPU world:** no WGSL. Pieces are ordinary `ShotIn` legs under `SHOTS_MAX`
  256 (`squad.rs:47`), and bursts go through `World::burst`. The cost is the
  read-back per kill burst, so this waits on batching or a hard cap.
- **Caps:**
  - depth 2 (three levels; the 11-12 research said about 3);
  - at most 4 pieces per carrier;
  - the existing bullet cap, with its drop counter;
  - a new cap of about 32 killing bursts a frame, with the rest dropped and
    counted (`Rounds::dropped`);
  - `PENDING_MAX` 64 for lodged children.
- **Budget:** card cost 2, tier 3. The child payload pays its own cost and slot,
  so a 10-budget rifle fits one chain and a deep long gun two.
- **Size:** about 300-400 lines with tests, plus a CPU/GPU twin of
  `trigger+bomb+gas`. CPU frame cost is one table lookup per spawned piece. GPU
  cost is one leg per piece in a pass already sized for 256, plus the bursts;
  time it on the 1070.

## 4. Homing

**Where it fits:** `fly` computes a straight leg from `dir` each frame
(`shots.rs:47-64`). A seeking bullet turns `dir` by at most `seek_turn` radians
toward a target before its leg. The leg is still a straight segment traced a
cell at a time, so walls, both engines' traces and replays are unchanged.

The target must come from what the game sees on both worlds. That is
`World::mould_grid`: exact on the CPU, and from the frame's packed readout on
the GPU with no extra read-back once it is wanted (`world.rs:120-126`,
`gpu/larder_world.rs:919-925`). The Thick trigger already relies on it
(`rounds.rs:770-774`). The bullet gets one byte of seek strength and a target
cell. It is about 150 lines game-side.

**Why "densest slime" is the wrong target:** soldiers already aim at the densest
block they can see (`squad.rs:645-690`). Homing to the same block is a bullet
that ends where it would have gone anyway. The dead end on spores shows the
second trap: spores that homed from launch "turned back to the crystal they
fruited on" (`Reports/dead-ends.md`, `spore_reach`). Seeking from the muzzle
finds what is nearest the shooter. So: fly straight for the first half of the
range, then seek.

**What makes it more than aim.** Homing pays where it does what a soldier's aim
can't:

1. **After the first hit.** Split, trigger and bomb pieces each seek a
   *different* block, skipping blocks already struck this frame. One shot
   becomes a cluster spread across the body. A soldier can't aim its shrapnel.
2. **Round a corner.** Aim needs a clear line (`squad::sees`, `cmd.rs:454`). A
   bullet that curves past a wall's end hits slime no soldier can see.
3. **Seek a link, not a lump.** Target the thinnest occupied block on the line
   between the body and the crystal it is eating. With blast or scorch that is a
   supply-cutter, and the one-body rule (a part cut from food withers and
   panics) does the rest. No soldier aims there.
4. **Seek moving targets.** Flung walkers, or a breakaway lump. The game would
   need the GPU's flung mark in the readout, so this is a second step.
5. **Read fields like a walker.** Later, a bullet with three sensors that climbs
   the trail along a vein, delivering germ or gas deep. It needs a sample pass,
   so it is structural.

Ship 1-3 as one flight card, ideally positional so it rides the next bullet
only, and leave 4-5 for later.

## 5. Explosives: tools or weapons?

### How it works today

The run's shop sells unlocks in a *Tools* group: blast 150, mine 150, det cord
200, sleep gas 200, and "Cheaper tools" at 120 a level (`run.rs:393-397`); the
germ is never sold (`draw.rs:1679-1680`). In play, E opens a tools page
(`draw.rs:1110`, `:1153-1173`). Each soldier at the larder takes one tool, a
selected one up to five (`src/bin/larder-world/main.rs:274-286`,
`kit.rs:279-291`). The player clicks a point within 24 cells of a soldier
holding one; only distance and open ground are checked, not line of sight
(`kit.rs:253-269`, `:305`). Each use is paid from the bank (blast 60, mine 80,
cord 3 a cell, gas 150; `kit.rs:129-153`), a fifth less per tools+ level
(`run.rs:690-699`). The burst lands at once with no throw (`kit.rs:329-351`),
and since the `Kit` lives outside `Game`, uses are not in the replay
(`kit.rs:295`).

**The overlap.** The Blast *card* casts the kit's blast exactly: `rounds.rs:543`
reads the same `blast_r`, `crater_r` and `crater_secs` as `kit.rs:389`, free,
every second (18:38Z, `rounds.rs:12-18`). Gas is the same burst on both sides
(`rounds.rs:548`, `kit.rs:346-348`), a mine is Thick plus Blast
(`rounds.rs:28-32`), and det cord is a heavier scorch line. A tool is a card's
burst, for food and a click.

### Keep them separate

In an auto-firing RTS the tools are the only effect the *player* aims: guns are
a program you write, tools a decision you make. Mines and cord go down *before*
the slime arrives, while guns fire only at slime a soldier can see
(`cmd.rs:450-456`). Tools reach behind walls, and they are a food sink tied to
pumps now that guns spend no food.

### Merge them

Two shops, two UIs and two code paths serve one `Burst`, and the tool is
strictly worse than the free bullet with the same burst. The kit sits outside
`Game`: no replay, no player AI, and a hidden restock the owner once read as
soldiers refusing orders (`kit.rs:159-168`). Tools take no modifiers or
triggers, so they are outside every combo. And "the squad owns its gun" invites
"the squad owns its kit".

### Pick: one device slot per squad, in the weapon system

The slot holds a cast in the gun's own card grammar: a payload card plus up to
two modifiers or a trigger. The player still places it on a point, which keeps
the player's verb, the reach behind walls and the pre-placement. It uses charges
carried from the larder (today's carry rule, `carry_max` 5) instead of a bank
price per use. Mine and det cord become *delivery* cards that fit only the
device slot ("lies until slime crowds it", "a line"), the Tools shop group
becomes device cards, and the `Kit` moves into `Game` so device use enters the
replay. The owner's "each squad carries one type" gives squads roles (the gas
squad, the mine squad), and roles are where RTS combos live (section 6). One
flag for the owner: charges are a carry limit, as tools have today, not a gun
recharge. If they want no limit at all, the device needs some other price, and
that is their call.

## 6. What has been missed, and other directions

1. **The world doesn't multiply the cards.** Noita's lesson (principle 5) is that
   the simulation turns two spells into a combo; here payloads never read what
   they land on. World rules that change walkers, and so show: asleep slime is
   brittle (a strike on dormant walkers kills at twice the radius), so gas then
   plain is a combo; a sick walker killed bursts germ into its neighbours, so
   germ then bomb spreads plague; a flung walker lands carrying its burst's
   field (whether sickness survives `BigDish::fling` today is worth checking).
2. **Model verbs no card uses:** a *sever* payload that cuts the body's energy
   flow along a line, so the cut-off part withers and panics (the most
   slime-specific verb in the brief); moisture (douse a fire line, or dry ground
   for one); plants (sow dry brush to burn later); breakaway (force a lump to
   break off and run into your mines); surges (provoke one into a killzone); the
   player's own mould as a payload, to seed and convert.
3. **Order barely matters** (section 2). Positional flight cards would make the
   same six cards in another order another gun.
4. **Combos between squads.** In an RTS, multicast happens across squads: one
   gasses, a second shatters, a third bombs the sick. Each squad holds its own
   deck (`Squad::deck`), but nothing in the shop or HUD rewards specialists.
5. **Turrets carry no cards** (`shots.rs:40`): they could be the slow, heavy
   casters, with mortar decks of Delay and Trigger.
6. **Luck lives only in numbers.** Noita's jackpot is a rare rule. Tier 3 holds
   only Bomb and Germ, both payloads, which the card row filters out
   (`run.rs:864`), so the rarest tier offers nothing there. Trigger, multicast,
   always-cast, seek and on-expire belong at tier 3; numbers keep their
   normal-curve rolls.
7. **Legibility.** Blast and bomb share an orange (`draw.rs:1638`, `:1642`).
8. **Another direction: release agents, not projectiles.** A "bullet" could be a
   handful of your own walkers (the own-mould species) with sensors and a
   payload, released into the slime's veins to go off where trail is thickest or
   to convert. Combos then come from what your agents read, not from deck order.
   Only this game can make that.

## 7. The five changes I would make first

1. **Four shapes per shell, not four sizes, and the germ back in the shop.**
   Slug becomes one wide burst plus a short outward shove (the knock-back the
   splash dead end left open "as play", `Reports/dead-ends.md`). Needle becomes
   a line of three small bursts along the heading, Buckshot a cluster of 3-5,
   and Standard stays as today. Pull blast and bomb apart (blast removes and
   holds; bomb throws and kills little, in its own colour), and drop the stale
   germ exclusion.
   - *Size:* small, about 150 lines in `rounds::bursts`, `run.rs` and
     `draw.rs`, with no GPU change.
   - *Experiment:* film the gun range's 5 × 4 grid on one front and seed, 3 s a
     card. Put stills on a page with killed, flung, slept, sick and crater cells
     beside each, and keep the labels off. Deciding measure: can the owner name
     the set from the still?
2. **The trigger card, with cast-bearing split pieces and an on-expire hook.**
   - *Size:* medium, about 400 lines plus twin and cap tests, after a GPU
     kill-count batch or cap (about 100 lines in `GpuWorld::burst`).
   - *Experiment:* 8 rifles on `trigger+bomb+gas` against `bomb+gas`, 10 s in
     the gun range. Count bursts a second, walkers slept and flung, and frame
     time on the 1070 at the 256-bullet cap.
3. **Seek**: straight, then homing; pieces seek blocks not yet struck; a variant
   seeks the thinnest link to the crystal being eaten.
   - *Size:* about 150 lines game-side, on both engines through `mould_grid`.
   - *Experiment:* `split+seek` against `split` at a body beside a wall's end.
     Count distinct blocks struck per shot and kills beyond the wall's end, and
     film it.
4. **The device slot**, moving the toolkit into the weapon system.
   - *Size:* medium-large: `Kit` into `Game`, the slot UI and the shop group,
     about 500 lines and up.
   - *Experiment:* a one-hour hack where the tools page's Blast throws the
     squad's first payload cast with its modifiers, free, limited by held
     charges. Play 5 minutes and count throws and which casts get thrown.
5. **Status combos through the fields**: brittle sleepers, sick walkers that
   burst germ, flung walkers carrying their field, and the sever payload.
   - *Size:* medium, structural on the GPU (the strike pass reads the life
     state).
   - *Experiment:* CPU only, same body and seed: gas then plain against plain,
     counting kills per shot; and germ then bomb against germ, counting sick
     walkers at 20 s and how far the farthest sick one is from the hit.
