# Review r2: bodies built from the model's own rules

2026-11-16. Independent answer to `../brief.md`, from the simulation side:
how a sub-population holds together and travels, how it senses and picks
targets, how its attacks are field writes or walker rules, how it is born and
ends, and what each part costs a frame. Sections 1 to 6 were written from the
brief alone; only the feasibility check at the end looked at code.

## 1. Ideas, generated before judging

What it is in walker and field terms; what it does; what the player sees.

1. **Slug.** A few hundred walkers converted into a "body" species with its
   own trail, crawling as one lump by avoiding ground it has just covered, to
   root on a far crystal as a new hub. A glossy lump with a fading track.
2. **Tracker.** A slug that reads a footprint field soldiers lay as they
   walk, plus alarm. It follows the squad's route, not its scent.
3. **Tick.** A lump that climbs pump scent, sits on a pump at a raised crowd
   limit and takes the pump's output into its energy. It swells, turns
   bronze, and buds a slug when full.
4. **Phalanx.** Dense walkers sort to the side facing the alarm gradient,
   light ones away from it. A bronze crescent faces the guns; flank it.
5. **Marching band.** Long-crowded walkers switch to a phase that turns toward
   the heading neighbours last wrote into a heading field. A crescent front
   sweeps the map as a wave, not a lump.
6. **Mill ring.** Walkers that deposit offset to one side follow each other
   round a loop; with a building's scent at the centre the loop closes on it
   and tightens. A turning ring that grinds what it holds.
7. **Engulfer.** On touching a building, walkers switch to wall-following and
   flow round both sides until the ends meet; damage scales with walkers in
   contact. The building vanishes into the lump; soldiers can cut it open.
8. **Sapper.** A lump that packs against a wall standing between it and pump
   scent, at a raised crowd limit, multiplying wall damage. A bite eaten out
   of your wall.
9. **Fruiting tower.** A mound at a crystal that stacks walkers, then fires
   spores ballistically over walls at the pump-scent peak. A structure with a
   fuse you can watch fill.
10. **Herald.** Writes alarm or food scent where it stands, so the main
    slime's veins bend after it. It leads a raid.
11. **Heart.** A still node whose pulse field sets when nearby slime steps, so
    surges arrive in time. Kill it and the rhythm breaks. An organ.
12. **Artery.** Dense walkers locked into a straight cord between two crystals,
    long step along it, carrying energy fast. Cut it and downstream withers.
13. **Hydra.** Cohesion judged per piece: a cut piece above quorum stays a
    body, one below reverts. Shooting it in half makes two.
14. **Pack.** A slug that splits into four lumps repelled by each other's
    trail and drawn to one target, so they encircle a squad with no planner.
15. **Lash.** A lump that throws a thin tendril (narrow sensors, long step) at
    a soldier; when the tip bites, the news runs back and the tendril reverses.
16. **Snare.** A thin dormant mesh across a path. Soldier scent wakes it and
    its walkers switch to short sensors, so it contracts onto the squad.
17. **Blister.** A lump that hoards energy beside a building until it bursts,
    flinging its walkers outward with a puff of gas: new slime inside the base.
18. **Smoker.** Emits the gas cloud as it walks and keeps upwind of the base;
    soldiers in gas lose range.
19. **Cocoon.** A dense lump goes dormant and hard, stores energy, and hatches
    later as a larger body. Its swelling is the countdown.
20. **Cannibal.** A species that converts the main slime on contact and grows.
    It eats your enemy and becomes your enemy.
21. **Tide.** Not a creature: food news oscillates between two halves of the
    map, so mass shuttles back and forth every forty seconds. The front
    breathes.
22. **Ripple swarm.** Walkers reverse at intervals, more often on a building,
    so counter-running waves pass through each other over it.
23. **Ambush line.** A dormant strip along the route soldiers walk most (the
    footprint field), waking under the squad.
24. **Wildfire front.** Moves by dividing at its front and dying at its back.
    Retreat doesn't escape it; only a cut does.
25. **Amoeba.** A gel skin of still walkers round a sol core that flows forward
    and gels at the front. A hit ruptures the skin, specks spill, it re-skins.
26. **Twins.** Two lumps joined by an energy-sharing vein. Cut the vein first.
27. **Crown.** A thickening ring of dense walkers round a held crystal. A
    fortress; holding it raises the slime's brood level (section 3).
28. **Spore front.** Spores carried on the gas drift, landing as a band of new
    growth. Weather.
29. **Throb** (a modifier). Walkers carry a phase coupled to neighbours; sensor
    reach shortens in half the cycle, so the body really contracts and relaxes.
30. **Gardener.** Writes plant growth round the base so the existing fire can
    reach buildings. A trap you can burn early.

## 2. How they arise, and how they end

**What makes one appear.** Not a die rolled per minute: every walker has a
small chance per step of becoming a *founder*, growing with how far its own
state is past a threshold. That is nucleation: random in time and place, but
its rate is read off the dish, so the player can move the odds. The states
are local:

- *Plenty*: time fed on a crystal (the dense-walker clock).
- *Crowding*: a per-walker running average of trail at its cell.
- *Famine*: time without food news. Social amoebae aggregate when starving,
  so cutting the slime off, the player's best move, breeds a slug that goes
  looking for food.
- *Wounds*: the scar under the walker.
- *The player's builds*: founders near walls, pumps or turrets pick a
  matching verb (section 3).

The planet picks which body plans its slime can make; a cap on bodies alive
and a cooldown, both event counters, keep them rare.

**The forming moment is the warning.** A founder lays the body species' trail
in pulses; main-species walkers that sense it convert (the existing species
conversion) and climb toward it. The player sees a spot of new hue in the
veins; the hue spreading as conversion passes neighbour to neighbour;
converted walkers streaming inward, so the veins round it thin and a star of
streams converges; a mound pulling tight with a bright rim; then lift-off,
leaving a hole in the network and a track. Ten to twenty seconds, with
streams thirty to sixty cells long, so it reads across the screen at play
zoom. The most striking call is an excitable relay: a walker that senses a
pulse re-emits it and goes refractory for a few seconds, which gives the rings
and spirals of aggregating amoebae, and walkers step toward the source only
on the wave front, so the streams move in visible bursts.

**Size without a global count.** The founder holds K tokens. Converting a
neighbour hands it half the converter's tokens; a one-token walker cannot
recruit. The body never exceeds K+1 walkers and nothing is counted. K comes
from the trigger and the brood level.

**Getting there early.** Shots on the mound kill token holders, so
recruitment stops sooner: a smaller body, not a cancelled one (small or none
at the first spot, most of it at the mound). On the tier that floods to
alarm, shooting draws in more slime to convert, so thrown tools serve better.

**How it ends.** Body walkers leave with energy and burn it faster than
ordinary slime, and keep a "body memory" that rises with nearby body density
and decays otherwise.

- *Broken*: pieces below quorum lose body memory and revert to main slime,
  which then panics. You break a body; you needn't shoot every walker.
- *Starved*: energy runs out far from food and the densest core goes dormant
  as a hard lump, a sclerotium, that wakes on scent or alarm. Spend shots on
  it or leave it.
- *Merged back*: a body low on energy reads the main trail positively, walks
  home and converts back on contact, carrying its energy in. A raid coming
  home with loot.
- *Settled*: a slug on its crystal reverts there as a new hub; a tick buds; a
  tower fires and slumps into ordinary slime.

A dormant lump counts as mass, so it must stay reachable and killable or a
planet can sit above 1% for ever: a smoke check.

## 3. Structure and variety

Three small sets, combined: a **body plan** (how it holds and moves; this is
the engineering, so about five), a **verb** (what it does on arrival; a field
write or contact rule, so many), and an **origin** (what made it, which picks
the verb, so cause and effect read). Two things carry across the run: tier
inheritance and a lineage.

### Body plans, and what holds each together

The break-away streams because every walker climbs the same scent on its own
and nothing makes the front wait for the back. The two tried rules fixed that
and failed only because they were applied to the whole slime. Given to a separate body species, "separate tight crawling lumps" and
"still droplets" are what a stronger enemy should look like. What the droplet
lacks is the front-back difference the published model needs before a
droplet migrates, and the body can make that itself.

The **lump kit** (slug, tracker, tick, sapper, phalanx, pack):

1. A private trail field, one shared by all bodies rather than one each, with
   short sensors (two or three cells), a wide sensor angle and high deposit,
   so the body species forms one blob.
2. The touch-step rule for the body species only, so no speck can leave.
3. A **wake**: each body walker writes the current step number into a "last
   covered" field and avoids cells covered fewer than W steps ago. The rear
   stands on its own wake and is pushed; the front stands on fresh ground, as
   real plasmodia avoid their own extracellular slime. A timestamp decays
   implicitly, so there is no per-cell pass.
4. Log sensing of the target: compare ln(1+s) at the three sensors, so a weak,
   distant scent still gives a clear front-back difference. Otherwise a
   fifteen-cell body sees almost no gradient of soldier scent eighty cells
   off.
5. Optionally a second long sensor on its own trail at low weight, to pull
   split lumps back together.

Two bodies that meet merge into a bigger one. Keep it: a reason to kill them
apart.

**Band.** Crowding past a threshold makes a walker gregarious: it writes its
heading into a heading field (last writer wins, no atomics) and turns
part-way toward the heading at its sensor cells as well as the target. That is
a grid flock, which forms travelling bands at middling density: a front, not a
lump. Headings carry a timestamp so stale ones are ignored without a pass.

**Ring**: deposit offset to one side, a target at the centre; each walker
follows the one ahead round a loop. **Net**: long sensors, low crowd limit,
dormant; it contracts by switching to short sensors. **Mound** (tower, crown,
heart): a raised crowd limit at a point and a call, any extra field windowed
round it. The **amoeba** (keep-heading gel skin, lure-following sol core) is a
second lump design: more striking, harder to tune.

**Throb** fits any plan: a per-walker phase nudged toward the phase neighbours
wrote, sensor reach scaled by it. A phase wave running rear to front gives a
peristaltic crawl, and the body beats because its walkers really draw in.

### Verbs, origins, tiers

Verbs: bite (exists); smother (a building under N walkers stops working);
siphon (pump output into body energy); sap (raised crowd limit at a wall);
shield (dense walkers sort toward alarm); launch (spores at a field's peak);
gas; call (write alarm or food to pull the main slime); split; burst.

Origins pick the verb. Famine makes a colonist that seeks a crystal; long
feeding a tick or phalanx; wounds a tracker. Founders near walls make
sappers, near pumps ticks, near turrets phalanxes. The player learns "I walled
off the north, and the north made a sapper".

A body species is the tier's settings plus a body-plan change. The droplet
tier's bodies hop by flinging; the thin mesh makes nets and lashes; thick cords
make arteries; the flooding tier makes heralds; the spore tier's bodies burst
and split; the sleeper makes ambushes and cocoons. Each planet gets its own flavour from
one code path.

### Lineage, and what you win or lose

A planet-level brood level rises by events: plus one when a body finishes its
job (settles, buds, gets home), plus one per crystal held a minute. Each level
raises K and adds a modifier from the tier's list. That is "keeps getting
stronger by holding crystals", and killing bodies early visibly stalls it.

Kill one and its stored energy drops as a short-lived residue a squad can
drain like a small crystal, so a tick fat with your income gives it back.
Ignore one and you lose income, face a new hub or a breached wall, watch the
slime grow when it gets home, and raise the brood level.

### One thing about the soldiers

Soldiers pick their own targets, so a body's danger depends on whom they
shoot. If they shoot the nearest slime, a body escorted by ordinary slime is
screened for free, and a phalanx is trivial or maddening by that one rule.
Settle soldier target priority (prefer body or dense walkers in range, or a
focus order on attack-move) before tuning any body.

## 4. Full bosses

**For.** Eight rising planets want a last beat, and the shop needs something
to build toward. A whole slime transforming is the strongest clip the game
could have.

**Against.** Each planet's slime is already the boss: the win is a mass
threshold, not a kill, and a separate boss either replaces that rule or sits
awkwardly beside it. Designed bosses pull toward scripted phases, drawn
effects and a yes-or-no outcome, all ruled out. A one-off costs build time,
and frame time on the last planet, already 12 to 14 ms.

**Pick: for, as the slime itself.** On planet 8, once the player has cut the
slime below about a quarter of its peak, the remaining mass aggregates across
the map (the famine trigger at full scale) into one migrating body carrying
every modifier the planet's lineage earned, and heads for the base. Breaking
it to 1% is still the win, its pieces still revert below quorum, and it uses
the mini-bosses' body field, so it adds no walkers and no field. Planet 4 or
5 can rehearse it with an elder: a mini-boss with no lifetime cap.

## 5. The owner's first thought

**What it gets right.** It comes from the simulation's state, not a timer. It
is a phase change, like the condensation the slime already does. It leaves
and comes for the player, the pressure the owner wants. It asks for variety in
behaviour, attack and look, not numbers.

**What I would change.** Make the chance a nucleation rate set by how far the
local slime is past its threshold, so timing is random but the odds are
readable and movable. Give the sub-unit its own species and trail, which is
what turns specks into a body. Let it seek the player's things by kind as its
verb dictates, so its heading says what it wants. Add famine beside
concentration. Get variety from plan, verb, origin and tier rather than a list
of bespoke monsters.

**Beside what exists.** I read "the condensation event" as the existing
turn-dense-then-break-away step. Keep the turning dense: dense walkers are
what bodies are made of, and bronze is how a body reads as stronger. Replace
the break-away's speck stream with a slug: it already picks a far fresh
crystal, it just doesn't hold together. Surges stay as the bodiless local
aggression, and a body near soldiers can set one off. Raids stay periodic,
and a herald can lead one. One pressure budget: a body forming in a region
delays the next raid from it, because it spent the mass.

## 6. Ranking, with a first cheap experiment each

Shared smoke checks: idle doesn't win, no planet ends inside a minute, no
dormant lump strands a planet above 1%, and each new per-step rule's frame
cost is quoted.

**1. Slug, on the lump kit.** The base every lump reuses; it replaces the
break-away and answers cohesion directly. *Build*: body species with private
trail, touch-step, wake and log sensing; seed 500 body walkers as a disc on an
empty dish with a crystal 120 cells away, then on a dish with the main slime.
*Film*: the crossing at play zoom. *Count* every 30 steps in the probe: share
of body walkers within 1.5 start radii of their centre, aspect ratio, pieces,
speed. *Works* if one piece keeps at least 80% with
aspect ratio under 2 and arrives inside a minute on 10 paired seeds, and
turning the wake off stops it (sign test). *Fails* if aspect ratio climbs
(streaming) or speed falls to near zero (still droplet). *Cost*: per walker, a
few reads; per cell, one trail pass like a species field's; global, none.

**2. Tick.** It attacks what the player builds and grows by holding what the
player holds. *Build*: lump kit aimed at pump scent; on a pump, raised crowd
limit and the pump's output into body energy; bud a slug at a threshold.
*Film*: arrival, swelling, budding. *Count*: income lost a minute, time to
first bud, shots for one squad to clear it. *Works* if the income drop shows
within 30 s and clear time is a spread across seeds. *Cost*: per walker, plus
a per-pump check.

**3. Phalanx.** It changes how you move squads, not how much you shoot.
*Build*: in a body, dense walkers read alarm positively and light ones
negatively. *Film*: one squad fires from the west, a second flanks from the
north. *Count*: body walkers killed per 100 shots from front and flank on
paired seeds, and the crescent's turn time. *Works* if the front costs about
twice the flank's shots and the turn takes 5 to 15 s, a usable window. It
hangs on soldier target priority. *Cost*: per walker.

**4. Fruiting tower.** The best transformation clip, the non-creature, and it
reaches over walls. *Build*: founder at a crystal, relay call in a window,
raised crowd limit at the centre, spores launched at the pump-scent peak after
a fuse. *Film*: rings, streams, mound, volley. *Count*: time from first pulse
to launch, share of nearby slime recruited, spores landing behind walls,
volley size when attacked at each stage. *Works* if forming reads as one event
in at least 9 of 10 seeds and an early attack gives a smaller volley, not none
or all. *Cost*: one windowed field while forming.

**5. Marching band.** A different shape and verb: a front that sweeps, not a
creature that hunts. *Build*: crowding memory, gregarious switch, timestamped
heading field. *Film*: a crowded fed region turning and sweeping out. *Count*:
bands formed, their size, how long they last, and alignment (polar order).
*Works* if a band travels 100 cells as one front; *fails* if alignment only
makes specks or the whole slime flocks. *Cost*: per walker, one write and
three reads.

Next in line: the pack (a slug modifier), the mill ring, the engulfer.

## Feasibility check (read after writing the above)

From `BigParams` and `BigDish` in `src/mould/big.rs`.

**Slug.** Most of the kit exists. `species` with `species_sets` gives a body
species its own trail (`BigDish::species_trail`) and its own sensor angle and
offset, turn, step, deposit, keep, `crowd` and `blocked_deposit`. The wake
already exists as grazing: `graze_weight` (per species in `SpeciesSet`),
`graze_eat` and `graze_regrow` make "a body eat the ground under it bare and
lean toward fresh ground", added to make bodies roam; it costs a per-cell
regrowth pass, which a timestamp would avoid. The touch-step rule is `tether`
with `tether_hold`; the front-waits rule is `lone_step` with `lone_floor`;
`body_pull` is my optional item 5; `blocked_keep` is the droplet rule. All
four are global `BigParams` fields, not in `SpeciesSet`, which is why they
reshaped the whole slime: making them per species is the one new piece, and
the cheapest experiment. The `lone_step` comment records that `body_pull` and
aligning headings did not hold a hunk together, so item 5 should stay
optional. Birth can reuse `species_convert` and `species_convert_r`, and
`species_cap` bounds the body species' births but not its conversions, so the
token budget is new. Targeting can borrow the break-away's crystal choice and
group departure (`roam`, `roam_far`, `roam_share`) and the walking-distance
`raid_field` round walls, but it should add the target to the trail reading
rather than replace it, as panic and roam do; that replacement is likely why
break-aways run as specks. `hunger_scent` and `scent_offset` cover part of log
sensing.

**Tick.** `crowd` per species, `fed_crowd` and `fed_full` give the swelling,
`crystal_eat` the take; pump income lives game-side.

**Phalanx.** The dense word (`BigDish::dense`, from `condense` or `thicken`)
marks the armour; `alarm_weight` is global, so a weight per dense or light
walker is new.

**Fruiting tower.** Spores exist end to end (`spore_rate`, `spore_burst`,
`spore_steps`, `spore_speed`, and `spore_reach` homing on scent in the second
half of flight). The relay call can start from the dormant crust's wake relay:
`alarm_relay` re-emits once and `sleep_rest` keeps it deaf for a while, which
is already an excitable medium.

**Marching band.** There is no heading field, and alignment for cohesion is a
recorded dead end: check its condition first (the band wants a front, not a
held lump).

**Triggers and ends.** Condensing's ripeness (`condense_crowd`, `condense_r`)
is the crowding memory; `news` and `wither_lag` give famine; `scar` gives
wounds; dormancy (`sleep_rate`, `sleep_death`) gives the sclerotium. Watch
the CPU-only notes: `species_avoid_of` and `species_convert_per` are not on
the GPU engine yet.
