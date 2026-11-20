# The last-stand heart (2026-11-20)

The Enemy lane, the thread "Make the slime a more interesting enemy". This is D of
the balance thread's brief "Keeping a big squad under threat". It comes after the
condensed strikers (A) and the late lunge (B), which the owner picked on 11-20 at
15:10Z.

Code: `src/mould/larder_world/stand.rs` (`StandRules`, `Stand`, `Heart`). It is on
in every run (`run::ENEMY_RULES`' `stand`). The look is `LumpLook::Heart`
(view.wgsl `lump_skin` and `lump_core`, case 7). GPU only.

## The rule

The heart forms when the slime is under 20% of its peak and the squad has 24 or
more soldiers.

- **Gathering.** Every walker that is in no other lump grows into one lump on the
  heart crystal (`Game::heart_site`). It takes in new slime every 3 s.
- **Toughness.** Each member takes 4 blows and bites as two walkers.
- **Lunges.** Every 20 s the whole heart lunges at the nearest soldier, at 3 times
  its pace, for 10 s. Then it draws back.
- **Breaking.** It breaks back into slime when it falls under 15% of the most it
  held, or when it is 150 s old.

## Engine test

Test: `the_last_stand_heart_gathers_the_slime_and_lunges`, on a race world.

- The heart gathers all 3,000 of 3,000 walkers.
- In an 8 s lunge, its distance to the nearest soldier falls from 179 to 66 fine
  cells.
- At rest afterwards, it draws back toward its crystal.

## Bot games

These are strat bot games (`econ,go=4,aim=6,cards=0,want=wide/rate/blast.needle`)
on planet 2, one game each on lavapipe. They are compared with the strikers retune
of the same evening.

| Map | Army | Without the heart | With the heart |
|---|---|---|---|
| 1 | 80 | won 7:39 | won 7:55; two lunges took the army from 73 to 57 and from 74 to 56 |
| 5 | 80 | won 3:31 | won 3:59 |

Rejected:

- **Keeping the heart to the end.** On map 5 the heart was cut to 30 and sat on its
  crystal for 130 s. The planet ran to 7:19 with no soldier lost.
- **A last charge until it died.** On map 2, 29 walkers stayed pressed on the
  ground until the game's time cap.
- **An army threshold of 12.** On map 2, 3,578 walkers met 14 soldiers and the
  planet was lost at 5:40.

## Clips

The clips are forced, so the heart can be seen at once. It forms at landing
(`set=stand_at=2 set=stand_army=1`), so it holds the whole slime (13,845
walkers), not a remnant. The command was
`larder-world look=DIR run=1:2 mode=squads hand=greedy set=stand_at=2 set=stand_army=1`.
The clips run from 19 to 26 s, half a second a frame, and the lunge comes at 21 s.

- `heart-zoom6.mp4` (`centre=365,160,6`) and `heart-zoom6-22s.jpg`
- `heart-zoom12.mp4` (`centre=355,178,12`) and `heart-zoom12-22s.jpg`

The first skin, a gold of (150, 74, 8) over a (255, 236, 150) core, washed the
packed body pale yellow at zoom 12. It is now a deeper amber of (110, 48, 4) over a
(255, 196, 80) core.
