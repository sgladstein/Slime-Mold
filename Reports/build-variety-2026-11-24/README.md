# Build variety: three combos a rule now links (2026-11-24)

Direction's ask (11-24 18:37Z, Milestone 2 test 2): the owner's balance
rule is that strong builds come from combinations, often with rarer cards.
Find two or three in today's pool that should be strong together but are
not, because no rule connects them, and make each work and show in play,
with no single card made stronger alone. No census.

All films: `run=1:1 mode=squads hand=greedy load=5,CARDS centre=squad,6`,
software Vulkan; counts from the film's log.

| Combo | Before (no link) | With it | Still |
|---|---|---|---|
| Heave + a sprayer (also mortar, hunter) | heave pushed slime back unless a *turret* stood near | slime herded onto the sprayer: 96 killed by 25 s, 114 by 40 s (plain shots: 78, 93) | [heave-sprayer.jpg](heave-sprayer.jpg) |
| Split + gas (also germ, Mind) | the pieces only hit | each piece casts the gas at 0.7 its disc: 183 gassed by 20 s (gas alone: 21) | [split-gas.jpg](split-gas.jpg) |
| Flare + gas | a flare lit nothing | a flare in a gas cloud lights it: 62 burned by 26 s, 158 by 60 s (flare alone: 0) | [gas-flare.jpg](gas-flare.jpg) |

Each card alone is as before: a heave with no kill zone pushes back; a
split blast or plain round's pieces cast nothing; a flare on plants lights
nothing (its light is 1e-6). The danger stays: herded slime comes at the
squad, and a lit cloud burns whatever stands in it.

The films for the gas counts differ in more than the card (each loadout's
game runs its own way from 5 s), so the counts show the combo working,
not a measured gain.
