# The origin: a design for Direction (2026-11-18)

The Enemy lane's job 3 from the plan of 11-17: the origin, written as a
design for the Direction lane before any code. Nothing here is built. The
plan puts the origin in Milestone 2 ("A whole run"), where it replaces the
1% rule; Milestone 1 does not wait on it.

## What the owner said

- 2026-11-16 04:58Z, answer 5 of the mini-boss questions: "I am ok,
  holding off but I wasn't referring to a boss planet. I was thinking
  every planet there is a more boss like entity at the slime start crystal
  that you need to kill to win. The origin of the slime of the planet."
- 2026-11-12 20:21Z, the no-king ruling, which still stands: "remove the
  king rule. As long as the slime has food/crystal energy it can make
  walkers. Headless."
- 2026-11-18 03:11Z, on the Trunk's walls: "I thought the walls would form
  protecting the core of the slime or the slime crystals giving them a base
  of sorts". Built as the Trunk's fort (#473): an amber ring with four gates
  round the home crystal from 0:15 ([page](https://claude.ai/artifact/1D13WjouotXkrob7zpPWHt)).
- The win rule today, his 11-15 notes: cut the slime to 1% of its peak
  (`run::WIN_SHARE`). The plan's reason to replace it: the threshold moved
  from 25% to 2% to 1% and still ends in mop-up.

The two rulings fit if the origin is a win condition and not a king:
killing it ends the planet, but while it lives the slime does not depend on
it to make walkers. The slime stays headless in play; the origin is what
you must reach.

## The proposal

**The origin is a body of the slime's own walkers that sits on its home
crystal for the whole planet.** It is built on the mini-boss machinery
(`bosses.rs`, the lumps of `big::Lump`): a lump of walkers bound by a leash,
drawn by their own state, which the squads already shoot first.

1. **Where.** On the slime's start crystal (`Game::core`), formed at the
   landing from the walkers there. It never leaves. On the Trunk's planet
   it sits inside the fort's ring, so the walls are its keep; on other
   planets it sits in the open.
2. **How big.** Its members are its life. It starts at a size set by the
   planet (a first number: 400 walkers on planet 1, rising with depth, to
   tune) and takes in walkers that stand on the home crystal while the
   crystal has charge, up to twice its start. Shot members die as any
   walker does. It has no health bar of its own; the HUD shows its members
   against its peak.
3. **It is fed by its crystal.** Its regrowth draws on the home crystal's
   charge. Draining the home crystal, or cutting the tubes to it, stops the
   regrowth: you can starve the origin as well as strike it. This keeps the
   Trunk's old answer ("starve it") alive and gives the pump and drain
   players a way in.
4. **It calls the body home when struck.** When it loses members to shots,
   the slime's alarm at home rises and a surge goes for the soldiers
   within reach of it (today's surge, aimed). Striking the origin is the
   planet's fight: the whole slime comes back to defend it.
5. **Its death wins the planet.** Under a floor (a tenth of its peak, or
   50 walkers), the lump breaks and the planet is won. The rest of the
   slime then dies back over five seconds, its walkers' energy draining
   to nothing, so the end is something the simulation shows and not a
   banner over a living map. The run moves to the shop as now.
6. **The 1% rule goes** in the run (the campaign keeps its goals). If the
   slime is cut very low the origin cannot regrow and dies to the next
   volley, so the old way still ends the planet, by way of the origin.

## What it costs and where it lives

- **GPU engine only**, as every new slime rule (owner's card, 11-17
  19:21Z). The lump steering, the leash and members already run there.
- **One of the four lump slots** (`big::LUMPS` is 4 and `big.rs` is
  frozen), so mini-bosses go to three at once at most.
- **Files:** `bosses.rs` (a kind that sits, regrows from its crystal, calls
  the alarm), `raids.rs` (the surge it calls), the run's goal in
  `campaign.rs`/`run.rs` (a goal "kill the origin"), and the die-back as
  one engine key. Other lanes: the run bot learns to strike it (Engine and
  tools), the HUD's goal line and the origin's look (Look and interface:
  their list already holds "a look for the origin when Enemy builds it").
- **Frame cost:** the lump passes already run for mini-bosses; one more
  lump, and a once-a-second sum of the home crystal's walkers.

## How it would be judged

- The run bot wins planets 1 to 3 by killing the origin, each in 8 to 12
  minutes (exit test 1's band).
- Idle does not win; nothing soft-locks (every home crystal must be
  reachable on foot; on the Trunk's planet the fort keeps at least two
  gates open, `open_gate`).
- The strike is a fight: the count of walkers that come home in the 30 s
  after the first hit on the origin, against the same 30 s with no hit.
- The end is shorter than today's mop-up: time from the last big fight to
  the win, before and after.

## The risk: the rush

A squad of 40 soldiers that walks to the home crystal at 3:00 may kill a
400-walker origin in seconds, which would make the rush stronger than it
is under the 1% rule. The defences above (regrowth, the call home, the
fort on planet 3) push against it, but the brake is D7, which the owner
holds. The first build should measure the rush against the origin before
any number is tuned, and report it to Direction rather than add a brake.

## Questions for the owner (Direction to ask, at most once, with these recommendations)

1. **Does killing the origin end the planet at once, or does the rest of
   the slime die back over a few seconds first?** Recommended: die back
   over five seconds, so the win is seen.
2. **Does the origin call the whole slime home when you strike it?**
   Recommended: yes; it makes the strike the planet's climax.
3. **Does "cut the slime to 1%" stay as a second way to win?**
   Recommended: no; a slime cut that low starves its origin anyway, and
   the mop-up is what the origin is for.

Everything else here is the Enemy lane's to decide and change in play.

## Order of building, once Direction says go

1. The origin as a sitting lump on the home crystal, its regrowth from the
   crystal, its death as the win (one test on the software GPU: formed at
   the landing, grows on a fed crystal and not a dry one, the planet is
   won when it is cut under the floor).
2. The call home when struck.
3. The die-back.
4. Films of the run bot on planets 1 to 3 to the win, and the rush against
   the origin, on a page with the counts.
