---
title: Two Fevers (playable demo)
kind: prototype
status: open
updated: 2026-11-10
sources: [Reports/two-fevers-2026-11-10/README.md, Reports/rts-design-2026-11-10/brainstorm/README.md, Reports/rts-design-2026-11-10/red-team/README.md]
code: [Fevers, Rules, Hand, Player, field_piles, play_match]
---

# Two Fevers (playable demo)

## Now

**Playable in the browser:** https://claude.ai/artifact/UMzPJmevoYJYmx8KRqG37a.
Two fevers fight over one neutral host of 5,000 Jones walkers on the
field's 23 regrowing piles (`src/mould/fever.rs`, its own small host rather
than a `Dish`). Click a tube to seed (a spore every 10 s, three at most). A
fever runs along the tubes at about 7 cells a second. A walker keeps the colour of
the last fever it caught, so the score is the share of the host each fever
owns, averaged over 3:00. Ground just taken is held for 15 s
(`Rules::hold_seconds`); the owner's own fever cannot refresh it for 40 s
(`Rules::immune_seconds`). Hush stops a fever's carriers laying trail, so
the tubes under them break.

This is version 2. Version 1 scored carriers only, and the owner found no
strategy in it: rings swept the host and left it as before. Version 2 is the
owner's pick, owned ground plus one host-shaping trait.
In version 2 the planner beats random clicks on 16 of 16 seeds, but by only
0.537 of the summed score, about the same as version 1's 0.536. The hold is
what makes aim count: without it, random clicks win 14 of 16. With it, a
planner mirror's 1:30 leader loses at the bell 14 times in 32. Hush, used the
plain way, is noise (10 of 16). No one has judged version 2 by hand yet.

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| Lone front, catch 0.03, sick 4 s: p90 carrier distance at 1 / 5 / 9 s | 10-13 / 46-47 / 75-76 cells; 1,983 of 5,000 carrying at the peak | 2 | | [report, *Tuning the front*](../../Reports/two-fevers-2026-11-10/README.md) |
| v1 plan v random, carriers' share | plan's part 0.536 (0.511-0.569) | 16, both colours | 16/16, p 1.5e-5 | [report, *Results*](../../Reports/two-fevers-2026-11-10/README.md) |
| v1 plan v idle; random v idle | 0.801; 0.795 | 16 | 16/16 each | [report, *Results*](../../Reports/two-fevers-2026-11-10/README.md) |
| v2 plan v random, owned share | plan's part 0.537 (0.514-0.576); owns 0.51 against 0.44 | 16 | 16/16, p 1.5e-5 | [report, *Results*](../../Reports/two-fevers-2026-11-10/README.md) |
| v2 plan v idle; random v idle | 0.934; 0.887 | 16 | 16/16 each | [report, *Results*](../../Reports/two-fevers-2026-11-10/README.md) |
| v2 with no hold, plan v random | plan's part 0.454; random wins 14/16 | 16 | p 0.002 for random | [report, *Results*](../../Reports/two-fevers-2026-11-10/README.md) |
| Plan mirror, leader at 1:30 still ahead at the bell | v1 18/32 (24/32 when immune to both); v2 18/32; v2 no hold 30/32 | 16 | | [report, *Results*](../../Reports/two-fevers-2026-11-10/README.md) |
| Hush when behind v the same planner | 0.508; 10/16 | 16 | p 0.23 | [report, *Results*](../../Reports/two-fevers-2026-11-10/README.md) |
| Step cost | 0.49-0.64 ms a step native; 2.5 ms a 60 Hz tick in V8 | 1 | | [report, *The model*](../../Reports/two-fevers-2026-11-10/README.md) |

## Owner's word

- 2026-11-10 20:33Z: "lets focus on actually building playable demos" (the RTS thread).
- 2026-11-10 21:11Z, on version 1: "Not sure I follow what the strategy or challenge is in this game. I click and it send a wave through the network, a few seconds later the immunity moves through the network and eventually everything ends back at the initial state."
- 2026-11-10 21:15Z, card: "Both together", owned ground first and then one host-shaping trait. See [decisions.md](../decisions.md).

## Tried and rejected

- Scoring on carriers alone (version 1). See the `Rules::owned` entry in [`dead-ends.md`](../../Reports/dead-ends.md).

## Open

- Human play of version 2: does reading the holds give more skill than the planner shows?
- Smaller fronts, so one seed cannot swing 30% of the host.
- An aimed Hush (hush, then seed across a road) and the other traits (Swarm, Roam, Linger, Beckon, Bloom).
- A native window build; today it plays only in the browser.

## Sources

- [two-fevers-2026-11-10/README.md](../../Reports/two-fevers-2026-11-10/README.md): this build, both versions, the census.
- [brainstorm](../../Reports/rts-design-2026-11-10/brainstorm/README.md) and [clean-slate-player.md](../../Reports/rts-design-2026-11-10/brainstorm/clean-slate-player.md), *Concept 2*: where the idea came from.
- [red team §3](../../Reports/rts-design-2026-11-10/red-team/README.md): its risks, including the rubber band.
- [The RTS directions](directions.md).
