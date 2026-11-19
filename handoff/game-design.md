# Handoff: game designer review (2026-11-19)

The standing review thread "Review the project as a game designer", which the
owner started 11-12 04:29Z: "review what all the other [threads] are doing and
suggest improvements, new ideas, and additional threads", and keep reviewing
"as the other threads finish each task". The thread changes no code. It sends
notes straight to the threads (owner, 11-12 04:43Z: "You don't need my
approval"). When the owner is away, design questions go to this review, the
tech lead and integration, with integration as the tie-breaker (11-12 04:48Z).
Since 11-17 the Direction lane owns decisions and `NOW.md`. A successor review
sends its calls to Direction for recording.

The full thread memory up to 11-17 is
`handoff/project-memory-2026-11-17/design-review.md`.

## How this review judges work

- The owner's bar (11-10): the mould is integral to play, several strategies
  counter each other, the visuals follow the simulation, and biology is
  inspiration.
- Build first, balance later (CLAUDE.md). A note is advice, not a gate. Ask
  for a census only for a strategy question, keep it small, and retune
  numbers rather than reject a feature or a look.
- **The owner's hands outrank the AI** (11-14 01:08Z: "I have easily beat the
  game in all my playtests"). A scripted stalemate measures the bot. Never
  make the slime easier because of a bot result.
- Every mechanic needs three things before it is a strategy: a tell drawn from
  the simulation that warns the player (panic's lean, the surge's build-up),
  a counter, and a visible result (withering greys, germ red).

## Calls made, and where they live

| Call | Where it is recorded |
|---|---|
| Germ, one cast in bands: Sporer and Drifter about 45-55%, C, Trunk and Sprawler 25-35%, Swarmer and Grudge 10-20%. A sleeping Sleeper is immune, and Flare then germ is the combo (28%). The Drifter's 43.5% counts as in band | `handoff/gun-cards.md`, `wiki/world/gun-cards.md` (#412, #416, #417) |
| Detonate: "let it spread first" is the lesson, so don't retune it | gun-cards (#418) |
| No Cut card. Re-test if the largest part one cut can starve is 5% or more of the slime in most counts | `Reports/dead-ends.md`, gun-cards (#435) |
| Fire-shy is a tier trait, never global: the Sprawler, whose card says "Bring Tinder" | #434, `handoff/run-progression.md` |
| Set slime traps rather than blocks (`set_hold`). A rush into it gets eaten | `handoff/simulation-use.md` (#414) |
| One switch per mechanism: `fire_weight` deleted, `fire_alarm` kept | #430 |
| Panic reach follows gun range (range + 5) | #264 |
| Surges are paced by food eaten, not a clock | slime enemy (after #343) |
| A counter table per tier (11-14, D3 of the cloud plan) | only in the 11-17 memory copy. Today's version is `run::does_to_you` (#372) |

## Open design gaps, by milestone 1's exit tests

1. **Exit test 3, the rush.** Drain one crystal, buy to 40-50 soldiers, walk
   in: planet 1 falls by 2:48. Every review since 11-15 names it the dominant
   plan. The brake is the owner's call (D7), and he wants to play more first.
   Keep it that way, but put it in front of him with each play session. The
   review's preferred brakes, in order: the slime contests hand-drained
   crystals (the Drifter smelling the drain), then a per-crystal drain cap.
   Avoid a flat soldier tax, since upkeep was ruled out.
2. **Exit test 4, one nameable thing a planet.** The tiers were tuned on
   other maps and partly don't look like themselves on run planets. The cut
   count found the Trunk plays as a sheet (under 2% behind a neck). Its
   veins are parked until milestone 2, so its "cut a cord" identity is
   missing for now. Planet 3's thing is the Trunk walling its crystals
   (#473). Check that it reads in one sentence after play.
3. **The shop as the strategy.** The design aim is that the next planet's
   tier decides the purchase (Tinder before the Sprawler, `run::bring`).
   Extend that to every planet when the ladder settles. A card with no tier
   where it is the best pick is a candidate to merge or cut.
4. **Defensive play has few reasons yet.** Raids are rare early (#490), and
   planet 1 raids pumps at 0:59. Watch that building pumps and turrets pays
   on planets 2-3, or the rush stays the only plan whatever D7 does.
5. **Off-switch build-up.** Every experiment that stays off after the owner
   plays gets a default or a delete (the Experiments tab and the switch
   list).

## Pitfalls for a successor

- Find the owning thread before sending a note. Gun looks, gun cards, slime
  enemy and simulation use are separate lanes, and notes sent to the wrong one
  cost a relay.
- A pick that is a look stays the owner's, as a blind A/B with the key kept
  off the page. Don't call looks for him.
