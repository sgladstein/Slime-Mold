# Now

The front page: what is being built, who builds it, and what waits on the
owner. Kept by the Direction lane (last change 2026-11-18), which owns it; send it
changes, don't edit it. At most 100 lines. The rules are in
[`CLAUDE.md`](CLAUDE.md), each lane's state in `handoff/<lane>.md`, the
reasons in [the plan](Reports/continuation-plan-2026-11-17.md) the owner
approved on 11-17 ([decisions](wiki/decisions.md)).

## Milestone 1: Three planets (about a week from 11-17)

The game is the roguelike run in squad mode. The milestone is planets 1 to 3
as one sitting of about 30 minutes that the owner wants to replay and likes
to look at. It is done when all six hold:

1. A scripted player (the run bot) lands, pumps, defends, wins and shops through planets 1 to 3 without a soft lock, each planet in 8 to 12 minutes, and so can the owner.
2. The fight looks good in motion at zoom 6 to 12 on his PC: shots, hits and the slime's reaction are clear.
3. The rush is not the best plan. The brake is his call (D7) and he is holding it until he has played more: **add no rush brake unasked.**
4. After playing he can say in a sentence what each of the three slimes did to him.
5. The starting rifle feels good, and one shop visit changes how the gun plays.
6. Everything built is on in the default game, and the game's own pages list what is new and the few experiments that are off.

Next is Milestone 2, "A whole run": the origin as the way a planet is won, a
chosen run length, the bot's curve over all eight planets, save and resume,
the core world at 60 frames a second. A release only if the owner chooses one.

## Each lane's top jobs

| Lane | Exit test | Top three jobs, in order |
|---|---|---|
| Direction | all | This page, `CLAUDE.md` and the 11-17 decisions. Sort every pending pick ([`handoff/direction.md`](handoff/direction.md)) and keep the list of things to try. Sort each play session into lane jobs within the hour; post the bot's curve for planets 1 to 3. |
| Enemy | 4 | Settle the built-but-off switches (`src/build/switches.rs`): far smell and charges round walls on, the rest a tier's trait or deleted. The lure's steering field as a real rule (below). Make each of planets 1 to 3 do one nameable thing to the player by minute 2; the origin waits, as a design page to Direction first. |
| Squad and arsenal | 5 | Finish Provoke and Still (branch `claude/project-thread-vx9t3p-wip`), then no new card families. The Lure building back in the run shop (below), then what the play notes say gets in the way of control. The rifle's feel, with shake and hit-stop as one Experiments toggle; then shop offers that give planets 1 to 3 distinct builds. |
| Look and interface | 2 | The pages that say what a build holds: the Experiments tab, "what is new", "things to try", the build number. Look A, the fight round the squad, starting with the flinch haze. One ranked list of look jobs and a review of every lane's visible change; win and lose screens. |
| Engine and tools | 1, 6 | The switch list in code, the build number, the "what is new" file and its lint. The play record (build, switches, planet, a note key; `play.cmd` pushes it) and a Windows build from CI. The run bot: planet 1 to a win or a loss, then planets 1 to 3 with the shop. |
| Lab | none | Steering, part two: a gun that lays the pull along its shots, as a clip and a recommendation, on its own branch. The pusher and per-tier pull stay here too. |

**The Lure, for planets 1 to 3** (his card, 11-18 03:03Z: "Lure now, in the
three planets"; [lab report](Reports/lab-steering-2026-11-17/README.md)). A
lure every walker leans toward, so walls and turrets make a kill zone. Enemy:
the field with its own GPU slot and weight key, ported from the lab branch
`claude/project-thread-4tc75h` at commit `dafbc22`, lures only, GPU only.
Squad and arsenal: the building and a shop item "Lures", after Enemy's
field. Look and interface: the rings' colour (not the lab's pink) and the
Lure off `run_hidden`. Direction's first numbers: pull 10, unlock 150, the
building 300 food and 150 health. The siren and the outpost stay out.

**Parked until Milestone 1 passes:** gun steps E and F, the Mortar and
Pinwheel mini-bosses, the next turret body, twists and rescue planets, the
Trunk's cords, more planet polish, the player's own mould (D12), and
everything frozen (campaign, RTS control, player AI, the dish games).

## What waits on the owner

Nobody asks him to playtest, and a lane's question goes to Direction, which
asks at most once a session with a recommendation.

| What | State |
|---|---|
| "Automatically delete head branches" in the repository settings | His switch to turn on (asked 11-17 19:14Z). |
| D7 the rush brake, the origin as the win rule, how long a run should be | His to settle in play. Not asked now. |
| Whether to remove today's CPU slime engine | Decided at Milestone 2 (his card, 11-17 19:21Z). Until then it stays as it is and gets no new rules. |

## Things to try at the next play

At most five, most important first; offers, not requests. The game shows them
beside "What is new": [`news/things-to-try.txt`](news/things-to-try.txt).
Today: the rush on planet 1 (D7), camera shake and hit-stop (D6, on the
Experiments tab), what each of planets 1 to 3 does to you, the starting
rifle, the four mini-bosses. The Lure takes a line when it lands. What each build holds is in
[`news/whats-new.txt`](news/whats-new.txt).

## Where things are

- Rulings, quoted and timed: [`wiki/decisions.md`](wiki/decisions.md), [`wiki/decisions-game.md`](wiki/decisions-game.md).
- What is true now, by subject: [`wiki/index.md`](wiki/index.md). Tried and rejected: `Reports/dead-ends.md`.
- Before 11-17: [`Reports/handoff-2026-11-17.md`](Reports/handoff-2026-11-17.md) and [`Reports/roadmap-2026-11-17.md`](Reports/roadmap-2026-11-17.md), history only.
