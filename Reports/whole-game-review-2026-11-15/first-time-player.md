# Whole-game review, first-time player (2026-11-15)

Role: a player who has never seen Larder World. I played in order through
captures on llvmpipe (`larder-world look=DIR`, main at 17f2e5b8): the
start menu, campaign mission 1, squad mode on the Fork, then the run's
planets 1, 2, 3, 4 and 6 and the shop, plus the gun range. About 40 stills
read; 15 capture runs, none longer than about 2 minutes of wall time. No
census. Scratch is in `/tmp/wgr/first-time-player/`; six key stills are in
`first-time-player/` next to this file. One seed (run seed 1) throughout,
so every count here is one game, not a spread.

## The game as a whole

What the game is today, in the order a newcomer finds it, is two games
that never meet. **The Fork sandbox** is a horror game: 25,000 walkers,
and the slime reaches the larder and eats the bank by 0:30, while four
soldiers who walk in are dead by 0:15. **The run**, which the owner calls
the real game, is an economy game: the slime lands at 3,000 walkers,
drops to about half on its own in the first minute, and then sits round
its home crystal for as long as I watched (5:00 on planet 1). Nothing it
does touches your crystals, pumps or squad. So the plan the owner
describes (drain the biggest crystal, buy to 40, march in) is not a
player's failure of imagination. It is the only plan the run asks for.
You build an army against an opponent that has stopped playing, and the
strongest weapon is time.

The simulation itself is the best thing on screen. At close zoom the
slime is rich, readable and alive: shots carve a hole you can see, a
blast throws chunks, the edge glows where it is hurt. Where the slime is
big enough to move (the Fork, or planet 1 given 8,000 walkers), it
reaches for crystals and runs a cord at you, and the game becomes a
fight over the map. The approach is right. The run is built on a slime
too small to use it. The first fix is to make the run's slime act like
the Fork's slime (findings 1-4). After that come the guns: the rifle you
start with is the least exciting thing in the game, and the blast you
could have is one of the most exciting (finding 6).

## Diary (by game minute)

**Start menu.** The title reads "A mould that grows, feeds and fights back"
over a live Fork round, which is a good hook. **Sandbox** is highlighted
first, and "Free play on the map, with today's defaults" sounds like the
easy start. **Run** is second and **Campaign** ("8 missions, one new verb
each") third. As a newcomer I would click Sandbox (Enter), or Campaign
for a tutorial. Nothing says that Run is the main game.

**Campaign, mission 1 Landfall (Flats), 0:00.** Goal line: "Grow your
squad to 12 and harvest 3 crystals at once". I understood it in about 10
seconds. The bank is 0 and Buy soldier (400) is greyed, so I would look
for Harvest crystal (G). At zoom 6 there is no slime on screen at all,
just soldiers and blue crystals (`m1/flats-idle-001s-near.png`). The
slime is a triangle on the far right of the map, 5,000 walkers.

**0:15-0:40, idle.** The slime breaks up into specks and thin trails
toward nearby crystals. Mould 5,024 at 0:15, then 1,891 at 0:40, with
only 4 killed by my squad. It shrank by itself, and I couldn't tell why.

**Same mission, farming (`hand=farm`).** 307 food at 0:30, 8 soldiers at
1:30. Goal met at **1:53, three stars**: 12 soldiers, 4,237 food, 105
walkers killed. The slime never came near. Lesson learned: ignore the
slime and farm. That is the plan the owner finds boring.

**Squad mode, the Fork, 0:00-1:30 (`hand=squads`).** Two squads of four.
The slime fills the Fork's stem, and by 1:30 it has a network of thick
bodies on five crystals. My squad camps on a crystal near the larder and
fires. Mould 24,839 at 0:02, 22,246 at 0:30 and 20,564 at 1:30 (4,453
killed). Here the slime is clearly the opponent. It is everywhere, and I
could see the bodies I would need to take.

**Sandbox Fork, as a newcomer would play it: four soldiers straight into
the slime (`hand=rush`).** Squad health 48% at 0:05, one soldier left at
0:10, none at 0:15. By 0:30 the slime is at the larder ("The mould is at
the larder, eating the bank") and the bank drains 2,000 to 1,533. That is
exciting and scary, but a newcomer's first minute ends with no soldiers.

**Gun close-up (zoom 10, Fork rush at 0:04).** The rifle's shot is a small
orange oval at the barrel. I couldn't see a bullet, only the slime
retreating round the squad, which leaves a clean dark hole. It is
readable, but it doesn't feel like a gun.

**Gun range, a carbine with Standard blast and Standard bomb.** At 0:03,
zoom 8: a white shock dome, flung slime, plants scorched pink round the
squad, the body visibly shoved back (`rounds`: 10 casts, 319 killed, 564
flung by 0:03). This is the first time the game felt powerful. I also
noticed the glow has a hard square corner (finding 7).

**Run, planet 1 (Dreskor, Drifter), squad mode, 0:00-0:30.** Goal: "Wipe
out the slime: cut it to 2% of its peak". The slime is one glowing ball
on its home crystal inside a rock arc. I walked 8 soldiers over: first
contact at 0:16, mould 3,077, then 840 at 0:22 and 4 at 0:26. **The
planet ended at about 0:30, won.** I was excited for about 6 seconds.

**Run planet 1, RTS mode, idle for 5 minutes.** Mould 3,088 at 0:01,
1,445 at 1:00, 1,516 at 3:00 and 1,515 at 5:00, sitting in one ball on
its crystal (`run1idle/planet-1-1-idle-300s-all.png`). The squad was never
touched. The goal bar read "(2%)" although the slime had halved: the bar
counts from the peak, so shedding doesn't count. No threat, no clock
pressure, no reason to move.

**Run planet 1 with three pumps and a turret (idle).** Pumped 476 food by
0:05, 8,325 by 1:30 and 13,741 by 3:20. Every building was at full hp
throughout. The turret never fired. The crystals under two pumps were
drained to nothing (crystals left 0.36), and the slime stayed at 1,503
in its ball. A lure placed beside a pump (`lure=200,60`) didn't bring it
either (11,950 pumped by 3:00, slime 1,516).

**Run planet 1, the owner's plan in squad mode (`hand=greedy`: drain the
biggest crystal, buy whenever you can).** 12 soldiers at 0:30, 19 at
1:00 and **38 at 2:00**, with 11,952 food drained by hand. The slime was
untouched at 1,479. The pump change slowed the first 30 s, but not the
plan.

**Shop after planet 1.** 250 points; six upgrades plus two folders; the
Scattergun (180) and Long gun (170) for sale; cards Needle gas,
Buckshot bullet, Delay, Thick, Pierce. No blast or bomb was on offer, so
the gun that looked best in the range is not one I could buy here. The
next planet is "Uldum, the Sprawler, 1.0 times the first planet's
walkers".

**Planet 2 (Sprawler), idle.** This is more alive: specks over half the
map, and trails along crystals toward my squad. Mould 4,897, then 2,134 at
1:00 and 2,251 at 2:30; my idle squad shot 231 walkers and fell to 82%
health. This was the only run planet where the slime came to me.

**Planet 3 (Trunk), idle.** Mould 7,016, then 1,581 at 1:00 and 1,604 at
2:00. One fat drop on its home crystal in the top-left basin, with
nothing anywhere else.

**Planet 4 (Swarmer), squad mode.** Mould 11,101 at landing, then 2,107 at
0:45 (5,319 killed by my squads) and 1,543 at 1:40. The remaining body is
a teardrop on its crystal inside a rock ring, the same shape at 0:45 and
1:40. A newcomer would read it as a building to knock down.

**Planet 6 (Nedarro), squad mode, after the 40 s head start.** Two thick
bodies at two nests, 23,011 walkers, then 18,529 at 0:45. They are bigger
and denser, but they also sit. With a 31:30 clock and bodies that stay
home, this is a siege, and the 40-soldier blob is the siege engine.

## Answers

- **Can a newcomer tell the goal in 30 seconds?** In a campaign mission
  and on a run planet, yes: the goal line is one plain sentence. On the
  Sandbox (the highlighted first choice) the goal is "Wipe out the mould,
  or out-harvest it and cut it down by 15:00", which I could read but not
  act on before my squad was dead. What the slime is *doing* is not
  readable on the run: it shrinks by half and the bar doesn't move.
- **Is the slime integral to play or a backdrop?** On the Fork it is the
  game. In the run it is a target at the end of an economy game. On run
  planets 1, 3 and 4 it never left its home crystal while I watched
  (2-5 minutes). The only exception was planet 2's Sprawler.
- **Does one plan win everything?** In the run, yes: drain, buy, march.
  Squad mode reached 38 soldiers in 2 minutes against a slime that
  never interfered. On planet 1, eight soldiers win in 30 s without
  even that.
- **Soft locks, rounds under a minute, idle wins?** Squad-mode run
  planet 1 ends at about 0:26-0:30. An idle player never wins a run planet
  but also never loses one: on planet 1 the slime never arrived in 5
  minutes, so idle runs to the 15:00 clock. The Fork rush leaves 0
  soldiers at 0:15 and a bank that drains; it isn't a lock while the bank
  holds 400.
- **Do the guns look and feel satisfying?** The starting rifle does not:
  a one-cell flash and a hole. Blast and bomb do: a dome, flung slime,
  scorch. The run hides the good ones behind luck in the shop.
- **First thing I'd fix for a friend's first ten minutes:** give the
  run's planet 1 a slime that comes for you, the 8,000-walker version in
  `planet1-walkers-3k-8k-25k.jpg`, and start the run with one payload
  card in the gun.

## Findings

### 1. The run's slime is too small to play: it sheds to about 1,500 walkers and sits on its home crystal
- **Kind:** issue
- **Evidence:** Run seed 1, idle. Planet 1: 3,088 → 1,445 at 1:00, 1,515
  at 5:00, never left its crystal. Planet 3: 7,016 → 1,581 at 1:00, 1,604
  at 2:00. Planet 4: 11,101 → 1,543 at 1:40, unchanged shape 0:45-1:40.
  The same planet-1 map with sandbox rules and more walkers is a
  different game (`first-time-player/planet1-walkers-3k-8k-25k.jpg`,
  `map=planet:1:1 hand=idle walkers=N`):
  - 3,000 walkers: 1,734 at 2:00, a sitting ball.
  - 8,000 walkers: 2,180 at 1:00 and 3,355 at 2:00. It takes the big
    crystal and runs a cord to the squad, which fires 175 shots.
  - 15,000 walkers: 3,568 at 2:00, the same.
  - 25,000 walkers: 27,332 at 1:00, covers half the map and eats the
    bank.
- **Why it matters for play:** This is the owner's "hang at their home
  crystal ... no rush ... no need to defend", traced to the walker budget
  the run lands with. Every other complaint (the 40-soldier plan, no
  walls or turrets, nothing to defend) follows from it.
- **Smallest next step:** Land planet 1 with about 8,000 walkers (or keep
  the count and find what lets 8k regrow but not 3k). Look at 0:30, 1:00
  and 2:00 in squad mode on three run seeds.
- **Impact:** 5
- **Effort:** S
- **Owner:** the run thread for the budget; the slime thread for why it
  sheds.

### 2. A confident report is wrong: "the slime sheds most of its walkers whatever it starts with"
- **Kind:** issue
- **Evidence:** `Reports/roguelike-run-2026-11-14/README.md` l.182 ("whatever
  it starts with"; the size it keeps is "the slime's own economy ...
  passed on and not tuned here"). On planet 1's map, 25,000 walkers grew
  to 27,332 (with `tier=drifter`, 26,541), and 8,000 and 15,000 shed and
  then regrew (3,355 and 3,568 at 2:00). Only 3,000 stayed down (1,734).
- **Why it matters for play:** The report closed off the cheapest fix,
  the starting count, by calling it pointless, and then handed the real
  problem to a thread where it is listed as not tuned. The run wiki
  still says it (`wiki/world/run.md` *Now*).
- **Smallest next step:** Correct the line. Then make a 4 x 3 table:
  walkers 3k, 8k, 15k and 25k, at 1:00, 2:00 and 5:00, run rules against
  sandbox rules, one seed, about 10 minutes of runner time.
- **Impact:** 4
- **Effort:** S
- **Owner:** the run thread.

### 3. The 40-soldier snowball survives the pump change: hand income is linear in soldiers, so the army grows exponentially
- **Kind:** issue
- **Evidence:** Squad mode (hand drain cut to a fifth), run planet 1,
  `hand=greedy`: 12 soldiers at 0:30, 19 at 1:00, 38 at 2:00; 11,952
  food by hand; the slime untouched (`first-time-player/run1-squads-greedy-38-soldiers-120s.jpg`).
  RTS mode run (larder, no cut): 35 at 2:00, 10,710 food. Pumps alone
  (idle): 13,741 by 3:20. One pump on the big crystal: 11,950 by 3:00.
- **Why it matters for play:** Each soldier bought drains faster, so the
  plan compounds, and pumps add to it. PR #322 changed the first 30 s
  (the owner's "40 in 30 s" is now 12 in 30 s) but not the outcome by
  2:00. Upkeep is ruled out (`Reports/dead-ends.md` l.9, owner "no
  maintenance costs"), and that entry names the allowed brakes: "prices,
  the mould eating the bank, or things worth buying".
- **Smallest next step:** Try two brakes in one look, one at a time:
  - A crystal drains no faster past four soldiers (a throughput cap per
    crystal: more soldiers spread over more crystals, which spreads the
    squad).
  - The soldier price rises 10% a soldier.
  
  Rerun `hand=greedy` on planet 1 and read the soldier count at 2:00.
- **Impact:** 5
- **Effort:** S
- **Owner:** the run thread / the economy owner.

### 4. Nothing gives a reason to defend: the slime never comes for what you drain
- **Kind:** direction (ranked first of the new directions)
- **Evidence:** Three pumps and a turret on run planet 1: all at full hp
  for 3:20, the turret never fired. A lure beside the pump at (200,60)
  didn't pull the slime 130 units (1,516 at 3:00). The owner: "They don't
  attack my pumping crystals so no need to defend"; "no need for ...
  walls or turrets".
- **Why it matters for play:** Turrets, walls, lures and outposts are
  dead verbs in the run. The owner's own description of the fun he wants
  is splitting the squad to hit a base from several sides; a slime that
  raids gives that move a defensive twin.
- **Smallest next step:** Make a draining crystal smell like food to the
  slime: a pump or a drain adds the crystal's draw to the food scent the
  walkers already follow, so that "drain loud, get raided" is the
  simulation, not a scripted wave. First check whether the existing
  scent reaches: put a `lure=` at 30 units from the planet-1 body and
  look at 1:00 and 2:00.
- **Impact:** 5
- **Effort:** M
- **Owner:** unowned (game design); the scent tuning is the slime
  review's.

### 5. Enemy progression is mostly a label: deeper planets settle at about the same slime
- **Kind:** owner decision / direction
- **Evidence:** After the first minute, at the time shown:
  - Planet 1: 1,515 (5:00).
  - Planet 2: 2,251 (2:30).
  - Planet 3: 1,604 (2:00).
  - Planet 4: 1,543 (1:40, squads hunting).
  - Planet 6: 18,529 at 0:45, still falling.
  
  The ladder (`run.rs:75` `LADDER`) changes the shape: a ball (planet 1),
  specks (2), a drop (3), a teardrop in a ring (4), two nests (6). But
  only planet 2's slime moved toward me. Meanwhile the shop sells better
  guns each planet (wiki `run.md`). The owner: "more interesting
  progression in the enemy ... especially when your guns get stronger".
- **Why it matters for play:** The squad grows each planet and the
  opponent doesn't. The run gets easier as it goes on.
- **Smallest next step:** Give each tier one thing it does *to you* that
  the last one didn't, starting with planets 1-4:
  - Drifter: eats a crystal you drain.
  - Sprawler: cuts your supply line.
  - Trunk: cords toward your pumps.
  - Swarmer: the alarm brings the whole body to the squad that fired.
  
  Write it as a one-line table the owner can rule on before any tuning.
- **Impact:** 5
- **Effort:** M
- **Owner:** owner decision, then the run thread and the slime thread.

### 6. The starting gun is the dullest thing on screen; the good guns are behind shop luck
- **Kind:** direction
- **Evidence:** `first-time-player/rifle-vs-blast-close.jpg`. Left: the
  rifle at zoom 10 (Fork rush, 0:04.1), a one-cell orange oval at the
  barrel, no visible bullet or hit, only the hole. Right: the range's
  carbine with blast and bomb (0:03.1, zoom 8), a white dome, flung slime,
  scorched plants; 10 casts, 319 killed, 564 flung. The squad-mode
  volley on planet 1 (`volley/planet-1-1-squads-20100ms-near.png`): eight
  orange ovals, 63 walkers killed in 0.1 s with no single hit you can
  point at. The planet-1 shop offered no blast or bomb.
- **Why it matters for play:** The owner wants guns to be "one of the
  draws of the game". The game already makes the satisfying picture; the
  run starts you on the least of it. The owner's rulings bound what can
  change: a hit lights only its damage (decisions-game 11-14 20:22,
  20:03), and nothing is drawn-only.
- **Smallest next step:** Make every rifle hit show as simulation:
  - Walkers killed by a bullet leave a splash of slime material and
    a short scorch on the ground, instead of vanishing.
  - The bullet's own flight is drawn, since it is a game object.
  - The soldier sprite kicks back like the turret's head already does
    (`view.wgsl:2926` `recoil`).
  
  Start every run with one rolled payload card in the gun. Show it as a
  close A/B GIF to the owner.
- **Impact:** 5
- **Effort:** M
- **Owner:** the gun thread / art lead.

### 7. A blast's glow is clipped to a square
- **Kind:** issue
- **Evidence:** `first-time-player/blast-square-edge.jpg` (range,
  carbine with blast and bomb, 0:03.1, zoom 8): a straight vertical edge
  and a straight horizontal edge in the white light over the slime.
- **Why it matters for play:** It looks like a bug at the most exciting
  moment, and it breaks "everything on screen is the simulation".
- **Smallest next step:** Find the light's bounding box in `view.wgsl`
  (likely a per-effect tile or quad smaller than the light's falloff).
  Pad it to the radius, or fade it to zero at the box.
- **Impact:** 3
- **Effort:** S
- **Owner:** gun / looks thread.

### 8. Squad-mode run planet 1 is over in about 30 s
- **Kind:** issue
- **Evidence:** `run=1 mode=squads hand=squads`. Contact at 0:16 (3,077),
  then 840 at 0:22 and 4 at 0:26, "mould wiped". In RTS mode the four
  starting soldiers would take longer, but the owner plays squad mode.
- **Why it matters for play:** The owner's real mode has a first planet
  that ends before the player has learnt a key.
- **Smallest next step:** Fix it with finding 1 (more walkers) and check
  that it lasts at least 3 minutes for the squads hand.
- **Impact:** 4
- **Effort:** S
- **Owner:** the run thread.

### 9. An idle player can't lose the run's planets
- **Kind:** issue
- **Evidence:** Planet 1 idle, 5:00: squad 100% health, 0 shots, slime
  1,515 at home; planet 3 idle at 2:00: 0 shots. CLAUDE.md's smoke check
  is "idle doesn't win". The run passes it only because the goal needs
  a wipe; idle also never loses, so the round runs to the 15:00 clock.
- **Why it matters for play:** A planet you can't lose by waiting has no
  tension, which is the owner's "there is no rush".
- **Smallest next step:** Add "idle loses or is clearly losing by 5:00"
  to the run's smoke checks (one capture a planet). Findings 1 and 4 are
  the fix.
- **Impact:** 4
- **Effort:** S
- **Owner:** the run thread.

### 10. The newcomer's path is inverted: the highlighted Sandbox is the hardest round, and Run planet 1 the easiest
- **Kind:** low-hanging (tag: menu/HUD review, because it changes what a
  newcomer plays first)
- **Evidence:** The menu still highlights Sandbox first
  (`menu/fork-rush-001s-all.png`). The Fork rush is 0 soldiers at 0:15
  with the slime at the larder at 0:30. Squad-mode run planet 1 is won at
  0:26.
- **Why it matters for play:** A friend's first minute is either a
  wipe-out or a walkover, and neither one is the game.
- **Smallest next step:** Put Run first and highlighted (the owner says
  it is the real game). Once finding 1 lands, planet 1 is the tutorial.
- **Impact:** 3
- **Effort:** S
- **Owner:** the menu/HUD review.

### 11. Mission 1 teaches the boring plan: farm and ignore the slime, three stars at 1:53
- **Kind:** direction
- **Evidence:** `mission=1-landfall hand=farm`: goal met at 1:53, three
  stars, 105 walkers killed. The slime fell 5,024 → 1,891 by itself and
  never came near.
- **Why it matters for play:** The first lesson is "the slime is
  scenery".
- **Smallest next step:** Make mission 1's goal one the slime contests
  ("hold 3 crystals at once" while the slime reaches for them), or land
  the squad nearer the body.
- **Impact:** 3
- **Effort:** S
- **Owner:** the campaign thread.

### 12. The goal bar doesn't move when the slime shrinks on its own
- **Kind:** low-hanging (tag: menu/HUD review)
- **Evidence:** Planet 1 idle 5:00: slime 3,088 → 1,515, bar "(2%)".
  Planet 3 idle 2:00: 7,016 → 1,604, bar "(1%)". The bar counts from
  the peak (`run.rs:493`, `campaign.rs:650`).
- **Why it matters for play:** The player sees half the slime vanish and
  the bar say they've done nothing, so they can't tell why they are
  winning or not.
- **Smallest next step:** Show the walker count against the goal count
  ("1,515 → 62") next to the bar.
- **Impact:** 2
- **Effort:** S
- **Owner:** the menu/HUD review.

### 13. Rivers disappear under plants within two minutes
- **Kind:** issue
- **Evidence:** `first-time-player/river-reeds-2s-vs-120s.jpg` (run planet
  1, zoom 10, 0:02 against 2:00). Pixel at (560,330) on the full map: RGB
  (24,43,56) at 0:05, then (34,56,42) at 3:20, from blue water to
  grass-teal.
- **Why it matters for play:** Deep water stops soldiers (decisions-game
  11-14 17:02), and it stops being visible as water. The map's main
  terrain rule becomes unreadable.
- **Smallest next step:** Keep a blue base under reeds on deep water, or
  stop reeds growing on deep cells.
- **Impact:** 3
- **Effort:** S
- **Owner:** the plants thread.

### 14. Pumps empty crystals in 2-3 minutes, and then the economy is over
- **Kind:** direction
- **Evidence:** Three pumps on planet 1: crystals left 0.62 at 1:30, 0.36
  at 3:20, rate "+0/s" at 3:20 (`first-time-player/run1-pumps-untouched-200s.jpg`).
  The crystals inside the rigs are gone.
- **Why it matters for play:** On a 15:00 clock, the economy is settled
  by minute 3. After that the only thing left to do is the attack. A
  regrowing crystal, or one the slime refills, would keep the map
  contested.
- **Smallest next step:** Make crystals the slime holds regrow, and
  crystals you have drained regrow only if you hold them. Look at the
  map at 5:00 with pumps.
- **Impact:** 3
- **Effort:** M
- **Owner:** unowned.

### 15. Splitting the squad is the owner's best moment, and the game doesn't reward it yet
- **Kind:** direction
- **Evidence:** The owner: "splitting my squad up to attack a slime base
  from multiple sides". Planet 4's body sits in a rock ring with one
  mouth; planet 6 has two nests.
- **Why it matters for play:** This is the emergent tactic the owner
  enjoys. A body that turns its whole mass toward the squad hurting it
  (the Swarmer's alarm already exists) makes a two-sided attack beat a
  one-sided one, and you can see why.
- **Smallest next step:** On planet 4, compare one 8-soldier squad
  against two 4-soldier squads from opposite sides. Look at the body's
  shape at 0:10 and 0:20 (two captures, no census).
- **Impact:** 4
- **Effort:** S
- **Owner:** unowned.

### 16. The shop can't sell what made the range exciting
- **Kind:** owner decision
- **Evidence:** Planet-1 shop: Needle gas, Buckshot bullet, Delay, Thick,
  Pierce; no blast or bomb. The wiki says the fixed payload cards are no
  longer sold, and bullets are rolled.
- **Why it matters for play:** The run's power curve is luck-gated, and
  the first shop may not change how the gun looks at all.
- **Smallest next step:** Ask the owner whether every shop guarantees one
  payload card.
- **Impact:** 3
- **Effort:** S
- **Owner:** the run thread.

### 17. Close zoom shows squads as stacks of overlapping circles
- **Kind:** low-hanging (tag: menu/HUD review)
- **Evidence:** `volley/planet-1-1-squads-20100ms-near.png`: eight
  soldiers in a heap, five selection rings overlapping. On the Fork
  squads still (`sq/fork-squads-030s-near.png`) the "2 AUTO" tag sits on
  a crystal.
- **Why it matters for play:** At the owner's zoom, the soldiers you
  are proud of are a blur.
- **Smallest next step:** Spread the formation by one soldier radius;
  draw one ring a squad.
- **Impact:** 2
- **Effort:** S
- **Owner:** the squad mode thread.

## Not done

- No GIFs. The bursts are in `/tmp/wgr/first-time-player/burst2`,
  `range3` and `volley`.
- No sound.
- No planet 7 or 8 (planet 8 costs 11 s a game second here).
- Single seed; the walker sweep is one game per count.
- `squad=40` is not an engine key (`larder-world` refused it), so the
  40-soldier attack itself was not filmed: the greedy hand builds the
  army but never attacks.
