# Turrets, grown up: tech tree or cards (2026-11-16)

The owner (2026-11-16 21:07Z): "Start a thread to explore improving and
expanding on the turrets. Two ideas: 1) We build a tech tree for turrents
that can give you new types of turrets and abilities. 2) We integrate the
turrets into the card system and you can modify turrets with your cards."

Page: https://claude.ai/artifact/V16Dp6AyxrZ6KQ4KMPzbwr (copy: `page.html`
and `page/`).

## Where turrets stood

One kind (`Kind::Turret`): a plain strike at the densest slime in sight every
0.5 s (`BuildRules::turret_every`), 15 cells, 300 health, 500 food; one
in-mission upgrade, Render (600 food, kills in range pay 0.25 food); in the
run shop an unlock and three "Turret power" levels (`run::ITEMS`). Turret
shots carried no cards (`rounds.rs` said "turret rounds later"; the gun
review's *What else we missed* listed a turret deck). Cards reached turrets
only from outside: `Corral` shoves slime toward the nearest turret. Since
PR #398 turrets fire over walls.

## The two ideas, and the pick

- **Tech tree.** New bodies do what a squad can't (lob over walls, hold a
  field, herd); bases read at a glance; branches give a run its identity in
  the shop. Against: authored combos only, and a second weapons system beside
  the cards (the reviewers' warning, 11-14).
- **Cards.** Reuses every card built (shells, trigger, expire, multicast,
  shoves); combos in a fixed spot; giving cards to turrets instead of squads
  is a real trade, which keeps turtling honest. Against: alone, a carded
  turret is a squad gun that can't walk, and per-turret decks mean clicking.
- **Recommended and picked: both.** The tree unlocks turret *bodies*, each a
  different way to deliver a shot (gun turret, mortar over walls, sprayer
  laying a field, fan herding); cards fill each body's barrel. Owner's card,
  21:36Z: "Both: tree bodies, card barrels". The tree lives in the run shop
  only, not as research during a mission (owner 21:41Z: "I lean shop").
- **Still open** (defaults in brackets): one deck per body, per turret, or
  copied from the builder [one per body]; turret cards from the squads' bag
  or a shelf of their own [the squads' bag]; which body first after the gun
  turret [mortar].

## First step, built: turrets fire a turret deck

- `Game::turret_deck` (None: plain strikes as before), loaded by
  `Cmd::Load(TURRET_DECK, ..)` (`TURRET_DECK` 255, so replays carry it:
  `load 255 blast`). One deck for every turret, as one gun is a squad's.
- `Game::building_units`: a turret's shot carries the deck's next cast
  (`Building::next`, as `Soldier::next`), its bullet card's strike, set
  pellets and range; its multicast volley; the deck's Range and Rate cards
  stretch every turret's reach and rate (`Rules::range_card`,
  `rate_card`). A shot is a turret's when `by` is None, so no new field:
  `Game::turret_mods`, `Game::mods_by`.
- `shots.rs`: flight cards (pierce, split, trigger, expire) read
  `mods_by`; a turret's let-out cast flies at a turret's strike and reach,
  over walls as its shells do.
- `rounds.rs` (`Rounds::after_step`, `lay_trails`): a turret hit casts from
  `rounds::turret_casts`; its trail cards lay trails. A `Pull` on a turret
  shot finds no gun to drag toward and shoves on along its line.
- Film key `tload=T,NAME` (a preset or card list, as `load=`); the headless
  rounds line now prints when the turrets carry a deck.
- Not built yet: any menu to set the deck (the armory), budget or slots for
  it, and the bodies.

Test: `buildings::tests::turrets_fire_the_turret_deck` (seed 3, a turret by
the core, the squad holding fire, 10 s): plain 0 casts; blast 19 shots, all
19 carrying and casting, 687 killed against 307 plain; the command
round-trips. Full `cargo test --release --locked` and clippy pass.

## Films

The shove page's kill box (`raw/film.sh`): the Fork, `hand=hold@150,265
bank=8000 rounds=plain`, three turrets at 172,240, 180,240 and 188,242 from
29 s, the turrets loaded from 29 s with the cards named; near view
`centre=170,252,10`, 960 x 600, llvmpipe. Clips 29-32 s at 30 fps
(`raw/make_clips.py`); counts over 29-39 s from `raw/counts.sh` (frames only
at 29, 32, 39 s; raw logs in `raw/counts/`). One game each: a look, not a
census.

| 10 s, 29-39 s | Turrets' shots killed | Casts | Cast effect | All kills | Slime at 39 s |
|---|---|---|---|---|---|
| plain turrets (`plaindeck`: one plain card) | 231 | 0 | | 661 | 24,669 |
| blast | 93 | 19 | 261 killed | 868 | 24,475 |
| sleep gas | 236 | 26 | 4,373 walkers gassed | 1,299 | 24,009 |
| corral + heave + plain | 427 | 57 shoves | 1,623 thrown | 847 | 24,465 |
| bomb | 61 | 28 | 4 killed, 1,648 flung | 541 | 24,810 |
| multicast + bomb + blast | 96 | 30 (51 beside another) | 43 killed, 1,093 flung | 601 | 24,730 |

"All kills" is the game's whole count, squad included; the squad fires the
same plain rounds in every row. Gas doubled the game's kills (the front
sleeps and everyone shoots it); corral threw the slime at the turrets' feet
(427 against 231); bomb flung it away from the guns and lowered kills: a
trade, which is the reason a turret deck is worth choosing.

Frame cost: no new pass; turret shots and casts use the soldiers' paths
(bursts batched per step since #387). Not timed on the 1070.

## Review

An independent agent reviews the design and the code from [review-brief.md](review-brief.md) (owner 21:37Z: "once you finish all this, use an independent agent to review").
