# Gun builds: method (draft for the adversarial review)

Question: which gun decks (cards in a gun, in order) are strongest, does
any single card dominate alone (the owner's Noita rule: no single card is
a win alone; a strong one carries a danger; strong builds come from
combinations, often of rarer cards), and do the best combos need rare
cards. Research only: no tuning changes.

## Harness (branch claude/project-thread-i0vg7o)

`larder-world gunlab=DEPTH:SEED[:SECS] deck=CARDS gun=KIND [arena=N]`
(`src/bin/larder-world/runbot.rs`, `gun_lab`, `lab_deck`, `arena_hand`).

- Landing: `Run::new(SEED)`; the planets before DEPTH are counted won with
  2 stars each and the points spent by the run bot's shop on upgrades only
  (`runbot::shop` minus offers: strike, rate, hp, squad, harvest, range,
  bank). The gun is the kind's average at DEPTH (`GunKind::average_at`),
  the deck is set as the run's cards, all in the gun. Budget is printed,
  not enforced.
- Deck syntax: card names joined with `+`; a payload may take a set,
  `blast.buckshot` = the standard bullet with the set's exact lean and shape.
- Full game (arena=0): `hands::bot_hand` (the run bot: buys soldiers,
  drains crystals, volleys at heaviest slime in range, hunts, steps back
  when eaten) plays to the verdict or SECS.
- Arena (arena=N): N soldiers bought at landing, bank then set to 0
  (income still trickles in but nothing spends it), one squad; each
  second: volley at the heaviest 4x4 bin of slime within gun range
  (weighted by distance); if none in range, walk to 0.7 range of the
  heaviest bin weighted by walk; if mean hp < 0.5 and bitten, step back a
  range. Runs SECS game seconds on the planet's real slime and ground.
- Output: one `gunlab` line: won, stars, secs, killed (walkers killed,
  cumulative), mould (walkers alive at end), start_mould, alive, peak,
  min_alive, hp (mean over seconds of the living soldiers' mean health),
  marks every 30 s (killed/mould/alive).

## Measures

- Full game: won (0/1), stars, time to verdict, soldiers alive.
- Arena: slime removed = start_mould - mould (net of regrowth), killed,
  soldiers lost (N - alive), hp.
- Comparison on paired seeds against the empty deck (plain bullets) and
  against each other; sign test p; Holm correction across cards.

## Stages

1. Singles: each of the 50 distinct cards (35 cards; plain/blast/bomb/gas/
   germ in 4 sets) alone, planets 1-4, seeds 1-5, arena; plus full games
   for a subset to check that the arena ranks like the full game.
2. Pairs among the top ~15 and all known partners (readers after their
   setter, trigger/expire/multicast with payloads, tinder+torch, heave).
3. Beam search to 4 cards (rifle slots) keeping the best 20 per step.
4. Top ~20 decks in full games, 5 seeds, planets 1-4.

## Known gaps

- The run bot builds no turrets: turret decks wait for the strategy bot
  (sibling thread, `hands::plan(g, "strat:...")`).
- Determinism across runs on llvmpipe not yet checked.
- Order of cards matters (modifiers apply to the next payload, triggers
  carry the next cast); stage 1 ignores order.
