# The kit's own sounds (2026-11-24)

Direction's job: the Sprayer, Hunter, Mortar, Grenade, Mine, Line,
Canister and Dart each get a sound of their own, scaled to their damage and
kept under the music bed's mix (#692).

What each sounded like before, and now (`src/sound/world.rs`):

| Kit | Before | Now |
|---|---|---|
| Mortar | nothing when fired; a plain turret's crack where the shell landed | a hollow tube thump (`GunVoice::Mortar`), a whistle down, a crump where it lands, sized by its strike |
| Hunter | as the mortar | a ringing crack with a bolt after (`GunVoice::Hunter`); a sharp knock where it lands |
| Sprayer | silent | a hiss (`GunVoice::Sprayer`) and a patter of drips across its arc, the quietest gun |
| Grenade | a thud, impact and dirt | the same: the kit's biggest blow, the one the rest are weighed against (`blow`) |
| Mine | the Grenade's sound | a click, then the same blow, with more dirt |
| Line | a full Grenade thud for each of its blasts (13 on a 24-cell line) | one ripple of at most 6 small thuds end to end, a crackle and one impact for its kills |
| Canister | silent | a clink and a long hiss, a short soft one for each later puff (`Kit::puffs`) |
| Dart | silent (it leaves a lure mark, no pop) | a thwip, a wet squelch and a few drips where it sticks |

A blow's level is the square root of its area against the Grenade's
(`blow`, radius over 6 dish cells): a mortar shell (3 cells) half the
Grenade, a Line's blast (2.5) 0.42 each.

## Levels

Planet 1, run seed 1, squads held at 222,114 (`hand=hold@222,114`), a
mortar, hunter and sprayer by them, a Grenade at 9 s, a Line at 17 s, a
Canister at 23 s, Darts loaded at 28 s (`LARDER_DEVICES`, new in
`headless.rs`); the mine filmed apart in the slime's body. RMS in 0.4 s
windows:

| | dBFS |
|---|---|
| whole clip (the #692 measure was -20 and -18 on planets 1 and 2) | -18.8 |
| loudest window: the Grenade | -14.9 |
| the Line's ripple | -15.7 |
| the Canister | -16.6 |
| a Mine (its own clip, median -20.6) | -15.6 |
| Darts among the gunfire, 28-30 s | -21.6 to -16.4 |

The limiter pulled down 0.18% of samples. Over 36 s: 17 shells sent and
landed, 22 sweeps, 1 ripple, 1 canister release, 64 darts heard.

Clips with sound: [kit-sounds.mp4](kit-sounds.mp4) (6 to 36 s, the squad's
view at zoom 6, 4 frames a second) and [mine.mp4](mine.mp4) (two mines in
the body at 10 and 12 s). Filmed on software Vulkan.
