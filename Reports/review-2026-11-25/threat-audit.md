# Threat audit, 2026-11-25

Step A of the owner's order (11-25 21:35Z: "a then c ... then b"). This is
every way the slime hurts or pressures the squad on main 1059a0f, sorted by
**where it comes from**: the simulation's own rules, or an event the game
schedules on top of it. Sources: `run.rs` (`ENEMY_RULES`, `FIRST_PLANET`,
`RAID_*`), `wiki/world/run.md`, `bosses.rs`, `wiki/world/mould-tiers.md`.
Nothing here was changed.

## From the simulation (walkers following their own fields)

| Threat | Planet | What starts it | Reaches a moving squad? |
|---|---|---|---|
| Surge / panic charge | 1+ | alarm from shots; walkers within the guns' reach plus `panic_margin` 5 charge | Only within about 5 cells of the guns. Walking away ends it. |
| Pump smell (Drifter) | 1 | the scent of a pump draws the body to eat its crystal | It goes for buildings, not the squad. |
| Threads (Sprawler) | 2 | lanes and fed walkers shuttling along scent | Spokes reach toward food, and soldiers carry food scent. |
| Amber walls and setting where shot (Trunk) | 3 | trail and hits | Defensive. It slows the squad and never chases. |
| Swarmer lunge | 4 | its own charge numbers (reach +25, 3x pace) | Yes. It took 32 soldiers in 30 s on his 11-19 planet 4, the one rule in his logs that broke a big army. |
| Sleeper and Dozer waking | 6, 8 | alarm wakes them | Short range. |
| Twists (wind, cold snap) | 2+, 1 in 3 | change to the walk itself | They shift the body and don't add a threat. |
| Fire on plants | 3+ | burning cells hurt soldiers | Yes, but unwarned (balance review 11-24, finding 3). |

## Scheduled or rule-triggered events on top

| Threat | Planet | Trigger | Trigger type |
|---|---|---|---|
| Raids (armoured marchers) | 1+ | first at 4:00 (P1) or 2:00, then every 150 s or 100 s, as a held lump | **clock** |
| Heartbeat | 1+ | every 6 s a ring pushes the body outward | **clock** |
| Starfall | 1+ | 3 to 12 falls drawn between 0:50 and 10:00, kills at its radius | **clock** |
| Mini-bosses (hunter, wanderer, sapper, leech) | 2+ | form at the crowded knot nearest the player. Three at once from P6 | rule, rising by planet |
| Pinwheel | 3+ | a hunter kind | rule |
| Slime's mortar | 4+ | lobs its edge at your nearest thing every 5 s | rule |
| Boss cores re-forming | 2+ | walkers eat half a dropped core | rule |
| Fruiting | 2+ | **the squad is winning**: 8% of the slime knots, then bursts into spores 30 s later | comeback |
| Waves | 3+ | **kills over 20 s reach 1/8 of the mass** and the mass is falling | comeback |
| Condensed strikers | 2+ | **under 60% of peak**, scaled by army size | comeback |
| Late lunge | 3+ | **under 40% of peak**: reach +25, 3x pace | comeback |
| Last-stand heart | 4+ | **under 20% of peak** with 24+ soldiers | comeback |
| The heart (origin) | 1+ | always. Armour by planet (D7) | win condition |
| Soldier price climb | all | past 30 alive | economy brake |

## What the list shows

1. **Five comeback mechanics read the same signal.** Fruiting, waves,
   strikers, the late lunge and the last stand all fire when the squad is
   winning (the slime's fall from its peak or the squad's kill rate). They
   stack in a planet's last third, which fits the balance review's shape:
   the slime holds at half its peak, then falls from half to 5% in 0.5 to
   2.2 minutes, and he loses soldiers only once it is under a fifth. Early
   and mid-planet, nothing comes for a squad that keeps moving except clock
   raids.
2. **From the simulation, reach is about 5 cells.** The body's own pressure
   on the squad is the panic charge within the guns' reach plus 5, so a
   squad that walks away is safe. The one exception is the Swarmer's
   +25 reach at 3x pace, and that is the one rule his logs show breaking an
   army.
3. **Three threats run on a clock**: raids, the heartbeat and starfall. They
   happen whatever the slime has sensed, so the player can't read them from
   the slime.
4. **Several threats overlap in what they ask of the player.** The hunter,
   the Pinwheel, the strikers and the late lunge all mean "something
   faster comes at the squad". Waves and the heartbeat both mean "the body
   pushes outward".

## Proposal (for Direction and Enemy, after the Lab's result)

Keep what the player can see coming and answer with a choice. Then, if the
Lab's chasing body (brief beside this one) works:

- **Merge** fruiting, waves, strikers and the late lunge into the body's
  own chase, since all four exist to reach a winning squad. Keep whichever
  one reads best as a look. Fruiting's spore burst is the most distinct.
- **Drive raids from the slime's state** (hunger, a crystal lost, the
  squad's scent) instead of a clock, so the player can read one coming.
- **Keep** the heart, the planet traits, the mini-boss kinds that differ in
  what they target (hunter, sapper, leech, mortar, wanderer), starfall
  (its job is income, not threat), twists and the soldier price.
- **Give fire a warning or a rule** (balance review finding 3).

Each cut gets its `Reports/dead-ends.md` line with the condition that would
bring it back.
