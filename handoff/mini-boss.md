# Mini-boss slimes: handoff (2026-11-17)

**State: built, merged, on in the run.** No open branch or PR. Record:
[`Reports/mini-boss-2026-11-16/README.md`](../Reports/mini-boss-2026-11-16/README.md)
(rounds 1-3 and the leech ring); its page source is `round3/index.html` with
the four clips (the claude.ai page links no longer open). Wiki:
`wiki/world/run.md` (Mini-bosses paragraph). Rejected ideas:
`Reports/dead-ends.md` (grep "mini-boss", "ring lump", "straining").

## Owner rulings (2026-11-16, in `wiki/decisions-game.md`)

- 04:58Z: prototype every kind; no bronze (the specks stay, separate, more
  often early); a mix of hunters and stay-put kinds; form only where the
  slime crowds or a big part is cut off, chance rising through the run;
  squads shoot mini-bosses first. Full bosses held, but the idea is an
  **origin** at each planet's slime start crystal that you must kill to win.
- 05:23Z: much stronger than ordinary slime, varying a lot boss to boss.
- 05:21Z: they must look cool (the look work is its own area, PRs #386,
  #393, #397).

## What is built (PRs #385, #391, #396)

- Engine: `big::Lump` (4 lumps, both engines, GPU twin exact). A member's
  lump is bits 25-27 of its dense word, its hits bits 28-31 (`KEPT`);
  nothing in a lump needs condensing. `Lump::ring` holds a band round the
  aim; `LUMP_CROWD` 3 lets members push through thick slime.
- Game: `larder_world/bosses.rs`. `BossRules::for_depth` (on in the run),
  grade sets armour 4-15 / bite / size / pace. Kinds (`BossKind`, weights
  1 each, `set=boss_kind=N` forces): **hunter** (lunges every 5 s),
  **wanderer** (roots on far fresh crystals, `grow_lump`), **sapper** (holes
  the nearest wall, goes through, hunts), **leech** (rings a pump, takes its
  pay, gives it back if broken). Members don't starve; 1.5 s fade.
- View word: `Game::lump_looks()` u16 (bits 0-2 kind, 3 not gathering, 4-7
  grade, 8 fading, 9-15 fade progress).
- Films: `larder-world run=1:3 mode=squads hand=hold boss=85,310,85
  set=boss_kind=K look=DIR` (+ `wall=`, `pump=`, `set=pump_hp=`); script
  `Reports/mini-boss-2026-11-16/raw/round3-films.sh`.

## Next

1. **Mortar** (sits on a crystal, lobs slime past gun range) and
   **Pinwheel** (members circle and sweep a squad): both need engine work.
2. **The origin** at each planet's start crystal (the owner's 04:58Z idea;
   it belongs with run progression).
3. Time the lump passes on the GTX 1070 (never done).
4. Lumps don't path-find (their aim pulls straight); a walking map would
   let a sapper hunt from its hole (see `dead-ends.md`).

No owner picks are pending from this area.
