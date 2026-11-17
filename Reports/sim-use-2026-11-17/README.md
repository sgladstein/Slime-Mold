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

Round 1 (11-16, posted in the thread, not a page): since the 11-15 review
the run uses the body well (shape C, flinch, pump smell), but new pressure
is mostly targets written into the model (`panic_to`, `surge_to`,
`raid_to`, roam, lumps); the player writes only damage fields (`Burst`:
kill, burn, alarm, crater, gas, germ, fling, plus shove and Mind), never
the slime's trail, scent or food news; unused: the network's flow, the
Grudge's scar (CPU only, not in `LADDER`), species rivalry, plants (not
eaten), char (inert), a kill's leftovers, reactions between fields.
