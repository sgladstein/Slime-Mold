# Flashes of Chaos guns, and what Larder World can take

2026-11-12. Research only, no runs. Asked for by the owner (2026-11-12 15:22Z):
"Is there any info only on how their gun/projectile system works and what we can
learn from it. It is really fun and could be incorporated into our game. I know
it is inspired by noitas wand system." Live doc:
https://claude.ai/artifact/JHYpQaByeswdUuTX3zFY3S (this file is its copy).

## Short answer

Flashes of Chaos is too new to have a public breakdown of its gun rules: it
launched on Steam on 6 November 2026, and there is no wiki, dev blog or
interview yet. What the store page, press release and player reviews do confirm
is the core idea it takes from Noita: **a gun is a set of slots, the cards in
the slots decide what the shot does, and modifiers and triggers combine into
builds** like "machine gun bullets that trigger a rotating laser beam that
triggers an electric field every hit".

The lesson for Larder World is that a shot can be a small program the player
writes. In Noita that program pays off because the world is simulated and
reacts. We have the same thing: every card can write into one of the mould's
fields (trail, scent, alarm, crater, scar, sleep, germ), so the mould is what
makes each combination pay off. That also gives each tier a different best
loadout, which is the counter-play the owner asked for.

## What is known about Flashes of Chaos

A top-down roguelike shooter by Brainburn Studio of Utrecht. Demo 8 August
2025, full game 6 November 2026. The developer never names Noita, but five of
the ten top reviews do, and one calls it "Hotline Noita".

| Mechanic | What the sources say | Confidence |
| --- | --- | --- |
| Three guns a run | Primary, Secondary and Melee | Confirmed: store page, press release |
| Ammo-card slots | "Each weapon has ammo card slots that define its behavior" | Confirmed: store page |
| Cards move between guns | "All ammo cards are interchangeable across weapons" | Confirmed: store page |
| Modifiers stack | "modifiers that stack and synergize" | Confirmed: store page |
| Trigger plus payload | A shot that triggers a second effect on hit, chained more than once | Player review |
| Editing a card's stats | A "FlexMod/Sticker system" lets you "modify the card's stats directly" | Player review |
| Melee gets modifiers too | A saw with a teleport modifier flashed in front of the enemy, then sawed everything nearby | Player review |
| Friendly fire | The same reviewer "instagibbed myself from sawblades going everywhere" | Player review |
| No duplicate modifiers in one shot | Developer, demo forum: "Not allowing double modifiers in a shot is a design choice", because doubles "would allow builds that crash the game" | Developer |
| Aggression pays | Kills pay loot, temporary perks and card draws | Store page; perks from a pre-release article |
| Damage types, perks, more cards | Added in the full game | Store page |
| Example cards | Piercer, Splitter, Ricochet, Cluster, Incendiary, Homing | Pre-release article only, unverified |

Still unknown: the firing order, whether a gun reloads or cycles its deck, slot
counts, and how a trigger's payload is chosen. That would take playing the free
demo or watching a video, which this thread could not do.

Two complaints point to the gap between it and Noita: walls are hard to tell
from the floor, and one player asked for "breakable obstructions, breakable
windows, things to push around with explosions". The world does not react to
the guns much. One negative review found most builds collapsed into an LMG, a
sniper or a shotgun, with some cards that don't interact.

## Noita's wand system, the parent design

Wand stats from the Noita wiki's Wands page; the Wand Mechanics page refused the
fetch, so the modifier and trigger rules below are from general knowledge of the
game, not re-checked.

- **The deck.** Capacity sets the slot count (1 to 26 in practice). A wand
  without shuffle casts its slots in order, then waits out its recharge time.
- **The budget.** Spells per cast, cast delay (each spell adds its own), mana
  per card. A powerful combination is slow or expensive.
- **Modifiers** change the next projectile drawn, so order matters.
- **Multicast** fires the next 2-4 spells as one volley.
- **Triggers** ("with trigger", "with timer", "with expiration") carry the next
  spells as a payload, cast on hit, on the timer or on death; payloads nest.
- **Always cast**: one spell glued to the front of every cast.
- **Shuffle** breaks careful chains, so players avoid it.

What makes Noita more than a deck builder is the simulated world: fire burns
wood, digging carves tunnels, water puts out fire, and a mistake can burn or
flood the player.

| | Noita | Flashes of Chaos |
| --- | --- | --- |
| Where builds live | In each wand's slots | In portable ammo cards across three fixed guns |
| Budget | Mana, cast delay, recharge, capacity | Not public |
| Chains | Trigger, timer, expiration payloads, nested | Trigger plus payload, nested at least twice |
| Doubling up | Mostly allowed; builds can lag or crash | One copy of a modifier per shot |
| Editing a part | Swap spells | Stickers change a card's stats |
| The world's part | Central: falling sand, fire, liquids, digging | Small: tiled map, no breakable cover |

## Principles worth taking

1. A shot is a short program: payload, modifiers, triggers.
2. Order matters; the same cards in another order are another weapon.
3. Chains (trigger inside trigger) are the fun.
4. A budget (fire rate, slots) keeps more fun than a ban.
5. The world multiplies the cards; where it is inert, builds converge.
6. Builds can hurt you, a cost the player chooses.
7. Progress lives in portable cards.
8. Cap what multiplies (chain depth, live shots) from day one.

## Ideas for Larder World

A **round**: a soldier's or turret's ammo, up to four cards in order, each
writing a field the mould already reads. Today's shot (stops at the first slime
cell, trail 26/255, or a wall; strikes a disc; lays a crater for 10 game s and
the tier's alarm) is the default round of one payload card.

Payloads: crater (`Rules::crater`), bait (food scent, as the lure), alarm spike
(the tier's alarm), germ, sleep (sleep gas), scar (the Wary and Grudge field).

Modifiers: pierce (pass thin slime, strike the first thick cell), scatter
(`Rules::spread`), wide or long (bigger disc or longer crater, slower fire),
ricochet (off rock and walls), quiet (no alarm), seek (toward the densest
block in sight).

Triggers, the part neither game has, because they read the mould: on hit, on a
timer (lodges, fires after the mould flows back), when thick (trail passes a
level), carried by the mould (sticks to walkers, fires along the tubes), on
alarm.

Counter-play: alarm rounds are a mistake against the Swarmer; quiet rounds suit
the Sleeper; a decoy scar pulls the Grudge and pushes the Wary; carried germ
suits the Sporer and Swarmer, where the toolkit census found plague best.

RTS fit: loadouts, not aiming; turrets take a round; cards bought with crystal
charge at the larder (a sink for food that doesn't bind); the budget is fire
rate and slots, not food per shot (shots are free since the owner's playtest).

## Risks

- Take the structure, not the content: no card names, art, three-gun layout,
  setting or tone from either game.
- RTS pace: edit loadouts at the larder or between missions; four slots first.
- Builds that converge: every card writes or reads a field, none only change
  damage.
- Exploit builds: search loadouts with the player AI and census.
- Frame cost: fixed GPU shot buffer, chain depth cap about 3, measured on the
  1070 (today 8-40 units add 1.0-1.5 ms a frame).
- Readability: a round leaves a mark in the mould that says which card did what.

## Suggested first experiment

1. CPU world: a round of up to four cards; payloads crater, bait, alarm, germ,
   sleep; modifiers pierce, scatter, quiet; triggers on hit, when thick; depth
   cap 3.
2. Hand-written loadouts plus random loadouts as the control, player AI, every
   tier, 16 paired seeds.
3. Deciding measure: the best loadout differs on at least three tiers, and on
   every tier beats random by a sign test surviving Holm. One loadout winning
   everywhere means the cards are just a better gun.
4. GIFs of each loadout on its best and worst tier on a page; try it in the
   window.
5. If it passes, port to the GPU behind a fixed shot buffer and quote the 1070
   frame cost.

## Sources (opened 2026-11-12)

- https://store.steampowered.com/app/3399100/Flashes_of_Chaos/
- https://store.steampowered.com/app/3806250 (demo)
- https://www.gamespress.com/Flashes-of-Chaos-Launches-Today-on-PC
- https://steamcommunity.com/app/3399100/reviews/?browsefilter=toprated
- https://steamcommunity.com/app/3399100/discussions/ (demo threads "not much
  synergy?" and "Thoughts about the demo.")
- https://cogconnected.com/2026/10/flashes-of-chaos-has-a-release-date-and-things-are-about-to-get-explosive/
- https://finalboss.io/flashes-of-chaos-demo-drops-aug-8-heres (speculative)
- https://playtester.io/flashes-of-chaos
- https://noita.wiki.gg/wiki/Wands
