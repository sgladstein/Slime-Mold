# Review r3: squads, orders and the gun

2026-11-16. Independent review from a squad-tactics and RTS lens. Read only
`brief.md`; nothing else in the repository.

## Up front: what "cards in a gun" misses in an RTS

Noita-style wands work because one player aims one wand. Every odd card
(bounce, trigger, homing) meets the player's hand on every shot. Here the
player never aims, so three things the wand framing takes for granted have
to be rebuilt somewhere else.

1. **The aim.** If every soldier shoots "the nearest slime in range", every
   gun is the same gun with a different splash. A build needs a card that
   decides *what the squad shoots at*: the thinnest neck of a vein, the root
   that feeds it, the sleeping blob, another squad's mark. Targeting is the
   RTS stand-in for the wand-holder's hand.
2. **The wand is the army, not the gun.** In an RTS the combos that matter
   span squads: one squad writes a state into the world, another reads it.
   If every synergy lives inside one gun's slots, the best army is six
   copies of the best gun. Use the simulation's fields as the bus between
   squads. It needs no linking UI, and all of it is visible.
3. **The player's moment.** A gun that fires forever holds no decision. RTS
   skill is position, facing and timing: where you stand becomes what you
   hit, and "hold, then volley" is a decision. Cards should give those
   orders teeth.

Also missing: the slime's *reaction* as the drama (cuts cause panic), a way
to *see* a build before planet 6, and the zoom. At 24 px a cell the screen
is about 80 by 45 cells, so long-range work lands off-screen. Builds read
well when an off-screen cause shows on-screen through the simulation, like a
root cut that withers the vein in front of you.

The structure in one line: **a gun is a selector plus a cycle; a device is a
delivery body with a trigger; both speak one card language; squads combine
through fields.**

## 1. Generate: 32 effects

Each line: what changes in the simulation; what the player sees.

**Cutting and supply**
1. **Neck-cutter**: targets the narrowest vein in range and kills a thin line across it (walkers, energy sharing). The vein snaps; the far side dims, then panics.
2. **Furrow**: the bullet scorches every cell it flies over (no-trail ground). A dark streak; where it crosses a vein, the vein can't regrow for a while.
3. **Tourniquet**: a band where energy sharing stops but walkers live (energy). The far side stops dividing and fades, with no gap to see.
4. **Root-seeker**: crawls upstream along the trail gradient to the crystal feeding this vein and bursts there (trail, food scent). The branch withers from its base.
5. **Vein-runner**: the kill travels along the vein it hits, cell by cell, forking with it up to N cells (walkers, trail). The vein unzips toward its tips.

**Steering fields**
6. **Bait**: a food-scent spike at impact. Veins bend toward an empty spot: the lure for a kill zone.
7. **Deodorant**: erases food scent in a disc. The slime's reach toward a crystal loses its way and splits.
8. **Highway**: a strong trail ribbon along the flight path. Slime pours along a rail you laid, into water, fire or a mine line.
9. **Repellent**: negative trail at impact. Walkers turn back at a soft wall that fades.
10. **Flare**: writes alarm. Fleeing species part, swarming species converge: one card, opposite uses by species.
11. **Scent mask**: removes soldier scent around the squad. A surge aimed at it loses its target and stalls.
12. **Taunt**: amplifies the squad's soldier scent. The slime commits to this squad: the anchor role.

**Walker states**
13. **Sleep puff**: a small drifting sleep-gas cloud. Walkers freeze and stop laying trail; veins fade while they nap.
14. **Infect**: a few walkers turn sick (contagion). Sickness creeps through dense parts; condensed lumps suffer most.
15. **Dye**: a mark the walker carries. The tint flows into the network, showing where the body is heading; a primer for other squads.
16. **Thin**: splits condensed walkers back into many weak ones. A hard lump becomes a soft film.
17. **Curdle**: forces condensing so a lump breaks away and runs for a crystal. With bait, you choose where it runs.
18. **Panic**: triggers the cut-off panic state now. Part of the body charges the nearest soldier: your kill zone, or your weakest squad.
19. **Overfeed**: pumps stored energy in (energy, growth). A bloom of division, then mass starvation, unless a crystal is near.

**Motion**
20. **Burst-fling**: throws hit walkers outward (flung walkers). Droplets live where they land; face the squad so they land in water or fire.
21. **Shove**: flings walkers along the shot line. The front steps back; facing decides where to.

**Fire, water, ground**
22. **Dryer**: lowers moisture. Plants pale; fire is primed.
23. **Wetter**: raises moisture. Dark damp ground; a firebreak round your pumps.
24. **Igniter**: lights plants at impact. Fire runs the dry plants and stops at damp ground, water and slime body.
25. **Seeder**: plants dry fuel, or a kind the slime avoids. Lay fuel lines now, light them later.
26. **Fan**: pushes existing gas along the shot line. Steer another squad's cloud down a corridor.

**Delivery and time** (game-side, in flight)
27. **Lob**: arcs over slime, lands behind the front. Hits the vein's base, not its tip.
28. **Bank**: bounces off rock and walls. Shots round corners into pockets.
29. **Capsule**: carries the next card(s) and releases them on impact (chain).
30. **Echo**: the impact repeats at the same cell after a delay, after the slime has refilled the hole.

**Readers (payoffs)**
31. **Burst-on-state**: if the hit cell holds sleeping, dyed or sick walkers, kill the connected walkers in that state, up to a radius. One shot clears a sleeping blob.
32. **Grounding**: flung walkers that land on scorched or wet ground die (fling, no-trail, moisture). The spray dies where it falls.

## 2. Structure

### The gun

Keep the owner's body (rolled range, rate, spread, bullets a shot, budget,
slots) and add two things: an **arc**, a rolled stat for how far off its
facing the squad may fire; and one **selector socket**, outside the slots.

### Six card kinds

- **Selector** (exactly one): where the squad shoots. Nearest, densest,
  narrowest vein, root, front tip, dyed / asleep / sick, or a ground point
  between squad and nearest crystal (for wall-writers). Falls back to
  nearest after a second with no match, so a squad never idles while eaten.
- **Bullet** (rolled): damage plus speed, range and spread multipliers. The
  only card that carries damage.
- **Delivery**: lob, bank, furrow, vein-runner, slow seeker. Changes how the
  next bullet travels; may override the selector (lob retargets the base).
- **Writer**: writes a field or walker state at impact. No damage.
- **Reader**: "if the cell holds X, do Y". A weak plain hit without X.
- **Capsule and timing**: capsule carries what follows; echo, keystone and
  hold shape rhythm.

### How a gun reads them

Cards group left to right into **shots**: `[delivery] [bullet] [writers,
readers]`, closed by the next bullet. A capsule swallows the group after it.
The cycle's last group is its **keystone**. The squad banner shows the cycle
as pips, so the player reads "three quiet, one loud".

### What stops a combo being "more damage"

- Damage lives on bullet cards only.
- Fields **saturate** per cell: a second sleep puff on one cloud adds area or
  time, never depth.
- Readers scale with how many state-holding walkers are **connected** to the
  hit cell. States spread over time and space (gas drifts, sickness creeps,
  dye flows), so the best payoff comes from a squad *elsewhere* that wrote
  the state *earlier*. A self-combo in one cycle works, but small.
- A reader **consumes** its state (burst sleepers are gone), so two readers
  don't stack on one setup.
- Writers and readers are cheap; capsules are dear and taxed by depth.

### Where luck enters

Body rolls, bullet rolls, writer magnitudes (radius, strength, duration),
capsule release count and scatter, interlock distance (below), and the shop
offer. The simulation adds its own spread: where flung walkers land, which
way a panic runs, whether thick slime outgrows a seeker.

### Squads combine through fields

Roles are not classes, just what a squad's cards do: **primer** (sleep, dye,
dryer, infect), **trigger** (readers, capsules of readers), **shepherd**
(bait, highway, flare, fan), **cutter** (neck-cutter, furrow, tourniquet,
root-seeker), **anchor** (taunt, wetter, repellent; holds a pump), **sapper**
(devices, 4c).

A reader fires on whatever state is in the world, whoever wrote it. Two
orders turn that into a plan: **Hold** (stop at the keystone and wait) and
**Volley** (selected squads release on one beat). Holding stores nothing, so
there is no recharge; it only aligns phase, and its price is the shots not
fired.

### Positioning is the aim

- **Facing**: drag at the end of a move order; narrow-arc guns fire only
  that way.
- **Line-of-fire geometry**: furrow, highway and shove use the flight line.
  Fire across a vein and a furrow cuts it; fire along it and you erase a
  length of it.
- **Range shape**: lob wants a wall in front, bank wants rock corridors, a
  root-seeker wants a vein rather than a front.
- **Bodies are fields**: soldier scent means where a squad stands steers the
  slime. Taunt and mask make that a build.

### Readable at RTS zoom

- **One field, one look**, whichever card wrote it. Players learn ten
  fields, not two hundred cards.
- **Setup quiet, payoff loud**: writes are faint and lasting; reader bursts
  are rare, large and sudden.
- **Rhythm**: keystones and volleys make peaks instead of a constant fizz.
- **Tracers in the squad's colour**: bullets are real state, so colour is a
  legend, not decoration.
- **Caps**: a per-squad limit on live sub-shots bounds the GPU and the eye.

### What the AI does when you're not watching

- The **selector** drives everything, with its fallback.
- **Self-harming writers** (igniter, burst-fling, panic, overfeed) fire only
  beyond an **interlock distance** from friendlies and pumps. It is a rolled
  stat: a short roll is a cheap card that lights the brush under its own
  feet. Luck as risk, and visible.
- **Readers** fire plain when the state is missing; a "patient" reader skips
  its turn until the selector finds the state.
- **Shepherd cards** write toward the selector's target, so a bait squad left
  alone baits toward itself: fine for an anchor, deadly for a weak squad.
- **Devices** never act alone unless autocast is on (4c).
- A **minimap ping** when a reader fires off-screen: UI pointing at a real
  event, not an effect drawn into the world.

## 3. Big picture

### Three planet-6 builds

1. **Starve it.** A cutter (narrowest-vein selector, furrow, neck-cutter,
   tourniquet) on a flank where its fire crosses the line from body to
   crystal; a root-seeker behind a wall; an anchor with taunt and repellent
   to catch the panic charge. The energy-sharing rule does the killing.
2. **Lullaby and hammer.** A primer (two sleep puffs, a fan) upwind in a
   corridor; a trigger squad beside it (asleep selector, capsule of
   burst-on-state); a sapper with charges in the gas pocket for the volley.
3. **Burn line.** A shepherd lays highways and bait toward a plant field; a
   primer dries it; an igniter on a long interlock; wetters ring the pumps.

### Do they differ?

Yes: by verb, by where squads stand, and by what the player *does* (flank
and wait; set and volley; herd and light). The risk is build 1: starving is
free damage, so a cutter could win everywhere. Species and maps should push
back:

- **Spore spreaders** relocate rather than starve: fire and infection.
- **Condensers** break away when cut, so a cut makes runners; cutters need
  curdle plus bait to choose the runway.
- **Alarm swarmers** turn a loud gun into a beacon: quiet builds, or a flare
  trap.
- **Sleep-until-disturbed** makes the first shot matter: volley builds shine.
- **The one that remembers where it was hurt** learns a build that hits one
  spot every cycle; varied selectors win.
- **Maps**: plants favour fire; rivers favour fling and shove into water;
  rock mazes favour bank and lob; many close crystals mean many roots and
  weak cutters; one rich crystal makes cutters kings.

### What is still missing

- **A preview.** A six-card build with a capsule is a gamble the player
  can't judge. Put a **proving dish** in the shop: a small live dish where
  the candidate gun fires for ten seconds at a stock slime. The picture is
  the data, so the preview is honest.
- **Defence.** Income is pumps and the slime eats them. Half of a real build
  holds a place (wetters, repellent, mask, mines); the framing is all
  offence.
- **Grading by reaction.** Panic, break-away and flight are the drama; judge a
  card by the reaction it causes, not its kills.
- **Squad identity across planets**: a name or veterancy, so the squad that
  rolled the absurd gun is worth keeping alive.

### Other directions

- **Doctrine as the build.** Guns stay simple; cards slot into a squad's
  rule list ("surge within 6: shove; sleepers in range: burst"). Very RTS,
  less loot joy.
- **Field engineering.** Buildings that write fields (scent towers,
  sprinklers, fans, sirens) are the main weapon and guns the clean-up: tower
  defence against a living network.
- **A tame strain.** A friendly culture of walkers that competes for crystals
  and lays a trail the wild slime avoids. The most simulation-native weapon
  there is.

Keep cards in guns as the spine; borrow doctrine (as device autocast) and
field-writing buildings (in the shop) as subsystems.

## 4. The owner's questions

### 4a. Chains: yes, depth 2 normally, 3 as a rare roll, never more

- **Every level must change verb.** A capsule of a capsule of bullets is a
  cluster bomb; the depth tax should make it a poor buy.
- **Conditional capsules** release only if the hit cell holds state X, so a
  chain grows where the world was set up (usually by another squad): a combo,
  not a fan.
- **Cost**: contents ×1.5 at depth 1, ×2.25 at depth 2; release count rolled.
- **Bound**: a per-squad cap on live sub-shots (say 40; six squads, 240,
  inside "a few hundred a frame"). Over-cap releases are dropped and count as
  the spread's bad tail.

At this zoom three stages is what a player can follow: shot, cloud, burst.

### 4b. Seeking: only if it seeks a field, never "the enemy"

Plain homing adds nothing, because soldiers already aim. A seeker earns its
place by reading a field the soldiers don't, reaching what the squad can't
see: upstream along trail to the feeding crystal; toward alarm, the freshest
wound; toward another squad's dye or sleep; along the narrowest neck; away
from soldier scent, round to the slime's back. Keep seekers slow and in the
world, so thick slime can outgrow them: an outcome with a spread, and a
reason to cut a path first. Seeking is how a squad behind a wall fights at
range, which is positioning again.

### 4c. Explosives and devices

**For keeping them separate**

- **They are the player's hands.** In an auto-fire game the thrown tool is
  where agency lives. Fold it into a squad and either the AI spends it badly
  or the player forgets it in a pack.
- **Readable.** A thrown thing is rare, big and player-caused; as "slot 4 of
  squad 3" it becomes noise.
- **The reliable lever against bad luck.** Gun builds are a lottery; a
  grenade you bought is the dependable answer. Card payloads put it back in
  the lottery.
- **Shop pressure**: consumables are the roguelike valve (stock up for
  planet 8).
- **Simpler code**: no payload runner without a gun body.

**For joining**

- **Position.** A global hand makes squads into turrets. A throw from a
  squad's own position and range means walking a sapper into danger: the RTS.
- **Combo depth.** A mine holding a conditional capsule is where the absurd
  chain lives, and the player placed it.
- **One language.** A card is useful in a gun or a pack; fewer dead offers.
- **Roles.** A sapper is a role you build, not a hotbar.
- **The lever survives.** The player picks place and time; only payload size
  is rolled.
- **Consumables get hoarded.** "Too good to use" is the standard failure; a
  device on a cooldown gets used.

**Pick: one card language, a different trigger, carried by the squad.**

- A device is a **body in the squad's pack slot** (one, upgradable to two).
  The body is delivery plus trigger: **grenade** (thrown to a point in range,
  on order), **mine** (placed; fires when walkers cover it), **charge**
  (placed; all of a squad's charges fire on Detonate), **canister** (emits
  its payload every second for N seconds).
- **One to three payload slots** take bullet, writer, reader, capsule and
  timing cards, never a selector (the player is the selector) or a delivery
  card (the body is).
- **A cooldown, not ammo**, on the device. Guns keep their no-recharge rule;
  waiting is the price only in the pack. If the owner wants consumables,
  sell restocks at a depot building, not counts in the shop.
- **The shop sells bodies** with rolled throw range, cooldown, payload slots
  and budget.
- **AI**: hold for orders by default; the banner glints when ready. Each body
  has a free autocast toggle with one rule: *defend* (surge within r),
  *follow up* (another squad's state on more than N walkers in range), or
  *cover retreat* (drop a mine on a retreat order). If autonomy cost a card,
  nobody would buy it and devices would die in packs.

A grenade-like shot inside a gun cycle (lob plus capsule) still exists, and
it is just a gun shot. The pack is for the moment you choose.

## 5. Top 5, each with a first cheap experiment

1. **Selector socket** (narrowest vein, root, asleep): cheapest and most
   structural; it gives auto-aim a build. *Experiment*: one squad, same gun,
   nearest vs narrowest-vein selector, one dish, 10 paired seeds, 120 s.
   Film both. Count slime walkers at 120 s (deciding measure, paired sign
   test), cut-off fragments made, and crystal energy the slime took. Smoke:
   idle doesn't win; the panic it causes doesn't overrun the squad.
2. **Cross-squad primer and reader** (sleep + burst-on-state, dye + burst,
   dryer + igniter). *Experiment*: two squads on equal budget as A+B, A+A
   and B+B. Count kills in 60 s and the largest one-second kill; A+B should
   beat both same-card pairs on paired seeds. Film A+B zoomed in: can a
   viewer tell who did what?
3. **Device bodies in the card language** (a mine with a conditional
   capsule). *Experiment*: three mines (sleep puff, then capsule of
   burst-on-asleep) vs three plain kill mines across a surge's path. Count
   surges reaching the squad, walkers killed, sub-shots dropped by the cap;
   film the surge hitting the line; quote the payload runner's frame cost.
4. **Hold and Volley with visible keystones.** *Experiment*: three squads
   with 6-card cycles, free-running vs volleyed. Count kills per second and
   its peakiness (largest one-second count over the mean). Film both and
   show the owner a blind A/B: which reads better?
5. **Proving dish in the shop.** *Experiment*: a 64 by 64 live dish, stock
   slime, each of 10 cards firing for 10 s. Record GIFs and run time. Give
   two testers five GIFs and five card names; count correct matches. A card
   they can't match isn't distinct by look, so this also tests the brief's
   main goal.

Runners-up: soldier scent as a build (taunt and mask, the anchor role), and
furrow geometry driven by facing.
