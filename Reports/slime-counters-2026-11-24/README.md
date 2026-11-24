# A counter for every slime (2026-11-24)

Direction's job for Milestone 2: for each planet's slime and each twist,
check that at least one kit choice answers its trait visibly within a
minute, and leave the numbers alone where one already does.

Films: `larder-world look=DIR run=SEED:PLANET mode=squads hand=HAND
load=1,CARDS` (or `turret=`/`pump=` for buildings), software Vulkan,
paired against the same game with plain shots. A smoke check, not a
census: one seed unless stated. Planets 4 and up start bare (no run's
upgrades), so a wipe there says only that the pair differs.

| Planet | Slime | Answer | Plain shots | With it | Still |
|---|---|---|---|---|---|
| 1 | Drifter (smells your pumps) | a turret beside the pump | pump worn to 0 by 1:30 | pump at 191 of 200 at 1:30 | [p1](p1-drifter-turret.jpg) |
| 2 | Sprawler (threads find soldiers) | Tinder and a Torch | threads touching a standing squad (0.8 mean, 3 most) | none touching; 316 burned | [p2](p2-sprawler-torch.jpg) |
| 3 | Trunk (sets hard, walls crystals) | a Torch | slime 37,406 at 1:00 | 29,361; 10,588 burned | [p3](p3-trunk-torch.jpg) |
| 4 | Swarmer (surges charge you) | a Bomb (or turrets by the squad) | wiped at 0:44 (seed 2: 11 soldiers at 1:30) | 25 soldiers at 1:30 (seed 2: 21); two turrets 18 | [p4](p4-swarmer-bomb.jpg) |
| 5 | Sporer (struck, it scatters spores) | a Germ | slime 91,854 at 0:40 (seed 2) | 18,714; seeds 1 and 3 alike (27,335 against 89,973 at 0:30; 26,178 against 110,905 at 0:40) | [p5](p5-sporer-germ.jpg) |
| 6 | Sleeper (grows while it sleeps) | a Flare, then a Germ | wiped at 0:38 | 10 soldiers at 1:00, slime 80,903 against 137,363 | [p6](p6-sleeper-flare-germ.jpg) |
| 7 | Grudge (tough where it was hurt) | none shown | | | |
| 8 | Dozer (each shot wakes it, awake it charges) | a Bomb | wiped at 0:48 | 13 soldiers at 1:00 | [p8](p8-dozer-bomb.jpg) |

What did not answer: a Blast on planet 4 (wiped at 0:40) and a sprayer by
the squad (0:35); Gas on planets 4 to 8 (bit for bit the plain game: a
shot's alarm wakes a gassed walker at once, and these slimes all keep the
alarm); Gas then Shatter on planet 6 (the Shatter kills only gas-sleepers).

**The Grudge.** Its scar tissue makes blows miss on ground fought over
(up to 0.5 at 2,000 scar, about a minute of fire on one spot), and fire
burns the scar off (`scar_burn`). In a minute's film no squad built the
scar: a farming squad fired 198 shots by 1:00; a standing squad 98 by 1:30;
four soldiers rushing were eaten by 0:53 with or without a Torch. Its
answer needs a longer film on one spot.

**The twists** are answered by play, not kit, and their cards say so: the
cold snap is the minute to push; the wind blows the slime onto guns set
downwind (the Lab's planet 1: wind towards an idle squad killed 60% more
by 1:30, `Reports/lab-twists-2026-11-24/`).

**What changed.** `run::bring_shell` asks for the counter bullet on
planets 4, 5, 6 and 8 (Bomb, Germ, Flare, Bomb), as it asked for a Torch
on 2 and 3: the shop before the planet puts it first until you own one,
and the run bot buys it (`runbot::shop`). No number changed.
