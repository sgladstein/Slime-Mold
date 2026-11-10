# Extra rules for the two plant-engine probes (7 Kudzu, 8 Heartwood)

These two probes run on the shipped plant engine (`src/sim/plant.rs`, `src/sim/organism.rs`, `assets/species/`), which the lab's plants and ants live on. The lab is the main game and is being trained against, so:

- **Never change plant behaviour for other binaries.** Speed-ups, weight overrides, wall-clinging and so on must be switched on by your game only: a per-world or per-species setting that defaults to today's behaviour. Do not change `ORGANISM_TICK_INTERVAL` or any shipped species file in place. If you need a faster species, add your own species file or a per-world multiplier.
- **Before editing `plant.rs`, `organism.rs` or anything in `assets/species/`, run `bash scripts/branchcheck.sh --who-touched <path>`.** The Plants lane and the Plants-and-ants lane work there. Keep your shared-file diff small and additive, so it merges cleanly against theirs.
- **Adding a species enrols it in every rule over the species registry** (`CLAUDE.md` Gotchas). Run the `tests/*.rs` files that iterate species before you push.
- **Prove that the lab did not move:** in your PR body, give one lab result before and after your change on the same seed (for example a `labshot` or `latecensus` run that is short enough to be cheap). It should be identical to the cell.
- **The `plants-and-organisms.md` rules file** arrives when you Read `plant.rs`. Follow it.
- **Speed costs frame time** as tips multiply. Quote the `ascii` worst frame at the peak tip count, not at the start.
- `wiki/plants.md` is the plain-words description of what the engine does. Read it before the source.
