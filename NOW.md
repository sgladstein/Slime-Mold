# Now

The front page: what is being built, who builds it, and what waits on the
owner. Kept by the Direction lane (last change 2026-11-20), which owns it; send it
changes, don't edit it. At most 100 lines. The rules are in
[`CLAUDE.md`](CLAUDE.md), each lane's state in `handoff/<lane>.md`, the
reasons in [the plan](Reports/continuation-plan-2026-11-17.md) the owner
approved on 11-17 ([decisions](wiki/decisions.md)).

## Milestone 1: Three planets (about a week from 11-17)

The game is the roguelike run in squad mode. The milestone is planets 1 to 3
as one sitting of about 30 minutes that the owner wants to replay and likes
to look at. It is done when all six hold:

1. A scripted player (the run bot) lands, pumps, defends, wins and shops through planets 1 to 3 without a soft lock, in about 30 minutes for the three (planet 1 may be the short opener), and so can the owner. A planet held to the clock ("held on") is counted apart from a win.
2. The fight looks good in motion at zoom 6 to 12 on his PC: shots, hits and the slime's reaction are clear.
3. The rush is not the best plan. The brake is his call (D7) and he is holding it until he has played more: **add no rush brake unasked.**
4. After playing he can say in a sentence what each of the three slimes did to him.
5. The starting rifle feels good, and one shop visit changes how the gun plays.
6. Everything built is on in the default game, and the game's own pages list what is new and the few experiments that are off.

Next is Milestone 2, "A whole run": the origin as the way a planet is won, a
chosen run length, the bot's curve over all eight planets, save and resume,
the core world at 60 frames a second. A release only if the owner chooses one.

**Where it stands (11-21 morning, [check](handoff/direction.md)).** Nothing
is closed. Test 1's bot half passes: CI's run bot wins planets 1 to 3 on
all three seeds (11-21, siege=3), planets 2:48 to 6:37, runs 12 to 17 min. His 23:32 ruling paces the
threats (#606, #644): planet 1 the base slime; fruiting and strikers from
2, waves and the late lunge from 3, the heart from 4. Test 4 by minute 2
([clips](Reports/planet-things-2026-11-21/)): 1 smells your pumps and eats
them; 2's threads find you from afar and spawn mini-bosses; 3's cut edge
condenses into gold strikers. Tests 2, 5: his. Test 6: `crystal_regrow`.

## Each lane's top jobs

| Lane | Exit test | Top three jobs, in order |
|---|---|---|
| Direction | all | This page, `CLAUDE.md` and the 11-17 decisions. Sort every pending pick ([`handoff/direction.md`](handoff/direction.md)) and keep the list of things to try. Sort each play session into lane jobs within the hour; post the bot's curve for planets 1 to 3. |
| Enemy | 4 | Done: the off switches settled (#451), the lure's field (#457), walls that guard the Trunk's crystals as a base (#473, his note 11-18 03:11Z). Make each of planets 1 to 3 do one nameable thing to the player by minute 2, with a clip a planet; then the origin, as a design page to Direction first. |
| Squad and arsenal | 5 | Done: the brief's five jobs, with Provoke and Still (#448) and the shop's offers (#459); no new card families; the Lure building, from planet 1 (#461, #464). Next is what his play notes say about control and the gun. |
| Look and interface | 2 | The pages that say what a build holds: the Experiments tab, "what is new", "things to try", the build number. Look A, the fight round the squad (the flinch's look is done, #454). One ranked list of look jobs and a review of every lane's visible change; win and lose screens. |
| Engine and tools | 1, 6 | Done: the switch list, build number and "what is new" file with its lint (#449); the play record and a Windows build from CI (#452); the run bot's first cut (#463), planet 1 won. **First:** one bot through planets 1 to 3 in a run, joined with the strategy bots (#529) rather than a second bot; then each planet's length against 8 to 12 minutes. |
| Lab | none | Delivered 11-17 and 11-18: steering (four parts, in the game as the Lure), a second species that hunts the first (parked), and the forest floor, rain that pools and dries ([report](Reports/lab-forest-2026-11-18/README.md); recommendation: change, then build). With the rot, and (11-18, his play notes) the slime as one body: marchers that will not step where fewer than two others touch keep a raid in one piece, 114 loose walkers against 365 ([report](Reports/lab-body-2026-11-18/README.md)); Enemy builds it into the raid march. The lab waits for a brief. |

**The Lure is in, from planet 1** (his card, 11-18 03:03Z: "Lure now, in
the three planets"; [lab report](Reports/lab-steering-2026-11-17/README.md)).
A lure every calm walker leans toward, so walls and turrets make a kill
zone: Enemy's field (#457; pull 10 on every planet; panic, surge and raids
ignore it, and a raid still turns for a lure near its target), Look and
interface's rings (#458), Squad and arsenal's building (#461, #464: on the
Build page from planet 1 with nothing to unlock, 300 food, 150 health).
Tinder is 100 and the shop's sure Torch at most 100, so the pair can always
be bought before planet 2. The numbers are Direction's to retune after he
plays. The siren and the outpost stay out.
The dart, the Lure's second form (lab part three), waits until he has played
the building.

**His play notes of 11-18 15:56Z come first** ([record](wiki/decisions-game.md)):
Enemy makes raids rare early and more frequent later in a planet (first
raid mostly 3:00 to 5:00, one game in five before 2:00, the gap from about
150 s to 45 s by 10:00), takes mini-bosses off planet 1, and makes slime
that leaves its body move as one (his words: the surge at the squad is
fine). Engine and tools ends the run when the last soldier dies. The Lab
explores the slime as one body on its branch. Direction set a planet won
only when its last walker dies (`WIN_SHARE` 0) and planet 2's slime at
0.6 of its size (#485).

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
Today: the pacing of threats across planets 1 to 3, starfall and boss
cores, the rush on planet 1 against building first (D7), what each of
planets 1 to 3 does to you, the starting rifle and one shop visit.
What each build holds is in
[`news/whats-new.txt`](news/whats-new.txt).

## Where things are

- Rulings, quoted and timed: [`wiki/decisions.md`](wiki/decisions.md), [`wiki/decisions-game.md`](wiki/decisions-game.md).
- What is true now, by subject: [`wiki/index.md`](wiki/index.md). Tried and rejected: `Reports/dead-ends.md`.
- Before 11-17: [`Reports/handoff-2026-11-17.md`](Reports/handoff-2026-11-17.md) and [`Reports/roadmap-2026-11-17.md`](Reports/roadmap-2026-11-17.md), history only.
