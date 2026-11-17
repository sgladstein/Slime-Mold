---
name: game-sound
description: Game sound: dish sound (PR #23), Larder World sound (11-12, O toggles; voice K), C7 new sounds + cut-off pieces (PR #217, 11-14); levels await the owner's ears
metadata:
  type: project
  modified: 2026-11-12T04:43:37.992Z
---

Dish sound built and merged 2026-11-10 (PR #23, 7abe2a9): src/sound/ (listen.rs reads &Dish, synth.rs voice bank, device.rs cpal gated), on by default, S toggles; examples/mouldsound renders WAVs (hands idle/random/plan/tour).
Listening page: https://claude.ai/artifact/6GXBj1swj93DzdytbUjqvA (source Reports/mould-sound-2026-11-10/). Report Reports/mould-sound-2026-11-10.md, wiki looks/sound.md.

Larder World sound (2026-11-12, thread "Add sound and hit feel to Larder", branch claude/project-thread-672puq): src/sound/world.rs WorldListener reads &Game<W>; each shot = Crack + Impact weighted by Shot::killed (kill_weight = sqrt(killed/FULL_KILL), FULL_KILL 80 = rush 90th centile on fork s1); kill crackle; pulse from Soldier::aim distance; hurt/fallen; larder steal, banking (gathered 1/s), buy/wall/dry/core/end. Ear = camera view. `O` toggles in larder-world; `larder-world look=DIR hand=rush at=120 sound=x.wav` records headless; examples/shotsound.rs renders the kill ladder. Report Reports/larder-sound-2026-11-12.md, page https://claude.ai/artifact/P4hk1fsSXQj3qXuwieffa7. Graphics thread owns hit visuals (no screen shake: drawn-only). Game integration thread wants to be told when sound PRs open/merge; squad toolkit's Frame.bursts (blasts, mines, siren) should get sounds later.

C7 (2026-11-14, PR #217 merged 03:16Z, 7d09bb0): zips for fired bullets, siren wail on Game::sirens, kit blasts via WorldListener::bangs (window wired), and src/sound/pieces.rs: cut-off pieces from the mould grid (one dish cell a square, link floor 2 walkers, piece 100 walkers = 0.4% of start up to 10% of the mould, torn after 2 s, ghost 2.5 s): tear, wither texture, rejoin, snarl when a piece breaks up beside a soldier, up to 3 piece voices (Body slots 1-3). Missions set piece_bin = campaign BIN. Listener 21-23 us. Page v3; WAVs in /mnt/project-files/larder-sound-c7/. A pre-charge warning would need a stale-news count in the GPU readout (not built). The staged `cut=` ring leaves stranded walkers that bridge the cut ~7 s.

**How to apply:** level references are first guesses; tune only from the owner's listening feedback. The owner has not heard either live yet (no device in the cloud). Re-measure FULL_KILL if strike or kill counting changes. Lamp sizzle is silent while Params::burn is 0. Related: [[pixel-physics-review]], [[larder-world-playtest]].

Mould voice (PR #99 merged 2026-11-12 05:15Z): SoundCmd::Body drone from Game::mould (log level), natural change (hunger/growth), eating (charge drop minus squad carry), kill-share inhales; K toggles, voice=0 headless; A/B on the Larder Sound page. Pieces (per cut body) wait on the one-body thread's piece id.

Follow-up owed (2026-11-12): once the Larder thread's PR with `Game::density` (start walkers per open fine cell, commit 05f5f29) merges, switch `sound::world::full_kill` to read it (currently start_mould per dish cell ^ DENSITY_LAW 0.3, measured fork s1 5k vs 50k).

Pending (integration thread, 2026-11-12 16:21Z): if the owner takes the denser-mould census reset (`Game::walker_mass()`, ROADMAP "Census reset held"), scale the touch texture (`density(touching, 2, 20)`) by `Game::mass(touching)` and re-check the shot weight (full_kill already scales start walkers per dish cell ^0.3); check by ear/recording. Nothing changes before the owner answers.
