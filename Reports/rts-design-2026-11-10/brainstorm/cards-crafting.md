# Cards and crafting in Bloomfront: an exploration (2026-11-10)

**Status: brainstorm only. Nothing in the repo was edited, built or committed;
one census run used the already-built probe, read-only.**
Asked for by the owner, who is curious how card-game mechanics could combine
with the real-time mould. Read against `Reports/rts-design-2026-11-10.md`
(Bloomfront), `CLAUDE.md`, `src/mould/mod.rs`, `Reports/dead-ends.md`,
`Reports/mould-review-2-2026-11-10.md` and
`Reports/mould-game-2026-11-08/depth-2026-11-09/complexity.md`.

**A note on sources.** Every fact about another game below comes from a page
fetched in this session, with its URL beside it. The session's shared fetch
budget (100 an hour across every agent) ran out more than once, so some games
have fewer facts than planned. Anything not from a fetched page is labelled
*from memory, unverified*.

## Summary

- **Measured first: the food store is never a bank.** In The Wild One the
  store read 0 at the median and never above 108 over 56 samples (4 games),
  because births spend food the moment it arrives. "Pay from the store", as
  Bloomfront's Stage 1 is written, has nothing to draw on. Any card layer
  needs a currency first; a capped reserve filled by a share of eating is
  proposed (section 2).
- **Recommended: the Rack.** Each keeper brings an ordered rack of about ten
  tool cards. Three sit in the tray; playing one slides the next in. Cards
  are paid from the reserve and can only be played near your own fed tube.
  A few *tincture* cards bend the next tool you play, which is Noita's
  bind-to-the-next rule moved into a live hand.
- **Runner-up: the Strain.** A species built from trait cards, each bending
  one or two `Params`, drafted between dishes of a run. Cheap and very
  visible, but it does not fix the opening by itself.
- **Third: Recipes.** Tools composed in advance like a wand: a base, its
  additives and a hook that fires on a simulation event. Deepest discovery,
  highest risk of unreadable combos and of a new solved opening.
- **Why the Rack:** it attacks the measured weakness head on. The opening
  salt wall empties a small early reserve, comes back only after seven other
  plays, and meets a Rinse card in the rival's rack. It builds on
  Bloomfront's Stages 0 and 1 and on today's tools, so every card's effect
  is already the simulation.

---

## 1. Research findings

### 1a. Noita: spells composed in a wand, in a simulated world

**How a wand composes** (all from https://noita.wiki.gg/wiki/Guide:_Wand_Mechanics):

| Part | What it does |
| --- | --- |
| Wand stats | Shuffle (yes/no), spells per cast, cast delay (0.17 s in the guide's examples), recharge time (0.48 s, after the wand runs out of spells), mana max, mana charge speed, capacity (slots), spread |
| Read order | A wand without shuffle reads its spells left to right, "one spell at a time until all spells have been cast," then recharges and starts over |
| Shuffle | The wand first builds a random order of its spells, then casts through that list |
| Modifier | Attaches to the next projectile spell; several modifiers in a row all apply to it |
| Multicast | Grabs the next spells in the list and casts them together in one go |
| Wrapping | A multicast that needs more spells than remain takes them from the start of the list |
| Trigger and timer | Carry the next spell as a payload, fired at the projectile's location later: on hitting something (trigger) or after a set time (timer) |
| Shuffle with a trigger | Any spell can end up as the payload, which the guide warns can be dangerous at close range |

**Why it produces discovery.** The developers describe the simulated world
as the source. Petri Purho (one of the three developers) called it "this
emergence-generating machinery that keeps on generating these interesting
situations of play," and wanted players to "explore as a set of systems"
(https://www.gamedeveloper.com/game-platforms/road-to-the-igf-nolla-games-i-noita-i-).
Purho also said "there's always something new to be discovered, even for
us," and warned that "the simulation can, in fact, be in the way of more
interesting gameplay" (same page).

My reading of why the wand works, from the two pages above:

- **Few, uniform rules.** Read in order, bind to the next, carry a payload.
  A player can predict a short wand and be surprised by a long one.
- **The output is a physical thing in the world.** A combination's result
  is decided by the simulation, not a lookup table. Nobody has to author
  each pair.
- **Order is the craft.** The same parts in another order do something
  else, so a small inventory has many builds.

**Building is separated from firing.** Wands "can usually only be edited
while inside the Holy Mountain or with the Tinker with Wands Everywhere
perk" (https://noita.wiki.gg/wiki/Wands). Each spell and modifier charges
its mana cost when cast, and mana regrows at the wand's charge speed. An
always-cast spell sits outside the queue and is added to the front of every
cast (same page). *From memory, unverified:* the Holy Mountains sit between
the game's areas, so editing happens between fights.

That separation matters for Bloomfront. The craft is done calmly; the fight
is aiming a built thing. A live hand (the Rack) moves some of the craft
into the match instead.

**What Bloomfront takes:** bind-to-next and payload-on-event, applied to
the existing tools, with the simulation deciding the result.

### 1b. Deckbuilders and roguelite build-crafting

**The loop.** Dominion (Donald X. Vaccarino, 2008) is credited as "the
first deck-building game." Each player starts with seven Copper and three
Estate, draws five, buys cards from a shared supply into a discard pile,
and reshuffles the discard when the deck runs out
(https://en.wikipedia.org/wiki/Dominion_(card_game)).

**Clogging and thinning.** Dominion's victory cards "usually have no use
while the game is in progress," so buying them weakens later turns. Some
cards trash others, "removing them from players' decks" (same page). The
lesson: a deck's size is a stat. A smaller deck finds its best card sooner.

**Variable supply.** Players "decide on the kingdom cards to be used," and
"each game has a different assortment" (same page). The set of options
changes per game, not only the draw.

**Balancing from data.** Mega Crit balanced Slay the Spire with a metrics
server. They tracked how often a card is picked when offered, what it was
chosen over, and how often it is in winning decks
(https://www.gamedeveloper.com/design/how-i-slay-the-spire-i-s-devs-use-data-to-balance-their-roguelike-deck-builder).
Anthony Giovannetti warned that changing one card affects every strategy
it belongs to. Cards were added in batches to create archetypes, then one
at a time to "sculpt" play styles (same page).

**Run structure.** Relics can "flip a run," random events add chance, and
because the game is single-player "decks don't need to be equal," so
balance can serve fun (same page). Slay the Spire's run map and its
between-fight card rewards are not covered on the page I fetched.

**What Bloomfront takes:** deck size as a stat (the Rack's cycle length);
pick rates and ablation as the balance census, which matches this repo's
scripted-hand method; archetypes added as batches.

### 1c. Real-time card games and card-RTS hybrids

**Clash Royale** (https://wikiwand.com/en/articles/Clash_Royale):

- A deck of eight cards; four in hand, "randomly chosen" from the deck.
- Playing a card draws a replacement automatically.
- Elixir starts at five and regrows one point "every 2.8 seconds to a
  maximum of ten elixir."
- Matches last three minutes, then two of overtime if tied.

The official help page gives the rate's schedule: x1 in minutes one and two,
x2 in minute three, x2 in overtime's first minute and x3 in its second
(https://support.clashroyale.com/hc/en-us/articles/49484936933403-Elixir).

**What the hand under time pressure teaches.**

- A small hand from a small deck cycles fast, so a player can count when a
  key card returns. Timing becomes a skill.
- A capped resource is use-it-or-lose-it, so sitting still has a cost.
- Raising the rate late forces a climax. Review 2 found the bell rings
  while both moulds are crashing; a rising rate could make the late
  game a choice instead.

**BattleForge** (EA Phenomic, 2009) was "an online card-based real-time
strategy (RTS) game." Each element's set held "50 units, buildings and
spells," and "each power was represented by an orb color in the game"
(https://en.wikipedia.org/wiki/BattleForge). *From memory, unverified:*
players captured wells for the play resource and monuments that unlocked
higher tiers and colours on the map. The page did not cover this.

**Against our food economy.** Clash Royale's elixir is equal for both
players and regrows on a clock. Bloomfront's store would be earned by
eating, so it rewards the better network and risks a snowball. Section 2
measures that the store is empty anyway, which is why the Rack uses a
capped reserve.

### 1d. Strategy and builder games that hand out tools as picks

**Against the Storm** (Eremite Games), from the developers' devlog
(https://eremitegames.com/devlog-7-blueprint-drafting-system-and-more/page/2/?et_blog):

- A settlement starts with a small set of basic buildings.
- "Every few Reputation Points" the player picks "one choice (from a set of
  2-3)," and never sees the full list.
- "The goal of the system is to introduce variety and encourage the player
  to experiment."

**Loop Hero** (https://www.pcgamer.com/loop-hero-combos-cards-tile): the hero
walks the loop on their own; the player places tile cards. Each card's
effect depends on where it sits (road, roadside or field), how many of a
tile are placed, and how long a card has stayed. Examples from the page:

- A Meadow beside any other tile becomes a Blooming Meadow.
- A Desert beside a River makes an Oasis, and enemies' attack speed drops 1%.
- Mountains in a 3x3 make a Mountain Peak: +48% HP, but harpies spawn.

Loop Hero is the closest precedent for Bloomfront: **you never control the
actor; your cards change the world it walks through.**

**Mini Metro.** At the end of each in-game week the player gets one new
locomotive automatically, then chooses between two offered resources
(https://shapes.inc/fandom/mini-metro/upgrades). The options are carriages,
new lines, tunnels or bridges, and interchanges (same page). A carriage
roughly doubles a line's capacity
(https://hamy.xyz/blog/2024-03_mini-metro-best-upgrades). The simulation runs
on in real time while the player waits for the next pick, which is close to
Bloomfront's situation.

**What Bloomfront takes:** tools handed out as picks keep a player adapting
instead of repeating one opening; adjacency on a world map can be the combo
system; and the indirect actor makes the cards the only verb.

### 1e. Cards that change rules, not just spawn things

**Balatro** (https://www.destructoid.com/how-to-order-jokers-in-balatro/):
jokers change how a hand is scored, and they "generally resolve left to
right, so position matters." Additive +Mult before a multiplying XMult
roughly doubled the score in the page's own example (6,966 against
3,480 with the same three jokers). Order of rule changes is a craft.

**Magicka** (https://strategywiki.org/wiki/Magicka/Elements): eight base
elements combine in real time.

- Opposites cancel: cold is the "opposite of fire," lightning and earth
  cancel, life is "cancelled by arcane."
- Combinations change the effect's shape: arcane makes beams, earth makes a
  launched projectile, shield with water makes rainfall.
- The casting method changes it again: a shield cast normally is a wall,
  cast on yourself it is a personal shield.

Magicka shows composition can happen live, from a handful of parts, if the
cancel rules are physical and guessable.

**Loop Hero's** adjacency tiles (1d) are rule changes on the map: an Oasis
slows every enemy that fights near it.

**Hades** (https://primagames.com/tips/hades-what-are-duo-boons-how-get-duo-boons):
boons from the gods modify the player's strikes, shots, dashes and cast.
A duo boon combines two gods' effects and "you can only be offered one
after you've already accepted specific Boons from both gods." Example: Sea
Storm needs one listed Zeus boon and one Poseidon boon, and makes knock-away
effects also strike foes with lightning. Meta-progression tilts the odds:
spending Darkness on a Mirror of Night talent makes these offers likelier
(same page).

The principle: **a combo is offered only once you hold its parts.** The
player reaches it by a visible path, so it reads as earned, not as luck.

**What Bloomfront takes:** rule cards are `Params` bends with a visible
shape; order matters only where it is guessable; cancel rules should be the
simulation's own physics (the lamp already dries water; water melting salt
is `complexity.md`'s R2).

### 1f. Principles carried into Bloomfront

Principles only. No names, card names or terms are taken from these games.

| Principle | Seen in | Bloomfront's own form |
| --- | --- | --- |
| A few uniform composition rules; the world decides the result | Noita | Tinctures bind to the next tool; hooks fire on census events; the mould decides what happens |
| Build calmly, act live | Noita (editing at the Holy Mountain) | The rack is ordered before the match; the tray is played in it |
| Deck size is a stat; thinning finds your best card sooner | Dominion | Rack length sets how often a card comes back |
| A capped, regrowing resource; a rising rate forces a climax | Clash Royale | A capped reserve filled by eating; a late bloom season |
| Picks instead of a fixed toolbox keep players adapting | Against the Storm | New cards offered one of three between matches |
| The actor is never controlled; cards change its world | Loop Hero | Bloomfront's core: you lure, push and wall a mould you cannot order |
| Order of rule changes is a craft, if it is guessable | Balatro | Order matters only for tincture-before-tool, nowhere else |
| Live composition from a few parts, with physical cancels | Magicka | Cancels are the simulation's own: Rinse undoes salt, the lamp dries water |
| A combo is offered once you hold its parts | Hades | A strain is offered a paired trait only when it holds both halves |
| Balance from pick rates and removal | Slay the Spire | Card ablation in the scripted-hand census |

---

## 2. What any card layer must answer

The measured weakness, from `Reports/mould-review-2-2026-11-10.md`:

| Finding | Number | What a card layer must do |
| --- | --- | --- |
| The first star is the opening salt wall | Stopping at 30 s keeps it on 8 of 8 seeds | Make the opening a cost and a gamble, not a free best move |
| Seconds 30-150 barely matter | Clumsy play to the bell beats letting go at 30 s 7-0 with a tie (p 0.016), median lead 1.93x against 1.81x; no star moves | Put real, recurring decisions in the middle |
| Stars 2 and 3 are an aiming test | Exact aim 2-3 stars on 7 of 8; three cells off, 1 star on 7 | Move skill from aim to choice and timing |
| The snip delivers nothing | Three cuts through a packed nest: 0 stars on 8 of 8; a wound heals in 4 s (`Params::wound`) | Give the cut a version that bites, and show the starving |
| Free tools | Stocks regrow on a timer (`OATS_REGROW_SECONDS` 15) | Every play costs food, so spending is the decision (Bloomfront's one rule) |

Two rulings bind every idea below:

- **What a card does must show up as the simulation.** The hand, costs and
  timers live off the plate as interface. On the plate there is only trail,
  walkers, ground, light, scent, wet and food.
- **Every card must deliver something you can see.** A card that changes a
  hidden number with no change in the mould's shape is cut.

One fact from the code shapes the economy. With `Params::flow` off (today's
default), a fed tube is drawn in one gold whatever it holds
(`look.rs`, `FED_NO_FLOW`). So a payment from a piece shows only as slower
growth and, if the piece runs dry, as hungry walkers dying from the far end.
With `flow` on, the walkers' load is drawn on a ramp (`food_level`), and a
payment visibly dims the piece that paid. The card layer reads best with flow
on.

### Measured here: the food store is never a bank

Bloomfront's one rule is that every tool is paid from the food store. I
traced the store in The Wild One to see how much there is to spend
(`LEVELTRACE=1 mouldprobe level=5 players=idle,warden seeds=1,2`, the built
probe at `b8efc14`'s code; raw output kept at
[`storetrace.txt`](storetrace.txt),
which should move under `Reports/` if this note becomes a report).

| Hand | Seed | `Dish::store` every 10 s, 10-140 s | Mould (walkers) | Share |
| --- | --- | --- | --- | --- |
| idle | 1 | 0-108 | 874-1,140 | 0.495 |
| idle | 2 | 0-48 | 900-1,099 | 0.494 |
| warden | 1 | 0-13 | 1,200-1,608 | 0.645 |
| warden | 2 | 0-6 | 1,198-1,704 | 0.653 |

Over 56 samples the store's median is 0 and its maximum 108, against a start
of 1,500. A birth costs 1 food, and a fed walker may divide whenever its
piece holds 1 or more (`economy_by_piece`), so food turns into walkers the
moment it arrives.
The mould lives at its food ceiling: one pile feeds `table x eat / upkeep` =
40 x 0.03 / 0.004 = 300 walkers.

**So "pay from the store" has nothing to draw on.** The design review's
"Small" cost for Stage 1 assumes a balance that does not exist. Any card
layer, and Bloomfront itself, needs a currency first. Three candidates:

| Currency | How it works | What you see | Catch |
| --- | --- | --- | --- |
| **Mould** | A card removes N walkers from the paying piece, farthest from food first (the hunger rule's order) | Your tips thin where you paid | At the ceiling a lost walker frees 0.48 food a second of upkeep, so the dent may refill in about 2 s (estimate, unmeasured) |
| **Plate food** | A card takes food from the piles feeding the paying piece | Their fill arcs drop | The big piles still held 60-83% at 140 s, so food spent from them may never cost a point |
| **A reserve** | A fixed share of what your mould eats (say 15%) goes to your keeper's reserve, capped; cards spend it; a card can pour it back into the mould as births | A mould about 15% under the ceiling; a surge of new walkers where you pour it back | Snowball: income follows eating. The cap and fixed card effects limit it |

The reserve is the one I would test first. It keeps Bloomfront's rule
(tools are paid with food the mould would have grown on), it gives a number
that rises and falls on a human time scale, and pouring it back makes "grow
or act" a real, visible choice. The cap borrows the use-it-or-lose-it
principle of real-time card games (section 1c).

---

## 3. Three ways cards could sit in Bloomfront

The names below are this design's own. Lab words were picked on purpose so
nothing borrows another game's terms.

| | A. The Rack | B. Recipes | C. The Strain |
| --- | --- | --- | --- |
| What a card is | A tool use, a tool modifier, or a short rule change | A slot in a composed tool | A lasting trait of your species |
| When you choose | Every few seconds, in the match | Between matches; in the match you only aim | Between dishes of a run; one or two taps in a match |
| Main principle borrowed | A cycling hand under a regenerating cost (1c); order binds modifiers (1a) | Order and composition make emergent results (1a) | Drafting, thinning, synergy across a run (1b) |
| Answers the opening | Strongly | Partly | Weakly |
| Cost to build | Medium | Large | Small to medium |

### A. The Rack: a cycling tray of tool cards (recommended)

**The parts.**

- **Rack:** about ten cards you bring, in an order you set before the match.
  No shuffle in competitive play, so the draw is never luck.
- **Tray:** the three cards you can play now. The next card is shown beside
  it, off the plate.
- **Cycle:** a played card goes to the back of the rack. With ten cards and a
  tray of three, a card comes back after seven other plays.
- **Reserve:** the currency from section 2, filled by a share of what your
  mould eats, capped.
- **Reach:** a card can only be played within a few dozen cells of your own
  fed tube. Your network is your play space, so a forward tube is a forward
  base, and a cut supply line also takes away where you can act.
- **Tinctures:** a few cards bind to the next tool you play and change it.
  This is the wand's modifier-before-projectile rule (1a), in a live hand.

**Minute to minute,** on Stage 0's mirrored two-keeper plate, five minutes:

- **0:00.** The tray holds, say, Oat Drop, Salt Line and Cinch. The reserve
  is small. Salt now buys the old winning wall but empties the reserve, so
  your next two plays wait on eating.
- **0:00-1:00.** You expand with oats and Cinch, and each play slides the
  next card in. Your salt card is now at the back: it returns after seven
  plays, which is how much you choose to spend.
- **Every 40 s or so** a bloom is announced 10 s ahead (Bloomfront's blooms;
  the preview is the re-test condition the late-seam-piles dead end names).
  What sits in your tray then decides how you contest it. Cycling so a lure
  and Brazen are both in hand at 1:20 is the mid-game skill.
- **Later.** A Long Wound knife across the rival's supply to the bloom: the
  far side greys and thins over 10-20 s, if it holds no pile of its own.

**Decisions it creates.**

- Build: which ten cards and in what order. A seven-card rack cycles your
  best card faster but carries fewer answers (the thinning trade, 1b).
- Spend now, hold for the bloom, or pour the reserve back into growth.
- Which of three to play. Holding a card blocks a tray slot.
- Where: only near your fed tube, so expansion opens new places to act.
- Which tool gets the tincture.

**How it answers the measured weakness.**

- The opening wall becomes a gamble. It empties a small early reserve, so
  the next plays wait on eating and nothing is left to pour into growth.
- The rival's rack can carry **Rinse**, which opens salt. Today nothing can.
- The cycle brings salt back mid-game, so walls come in waves you time.
- Announced blooms give recurring races the tray decides, not who happened
  to be nearer.
- Card effects work on areas and on the mould's rules, so choice and timing
  carry more of the skill and aim within 2 cells carries less.

**Risks.**

- **Snowball.** Income follows eating, so a leader plays more cards. The cap,
  fixed card sizes and upkeep (0.004 a walker a step) push back. The mirror
  census must show it.
- **Rule bloat.** Launch with nine cards (section 5, step 1). Each card
  says one line.
- **Luck.** No shuffle in competitive play; blooms placed in mirror.
- **Time pressure.** The mould answers over seconds, so a decision taking
  half a second costs little. That is unusual for a real-time card game.
- **Owner's rules.** A tincture that only adds a timer must show the wait as
  a process on the plate: a wall that crystallises cell by cell, never a
  countdown drawn on the dish.
- **Dead ends.** The lamp must stay a push (`light_weight` entry). Blooms need
  the preview (late-seam-piles entry).

**Cost: medium.** Code it touches:

- `mod.rs`: a reserve on `Dish` fed from `economy_by_piece`'s eating; tools
  that check reach (`piece_of`) and take a cost; per-use options on strokes
  (`Stroke` gains wound seconds, width and a dab delay); a timed overlay on
  `params` for rule cards (field, value, until); a salt-to-agar disc for Rinse.
- A new `cards.rs`: the card list, rack, tray, costs, tincture binding, and
  one `play_card(&mut Dish, card, x, y)` entry for windows and hands alike.
- `level.rs`: a mirrored plate, two racks, blooms with previews.
- `play.rs` and `mouldprobe.rs`: hands that see the tray. Rack-warden,
  rack-random, rack-dump (plays each affordable card the moment it can, at
  fixed spots), idle, plus the existing `STOPAT`, `HJIT` and `HLAG`.
- Drawing: the tray and costs, off the plate.
- **Frame cost:** cards add only an overlay check and a few queued dabs a
  step; expected under 0.1 ms, to be measured with the probe's frame timer.

**Measurement, named before the run.** Deciding measure: food share at the
bell (`food_share`), 16 paired seeds.

| Gate | Hands compared | Passes when |
| --- | --- | --- |
| 1. The middle matters | Rack-warden to the bell against the same hand with `STOPAT=30` | Play-on wins 13+ of 16 (p 0.02) and by a median 0.05 share. Today: 1.93x against 1.81x lead, no star moves |
| 2. The opening is not solved | Rack-warden against a wall-only hand (salt at 0:30, nothing more) | Rack-warden wins 13+ of 16 |
| 3. Choice beats aim | Clumsy rack-warden (`HJIT=6 HLAG=2`) against rack-random and rack-dump | Clumsy wins 13+ of 16 against each |
| 4. Fair | Rack-warden against itself, mirrored | Share 45-55% on every seed |
| 5. Every card earns its slot | Rack-warden with each card removed in turn | Each removal moves share; a card that moves nothing is cut, as the snip should have been |

About 15 comparisons, so the strict bar is 0.003: 15-1 (p 0.0005) passes,
13-3 (p 0.02) is a direction only. Quote both.

### B. Recipes: tools composed like a wand

**The parts.** You own three **droppers**, built between matches. A dropper
is a row of two to five slots, read left to right:

- **Base:** oats, lamp, salt, knife or water. One base fires per use.
- **Additive:** binds to the next base in the row (wide, long-lasting,
  slow-setting, thin).
- **Pair:** the next two bases fire together at one click.
- **Hook:** what follows rides a carrier, a real object on the plate, and
  fires on an event the census already reads. Events: a mould reaches the
  carrier (`Pile::eaters` or `Pile::foe_eaters` above 0), a join flashes
  (`Dish::snaps`), the carrier's ground goes unfed (`Dish::fed`).
- **Dropper stats:** slots, refill seconds and cost per use. The dropper is
  the build, as a wand's stats are (1a).

**Minute to minute.** No hand to manage. You aim and fire three droppers;
each refills and costs reserve. The thinking was done before the match.
Hooks put prediction in the middle: you place a carrier where the rival's
front will be in ten seconds.

**Three recipes, to show the shape.**

- *Tripwire:* Hook (rival arrives) + Long + Knife. An oat pile that, when the
  rival's walkers start eating it, cuts a long-lasting wound behind them.
- *Slow fence:* Slow-setting + Salt. A wall that grows from one end over 5 s;
  the rival can see it close and race the gap.
- *Wet bait:* Pair + Water + Oats. Oats on damp ground, to wake a sleeping
  reserve beside food (needs `Params::wet`).

**Discovery.** Order changes results: Wide + Salt + Knife widens the wall;
Salt + Wide + Knife widens the cut. Hooks keyed to the simulation do things
nobody scripted, such as a trap your own front springs.

**Answer to the weakness.** Refill times stop spamming, and hooks reward
prediction mid-game. But a single best opening recipe could replace the
single best opening move.

**Risks.** The rival cannot read what a carrier holds, so a hidden payload
must have a visible property: a baited pile holds salt, shown as its grain,
and gives off less scent. A crafting screen is a lot of interface. Balance
grows with the square of the parts.

**Cost: large.** Everything in A, plus carriers and event watchers (checked
at the census, every 6 steps, so cheap per frame), the crafting screen, and
an AI-against-AI census over recipe pairs.

**Measurement.** A's gates, plus two: no recipe among the top 20 wins over
60% against the field, and each hook recipe beats the same bases fired by
hand on paired seeds.

### C. The Strain: species built from trait cards across a run

**The parts.** A run is five to seven dishes against keeper AIs that
sharpen as you go (`HJIT` and `HLAG` already make difficulty). Your strain
starts with three traits. After each dish you take one of three offered, or
splice one out. Each trait bends one or two `Params` for good, with a cost.
Some traits can be **expressed** once a dish: a tap pushes them further for
15 s at a reserve cost.

**Minute to minute.** You play with the ordinary tools or the Rack. The
strain sets how the mould answers them. Expression taps add one to three
timed decisions a match.

**What you see.** Builds look different on the plate: 22.5-degree lace,
45-degree taut cables, fat trunks from a higher deposit, long thin fingers
from a longer feed reach, a bold border from a low foe repel.

**Decisions.** Which trait to take, which to cut, which dish next, and when
to express.

**Answer to the weakness.** Weak alone: traits are set before the match, so
the opening wall still works unless a trait changes which openings pay.
Its strengths are replay variety, Bloomfront's species (Stage 4) nearly for
free, and a run structure The Wanderer's carry-over already half builds.

**Risks.** A dominant stack, such as low repel with high divide. Bends past
the model's sane range: the sensor-reach dead end shows one cell less reach
lost Beat the Ruler half its seeds. Grind, if unlocks gate fun.

**Cost: small to medium.** A trait list applied to `Params` when a dish is
built (`Level::params_for`), a draft screen between dishes, and `wander.rs`
for carrying the mould on. The expression tap is A's timed overlay.

**Measurement.** Every trait pair, AI against AI, 16 seeds: each pair 40-60%
share (Bloomfront's Stage 4 gate). Before that, a picture gate: each trait
must look different in `mouldlooks` stills at 60 s, shown blind.

---

## 4. Example cards

Twenty cards, written for the Rack (approach A). Each one touches a real
lever in `src/mould/mod.rs` or a tool already in the game. Costs are tiers
(low, mid, high) to be set from the census, never guessed. Rule cards last
15-20 s and bend only the player's own mould, through a timed overlay on
`Dish::params`.

### Tools

| # | Card | Cost | Lever in the code | What you see on the plate | Answered by |
| --- | --- | --- | --- | --- | --- |
| 1 | **Oat Drop** | Low | `place_oats`: a 400-food pile, radius 3, gone after 35 s (`OATS_FOOD`, `OATS_R`, `OATS_SECONDS`) | A small pile; the nearest front turns toward it. It feeds whichever mould eats it (the `Dish::rival` doc: "Oats lure and feed both") | A rival front that gets there first; Salt Line across the approach |
| 2 | **Lamp** | Mid, per second held | `set_lamp`, `push_light`: trail and walkers carried outward (`push` 0.9, `lamp_reach` 16); with `wet` on it dries the ground (`dry_light` 0.05) | A light disc; tubes bend aside like a pushed rope, either mould's | The rival's own lamp: both lamps light one shared field (Bloomfront Stage 0) |
| 3 | **Salt Line** | High | `begin_stroke(Tool::Salt)`: up to 60 cells (`SALT_LEN`) of `Ground::Salt`, three cells wide; kills walkers under it; zeroes trail and scent | A white wall; fronts and scent stop at it | Rinse |
| 4 | **Knife** | Low | `begin_stroke(Tool::Snip)`: up to 30 cells (`SNIP_LEN`) of `Ground::Wound`, healing after `wound` = 4 s | A cut; the tube rejoins about 4 s later | It heals by itself |
| 5 | **Rinse** (new) | Mid | `Ground::Salt` back to `Ground::Agar` in a disc of radius 6, the reverse of a salt dab | White cells vanish; the front pours through the gap | Another Salt Line |
| 6 | **Water Drop** | Low | `place_water`: a wet disc of radius 12 (`water_r`); sleepers above `revive_wet` 0.7 wake. Needs `Params::wet` | Darker, damp agar; amber sleepers stir and move again | Lamp, which dries it |

Rinse is the cheap first form of `complexity.md`'s "water melts salt"
(its R2). It is what makes the opening wall answerable.

### Tinctures: each binds to the next tool you play

| # | Card | Binds to | Lever in the code | What you see on the plate | Answered by |
| --- | --- | --- | --- | --- | --- |
| 7 | **Long Wound** | Knife | That stroke's wound time 4 s to 16 s (`Params::wound`, read per stroke into each wound cell's heal step) | The cut stays open. A cut-off side with no pile lives on its own share (`Params::pieces`), greys, and dies at about 5 s mean life (`starve` 1/600 a step) | Set the Route; a loop that feeds round the cut |
| 8 | **Slow Set** | Salt Line | The stroke's dabs are queued and laid one at a time over 5 s from the first end; cheaper | The wall draws itself across the plate and the gap closes in sight | Racing through the gap |
| 9 | **Wide** | Lamp, Water, Salt, Oats | Radius x1.5: `lamp_reach` 16 to 24 with `push` 0.9 to 0.6; `water_r` 12 to 18; salt dab 1.5 to 2.25; oats radius 3 to 4.5 with the same food | A bigger disc of light or damp; a fatter wall | Costs double |
| 10 | **Twice** | Any tool | The tool fires again at the same point 8 s later, at half price | The effect repeats; a lamp pushes twice; oats land where the front will be | Reading it: the first firing shows where the second lands |
| 11 | **Baited** | Knife or Salt | The tool rides a new oat pile and fires when the rival first eats there (`Pile::foe_eaters` above 0 at a census, every 6 steps). The pile holds the salt or blade as a real property, drawn as its grain, and gives off half scent (`food_deposit` for that pile) | A dimmer-smelling oat pile with a grain in it; when the rival reaches it, a wall or cut springs round it | Not eating grainy oats; Sweeten a different pile |

Long Wound is the direct answer to review 2's "the snip delivers nothing".
It bites only where the cut-off side holds no pile: the `STORM_CUTS` dead
end records that pieces which each hold a pile are each fed.

### Rule cards: bend your own `Params` for a while

| # | Card | Lever in the code | What you see on the plate | Answered by |
| --- | --- | --- | --- | --- |
| 12 | **Cinch** | `sensor_angle` 22.5 to 45 degrees (The Wild One runs at 22.5 with `rotate_angle` 45) | Branches pull in; tubes go taut and straight; the mould obeys oats. This repo measured it: at 45 the mould "obeys every oat and then freezes", at 22.5 it joins all the food untouched (`Reports/mould-review-2026-11-10.md`; `wiki/model/sensors.md`) | Its own end: a frozen mould stops exploring |
| 13 | **Thick Trail** | `deposit` 5 to 10 | Tubes fatten and brighten. The rival reads your trail as repulsion (`foe_repel` times your trail, in `sense`), so your border pushes into theirs | Brazen |
| 14 | **Brazen** | `foe_repel` 1.0 to 0.3 | Your front presses into the rival's tube; the seam moves; deaths rise on both sides (`counts.fought`). At 0.3 with fights off the wild mould took 90-99% of the dish (`level.rs`, `WILD_REPEL`'s note) | Dig In |
| 15 | **Dig In** | The fed-tube fight factor 0.5 to 0.25 (a constant in `fight_losses` today; Bloomfront already plans it as a field) | Where your tube is fed, your walkers at the seam die half as often, so the seam creeps toward the rival | Long Wound across your supply: an unfed front's factor is 2.0 |
| 16 | **Lean Time** | `divide` x0.25; the reserve's share of eating doubled | The front stops advancing and holds; the reserve fills, off the plate | The rival pushes in while you bank |
| 17 | **Spurt** | `divide` 0.03 to 0.06 and `crowd` 7 to 9 for 10 s; pours the reserve back as births | Fronts bulge and fill gaps; a rush of new walker specks | A Knife at the root of the spurt |
| 18 | **Far Reach** | `feed_reach` 70 to 100 (flow off) | Grey tips turn gold again and keep growing toward a far bloom | A Knife nearer the root |

### Plate cards: change the food both moulds share

| # | Card | Lever in the code | What you see on the plate | Answered by |
| --- | --- | --- | --- | --- |
| 19 | **Sweeten** | One pile's scent doubled for 15 s (`food_deposit` for that pile), and `fed_scent` (0.15) ignored while it is reached | Both moulds' fronts turn toward it, the rival's too: bait them onto your salt | Salt Line across the approach |
| 20 | **Set the Route** (`flow` on) | `wall_grow` 1 to 3 for 20 s | Busy tubes widen into walled trunks; after a cut the tube regrows along the old wall (`wall_pull`) | Long Wound held longer than the wall's half-life (`wall_half_life` 30 s) |

### What was left out, and why

- **A darker lamp, or a lamp that repels by the sensors.** The
  `light_weight` dead end: the sensor lamp cut the join on 5 of 5 seeds.
- **A shorter sensor reach** as a "fine tubes" card. The `sensor_offset`
  dead end: reach 6-8 made every planned solution worse. A *longer* reach
  (a "keen scent" card, 9 to 12) is untested and would need its own census.
- **A pulse card with flow off.** The owner rejected the drawn pulse. With
  flow on, a faster `pulse_seconds` moves real sap, and could come later.
- **Late food that just lands.** The late-seam-piles dead end. Blooms are
  announced, and Sweeten works on a pile already in sight.

---

## 5. Recommendation

**Build the Rack (approach A), small, and measure it before adding more.**

### Why the Rack

- **It aims at the measured weakness.** The game is decided at 0:30 and the
  middle is a steadiness test. The Rack makes the opening a priced gamble,
  gives the wall an answer (Rinse), brings tools back in timed waves, and
  makes announced blooms a race decided by the tray.
- **It moves skill from aim to choice.** Most cards act on an area or on
  the mould's own rules, so a clumsy but thoughtful hand can win. Gate 3
  tests exactly that.
- **Every card is already the simulation.** Eleven of the twenty cards are
  today's tools, Rinse, or an option on a tool; the other nine bend a
  `Params` field whose effect on the mould's shape is visible. Nothing is
  drawn on the dish.
- **It reuses planned work.** Bloomfront's Stage 0 (two keepers) and Stage 1
  (spend to act) are its foundations. The Rack is Stage 1 with a hand.
- **The wand principle survives in it.** Tinctures bind to the next tool,
  Baited carries a payload fired by a simulation event, Twice delays a
  repeat, and the rack's fixed order is a wand read without shuffle.
- **It suits a scripted-hand census.** A hand picks one of three cards and
  a point. Random, dump and idle hands are natural controls.

### Runner-up: the Strain

Traits as `Params` bends are the cheapest layer and the most visible, and
they give Bloomfront its species almost for free. They come second because
they are chosen before the match, so they do not fix the middle of it. Add
them once the Rack passes its gates, as two or three traits per species.

### Not yet: Recipes

The deepest discovery, and the owner's own example. But it moves the solved
opening into a crafting screen, costs the most, and makes the rival's plays
hard to read. Its best ideas (bind-to-next, hooks on simulation events) are
already in the Rack as single cards. Revisit if players ask to build.

### The order to build it

| Step | Builds | Gate before the next step |
| --- | --- | --- |
| 0. Currency probe | The reserve: a share of eating set aside, capped; sweep the share 0, 10, 20, 30% with idle and warden hands | Idle mirror stays 45-55% at every share; mid-match the reserve affords a card every 5-15 s, a human pace |
| 1. A nine-card rack | Oat Drop, Lamp, Salt Line, Knife, Rinse, Long Wound, Slow Set, Cinch, Brazen (Water Drop joins once a plate runs wet); a rack of ten from these, copies allowed; the tray off the plate; reach from fed tube | Gates 1-5 of section 3A, 16 paired seeds |
| 2. Batches | Add rule and plate cards in batches of three or four that form a style (a border style: Thick Trail, Dig In, Sweeten) | Each new card passes ablation; the mirror stays fair |
| 3. Strains | Two or three traits per species | Each species pair 40-60% |
| 4. Recipes, maybe | Droppers between matches | Players ask for it |

Quote the frame cost at steps 1 and 2: the rack adds a timed overlay check
and a few queued dabs a step, expected under 0.1 ms.

### Main risks

| Risk | Where it bites | Guard |
| --- | --- | --- |
| No currency | The store is empty all match (section 2) | Step 0 decides the currency before any card exists |
| Snowball | Income follows eating | Cap; fixed card sizes; the mirror census |
| Rule bloat | Twenty cards, each with a lever | Launch with nine; add in measured batches |
| Unreadable combos | Tinctures and Baited | Order matters only for tincture-before-tool; a baited pile shows its grain |
| A new solved opening | Rack order | Gate 2; Rinse in the card pool; reserve small at 0:00 |
| Luck | Shuffled draws | No shuffle in competitive play; mirrored blooms |
| The owner's rulings | Delays and payloads | Every wait is a process on the plate (Slow Set crystallises); costs and timers stay off the plate |

### Questions for the owner

- Is a baited pile's payload allowed to be hidden, if the pile shows a
  grain? Or must every property be readable at a glance?
- Is a reserve number off the plate acceptable, like a bank shown in the
  bar? Its effect on the plate is a mould a little under its food ceiling.
- Shuffled racks for casual play, or never?
- Does the empty store change Bloomfront's Stage 1? The design review's
  "Small" cost assumed a balance that does not exist.

---

## 6. Sources

### Fetched in this session

| Page | Used for |
| --- | --- |
| https://noita.wiki.gg/wiki/Guide:_Wand_Mechanics | Wand stats, read order, shuffle, modifiers, multicast, wrapping, triggers and timers |
| https://noita.wiki.gg/wiki/Wands | Editing only at the Holy Mountain or with a perk; mana per spell; always-cast |
| https://www.gamedeveloper.com/game-platforms/road-to-the-igf-nolla-games-i-noita-i- | Petri Purho on the simulation as "emergence-generating machinery" |
| https://en.wikipedia.org/wiki/Dominion_(card_game) | The first deck-building game; the loop; victory cards clogging; trashing; kingdom cards |
| https://www.gamedeveloper.com/design/how-i-slay-the-spire-i-s-devs-use-data-to-balance-their-roguelike-deck-builder | Slay the Spire's metrics, archetypes in batches, relics that "flip a run" |
| https://wikiwand.com/en/articles/Clash_Royale | Eight-card deck, four in hand, auto draw, elixir start, rate and cap, match length |
| https://support.clashroyale.com/hc/en-us/articles/49484936933403-Elixir | The elixir rate's schedule by minute |
| https://en.wikipedia.org/wiki/BattleForge | A card-based RTS by EA Phenomic, 2009; units, buildings and spells; orb colours |
| https://eremitegames.com/devlog-7-blueprint-drafting-system-and-more/page/2/?et_blog | Against the Storm's blueprint picks and the developers' stated goal |
| https://www.pcgamer.com/loop-hero-combos-cards-tile | Loop Hero's tile placement and adjacency combinations |
| https://shapes.inc/fandom/mini-metro/upgrades | Mini Metro's weekly locomotive and one-of-two pick; the upgrade types |
| https://hamy.xyz/blog/2024-03_mini-metro-best-upgrades | What a carriage does to a line's capacity |
| https://www.destructoid.com/how-to-order-jokers-in-balatro/ | Balatro's left-to-right joker order and its effect on score |
| https://strategywiki.org/wiki/Magicka/Elements | Magicka's elements, opposites, and shapes by combination |
| https://primagames.com/tips/hades-what-are-duo-boons-how-get-duo-boons | Hades' duo boons and their prerequisites; Mirror of Night odds |

### Could not be fetched

- https://wiki.hoodedhorse.com/Against_the_Storm/Blueprints failed at its
  site; the developers' devlog was used instead.
- https://noita.wiki.gg/wiki/Deck (the wiki's draw guide) was refused when
  the shared fetch budget ran out; the wand-mechanics guide covers its core.
- Labelled *from memory, unverified* in the text: where Noita's Holy
  Mountains sit, and BattleForge's wells and monuments.

### In this repo

- `Reports/rts-design-2026-11-10.md`: Bloomfront, its stages and gates.
- `Reports/mould-review-2-2026-11-10.md`: the opening wall, the 0:30
  let-go, aim within 2 cells, the snip delivering nothing.
- `Reports/dead-ends.md`: `light_weight`, `sensor_offset`, `STORM_CUTS`, late
  seam piles.
- `Reports/mould-game-2026-11-08/depth-2026-11-09/complexity.md`: water melts
  salt (R2), crust, spores; built on, not repeated.
- `src/mould/mod.rs`: `Params`, `place_oats`, `begin_stroke`, `stroke_dab`,
  `set_lamp`, `push_light`, `place_water`, `sense`, `fight_losses`,
  `economy_by_piece`, `Pile::bite`, `Dish::step`.
- `src/mould/level.rs`: The Wild One (22.5 degrees, `WILD_REPEL`,
  `params_for`).
- `src/mould/look.rs`: `FED_NO_FLOW`, `food_level` (how food is drawn).
- `wiki/decisions.md`, `wiki/tools/knife.md`, `wiki/model/sensors.md`.
- The store trace run here:
  [`storetrace.txt`](storetrace.txt).
