# Whole-game review, 2026-11-15: simulation to play

Role: the model under the game, and what play could get from it. Every
capture ran on main (17f2e5b8) on llvmpipe, through `larder-world look=`, in
squad mode on run seed 1 unless named. Scratch frames are in
`/tmp/wgr/simulation-to-play/`. The five stills kept are in
`simulation-to-play/`. No census was run, and no game code was changed.

## The game as a whole

The model underneath is a good engine for a game. Many walkers follow a trail
field they lay themselves, eat crystal, divide and starve, share energy
through the body, and read scent, alarm, ground, plants and fire. It can
grow networks, reach for food, heal, wither when cut off, panic, spore, sleep
and convert. **The run uses almost none of that, and the economy around the
model gives the slime no reason to leave home.**

On run planets 1 to 3 the slime does the same thing every time. It lands too
big, sheds to what its home crystal can feed within about 30 s, and then
holds still. On planet 3 it went from 7,016 walkers to 1,581 at 1:00 and
1,658 at 2:30, and 15 of 16 crystals stayed untouched. On planet 2 it went
from 4,897 to 2,195 and then 2,330. On planet 1 the Drifter is a single
droplet, and the scripted squad hand wiped it in 30 s with 8 soldiers.

This is the owner's "hangs at their home crystal, no rush, no need to
defend", explained in model terms:

- A fed walker has no drive to leave.
- Spare walkers starve at home instead of reaching out.
- Food smell reaches about 25 dish cells, but crystals are 100 or more apart.
- Every food rule makes a crystal the player drains less attractive to the
  slime, not more.

So the real game today is a build-up with no contest followed by one
assault. That fits the owner's account (harvest to 40 soldiers, then
attack). The spread of outcomes sits only in how the assault goes.

The approach (the agent model as the enemy) is right for the goal. What is
wrong is how the model is wired to the economy. Two cheap changes moved the
picture a lot in look-only runs:

- **The owner's chosen shape C body.** The run never uses it. On planet 3 it
  reached 9,324 walkers at 2:00, still growing, and sent two cords to
  crystals.
- **A bigger appetite.** With 10 times the crystal eating rate it held 4,790
  walkers and three crystals at 2:00.

Gun hits are honest, but they cannot be seen in dense bodies, because the
body's picture saturates. The model can already supply a flinch, a spatter
and a spill, so a hit can be the simulation rather than a light drawn over
it.

What is drawn rather than simulated is mostly confined to units, buildings,
muzzle flashes, rings and tracers. On the slime itself, two things break the
"picture is the data" rule:

- The picture saturates, so a cell holding 6 walkers and one holding 60 look
  the same.
- Breakaway and panic steer straight to a target cell. That is a goal
  written into the model, not something the fields produce.

## Findings, most important first

### 1. The run's slime reaches its food ceiling in 30 s and then sits

- **Kind:** issue.
- **Evidence:** idle hand, squad mode, mould counts:

  | Planet | 0:01 | 1:00 | 2:30 |
  |---|---|---|---|
  | 2 | 4,897 | 2,195 | 2,330 |
  | 3 | 7,016 | 1,581 | 1,658 |

  Crystals left on planet 3 were 0.978 at 2:30 on an 18:00 clock. Still:
  `simulation-to-play/planet3-appetite.jpg`, top left. On planet 1 the
  Drifter was one blob on its home at 0:01 (3,088 walkers;
  `home-blob-and-breakaway.jpg`, left), and the scripted squads hand wiped
  it by 0:30 (2,791 killed with 183 shots, 8 soldiers, none lost).
- **Why it matters for play:** this is the owner's complaint, reproduced. In
  model terms the slime's size is its home crystal's feeding rate. Walkers
  past that rate starve where they stand rather than search. The home
  crystal barely drains in a round, so it never goes hungry enough to move.
  Nothing it does starts a clock for the player, and nothing threatens a
  pump.
- **Smallest next step:** turn surplus into reach, not death. Under a
  hunger threshold a walker becomes an explorer (`explores()` keyed on
  energy rather than age), instead of starving at home. Physarum spreads
  into a thin fan when starved. That is the inspiration, not the bar.
- **First probe:** planet 3, idle, 0-150 s. Measure crystals reached and
  walkers at 2:30, against today's 1 crystal and 1,658.
- **Impact:** 5. **Effort:** S. **Owner:** slime-enemy thread (rule); the
  run thread (default).

### 2. The roguelike run plays without the slime body the owner picked

- **Kind:** owner decision.
- **Evidence:**
  - Shape C (`upkeep` 0.0002, `energy_flow` 0.05, `wither` 0.01, `panic` 2)
    is applied only when no run, mission or own mould is set
    (`src/bin/larder-world/main.rs:1668`). The menu's Run passes
    `chosen = true` (`menu.rs:183`), and the test
    `the_menu_starts_a_runs_level_like_a_mission` asserts that the run gets
    no shape keys.
  - Every planet is therefore the race world's defaults plus the tier: no
    shared food, no withering, no panic, and five times C's upkeep.
  - With C's four keys added on planet 3: 7,565 walkers at 1:00 and 9,324
    at 2:00 (still rising), with two cords to crystals (`planet3-appetite.jpg`,
    bottom right). Without them: 1,658 and no cords.
- **Why it matters for play:** the owner chose C because "it networks the
  crystals", and its rules make holding crystals visibly matter. Withering
  and panic make a cut cord pay off. The run, which the owner calls the real
  game, shows none of this.
- **Smallest next step:** ask the owner on one card, "Run planets on C's
  body under each tier (recommended), or tiers as they are?" Meanwhile,
  film planets 1 to 3 with the C keys.
- **Impact:** 5. **Effort:** S. **Owner:** run thread, with the slime-enemy
  thread.

### 3. Every food rule makes the player's pumped crystal the least wanted thing on the map

- **Kind:** issue.
- **Evidence:**
  - A crystal's scent is its laid scent times its charge left
    (`src/gpu/engine.wgsl:1602-1609`; CPU `big.rs:3953`).
  - Once the slime has ever touched a crystal, the crystal is latched as
    "reached" for good and smells at 15% (`engine.wgsl:1450`, `fed_scent`
    0.15 at `big.rs:1153`).
  - Breakaway picks only crystals with at least 80% of their charge
    (`ROAM_FRESH`, `big.rs:80`).
  - Panic picks only crystals with more than 2% left.
  - The pump's own smell exists, but is 0 (`pump_smell`,
    `larder_world/buildings.rs:195-197` and `:265`).
  - So a pump lowers its crystal's pull from the moment it starts, and a
    crystal the player cleared of slime still smells at 15%.
  - On planet 3, a pump at (196,176) lost 82 hp in the first 40 s to strays
    as the slime shed walkers. It then lost only 11 more by 2:00, while the
    slime sat 160 cells away.
- **Why it matters for play:** "They don't attack my pumping crystals so no
  need to defend." In the model, the slime does not want what the player
  holds. Walls and turrets have nothing to answer.
- **Smallest next step:** two rules, no scripting:
  - A crystal smells of food moving, not food stored: its scent is
    proportional to the charge drawn from it this second, by anyone. A
    working pump then becomes the loudest thing in its area.
  - "Reached" holds only while the slime holds the crystal (trail over the
    tube line in the last N s), so a crystal the slime lost smells full
    again and it comes back.

  The dead end on full smell (`Reports/dead-ends.md`, lines 11 and 18) was
  about walkers pulled back onto a crystal they already sat on. Neither rule
  does that.
- **First probe:** set `pump_smell` 1 on planet 3 and count walkers within
  20 cells of the pump at 1:00 and 2:00, against 0 today.
- **Impact:** 5. **Effort:** S (pump smell) to M (flow scent). **Owner:**
  unowned (slime-enemy thread for the rule; the pump-economy thread for the
  pump).

### 4. The slime cannot smell the map

- **Kind:** issue.
- **Evidence:**
  - The scent pass is a 3 x 3 mean times `scent_keep` 0.97 every fine step
    (`engine.wgsl:1571-1618`).
  - A steady-state run of that kernel (a numpy model, not a game capture)
    falls by a factor of e about every 3 fine cells. Relative to the
    crystal's edge it is:

    | Fine cells past the edge | Scent |
    |---|---|
    | 14 | 1e-2 |
    | 24 | 4e-4 |
    | 54 | 3e-8 |

  - The scent sensor reads 18 x `scale` ahead (`world.rs:320`).
  - The Drifter's hunger multiplies scent by up to 17, which cannot lift
    1e-8.
  - Planet 3's nearest crystal to home is about 155 dish cells away.
  - The tiers were tuned with this reach on maps with 23 crystals close
    together (finding 9).
- **Why it matters for play:** a slime that cannot sense a crystal will not
  race the player for it. The only long reach in the model is the goal
  steer of breakaway and panic.
- **Smallest next step:** add a second "far smell" channel. It is coarse (a
  quarter of the grid), keeps about 0.999 a step, and only hungry walkers
  read it, weighted by hunger. A fed body stays put; a hungry one reaches,
  in model terms.
- **Frame cost:** a guess of under 0.05 ms on the 1070, since it is one
  small diffusion pass.
- **Impact:** 4. **Effort:** M. **Owner:** unowned (slime-enemy thread).

### 5. Deeper planets add walkers, but the model throws them away

- **Kind:** roadmap.
- **Evidence:**
  - Walkers grow 20% a planet (`WALKERS_PER_DEPTH`, `run.rs:102`), but the
    slime sheds to what its crystals feed. Planet 3 started 43% bigger than
    planet 2 and was smaller at 1:00 (1,581 against 2,195).
  - `wiki/world/run.md` already notes 65,600 walkers falling to about
    7,300 on the core world.
  - Appetite is a threshold dial on planet 3 at 2:00:

    | `crystal_eat` | Walkers at 2:00 | Crystals left |
    |---|---|---|
    | 1x (today) | 1,658 | 0.978 at 2:30 |
    | 3x | 1,823 | 0.947 |
    | 10x | 4,790, three crystals held, cords between them | 0.766 |

    Still: `planet3-appetite.jpg`.
- **Why it matters for play:** "There needs to be more interesting
  progression in the enemy ... especially when your guns get stronger." The
  run's difficulty knob does not reach the screen. The model's real
  difficulty dials are economic: eating rate, upkeep, regrowth, and how far
  the slime can smell.
- **Smallest next step:**
  - Replace the walker ladder with an appetite ladder per depth (higher
    `crystal_eat`, lower `upkeep`, then far smell).
  - Film planets 1, 3 and 5 at a proposed ladder and report crystals held
    at 2:00 and 5:00.
  - At 10x the map is eaten in about 8 minutes, so pair it with crystal
    regrowth (finding 14) or the slime starves itself.
- **Impact:** 5. **Effort:** S. **Owner:** run thread (the ladder);
  slime-behaviour review (the numbers).

### 6. A hit on a dense body cannot be seen, because the body's picture saturates

- **Kind:** issue.
- **Evidence:**
  - The drawn thickness is trail brightness times 1 - exp(-n / n0). n0 is
    6 walkers scaled by the world's average density (`src/world/view.wgsl:701-708`).
  - A Drifter blob holds many times the average, so every cell reads full.
  - Planet 1 at zoom 14: kills went from 167 to 483 between 17.6 and 19.0 s
    (`simulation-to-play/hits-invisible.jpg`). The edge barely moved, and
    the inside did not change colour.
  - The owner hit this before (11-12 04:13Z: "visually it didn't impact
    much"). The fix then scaled n0 by the global density, and that does not
    help a concentrated blob.
- **Why it matters for play:** "the picture is the data" breaks here,
  because 30% of the body dies and the picture does not change. This is
  also the first reason gun hits feel unsatisfying.
- **Smallest next step:** normalise n0 by local density (a blurred
  walker-count mip, or the body's own per-crystal mean), so a killed patch
  thins and dulls. A/B it blind at zoom 6 and 14 on planet 1.
- **Frame cost:** a guess of about 0.1 ms.
- **Impact:** 4. **Effort:** S. **Owner:** slime-look or graphics thread.

### 7. A hit should make the body flinch, and the model already does it

- **Kind:** direction.
- **Evidence:** planet 1, same seed and hand, at 19 s:

  | Run | Kills | Body |
  |---|---|---|
  | Default | 483 | pressed on the squad |
  | `alarm_keep` 0.98, `alarm_weight` -1 | 661 | pulls back, and the crystal shows blue under the gap |

  Still: `simulation-to-play/hit-flinch.jpg`. The flee rule is in
  `Reports/dead-ends.md` (the plain mould turning from gunfire, 11-12). It
  was rejected on a census gate, because random's attacks then held the
  core. Under build first, balance later, that is a tuning note, not a veto.
- **Why it matters for play:** the hit is the simulation. The body recoils
  from fire, opens a hole that refills when fire stops, and visibly reacts
  to where the squad aims.
- **Smallest next step:** make the recoil short (alarm keep about 0.9, so it
  is a flinch rather than a retreat) and keep it on for every tier. The
  alarm field now draws as a screen-wide mint wash
  (`hit-flinch.jpg`, right); draw it at a tenth of that strength.
- **Impact:** 4. **Effort:** S. **Owner:** gun thread, with the slime-look
  thread.

### 8. Hits throw nothing, and they should spatter real walkers

- **Kind:** direction.
- **Evidence:**
  - A blast boom on the Fork at 6.0 s killed 112 and flung 0 ("0 flung" in
    the rounds line). Only the bomb flings.
  - The blast frame (`simulation-to-play/blast-wash.jpg`) reads as a pale
    disc, with a lime tint and a grainy haze where the crater is. No chunk
    is taken out and no droplets fly.
- **Why it matters for play:** "The gun graphics need work ... They need to
  be really satisfying."
- **Proposed rule:** every hit flings k walkers outward (`fling_speed` is
  already on the GPU), with k equal to the damage. They land as specks, then
  crawl back or starve. A heavy gun throws a spray you can count, and a
  sniper pops a clean bead. The spray is walkers, so it is the simulation.
- **Smallest next step:** a gun-range burst at zoom 12 with fling on plain
  shots, k from 2 to 8, 0.05 s apart.
- **Impact:** 4. **Effort:** S. **Owner:** gun thread.

### 9. The tiers were tuned on other maps, and a planet is not those maps

- **Kind:** issue.
- **Evidence:**
  - `src/mould/tiers.rs:115-136` records the tuning:
    - The Drifter's hunger 16 "reaches 1-4 more crystals" on First contact
      seeds.
    - The Trunk "stopped growing at about 1,900 walkers" on the race world,
      which has 23 crystals.
  - On generated planets, crystals are sparser and the maps bigger. The
    Drifter on planet 1 reached none, and the Trunk on planet 3 held 1,658
    and reached 1 crystal.
  - The census and the player AI also play the race world and the Fork.
- **Why it matters for play:** any confident report on tier difficulty, or
  on "this beats that", was measured where crystals are close. The run is
  where they are not. Planet 1 is over in 30 s for a scripted hand.
- **Smallest next step:** one look-only pass of each ladder tier on its own
  planet (1-6), idle hand, 0-120 s, reporting walkers and crystals reached.
  That is about 10 minutes of runner time, not a census.
- **Impact:** 4. **Effort:** S. **Owner:** run thread and levelgen thread.

### 10. A kill returns nothing to the world

- **Kind:** direction.
- **Evidence:** in `life` (`engine.wgsl:917-1000`), a dying walker's energy
  is set to -1 and vanishes.
- **Why it matters for play:** if death spilled the walker's energy as a
  short-lived food patch (charge or grass field) that smells, the body would
  stream back to eat its dead.
  - Kill zones and turret lines would come out of the model: the slime is
    drawn into fire.
  - A fight left alone would visibly feed the nearby body.
  - The player would see a bright splash where walkers died, which then
    gets eaten. That is a hit effect with consequences, and a reason
    turrets matter.
- **Smallest next step:** a spill of 0.5 x energy into the grass field at
  the cell of death. Film 20 s of a squad holding at a crystal's edge, and
  count walkers inside 10 cells of the squad.
- **Impact:** 4. **Effort:** M. **Owner:** unowned.

### 11. Ranked directions from biology and agent-trail work

- **Kind:** direction.
- **Evidence:** papers; the model fields named below.
- **Why it matters for play:** these are inspiration, not realism, ranked by
  how directly each turns into play.
- **Directions:**
  1. **Starving spreads, fed stays** (Physarum foraging fans).
     - Rule: hunger makes explorers (finding 1).
     - The player sees a hungry slime throw out a fan. The fan tells the
       player the slime is losing, or where it will strike.
     - Probe: planet 3, as in finding 1.
  2. **Risk for reward** (Latty and Beekman, 2011: the mould crosses light
     for better food).
     - Rule: the alarm's push is weighed against the food smell, so the
       slime braves fire for a pumped crystal or the bank's smell.
     - The player sees it come through turret fire when the prize is big.
     - Probe: a turret beside a pump with `pump_smell` 1 and 4; film where
       the front stops.
  3. **Flux-reinforced tubes** (Tero et al., 2010).
     - Rule: a tube's conductance grows with the energy carried through it
       (`energy_flow` plus trail gain from flow).
     - The player sees arteries pulsing gold. Cutting one starves its end,
       which C's withering already makes visible.
     - Probe: log the per-cell flow on the Fork with C, and draw it as
       brightness.
  4. **Externalised memory** (Reid et al., 2012: it avoids its own old
     slime).
     - Rule: eaten ground keeps a "spent" mark that repels.
     - The slime moves on from used ground, so it travels: a nomad.
     - Probe: a spent field from the scar code with a negative weight.
  5. **Habituation, passed on by fusion** (Boisseau et al., 2016; Vogel and
     Dussutour, 2016).
     - Rule: the run carries which payloads killed the most walkers. The
       next planet's slime has a raised alarm threshold to them, so it
       stops scattering from them.
     - The player has to switch guns as the run goes on: progression that
       answers the build.
     - Probe: a paper design first (owner card).
  6. **Anticipating rhythm** (Saigusa et al., 2008).
     - Rule: the scar has a time phase. Fire from one spot on a beat, and
       the slime arrives on the beat.
     - Probe: the Grudge on CPU, with volleys every 4 s.
- **Impact:** 4. **Effort:** M. **Owner:** slime-enemy thread.

### 12. The only mould that remembers the player is not in the run

- **Kind:** roadmap.
- **Evidence:**
  - Grudge's scar field is CPU-only, and `LADDER` leaves it out "until the
    GPU engine takes its scar keys" (`run.rs:70-75`).
  - `wiki/world/mould-tiers.md` calls it the hardest tier: every plan does
    worse against it.
- **Why it matters for play:** a memory field is the model's natural
  escalation against a player whose guns grow. A slime that converges where
  it has been shot punishes the "40 soldiers, one push" plan.
- **Smallest next step:** port the scar to the GPU (one field, same pass as
  alarm). Put the Grudge at planet 6+ in `LADDER`.
- **Impact:** 4. **Effort:** M. **Owner:** slime-enemy thread.

### 13. Breakaway is the only long reach, and it is off and scripted

- **Kind:** low-hanging.
- **Evidence:** planet 3 with a pump at (196,176), `condense` 300 and
  `roam` 300:
  - A file of specks ran to the pumped crystal by 0:40
    (`home-blob-and-breakaway.jpg`, right).
  - Nothing settled anywhere by 2:00, and the mould had 1,709 walkers.
  - Roam steers straight to a chosen crystal's centre (`panic_to`,
    `engine.wgsl:616-660`). The target must hold at least 0.8 of its charge,
    so a pumped crystal drops out as a target.
- **Why it matters for play:** this is the right behaviour for the problem
  (pieces leave and settle new crystals), but it is driven by a goal, not by
  fields. It is off despite the "defaults on" ruling.
- **Smallest next step:** let a roaming piece follow the far smell
  (finding 4) instead of a fixed target, and drop `ROAM_FRESH`. Film the
  default on, on planets 2 and 3.
- **Impact:** 3. **Effort:** S. **Owner:** condense thread.

### 14. Crystals never regrow, so held ground never changes value

- **Kind:** direction.
- **Evidence:** `crystal_regrow` 0 (`big.rs:1284`). A pump drained the
  planet-3 crystal at (196,176) dry in about 80 s, banking 410 food in all,
  and then stood idle.
- **Why it matters for play:** with regrowth, a pumped crystal is a
  renewable asset worth defending. A lost crystal becomes worth retaking (for
  both sides, with finding 3's un-latching). The slime's appetite (finding 5)
  can also run high without eating the map bare.
- **Smallest next step:** `crystal_regrow` at about a tenth of the pump's
  draw. Film the planet-3 pump for 3 minutes.
- **Impact:** 3. **Effort:** S. **Owner:** pump-economy thread.

### 15. Plants and ground are built but barely felt

- **Kind:** direction.
- **Evidence:**
  - The slime does not eat plants. The owner said 11-14 17:29Z: "Slime
    doesn't eat plants at all."
  - Fire takes only the edge, since trail of `fire_damp` 3 or more does not
    burn (`wiki/world/plants.md`: 501 killed in 80 s while the slime grew
    from 5,405 to 7,669).
  - Plants do not slow anything or hide anything.
  - Barren ground costs three times the upkeep (`barren_up`, `engine.wgsl`
    in `life`), but the player can neither make it nor read it.
- **Why it matters for play:** "building walls" is the only way to deny
  ground today. A field version is to burn a firebreak whose char is barren
  for a while, which the slime avoids because living on it costs three
  times as much. It is visible (char), simulated (upkeep), and lasts.
- **Smallest next step:** char counts as barren while it lasts. Film a
  scorch line between home and a pump on planet 3, then count crossings.
- **Impact:** 3. **Effort:** S. **Owner:** plants thread.

### 16. Species rivalry and conversion are on the GPU and unused in the run

- **Kind:** direction.
- **Evidence:** `species_convert`, per-species sets and `mix_*` are on the
  GPU (`wiki/world/mould-tiers.md`, rival pair "not built"). The own mould
  is not in the run.
- **Why it matters for play:** a deeper planet with two slimes, one droplet
  and one network, that convert each other would let the player bait one
  into the other (lure, bait card). Play as a third party is a new strategy
  space that needs no new mechanism.
- **Smallest next step:** `map=planet:4` with `species` 2 and
  `species_convert`. Film a minute: do they meet and fight?
- **Impact:** 3. **Effort:** M. **Owner:** run thread.

### 17. The mould count falls far faster than the kill counter in a fight

- **Kind:** issue.
- **Evidence:** planet 1, squads hand, 16 s to 20 s: the mould count fell by
  1,691 (from 3,077 to 1,386), while kills rose by 947. About 750 deaths are
  unaccounted for. The cause is a guess: starvation when the blob is
  disturbed, or wading.
- **Why it matters for play:** either the score and the sound undercount
  what the guns did, or the Drifter collapses on contact. Either way it
  skews what a gun upgrade looks like it does.
- **Smallest next step:** print deaths by cause (shot, starve, wither,
  wade) in the look's t line.
- **Impact:** 2. **Effort:** S. **Owner:** gun thread.

### 18. The model's own knobs for these probes cannot be set from the command line

- **Kind:** low-hanging.
- **Evidence:** `gpu::set` (`src/gpu/mod.rs:700-900`) has no `scent_keep`,
  `fed_scent`, `food_deposit` or `pump_smell`. These are the four knobs
  behind findings 3 and 4.
- **Why it matters for play:** every thread probing "why it sits at home"
  must change code to look.
- **Smallest next step:** add the four keys (`pump_smell` through `Rules`),
  and list them in the kit.
- **Impact:** 2. **Effort:** S. **Owner:** unowned.

### 19. The alarm field takes over the screen when it is on

- **Kind:** low-hanging.
- **Evidence:** `hit-flinch.jpg`, right: with alarm on, the squad's whole
  area is washed mint, and the soil and plants wash out.
- **Why it matters for play:** the Swarmer, Sporer and Sleeper (planets
  4-6) all run alarm. If their planets look like this, the hit's own read
  is lost. Tag: the slime-look thread.
- **Smallest next step:** draw the alarm only on the slime, or at a tenth of
  its strength. Look at planet 4.
- **Impact:** 2. **Effort:** S. **Owner:** slime-look thread.

## Method notes

About 30 images were looked at, all from these captures:

| Capture | What it is |
|---|---|
| `r1sq` | Planet 1, scripted squads hand |
| `idle2`, `idle3` | Planets 2 and 3, idle |
| `p3pump` | Planet 3 with a pump |
| `p3roam` | Planet 3 with a pump, condense and roam |
| `p3eat`, `p3eat3` | Planet 3 at 10x and 3x `crystal_eat` |
| `p1eat` | Planet 1 at 6x `crystal_eat` |
| `p3c` | Planet 3 with shape C's keys |
| `p3spore` | Planet 3 with `tier=sporer` |
| `r1burst`, `r1flee` | Planet 1 bursts, default and with alarm flee |
| `range` | The gun range |
| `blast` | A blast boom on the Fork |

Every count above is from the t lines of those runs. Each is a single seed:
these are looks, not measurements.
