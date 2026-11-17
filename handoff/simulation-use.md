# Handoff: simulation-use review (2026-11-17)

**Area:** how the game uses the slime simulation (owner's ask, 11-16 21:44Z:
"How well are we utilizing this aspect of our engine... pathways we have
missed... think outside the box"). Plan, every report and page sources:
[`Reports/sim-use-2026-11-17/`](../Reports/sim-use-2026-11-17/README.md)
(start at its README; plan in `plan.md`, draft 3). Wiki: the "slime shapes
its world" paragraph in `wiki/world/larder.md`; rulings in
`wiki/decisions-game.md` (11-17 00:45Z, 01:13Z, 02:30Z).

## State

The whole of plan draft 3 is built and merged (last: #434, 11-17 15:4xZ).
No open branches or PRs. Branch name used: `claude/project-thread-i8ltol`.

| Step | Switch | Default | Report |
|---|---|---|---|
| Scar on GPU (Grudge) | `scar_*` | on in the Grudge tier | step0-scar-on-gpu.md |
| Crust | `crust=1`, `crustlook` | off | e1a-crust.md, e1b-c2-crust-effects.md |
| Setting (walkers set under fire) | `setting=1`, `setlook` | off, owner pick | e2a-setting.md, e2b-look.md |
| Bulwark wall or trap | `set=set_block=5` / `set=set_hold=0.05` | off, owner pick | e2b-bulwark.md |
| Dry slime burns | `fire_dry` | **on** | c1-dry-burns.md |
| Fed slime fireproof | `fire_fed=1` | off, owner pick | c1-dry-burns.md |
| Provoke / Still devices | Tool::Provoke, Tool::Still | hidden in runs until gun cards | b-provoke-still.md |
| Fire scares slime | `fire_alarm` | 50 on the Sprawler's planet only | c4-fire-alarm.md, c4b-fire-shy-sprawler.md |
| Gas ignites | `gas_burn` | **on** (0.2) | c5-gas-ignites.md |
| Charges round walls | `set=unit_paths=1` | off, in `play-new-slime.cmd` | s1-unit-paths.md |
| Far smell of food | `far_smell=1` | off, in `play-new-slime.cmd` | s2-far-smell.md |

## Pending owner picks

- **One blind film page** for setting on/off, wall or trap, the crust's
  pull sign (+8 keeps to old ground, -8 moves on), and `fire_fed`:
  [`Reports/sim-use-2026-11-17/page-pack/`](../Reports/sim-use-2026-11-17/page-pack/index.html)
  (open `index.html` locally; the old artifact link won't open on the new
  account). **The key is in `pack-films.md`**, so don't show that file to
  the owner before they pick.
- Set slime's look, glaze or amber: `page-e2b-look/`, key in `e2b-look.md`.
- `unit_paths` and `far_smell`: play `play-new-slime.cmd`. If the owner
  doesn't object, both become defaults in one PR (ROADMAP "Playtest pack
  11-17").

## Next steps

1. After the picks: set the defaults, and delete what loses (each delete
   gets a `Reports/dead-ends.md` entry, and its row comes off the ROADMAP
   switches table).
2. The run shop should offer Tinder before planet 2, the fire-shy
   Sprawler's planet (game designer, 11-17 15:26Z). This belongs to the run
   or gun card work.
3. Time every pass on the GTX 1070: scar, crust, setting look,
   walking-distance fields.
4. A film of the crust's +pull needs a hand that pushes the slime off its
   ground (blasts in run mode killed nothing).

## Rigs

- `Reports/sim-use-2026-11-17/page-pack/film.sh`: worked headless
  `larder-world look=DIR run=SEED:PLANET hand=... at=...` runs.
- On a machine with no GPU, set
  `VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json` (llvmpipe, about
  3x slower than real time).
- Probes: `examples/dryprobe.rs`, `farprobe.rs`, `wallprobe.rs`,
  `fearprobe.rs`, `gasprobe.rs`, `mindprobe.rs`.
