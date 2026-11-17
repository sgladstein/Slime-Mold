# How the game uses its simulation, and a plan to use more of it

2026-11-16/17. Thread *Review how the game uses its simulation*. The owner
asked (11-16 21:44Z) how well the game uses the simulation, what pathways it
has missed, "think outside the box"; after round 1's questions (21:5xZ) they
answered (11-17 00:45Z): field-driven over scripted where it doesn't make the
game worse; ideas may go outside the run; the slime building terrain first,
then reacting fields and faking its signals; "first we write a detailed
plan. Have it reviewed then start to implement"; independent reviewers yes.

- [plan.md](plan.md): the plan, draft 3 (after the reviews) (crust and bulwarks, reacting
  fields, provoke and calm, scripted pressure onto fields where no worse).
  Doc copy: https://claude.ai/artifact/9dMLbiVKoK58zVMAqSCCxb
- [reviews/prompts.md](reviews/prompts.md): the three reviewers' briefs
  (shown to the owner and agreed 01:13Z).
- [reviews/game-designer.md](reviews/game-designer.md),
  [reviews/alife-researcher.md](reviews/alife-researcher.md) (with an
  offline crust probe on the CPU engine and two numpy toys; not captures),
  [reviews/readability.md](reviews/readability.md): the reviews of draft 2.
  What they changed is the table at the top of the plan.
- [step0-scar-on-gpu.md](step0-scar-on-gpu.md): step 0 built, the Grudge's
  scar (threat memory) on the GPU, twin-exact on llvmpipe, and the Grudge
  back in the run as planet 7.
- [e1a-crust.md](e1a-crust.md): E1a built, the crust field in both engines
  and its satin look, behind `crust=1`; offline look in `crust-look/`, page
  in `page-e1a/` (https://claude.ai/artifact/7S91NqGRDSopYhhje3giS6).
- [e1b-c2-crust-effects.md](e1b-c2-crust-effects.md): E1b and C2 built, the
  crust's pull and fast ground, crust as fuel and barren char; films and page
  in `page-e1b/` (https://claude.ai/artifact/6Qpdh7rTKKpGEgMhD9btAR), raw
  lines in `e1b-raw/`.
- [e2a-setting.md](e2a-setting.md): E2a built, walkers on crust set where
  the scar says they were hurt (stop, no births, three blows to kill, wake
  as the scar fades), behind `setting=1`; films and page in `page-e2a/`
  (https://claude.ai/artifact/Vx8Qz8v3VAqvZaUL95FMK2), raw lines in
  `e2a-raw/`.
- [e2b-bulwark.md](e2b-bulwark.md): E2b's first part built, set slime is no
  ground to soldiers' feet and their walking maps route round it, behind
  `set=set_block=N`; films and page in `page-e2b/`
  (https://claude.ai/artifact/AKuE9RVmPtcNqsK1nW93Dx), raw lines in
  `e2b-raw/`.
- [e2b-look.md](e2b-look.md): set slime's own look, two materials (amber,
  glaze) for a blind A/B, page in `page-e2b-look/`
  (https://claude.ai/artifact/4nbrWEwjSK9vo243aJWWCw).
- [c1-dry-burns.md](c1-dry-burns.md): C1 built, starving and set slime
  burn (the trail no longer wets slime with stale food news), on by
  default; a stronger "fed slime is fireproof" form behind `fire_fed=1`;
  probe `examples/dryprobe.rs`, raw lines in `c1-raw/`, page in `page-c1/`
  (https://claude.ai/artifact/6CrHne8QszUbfgVzpcG825).
- [b-provoke-still.md](b-provoke-still.md): B1 and B2 built, Provoke (deaf
  to food news) and Still (no charge, no crystal) as Mind statuses on both
  engines and test devices on the tools page; probe `examples/mindprobe.rs`,
  raw lines in `b-raw/`, page in `page-b/`
  (https://claude.ai/artifact/6PUddP5ThBHR6AQnEZmwip).

Round 1 (11-16, posted in the thread, not a page): since the 11-15 review
the run uses the body well (shape C, flinch, pump smell), but new pressure
is mostly targets written into the model (`panic_to`, `surge_to`,
`raid_to`, roam, lumps); the player writes only damage fields (`Burst`:
kill, burn, alarm, crater, gas, germ, fling, plus shove and Mind), never
the slime's trail, scent or food news; unused: the network's flow, the
Grudge's scar (CPU only, not in `LADDER`), species rivalry, plants (not
eaten), char (inert), a kill's leftovers, reactions between fields.
