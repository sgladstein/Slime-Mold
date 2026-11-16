# Gun cards: a proposal to explore (2026-11-16)

The owner's ask (11-16 02:34Z): a deep research and brainstorm review of the
gun system. "We have five different categories of bullets and two of them are
just two different sized explosions ... all the cards in each category are
relatively similar. Just changing in some of the base stats"; the trigger card
"I thought we were planning on building"; a homing card; whether explosives,
mines and grenades stay tools or join the weapon system ("Debate"); and the
lucky, insane combos of Flashes of Chaos and Noita. At 02:37Z: "after you do
your first round of research and review and analysis. Ask me questions ...
before just writing everything up as a report."

Live doc: [Gun cards: a proposal to explore](https://claude.ai/artifact/XHycydbtsTv5XHZxqtCLoR)
(this file is its copy; the doc has one diagram, described in *The new shape of
a gun*). No runs: research, four reviews, a code reading and a plan.

How it was made:

- [brief.md](brief.md): the neutral brief three reviewers got (the model and
  the game, no current cards; owner's rulings as constraints).
- Research notes: [Noita's spells and wands](notes/noita.md),
  [Flashes of Chaos and roguelike build variety](notes/roguelike-builds.md),
  [explosives in squad games and combos from world state](notes/explosives-and-world-combos.md).
- Reviews, each written alone: [combos and luck](reviews/r1-combos.md),
  [simulation-first](reviews/r2-simulation.md), [squads and RTS](reviews/r3-squads.md)
  (these three from the brief only), and [today's code](reviews/r4-current-system.md)
  (main at 1562ac2, with file:line citations).
- Round-one questions to the owner (11-16 ~03:20Z) and their answers (03:37Z),
  below.

## Summary

Rebuild the gun cards around what each card does to the slime, not how big its
numbers are, and explore it through small prototypes, each filmed so the owner
picks what stays.

- **Fix the sameness first.** Each bullet set gets its own shape, blast and
  bomb become different effects with their own colours, and the germ goes back
  in the shop.
- **Build the rule cards that make combos possible:** the trigger first, then
  multicast, copies and on-expire.
- **Group bullets into six families**, each touching a different part of the
  simulation, so each family looks different on screen.
- **Combos happen inside a gun and across squads**, through states one card
  leaves on the slime and another cashes in.
- **Devices: one slot per squad on a cooldown**, holding cards in the gun's own
  language. The current tools move into it.
- **Seeking cards and aim cards follow the simulation** (veins, scent, marks).
  Aim cards stay only if they help in play.
- **Luck changes rules, not just sizes:** rare rule cards, rare rolls that cross
  a threshold in the simulation, and run-wide relics.

## What is wrong today

The bullets a run sells do three real things (kill and leave a crater, throw
slime, put it to sleep), and the four sets inside each only nudge numbers you
can't see at a front ([today's code](reviews/r4-current-system.md), section 1):

- **A set only leans four numbers** (`bullets.rs`, `Set::lean`). Damage sets the
  hit's radius (`Ammo::reach`, the square root): slug 1.26x, needle 0.77x,
  buckshot 0.89x. Speed, range and spread don't show while the slime is already
  in reach. The rifle's base spread is 0.5 cells, so even buckshot's 2.5x is
  about one cell.
- **A plain bullet's hit stops growing at 1.4x** (`squad::R_MAX`, the GPU's
  strike box), so higher damage rolls on plain bullets do nothing.
- **Plain, blast and bomb all kill a disc and leave a crater, at three sizes.**
  The bomb's throw lands around the hole, which fills back in within about a
  second ([explosions](../explosions-2026-11-14/README.md)). Blast and bomb are
  drawn in almost the same orange.
- **The germ is never sold** (`run::shells_at`, `card_power`): "the GPU world
  casts none yet", but the GPU has cast it since 11-14 (`has_germ`).
- **Card order barely matters**: 8 of the 19 fixed cards (the flight and trail
  cards) apply to every bullet wherever they sit (`rounds::flight`, `trails`).
- **Nothing chains.** A cast goes off only on a slime hit; split pieces and
  turret shots carry none; no hook on expiry, a wall or a kill.
- **Effects never read what they hit.**
- **The rarest tier holds only the bomb and the germ.**

For contrast, Noita has 422 spells; of its 143 modifier spells 77% change
behaviour and 17% numbers only, and 14 of its 122 projectiles are "bigger
version" cards ([Noita notes](notes/noita.md), section 1).

## The owner's answers

| Question (11-16 ~03:20Z) | Answer (03:37Z) | What it means here |
| --- | --- | --- |
| A card that sets what squads aim at | "It is an interesting idea. If it is actually beneficial in game, then yes. I don't totally see the bigger picture of this yet but i am willing to explore." | Built as a test, kept only if it helps |
| Which families first | "which categories, you talked about a lot of things." (answered in the thread: cut, route, mind, body, force, readers) | Laid out below, Mind first |
| Combos in one gun or across squads | "Both." | Rule cards in the gun, and states in the world that others read |
| Explosives | "a device slot per squad. Cooldown." | One slot per squad; the device's cooldown is the only one |
| Rare rolls and relics that change rules | "Yes." | Rule cards at tier 3, threshold rolls, relics |
| Next step | "A full written proposal on how to explore all of this. Then decide if you need to dig deeper on an area before building do it otherwise start implementing/prototyping." | This report, then the code checks under *Next steps*, then prototypes |

Kept from earlier rulings: payloads are bullets (11-14 18:05Z), so a family's
effect is a bullet's shell, not a separate card; no recharge and free firing
(18:05Z, 18:38Z); nothing labelled lucky (18:18Z).

## The new shape of a gun

A gun keeps its rolled body and reads its cards left to right, forever. Three
things change: order matters for every card, a card can make a bullet carry the
next ones, and one socket outside the slots sets what an auto squad aims at.

| Kind | What it does | Examples | Today |
| --- | --- | --- | --- |
| Aim (own socket, one a gun) | What the squad shoots when the player isn't steering it | thinnest vein, feeding point, sleeping slime | the densest slime in sight |
| Bullet | Its shell is the effect (one of six families, or a reader); its set is its shape; it carries the rolled numbers | a sleep needle, a cut slug, a lure buckshot | five shells, sets that only lean numbers |
| Delivery | How the next bullet flies and lands | lob, bounce, seek, line, ring | pierce, split, scatter, wide |
| Reader | A bullet that cashes in a state already on the slime and uses it up | shatter sleeping slime, pop marked slime | none |
| Rule | Changes how the gun reads its cards | trigger, timer, multicast, copy, always | delay and thick (timing only) |
| Always (trails) | Rides on every bullet | scorch, bait, mist, smother | as today |

**How a gun reads them.** The cards split into groups, each a run of delivery
and rule cards ending in a bullet. Every shot fires the next group. A trigger in
a group makes that bullet carry the group after it and release it where it
lands; a multicast fires the next two or three groups as one volley. Delivery
cards apply only to their own group, so the same cards in another order are
another gun.

The doc's diagram: an aim socket feeds group 1 (trigger + sleep needle), which
carries group 2 (shatter slug) and releases it where it lands; the gun's own
cycle is groups 1 and 3 (lure buckshot). Each puts a state on the slime
(asleep, marked, sick, scent, craters) that another squad (aim card on sleeping
slime, shatter bullets) or a device (a mine holding shatter, set off by a surge)
reads.

**Why categories stop being samey.** Two bullets of one family stack with
diminishing returns; bullets of different families combine into a different
outcome. Damage lives on the bullet's roll only, and no modifier multiplies
kills.

## The six families

Each family changes a different part of the simulation, so each looks different
by construction. Candidates come from the four reviews; none is built or tuned.

| Family | What it changes | What you see | Candidate bullets | Today |
| --- | --- | --- | --- | --- |
| Cut | Where walkers die, in thin shapes; energy sharing | A dark hairline, then the far side fades and panics | Hairline (a 1-cell kill line across a vein); Tourniquet (a band where energy stops flowing, nobody dies); Furrow (crater along the flight) | scorch trail |
| Route | Trail, food scent, soldier scent, alarm | The slime reroutes | Road (trail along the flight); False feast (food scent at the hit); Repellent; Decoy (soldier scent: surges charge empty ground) | bait, smother trails |
| Mind | How walkers sense, for a few seconds | The network's texture changes | Blind (sensors off: straight walks into walls, water, fire); Sour (trail-following flipped: fog, then knits back); Spin (rings that stop advancing); Lace / rope (fine weak mesh, or a few thick cords) | none |
| Body | Stored energy and walker state | Brightness and density change, tints spread | Drain (energy without killing: pales then withers); Mark (a tint that spreads as walkers divide, for readers); Sicken (the germ); Overfeed (forces lumps that break away) | germ, unsold |
| Force | Motion | Arcs, funnels, piled rims | Fling (today's bomb); Pull (a knot for a follow-up); Shove (pushes the front back along the shot line) | bomb |
| Element | The ground: fire, moisture, plants, gas | Fronts across the map | Igniter; Wet / dry (a firebreak or tinder); Seeder (a dry fuel line); Gas | gas, blasts light plants |

**Readers** are the payoff bullets: quiet setup, loud payoff. Shatter kills
sleeping slime connected to the hit; Detonate pops marked walkers wherever they
have flowed; Grounding kills flung walkers that land on scorched or wet ground;
Conductor sends a charge along the veins, stronger on wet slime. A reader fires
as a weak plain shot when its state isn't there, and uses the state up when it
is.

**Mind is the family only this game can have.** Jones's model has sharp pattern
changes as sensor and turning angles shift (fine mesh, coarse cords, spots), so
a Mind bullet's look comes straight from the walker rule
([simulation-first](reviews/r2-simulation.md)).

**Cut goes last.** The owner, 11-16 03:44Z: "so far this mechanism hasn't
worked out too much in play because there are always crystals all over and
that is where the slime so concentrated, so I don't often find a large group of
slime that can be separated ... I haven't given up on this feature yet. My only
point was that this may not be useful." Before any Cut card is built, count
how often a run planet has a part worth cutting (the mass one thin cut would
leave without food). Route, Mind, Body and the readers work on slime wherever it
sits, so they come first.

## Rule cards

The trigger first. It was in the owner's 11-14 15:06Z notes ("trigger next card
on hit"; [gun budget](../gun-budget-2026-11-14.md)), waited on what a bullet
type is, and was never built ([today's code](reviews/r4-current-system.md),
section 3). Noita's run-breakers are rule cards like these, never base damage
([Noita notes](notes/noita.md), section 7).

- **Trigger:** the bullet carries the next group and releases it where it lands.
  The child's set picks the release (buckshot fans 4 pieces, needle sends 1
  straight on, slug lodges it). The child pays its own budget and slot.
- **On-expire:** a bullet that runs out of range or hits a wall releases too.
- **Timer:** releases after a set flight time (today's Delay becomes this).
- **If-state:** releases only if the hit holds sleeping, sick or marked slime,
  else fires plain.
- **Multicast:** the next two or three groups as one volley.
- **Copy:** repeats the group before it without its cost. Rare.
- **Always:** one group glued onto every shot. Rare.

In code: `Cast` gets a child, split pieces carry casts (`Game::landed`), expiry
gets a hook; no WGSL, pieces are ordinary legs under `SHOTS_MAX`. First the
GPU's kill read-backs need batching (*Caps*). About 400 lines with tests (code
reviewer's estimate).

## Seeking cards

Only seekers that follow something in the simulation: soldiers already aim at
the densest slime, so plain homing lands where the bullet was going anyway.
Fly straight first, then seek: spores that homed from launch flew back to their
own crystal (`Reports/dead-ends.md`, `spore_reach`). Noita ships about ten
steering cards, each a different steering rule ([Noita notes](notes/noita.md),
section 3).

- **Shrapnel seek:** trigger and split pieces each seek a different patch not
  struck yet.
- **Corner seek:** curves round a wall's end into slime no soldier can see.
- **Scent hound:** climbs food scent upstream to the crystal the slime feeds on.
- **Vein rider:** steers by the slime's own three-sensor rule along a vein.
- **Chaser:** breakaway lumps, panicking fragments, flung walkers.

The first two need only `World::mould_grid`, which both engines have. Scent hound
and vein rider need a trail or scent sample in the GPU readout; chaser needs the
flung mark in it.

## Aim cards

The squads the player isn't steering shoot the densest slime they can see
(`Squad` aim); the steered squad is aimed by the player's volley, so aim cards
matter for AUTO squads and for the steered squad's defend shots. All four
reviewers named targeting as the biggest gap unprompted: against a network the
strong moves are to cut it, starve it or hit where it feeds, and auto-fire at
the densest spot does none of them. With a cut, withering and panic do the
killing.

Candidates, each falling back to the densest slime after a second with no
match: thinnest vein in reach; feeding point (slime on a crystal); growth tip;
in a state (sleeping, sick, marked, gassed); healthiest (away from the plague);
guard (nearest the player's pumps and buildings).

Test: the same gun on densest against feeding point, then against sleeping
slime paired with a reader, 10 paired seeds, 120 s, filmed side by side;
deciding measure slime mass at 120 s and crystals held. Thinnest vein waits on
the cut count (*The six families*): in the owner's play the slime concentrates
on crystals spread all over, so there is rarely a big piece to cut off (11-16
03:44Z, "this may not be useful").

## Devices: one slot per squad

Each squad carries one device on a cooldown: a body holding one to three cards
in the gun's language, set off on the player's order, thrown or placed from
where the squad stands.

| Body | How it goes off | Replaces today's |
| --- | --- | --- |
| Grenade | Thrown to a point in range, bursts on landing | blast tool |
| Mine | Placed; bursts when slime crowds it | mine tool |
| Charge | Placed; all a squad's charges burst on Detonate | none |
| Line | Laid along a drag; bursts along its length | det cord |
| Canister | Placed; lets out its payload every second for a few seconds | sleep gas tool |

- Rolled like guns: throw range, cooldown, card slots and budget. The shop's
  Tools group sells bodies instead of unlocks; no bank cost per use.
- The cooldown is the device's alone; guns keep no recharge.
- Same cards, so devices join combos (a mine of sleep then trigger shatter).
- AI: waits for an order by default; each body has a free autocast switch with
  one rule (defend against a surge in reach, or cover a retreat).
- The toolkit (`mould::kit`) moves into `Game`, so device use enters replays and
  the player AI.

Precedents ([squad-game notes](notes/explosives-and-world-combos.md), section 1):
Deep Rock Galactic gives each class one throwable type picked before a mission,
each a different effect; XCOM 2 keeps fire and poison always on in the gun's
ammo and keeps where-and-when effects as items.

## Luck, relics and the shop

- **Rule cards at tier 3** (trigger, multicast, copy, always); today tier 3 is
  only the bomb and the germ.
- **Rolls that cross a threshold in the simulation**: sleep that outlasts the
  gas's decay makes a self-sustaining cloud; a crater that outlasts the knit-back
  makes a lasting wall; a Mind roll past a Jones pattern boundary turns mesh into
  spots. Nothing labelled lucky.
- **Relics**: rare run-wide rules ("fire spreads through sleep gas", "sick
  walkers lay double trail", "marked walkers let out gas when they die").
- **Shop levers**: a free lock carrying one offer to the next shop; rerolls that
  cost more each time; about a third of card offers from owned families and at
  least one from an unowned family; occasional extreme gun shapes; optionally a
  pity counter for rare offers (Slay the Spire's;
  [roguelike notes](notes/roguelike-builds.md), section 5).

## Caps and frame cost

Chaos should live in states and fields (a fixed cost a frame), not in bullet
counts.

- **Needed first:** `GpuWorld::burst` waits on a read-back for every killing
  burst (`src/gpu/larder_world.rs`, the `struck()` call), so chains multiply
  stalls; batch the read-backs into the frame (ROADMAP L-K already lists it).
- **Chains:** depth 2 (3 with a rare card); at most 4 pieces a carrier; about 40
  live sub-shots a squad; the 256-bullet cap with its drop counter; about 32
  killing bursts a frame, the rest waiting a frame.
- **States** are bits on walkers; fields saturate per cell; readers use up
  their state.
- Every prototype quotes its frame cost on the GTX 1070.

## What else we missed

- **The slime should adapt**: real *Physarum* habituates to a repeated bitter
  stimulus (Boisseau, Vogel and Dussutour, 2016); per-slime habituation to the
  verb you keep firing stops builds converging (slime-enemy thread).
- **Turrets carry no cards** (`shots.rs`): a turret deck, mortar-like.
- **Field-writing buildings**: bait towers, repellent fences, sprinklers.
- **Defence is half a build**: wet ground round pumps, repellent, mines.
- **See a card before buying**: a short live preview in the shop.
- **Rewards for shape, not kills**: "cut it into three", "keep it off this line".
- **Backfire as part of the spread**: flung slime seeds growth, cut-offs charge,
  fire reaches pumps.
- **Readability**: one look per state, quiet setups and loud payoffs, tracers in
  the squad's colour.
- **A bigger direction**: the player's own mould as the weapon
  ([own mould](../../wiki/world/own-mould.md)).

## How to explore it

Each step is a pull request that lands working, tested and filmed on an Artifact
page with counts; no census gates any step (CLAUDE.md, build first). A, B and E
can run side by side.

| Step | What gets built | What you see | Deciding measure | Size |
| --- | --- | --- | --- | --- |
| A. Fix the sameness | A shape per set (slug: wide hit plus a shove; needle: a line of 3 small hits; buckshot: a cluster of 3-5; standard as today); blast holds ground, bomb throws and kills little, own colours; the germ in the shop | The gun range's 5 x 4 grid, 3 s a card on one front | Can the owner name the set from the clip, labels off? | Small |
| B. Rule cards | GPU read-back batching; trigger, on-expire, cast-carrying split pieces; multicast; order mattering for every card | trigger + bomb + gas against bomb + gas, 10 s | Bursts a second, slept and flung, 1070 frame time | Medium |
| C. Seeking and aim | Shrapnel and corner seek; aim cards feeding point and in-state (thinnest vein after the cut count) | split + seek beside a wall's end; densest vs feeding point | Distinct patches a shot; slime mass at 120 s, 10 paired seeds | Small to medium |
| D. The families | Mind first, then readers and marks, then route, element, force; cut last, after the cut count | Each status stamped on one network for 10 s, blind | Can the owner name the status from the clip? Frame cost | Medium each |
| E. Devices | The slot, five bodies, cooldowns, autocast switch; kit into `Game` | A mine of sleep then shatter across a surge | Surges reaching the squad, slime killed, sub-shots dropped | Medium to large |
| F. Luck | Rule cards at tier 3, threshold rolls, relics, shop levers; habituation to the slime thread | A run's shops; one lucky build filmed | Threshold crossings in 1,000 rolls, aimed at 3-8% | Medium |

Other threads: the gun-look thread owns hit looks (new effects go to them before
view.wgsl changes); the slime-enemy thread owns engine rules (Mind statuses,
habituation); the roguelike thread owns the shop layout.

## Next steps

No area needs more outside research. Three code checks first, because they size
steps B and D: a spare bit or word on GPU walkers for a timed status; batching
the GPU's kill read-backs; a cheap thinnest-vein finder on `mould_grid`. Then
step A as the first PR, filmed, and the trigger card after it.
