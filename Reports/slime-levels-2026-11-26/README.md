# Slime levels (2026-11-26)

Owner, 11-26 08:24Z: "You could change slime color so their are 3-8 levels of slime. The slimes could be mixed to so one area on a later planet is the easier to kill orange slime and this other crystal is full of stronger more dangerous slime." And 08:40Z: "You can make raids stronger too."

Every crystal has a level, 0 to `run::level_max` (0 on planet 1, 7 on the home world; the home crystal takes the top one, the others `floor((max+1)*u^1.5)`). A walker born on a crystal takes its level and keeps it as it spreads; the existing slime takes its nearest crystal's level when levels are set. Each level adds 0.5 hit, 6% speed and 0.3 bite (engine keys `level_blows`, `level_speed`, `level_bite`; rule `level_max`). The view colours each fine cell by its walkers' mean level, so the colour is the data: orange, red, crimson, wine, plum, violet, indigo.

Raids: gaps 150 s to 70 s from planet 1 to 8, size 15% to 30% of the slime, armour 0 to 8.

Page: `index.html` (https://claude.ai/artifact/4ioUW9Ga6b4b1Q8bhGibVZ). Run seed 1, step 7 (Sporer, levels 0 to 4), 0:40, main against the change, lavapipe.

Bot checks (rush plan, run seed 1, lavapipe): step 3 won in 3:12 (38 bought, 3 fell); step 6 lost at 3:05 (17 bought, 21 fell). On main with blows only, step 6 was wiped at 3:36. GPU test: all crystals at level 3 with `level_blows` 1 gives 90%+ level-3 walkers and fewer kills than level 0.
