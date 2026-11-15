# Bug-check leftovers fixed (2026-11-15)

Three of the open items in the [11-15 bug check](../bug-check-2026-11-15/README.md) (*Read in the code, left open*), in one change (thread "Fix leftover
bug-check items"). No shader change.

- **Fire kills.** `Game::finish_step` now asks `World::burned` once a
  frame and counts the plants' fire in `Game::killed` and the new
  `Game::fire_killed`; before, only a Scorch trail's frame asked, so a CPU
  game without one never counted a fire kill. On the GPU the strike
  counter (`wstat[3]`) is shared with `firekill`, so `GpuWorld` now reads
  it from every frame's readout (already copied there): what moved over a
  frame is fire's, as bursts run only between frames. A kill burst after a
  burn reads the counter first, so the burn's kills stay fire's and the
  blast counts only its own. No new read-back a frame (one before a kill
  burst only when a burn came first in that gap). A Scorch trail's burns
  still count in `Rounds::burned` on both. Test
  `the_plants_fire_kills_count_without_a_scorch_trail` (flats, full plant
  cover, a burn by the slime's home at 10 s): 5 fire kills by 40 s, all 5
  in `Game::fire_killed` (0 before: the field did not exist and nothing
  asked).
- **Plants in the tools.** `plants::planted(map, seed)` is the window's
  `stand_in` on a copy; the `campaign`, `larderworld` and `kitplans`
  examples plant each seed's map with it (`plants=0` plays bare, as the
  window's). `cutprobe` stays bare (hunk probes light no fire). The
  library's tests keep their bare maps.
- **N on a seeded planet.** N on `planet:D:SEED[:AREA]` goes on to
  `planet:D:SEED+1` (the name's seed wins over `seed=`, so the old fresh
  seed changed nothing); a seedless planet still rolls a fresh seed. Test
  `n_on_a_seeded_planet_goes_on_to_the_next_seed`.
