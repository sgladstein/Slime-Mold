# Review: what the player can see and read (lens 3)

2026-11-17. Reviewer for the plan `Reports/sim-use-2026-11-17/plan.md`
(draft 2), read on branch `claude/project-thread-i8ltol` at 85a1bd14. No code
changed. Line numbers are from that tree. Where something is a guess, it
says so.

## Short version

The plan's goal is right: let the slime change the world, and draw each new
field from its own value. But most of what it adds would sit **under the
slime, or between things that already own every free colour and value**.
Several steps would be real in the simulation and unseen in play. Two would
not even be real as written (Provoke, and the bulwark at its threshold).
The fix is cheap. Settle each look on a still before building the rule.
Say for each field where on screen the player sees it, and give the field a
channel nobody else uses. Then test cause and effect with a blind quiz, as
the Mind statuses were tested.

## 1. Is the approach right?

Mostly yes. "The slime builds" is the best answer to the 11-16 audit: the
player writes the world, and the world never writes back. Crust and
bulwarks are both read from fields, cost the slime food, and have
counterplay. That fits the owner's rules.

What I would change, from the readability side:

1. **Look first, rule second, in every step.** The plan builds the rule and
   hands the drawing to the slime-look thread ("View: the slime-look thread
   draws it", plan *E1 Code*). A key that is on but not drawn breaks "every
   tool must deliver something you can see". I would split E1 into
   **E1a: the field and its look, with no effects**, and E1b: the effects.
   E1a's page settles the glaze before anything depends on it. The same for
   bulwarks: settle the material on a still before E2a.
2. **Plan where each field shows, not only how.** The view hides all ground
   detail under the slime by mid-film: `under = 1 - smoothstep(0.004, 0.05,
   th)` (`src/world/view.wgsl:2318`; `m.vis`, 1187). Crust forms only where
   walkers stand, so it is drawn under them and hidden. The player sees
   crust **only where the slime has left**. The plan says "a dull glaze on
   the ground under and behind the slime". Only "behind" is true.
3. **Too many new fields in one place.** After E and C, five quantities sit
   on the cells the slime holds or just left: trail, crust, hard,
   moisture, alarm. Today's alarm haze already washes that area mint
   (`ALARM_LOOK` 0.25, `src/world/view.rs:36-44`). A player cannot learn
   five overlapping ground tints. I would give each a separate channel
   (value, hue, gloss, height, texture), and fold moisture into crust if
   no channel is left (see C1).
4. **Measure "can the player tell?", not only "did the count change?"**
   The Mind step did this well: four shuffled clips and a key off the page
   (`Reports/gun-mind-2026-11-16/README.md:113-120`). It also dropped two
   statuses because there was "nothing you could see" (Lace; Spin by
   turning harder, `Reports/dead-ends.md:113`). Every step in this plan
   should have that quiz as its deciding look.

## 2. What the plan misses

### 2.1 There is no free value slot for crust

The value order since 11-15 (`wiki/looks/art-direction.md`, rule 3's long
paragraph): ground 41-51, rock 50-60 (rubble too), soft rock 60-72, wood
about 91, thin film from 95, crystals 110+, thick slime 140+. Char is 25
(`CHAR_RGB`, `view.wgsl:315`). Rubble is 52,47,41 times 0.75-1.25
(`view.wgsl:232-234`), so 36-60. Barren is 55,55,53 (`view.wgsl:683`).
Damp bank is ground times 0.6 (`view.wgsl:2239`).

So crust cannot be told apart by value. Lighter than ground reads as rock
(rule 2: it would outrank rock). Darker reads as damp bank or char. It must
be told apart by **hue, gloss or texture**, at ground value.

Hue is nearly full too. Gold and amber are the slime's. A faint gold
behind a retreating front will read as the slime's edge going soft. The art
lead rejected exactly that (a faint tail past the walkers widened the
edge from 0.96 to 1.74 cells; `view.wgsl:1154-1159`, `Reports/dead-ends.md`
11-15). Warm brown is clay and rubble. Grey is barren and rock.

The free channel is **gloss**. Only water and the slime are glossy today
(`m.gloss` 0.05 on ground, 0.75 on water, `view.wgsl:1204-1210`). A satin
varnish on dry ground, in crisp-edged smooth patches that catch the top-left
light, is a look nobody owns. My guess is it reads at zoom 6 and up, and
not at zoom 1, where it needs a faint tint instead. That is a level of
detail of the same data, which the rule allows.

### 2.2 "No threshold edge" against "crisp at 24 px"

The plan's rule says each field is drawn "through one ramp, with no
threshold edge". The owner plays at about 24 px a cell and wants it crisp
(`view.wgsl:619`, "the fire is blurry. The assumption should be that I am
zoomed in"). A per-cell value blended smoothly across a 24 px cell is a
blur. Char solved this with a narrow rise of a per-pixel B-spline
(`smoothstep(0.06, 0.3, pp.char)`, `view.wgsl:2356`). Plants solved it with
crisp crowns whose size follows the density (`wiki/world/plants.md`,
*Look*). Crust should do one of these. My pick: crisp patches whose
**coverage** is the crust value (more crust, more of the ground glazed),
like the plants' crowns. Every pixel stays data and every edge is sharp.

### 2.3 The bulwark has a hard threshold, and it will flicker

`hard` runs 0 to 1, and "at 1 the cell is a bulwark". Shots chip `hard`.
So the first chip takes it below 1: no longer a wall. Under the same fire,
the alarm keeps it rising, so it is a wall again. The cell will blink
between wall and not-wall under fire, and so will the walking maps. A
ridge at 0.95 and one at 1.0 will look the same, but only one stops shots.

Two fixes: (a) on reaching 1, the cell becomes a real wall with its own
health (`build_wall_hp`, `src/mould/big.rs:2595`), and shots chip the
health, not `hard`; or (b) hysteresis (a wall from 1, back to ground below
0.5). Then draw the wall state crisp, as a wall is, because the simulation
is hard there. "Hard edges only where the simulation is hard" is already
the rule (`wiki/looks/simulation-look.md`, *Now*). The walls thread learned
this the hard way: rubble drawn as wall hid a ring three-quarters eaten
(`Reports/walls-2026-11-16/README.md`, *Why slime seemed to go over walls*).
E2 needs the same truth test: a cell drawn hard if and only if it stops a
shot, as `slime_on_rubble_is_drawn_and_rubble_is_ground` tests rubble.

### 2.4 A bulwark lies under the slime that built it

The slime walks through its own bulwarks, and they form only where at least
`bulwark_occ` walkers stand. So the cell is under a dense body. If the
bulwark is drawn as ground, the body hides it (2.1's `vis`). If it is drawn
as hard terrain over the body, the walkers inside vanish. Walls today never
hold walkers, so the view has no answer for this yet. The plan has to pick
one, and it is the main look question of E2. Two candidates for a blind A/B
(section 3, E2).

### 2.5 "Dark ridges" collide with the player's own walls

The plan says the slime's face "thickens into dark ridges". Player walls
are warm dark grey ridges (`Reports/walls-2026-11-16/page/truth-312.png`).
Rock is neutral dark grey. Dark ridges at the slime's face will read as the
player's own wall. The owner also turned down bronze twice ("Remove the
bronze", 11-16 04:58Z, `wiki/world/run.md:92`; mini-bosses "not bronze",
art direction rule 3). So the bulwark must not be a dark brown either. It
should carry the slime's own hue in a hard material, so it reads as "the
slime's wall".

### 2.6 Provoke (B1) does nothing in a fed body, and turns it grey

Two problems, both from the code:

- Every step each walker takes the latest news of its cell:
  `atomicMax(&news[k], ...)` in `nhear` (`src/gpu/engine.wgsl:1389`). The
  CPU docs say the same: "Once it touches the body again it hears of food
  and calms" (`src/mould/big.rs:758-759`, the `panic` docs). So a walker
  stamped "old news" inside a fed body hears fresh news one step later.
  The part you would want to provoke (a fed part on a crystal) cannot be
  provoked by a stamp. It needs a "deaf" status for its length, a word
  per walker as the Mind statuses keep (`BigDish::mind`, `big.rs:5248`).
- The view greys a walker whose news is old: `cut` is "the share of the
  walkers round the cell whose news of food is stale"
  (`view.wgsl:1026-1027`), and it mixes the hue to ash (`view.wgsl:1107`)
  and dulls the gloss (`view.wgsl:1189`). So a provoked part would turn
  ash at once, the colour that means "cut off, dying, burnable". It would
  also wither if `wither` is on, since that reads the same news
  (`big.rs:725-726`). The plan says B "needs no new drawing". It would get
  one, and the wrong one.

Fix: a separate status that panics the walker without touching its news,
so it stays gold and charges. Then the existing lean look ("a brighter,
denser front facing its target", art direction rule 5) is the cue.

### 2.7 Bulwarks will mostly form at the home blob, not "where it is shot"

A bulwark needs crust of 0.6 (about 36 s of steady living at the plan's
rise of 1/60 a second) and then more time to harden. A surge or a speck
holds a cell for seconds, so it lays no crust. On planet 3 at 2:00 the
slime is one blob on its crystal and a few veins
(`Reports/whole-game-review-2026-11-15/simulation-to-play/planet3-appetite.jpg`).
So the player will see ridges where they besiege the home body. They will
not see them where they hold off a surge. That is fine as play, but the
plan's *What the player sees* should say so, or the player is told the
wrong rule. Also the alarm on planets 1-3 fades in a few fine steps
(keep 0.9, `Reports/flinch-2026-11-15/README.md`, *What it is*). Sampled
once a second, it will catch only shots from the last few steps. My guess
is that the ridges then grow in specks where shots happened to land, not
in ridges. The tick should take the most alarm since the last tick.

### 2.8 Interactions the plan does not name

- **Flinch against bulwarks** (planets 1-3, where E2's test runs). The
  flinch sends walkers off ground a volley keeps hitting
  (`Reports/flinch-2026-11-15/README.md`). A bulwark needs walkers to stay
  on that ground. The two will fight. The player may see the face back
  off and leave a ridge behind (which could look good), or see nothing.
  Film planet 3 and a planet with its own alarm.
- **Tiers that run to the alarm.** The Swarmer floods to a wound
  (`Reports/flinch-2026-11-15/README.md`). C4 (fire lays alarm) will make
  the Swarmer run *into* fire, and planets 1-3 flee it. Fine if intended,
  but the player sees "fire herds it" on one planet and "fire baits it" on
  another, with no cue. Name it on the tier's card or test it.
- **Turrets and sirens lay alarm too.** Turret hits lay the alarm the
  Swarmer follows (`Reports/dead-ends.md:75`; I did not re-check the
  current turret code). A siren lays 150 over its radius
  (`src/mould/larder_world/buildings.rs:321`, 788). So turret fire builds
  ridges where its shells land, deep in the body, and a siren walls itself
  in. Ridges far from any rifle will puzzle the player.
- **Alarm haze round fire.** C4's alarm is drawn: a pale mint haze on bare
  ground and slime (`view.wgsl:2472-2500`). Every fire, even with no slime
  near, would gain a mint halo. The fire look was tuned hard (owner 11-14
  19:26Z, 20:21Z). Film it.
- **Moisture moves the plants' drawing.** Plant crowns on forms with `wet`
  set grow where moisture is high (`view.wgsl:407-411`), and fen moss tips
  turn yellow-green on the wettest ground (`wiki/world/plants.md`, *Look*).
  This is look only: the plant density does not change. If C1 makes
  moisture move with the slime, crowns would swell where the slime was and
  shrink a minute later, while the plant field stays the same. That is a
  drawn-only effect, against the rule. The view should keep reading the
  map's moisture for crown size, or the plants should really grow.
- **Sleep gas already stops a charge.** B2 Calm "stops a charge where it
  is". Sleep gas does that, and sleepers are drawn dark amber. A calmed
  front must look different, or the player has two tools with one look.
  The engine already has a pass named `calm` for gas (`engine.wgsl:3453`);
  pick another name.
- **The name "crust" is taken.** The dish look calls sleeping mould's matte
  ramp a crust (`wiki/looks/simulation-look.md`, *Now*), and
  `Reports/dead-ends.md:38` has "Larder crust" (dormant mould). Pick a new
  word ("varnish", "sheath" or "track") before it reaches a card or a page.
- **Hits on a dense body are already hard to see.** 30% of a blob died and
  the picture barely changed (`simulation-to-play.md` finding 6; n0 is still
  scaled by the global density, `view.wgsl:756-762`, as far as I can see).
  E2's player cue "the kill count falls" will be invisible for the same
  reason. The cue must be the ridge and the shots stopping on it.

### 2.9 Controls and confounds in the proposed checks

- **E1's look uses the idle hand.** With nobody pushing it off, the blob
  sits on its crust and hides it (2.1). The check at 3:00 will show little.
  Use a hand that pushes the slime back (the squads hand, or a blast at
  3:00 as the measure already does), and count the share of crust cells
  under film (`th > 0.05`) beside each frame.
- **Zooms.** The plan names zoom 6 and 14. The art page judges at 1, 6, 12
  and 24 (*How a look is judged*). 24 is how the owner plays. Use those.
- **Soils.** Crust lies on every ground. Film it on at least three planet
  soils (meadow loam, barrens hardpan, cave dust), plus sand, clay, barren
  and a river bank.
- **Grey test.** The art page's check: turn the frame grey and the order
  must still read. Add it to E1 and E2.
- **Frame cost of the drawing.** The plan quotes engine passes only. A
  per-pixel glaze at zoom 24 is view work. Quote the view's llvmpipe
  before and after, as the art page asks.

## 3. Each step: what the player sees

For each step: what is on screen at zoom 6 to 24, how fast, what it could
be confused with, whether cause and effect read without a tooltip, and the
blind test that would settle it.

### E1 crust

- **On screen:** a satin glaze behind the slime where it lived about a
  minute or more, fading over a few minutes. Not under the slime (2.1).
- **How fast:** a minute to build, half gone in two (plan numbers). Fine for
  15-55 minute planets.
- **Confusable with:** damp bank (darker), rubble (warm gravel), char
  (near-black), barren (light ash), the slime's soft edge (if gold).
- **Cause and effect:** "it was here" reads well once the slime leaves.
  **Memory roads** read only when the slime comes back along the glaze
  after being knocked off. **Fast ground** may not read at all: speed is
  hard to see (Spin by turning harder was invisible,
  `Reports/dead-ends.md:113`). The likely visible sign is a shot hole
  refilling faster on crust. Measure that. If it does not show, drop
  `crust_fast` or make it bigger.
- **Crater wipes it**: a clean disc in the glaze. That reads well; keep it.
- **Soldiers wade on it:** they walk on bare glaze, so the cue is under
  their feet. Good.
- **Film:** planet 3, squads hand, 0-8:00, zoom 1, 6, 12, 24; a blast at
  3:00 and the refill. Counts beside: crust cells over 0.3, the share under
  film, walkers on the blasted road at +30 s.
- **Blind tests:** (1) a "name the ground" quiz: six crops of crust, char,
  rubble, barren, damp bank and bare soil, shuffled, key off the page.
  Each glaze candidate passes if the owner names it right. (2) The plan's
  two glaze ramps as an A/B, with a third candidate, gloss-only patches.
  (3) Memory on and off, two clips of the return after a blast: "which one
  remembers?" If the owner can't tell, memory roads are not a feature yet.

### E2 bulwarks

- **On screen:** ridges at the face the squad fires at, after a minute or
  more of siege on ground the slime has held (2.7).
- **Confusable with:** player walls, rock, rubble, and the thick slime's own
  relief.
- **Cause and effect:** good if the chain shows: shots land, mint haze, a
  ridge rises in the haze, shots now spark on the ridge and no hole opens.
  The spark is the key cue. A shot that hits slime opens a hole; a shot
  that hits a ridge must look different, and the ridge must lose height or
  light as it is chipped (walls already show health as brightness, art
  direction rule 3).
- **The look question (2.4, 2.5):** (A) a hard ridge of translucent amber,
  crisp-edged like a wall, its walkers seen through it as specks; (B) the
  slime's own body raised and glossy along the ridge, with a crisp rim
  where it blocks. Neither dark grey nor bronze. A blind A/B, with a
  player wall, rock and rubble in the same frame, in colour and grey.
- **Film:** standing hand 90 s and moving hand, as the plan says, plus the
  same on a planet whose tier keeps its own alarm (flinch off). Counts
  beside: bulwark cells, shots stopped by them, kills a minute.
- **Truth test:** drawn hard if and only if it stops a shot (2.3).

### E3 tunnels, E4 regrowth

- E3: a wall wearing away is already drawn (soft rock darkens with damage,
  art direction rule 3). Readable. The player must see it coming: show
  the wear on the rock face long before it opens.
- E4: spores resting in crust are invisible until they wake. The refill
  will look like a timer. If it comes, draw the resting spores as specks.

### C1 wet and dry slime

- Today, thick slime does not burn (`fire_damp` reads trail 3; "fire
  takes the slime's edge, not its body", `wiki/world/plants.md`). Thick
  versus thin is the strongest cue on screen. C1 replaces it with wet
  versus dry, which shows only through colour and gloss. That is a weaker
  cue. Keep thickness in the wetting (more walkers, more wet), so thick and
  fed stays the clear "won't burn".
- **The cue and the rule must be the same number.** The view greys a part
  by stale news (`cut`, 2.6). C1 wets by energy share. A cut part keeps its
  energy for a while (`big.rs:727-733`: "a part cut off ... stays whole
  for 75 s or more" with shared food). So a part can look ash grey and
  still be wet. The player lights it and nothing burns. Either wet by
  fresh news, or draw the dullness from moisture. Measure the gap: seconds
  from the cut to grey, and from the cut to burnable. They should match
  within a few seconds.
- **Ground moisture is not drawn** today, except the fen's moss tips. If
  the slime wets the ground, nobody sees it. Draw it with the damp-bank
  cue (darker ground), which players already read as wet, or don't make
  the ground's moisture move (see the plants note in 2.8).
- **Film:** the plan's three-frame strip (fed, cut +10 s, +30 s, +60 s),
  each frame with the count of walkers that die if lit at that moment.
  The look passes if the greyest frame is the one that burns.

### C2 barren char, crust burns, bulwark burns

- **Fire running along old roads:** very readable, if the glaze is visible
  first. A line of fire along the glaze, char behind it.
- **Char as barren:** probably invisible. The fed network shares energy
  (`energy_flow`), so paying three times upkeep on a few cells may not
  thin the strand at all. That is a guess. Test it today on an existing
  barren patch (section 4, experiment 4). If nothing shows, the firebreak
  is a hidden rule. Then make the cost show (walkers on char lose light,
  so the strand dims as it crosses).

### C4 the slime flees fire

- **On screen:** a gap opening ahead of the fire front, a few cells wide.
  The alarm spreads only a few fine cells a second, so it moves only the
  walkers next to it (`Reports/dead-ends.md:40`). The gap is the cue, and
  it reads without a tooltip. Count its width.
- Watch for the mint halo round every fire and the Swarmer running in
  (2.8).

### C5 gas ignites

- **On screen:** fire racing across the gas cloud in a second, the cloud
  gone. Very readable. But if the sleepers are thick or wet, the fire's
  damp test spares them and the flash kills nothing. The player then sees
  a big flash and no hole. Decide if gas fire ignores damp, and film the
  kill count.

### B1 provoke, B2 calm

- B1 as written does not work (2.6). With a deaf status, the cue is the
  existing lean: a brighter front leaning at the squad, 0.5-1.5 s after the
  hit. Good.
- B2: "a charging front slows and spreads". Also stamps news fresh, so a
  cut part hit by Calm would turn gold again. That is honest (its news is
  fresh) but looks like it reconnected to food. Use a status instead.
- **Blind quiz,** as Mind's: five shuffled clips, plain, provoke, calm,
  sleep gas and a natural panic. Each tool passes if the owner names it.

### S1, S2, S3

- The surge today runs straight at a unit. Players read that at once. A
  surge that follows the soldiers' smell bends round walls, which is good,
  but the smell is not drawn. "A soldier who masks their smell isn't
  charged" is a cause the player cannot see. The far smell (S2) is a whole
  new invisible field. `Reports/dead-ends.md:13` already notes the
  invisible smell "argues against" the everything-on-screen rule.
- Either draw the field the charge follows (a faint scent light, as the
  lure's, `view.wgsl:2456-2460`), or keep the target.
- Add a quiz to the plan's side-by-side: show the first two seconds of a
  surge and ask where it will land. The field version should be as easy to
  predict as today's.

## 4. My ranked directions, each with an afternoon experiment

1. **Look at crust before building it.** Log per-cell occupancy and trail
   once a game second from the CPU twin, apply the plan's rise and fall
   offline, and render the crust as a heat map beside the game frame at
   1:00, 3:00, 5:00 and 8:00. Planet 3, idle and squads hands, seed 1.
   Decides: roads or a disc under the blob? How much of it is under the
   body? No engine or view change. (I have not checked which harness logs
   occupancy; `mouldprobe look=DIR` may need a small addition.)
2. **Ground line-up for the glaze.** A local shader test, not shipped:
   paint a fake crust value from a mask, draw three candidates (satin
   gloss patches, amber stain, coverage patches), beside char, rubble,
   barren, damp bank and thin film, on three soils, at zoom 1, 6, 12, 24,
   in colour and grey. Run the "name the ground" quiz. Settles E1a's look
   before any rule exists.
3. **Can a stamp provoke?** On the CPU twin, a fed body on a crystal: stamp
   walkers' news old in a disc, count stale and leaning walkers at +1, +10
   and +60 steps, then the same with a 5 s deaf status. Expect the first
   to return to fresh in a step (2.6). Decides B1's design.
4. **Is barren ground visible today?** Film shape C crossing an existing
   barren patch next to plain soil, zoom 6, 2 minutes. Count walkers a cell
   on and off it and look at the strand. If nothing shows, C2's
   char-as-barren needs a look of its own.
5. **Bulwark material on a still.** Take the walls page's ring frame
   (`Reports/walls-2026-11-16/page/truth-312.png` setup) and draw a fake
   ridge mask two ways, amber resin and raised slime. Blind A/B against a
   player wall, rock and rubble, colour and grey.
6. **Fire's alarm on two tiers.** With existing keys, lay a Burst alarm on
   a scorch round's burning cells on planet 1 (flinch) and on the Swarmer's
   planet. Film zoom 6. Count the gap ahead of the front, and look at the
   mint halo. Decides whether C4 reads the same on every planet.

## 5. Checks of names, lines and numbers

- Line numbers in the plan's table match this tree for `walls` (2092),
  `fire` (3812), `firekill` (3864), `ignite` (3888), `panic_to` (743),
  `surge_to` (835), `raid_to` (869), `mindlay` (3914, in `engine.wgsl`,
  not `big.rs` as the table's last row implies), `ngather` (1335),
  `build_wall_hp` (`big.rs:2595`), `bite_walls` (`big.rs:2698`),
  `BigDish::mind` (`big.rs:5248`), `Rules::smell` (`rules.rs:233`, the
  doc line is 231).
- `life` starts at `engine.wgsl:1197`; 1243 is inside it. Fine as a
  pointer.
- `build_wall` at 2203 is a comment above the `build` pass (2209), not a
  function named `build_wall`.
- `Burst::alarm` is at `squad.rs:223` (the struct opens at 211), not 200.
- There is no `diffuse` entry point in `engine.wgsl`; the trail pass is
  `trail` (1732). E1's code line should name it.
- "Zoom 6 and 14": the art page says 1, 6, 12 and 24.
- Crust's numbers: rise 1/60 a second fills it in 60 s of full occupancy;
  `crust_keep` 0.994 a second is a half-life of 115 s. Both as the plan
  says.
- Bulwark crust 0.6 needs at least 36 s of full occupancy before it can
  start to harden (2.7).
- The alarm haze default is a quarter (`view.rs:44`); a siren lays 150
  (`buildings.rs:321`); one shot lays 50 within twice its radius
  (`squad.rs:74-75`).

## Filming list (what has to go on the pages)

| Step | Film | Zooms | Counts beside | Blind test |
|---|---|---|---|---|
| E1a | glaze building and fading, 0-8:00, a blast at 3:00 | 1, 6, 12, 24; grey | crust cells > 0.3, share under film | name the ground; glaze A/B/C |
| E1b | the return after a blast, memory on and off; hole refill on crust and bare | 6, 24 | walkers on the road at +30 s; refill seconds | "which one remembers?" |
| E2 | standing and moving hand; planet 3 and an own-alarm planet | 6, 12, 24; grey | bulwark cells, shots stopped, kills a minute | resin vs raised slime, with wall, rock, rubble |
| C1 | fed, cut +10/+30/+60 s strip, then lit | 6, 24 | walkers killed if lit at each frame | none; the greyest frame must be the one that burns |
| C2 | fire along old roads; slime crossing a char stripe | 6 | crossings in 2 min; walkers a cell on char | none, unless a new look is added |
| C4 | fire front, alarm on and off, two tiers | 6 | gap width ahead of the front | fire look with and without halo |
| C5 | gas cloud then a scorch round | 6, 24 | walkers killed by the flash | none |
| B | plain, provoke, calm, sleep gas, natural panic | 6 | walkers leaning, seconds to lurch | name the tool |
| S | today's surge against the field surge | 6 | the plan's pressure counts | A/B, plus "where will it land?" |
