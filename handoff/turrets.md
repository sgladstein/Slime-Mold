# Handoff: turrets (tech tree and turret cards), 2026-11-17

Thread "Expand turrets with tech tree or cards", started by the owner
2026-11-16 21:07Z. Full record: `Reports/turret-growth-2026-11-16/README.md`
(page source `page.html` beside it; the claude.ai page link won't open on
the new account). Wiki: `wiki/world/larder.md` ("Turrets grow both ways"),
rulings in `wiki/decisions-game.md` (11-16 21:36 to 11-17 00:47).

## State: everything merged, nothing open

- **#400**: every turret fires one shared turret deck (`Game::turret_deck`,
  `Cmd::Load(TURRET_DECK, ..)`, film key `tload=T,NAME`).
- **#401**: shots carry the firing turret (`Bullet::turret`, `Firer`), so
  turret card kills count as turret kills, Render pays on casts, and Pull
  drags toward the turret.
- **#402**: in a run the turret cards come out of the squad's bag. The shop's
  "Edit the turrets' cards" (`Pick::EditTurrets`) switches the gun slots to a
  turret rack (2 slots, budget 4; +1 slot and +2 budget per Turret power
  level). The game loads `Run::turret_cards` at landing.
- **#404**: the mortar, the first turret body (`Kind::Mortar`). It's unlocked
  by the run shop's "Mortars" item, which needs Turrets, so it is the tree's
  first node. Range 80 dish cells (the longest in the game), blind within 8,
  a shell every 4 s, lobbed over walls. Shells fly 0.6 s + d/35 s
  (`Game::shells`). Command id MORTAR 37, build page slot (1,2) key B,
  shader building kind 7, film key `mortar=X,Y[,T]`.

## Owner rulings (this area)

- Both ideas: the tree unlocks bodies, cards fill the barrel (11-16 21:36Z).
- The tree lives in the run shop only (21:41Z).
- The mortar's point is a longer range than anything else (22:10Z). No hard
  cap on Range cards (22:11Z).
- The first body is the mortar (11-17 00:47Z).

## Next steps / open picks

1. Owner playtest of the mortar. One film: 5 shells killed 390 in 20 s, about
   2.5 plain turrets. If it's too strong, tune `mortar_every` and
   `mortar_blast` (BuildRules).
2. The next body is the owner's pick: sprayer (writes a field), fan (herds
   slime), picky-targeting turret, or mine.
3. Still open: one deck per body (proposal option A) or the one shared turret
   deck. Today the mortar fires the shared deck.
4. Not done: 1070 timing of Range cards and of the mortar's sight read
   (about 80k cell reads once every 4 s, estimated). Also the paired
   raid-on-base census of turret cards against the squad's gun, which is
   for the balance pass, not a gate.
5. The gun-card thread owns `rounds.rs` and `shots.rs`. Turret hooks there
   (`turret_mods`, `mods_by`, `Firer`) were agreed with it.
