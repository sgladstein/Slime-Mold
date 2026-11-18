# E2b: set slime's own look, blind A/B (2026-11-17)

The plan's E2 ([plan.md](plan.md), draft 3): "set walkers are drawn by
their own state, a material settled by blind A/B (not dark ridges, which
read as the player's walls; not bronze, which the owner turned down; try
translucent amber against the body raised and glossy)". Page (blind):
https://claude.ai/artifact/4nbrWEwjSK9vo243aJWWCw (source `page-e2b-look/`).

## What was built

`view.wgsl`'s `set_share` and `set_mat`: the set walkers a cell
(`BigDish::set_cells`, the engine's block after the crust's set words, E2b)
smoothed over the 4 x 4 cells round by a Gaussian (sigma 0.7 cells), a
share from 0.2 to 0.6 of a cell set; the body's material blended by it
where the lump's skin is blended (`prep`, both height samples, as the
graphics thread asked). Only albedo, gloss and glow: the body's opacity and
height stay the sheet's (the graphics thread, 11-17), so the set slime's
edge is the set walkers' own. The crust row's spare words carry the block's
first word and the look (`View::set_setting`); larder-world's `setlook=N`
picks it (1 amber, the default; 2 glaze; 0 off). Nothing shows unless
setting runs (`setting=1`).

| Look | Albedo | Gloss | Glow |
|---|---|---|---|
| 1 amber | toward (112, 46, 14), a dark resin | toward 0.5 | warm (255, 110, 30) x 0.14 |
| 2 glaze | 55% toward (255, 238, 196), pale | toward 1.0 | none |

The first amber, (150, 62, 10) with glow 0.22, read as bright orange next
to the news-age ramp's amber (cut-off slime); darkened before the A/B.

**The blind key** (kept off the page): A is 2 glaze, B is 1 amber.

## Film

Planet 3, seed 4, `hand=stand gun=rifle crust=1 setting=1 setlook=0|1|2`,
llvmpipe, one frame a game second from 48 to 63 s (about 1,100 walkers set
by 60 s, all on the rim the squad shoots). Same run each look (the look
doesn't touch the simulation).

## Frame cost

Off (no setting, or `setlook=0`): one uniform test a fine cell in `prep`.
On: 16 loads from `news` a fine cell in `prep` (the 4 x 4), a material blend
where the share is over 0. 1070 timing at zoom 1 and 6 (the graphics
thread's ask) to book with the scar's and crust's.

## Settled (2026-11-18): amber

The Look and interface lane's pick (the plan of 11-17 gave it the pending
look picks; `wiki/decisions-game.md`, 11-17 01:13, has amber as the set
slime's look). Amber is the only look; the glaze, the unmarked side, the
Experiments row `set_look` and the key `setlook=` are deleted, which
takes its row off the switch list (two rows are left: the free
recruits left it the same day, #459).

Judged on the game as it is on main at 7a38050, where the Trunk sets on
planet 3 by default: run seed 1, `hand=hold@344,169 gun=rifle`, the frame
at 72 s (1,699 killed, the two set patches the squad shoots), the same
game three ways (`setlook=1|2|0`), zoom 6 (`e2b-settled.jpg`: amber, glaze,
unmarked, left to right) and amber at zoom 12 (`e2b-settled-z12.jpg`).

| Look | A set patch, mean colour | Reads as |
|---|---|---|
| Amber | (160, 89, 28) | a darker, redder sheet in the slime's own hue family |
| Glaze | (193, 148, 78) | pale sand: it left the slime's hue for the team's cream, which is also the wanderer's pale bone and the Lure's rings |
| Unmarked | (169, 113, 30) | ordinary slime, though five set walkers round a cell stop a soldier |

The fed body beside the patches is (234, 153, 37), and slime under alarm
flushes to about (255, 161, 54) ([flinch look](../flinch-look-2026-11-17/README.md)),
so flushed slime and amber set slime part by value, the set darker by a
third. What amber lacks is a material: at zoom 12 it is a flat sheet with a
soft edge, not something hard. That is a look job (the art direction's
list, B), not a switch.
