---
title: Two Fevers (playable demo)
kind: prototype
status: open
updated: 2026-11-11T15:40Z
sources: [Reports/rts-research-2026-11-10.md, Reports/two-fevers-2026-11-10/README.md, Reports/rts-design-2026-11-10/brainstorm/README.md, Reports/rts-design-2026-11-10/red-team/README.md]
code: [Fevers, Rules, Hand, Player, field_piles, play_match]
---

# Two Fevers (playable demo)

## Now

**Playable in the browser:** https://claude.ai/artifact/UMzPJmevoYJYmx8KRqG37a
(version 3). Two fevers fight over one host of 5,000 Jones walkers on the
field's 23 regrowing piles (`src/mould/fever.rs`, its own small host rather
than a `Dish`). A walker keeps the colour of the last fever it caught; the
score is the share each side owns, averaged over 3:00. Settings:
`Rules::v3()`.

Version 3 (the owner's card, 22:06Z) puts the mould at the centre. Each side
starts owning the walkers round its end pile. **Call** (free, two at most):
your walkers, and only yours, are pulled toward it, so your body builds its
own tubes there. **Seed** (a spore): the walkers under it catch your fever,
which now travels only through tube. **Pinch** (free, every 5 s): your walkers
in a small disc lay no trail for 6 s, so your own tube there breaks. Your
walkers on food earn spores; a walker of yours cut off from food for 12 s
goes back to nobody's; a fed walker of yours catches the rival's fever ten
times less often.

The census of three scripted plans finds a **ladder, not a cycle**: Trunk
(call the best pile, seed everything) beats Raider (seed the rival's thickest
tube) 15/16 and Web (two far calls, pinch when attacked) 12/16; Web beats
Raider 11/16. Only the top link survives Holm. Raider is no better than
random clicks (7/16). The leader at 1:30 keeps the lead in 26-32 of 32
games in every matchup, so income snowballs. No one has played version 3
by hand yet.

Earlier: version 1 scored carriers only and had no strategy (the owner,
21:11Z); version 2 added owned ground, a 15 s hold and Hush, a tug of war
the owner found "overly simple" with the mould as "background" (21:52Z).


## Owner's word

- 2026-11-10 20:33Z: "lets focus on actually building playable demos" (the RTS thread).
- 2026-11-10 21:11Z, on version 1: "Not sure I follow what the strategy or challenge is in this game. I click and it send a wave through the network, a few seconds later the immunity moves through the network and eventually everything ends back at the initial state."
- 2026-11-10 21:15Z, card: "Both together", owned ground first and then one host-shaping trait. See [decisions.md](../decisions.md).
- 2026-11-10 21:52Z, on version 2: "This is better, but still feel overly simple with 1-2 tools that lead to mostly 1 strategy with little complexity ... The slime mold seems more like background in this game and not the integral part".
- 2026-11-10 22:06Z, card: "Build it after the checks (recommended)": version 3 as [designed](../../Reports/two-fevers-2026-11-10/v3-design.md).

## Tried and rejected

- Scoring on carriers alone (version 1). See the `Rules::owned` entry in [`dead-ends.md`](../../Reports/dead-ends.md).
- Version 3's first settings: call weight 30 (the host heaps on the call), uncapped foraging (44 seeds to 7), a pinch for a spore (Web spends everything on pinches), no resistance (the first seed into the neutral mould takes the game). See the `Rules::v3()` entry in [`dead-ends.md`](../../Reports/dead-ends.md).

## Open

- Human play of version 3: is the ladder a ladder for people too?
- The snowball: income that falls as a body grows, or a cap on spores from ground.
- A raid that pays: seeding a starving or cut-off piece is what the rules reward, and no script does it.
- A native window build; today it plays only in the browser.

## Sources

- [two-fevers-2026-11-10/README.md](../../Reports/two-fevers-2026-11-10/README.md): this build, both versions, the census.
- [brainstorm](../../Reports/rts-design-2026-11-10/brainstorm/README.md) and [clean-slate-player.md](../../Reports/rts-design-2026-11-10/brainstorm/clean-slate-player.md), *Concept 2*: where the idea came from.
- [red team §3](../../Reports/rts-design-2026-11-10/red-team/README.md): its risks, including the rubber band.
- [The RTS directions](directions.md).
- [rts-research-2026-11-10.md](../../Reports/rts-research-2026-11-10.md): outside RTS design research and feedback on this demo (Part 5); page [rts-craft.md](rts-craft.md).
