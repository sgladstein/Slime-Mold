# Playtest pack 11-17: the blind film page (2026-11-17)

The integration thread's playtest pack ("Playtest pack 11-17" in
`ROADMAP.md`, PR #433), group 2: one blind page with one film per open
simulation-use decision, from this thread's rigs. Page:
https://claude.ai/artifact/QNErAVSGJgoLWhSHY9MJDF (source `page-pack/`,
blind; the key is below). It also carries one line for the gun thread's
pick, whether Heave and Corral merge into one shove card, linking
[their page](https://claude.ai/artifact/2xhG8dfegCSryLGDPexpdE)
([report](../gun-shove-2026-11-16/README.md)); no new film for that.

This page supersedes the open cards for setting's pull sign, block or trap,
and `fire_fed` as the place to pick. The look (glaze or amber) stays on its
own page ([e2b-look.md](e2b-look.md)); these films use amber (`setlook`
default 1).

## The films

Made by `page-pack/film.sh OUT` (the runs) and `page-pack/film.py OUT
KEY.json` (side by side, A left), from main at 1b45d4ed, larder-world on
llvmpipe. Raw lines stay in the scratch output; every number below is
copied from them. A look, one seed each, not a census.

| Film | Run | A | B |
|---|---|---|---|
| 1 Setting | planet 3, seed 4, `hand=stand gun=rifle crust=1 crustlook=1 setting=0/1`, 0:30-1:29 | killed 771 from 0:45 to 1:15, 1,403 by 1:29; 3 soldiers at 1:15 | killed 934 from 0:45 to 1:15, 1,609 by 1:29; 2 soldiers at 1:15 |
| 2 Wall or trap | planet 3, seed 1, `hand=push gun=rifle crust=1 crustlook=1 setting=1 set=set_block=5`, trap with `set=set_hold=0.05`, 0:03-0:32 | lost at 0:28, killed 1,365 (bulwark 2,510 cells at 0:20) | lost at 0:20, killed 840 (bulwark 2,464 cells) |
| 3 Old ground | planet 3, seed 1, `hand=idle crust=1 crustlook=1 crust_fast=1 crust_weight=+8/-8`, every 5 s to 5:00 | 43,366 walkers at 5:00; 73,244 cells ever held (crust over 0.05) | 50,869; 98,284 |
| 4 Fed fireproof | `dryprobe seeds=1 look=DIR`, the joined disc (1,203 walkers, none stale) lit at once, `fire_dry` on in both | fire killed 777 in 20 s | 0 |

Films 1 and 3 match the E2a and E1b reports' numbers where they overlap
(E1b's ±8 rows exactly; E2b's push and trap rows exactly). Setting on
seed 4 cut the standing squad's kills from 0:45 to 1:15 by 17% here.

## The key (kept off the page)

Coin tosses (`secrets.choice`), one a film:

- **1 Setting: A is setting on (`setting=1`), B off.**
- **2 Wall or trap: A is the wall (`set_block=5`), B the trap (`set_hold=0.05`).**
- **3 Old ground: A is `crust_weight=+8` (keeps to it), B is -8 (moves on).**
- **4 Fed fireproof: A is `fire_fed` off, B on.**

## Left

- The owner's picks set the defaults: `setting`, `set_hold`, the pull's
  sign and size, `fire_fed`; each goes on or off in the same PR that
  records the pick.
- `fire_alarm` came out of the new-slime preset (the designer, 14:38Z): as
  a global switch it rings every fire. The next step is a per-tier key so
  only a fire-shy tier (the Sprawler) carries it.
