# Two independent reviews of the mould-tiers work (2026-11-11)

Both reviewers are fresh agents with no view of the thread beyond the repo.
They change no repo files; scratch work goes in /tmp/review-N/. Reruns use
`threads=1` and stay short (the machine has 4 cores shared with others).

## Review 1: the work that was done

> You are an independent reviewer of a research thread's work on a
> slime-mould agent simulation (Rust), repo /home/user/Slime-Mold on `main`.
> Read CLAUDE.md first for the project's rules. The owner wants interesting
> simulations that can be turned into a game, not a realistic slime mould;
> biology and the published Physarum models are inspiration, not the bar,
> but should not be ignored. The game is Larder World: a squad fights
> several mould species over energy crystals that run dry, in a persistent
> world on the GPU (plan: Reports/larder-world-roadmap-2026-11-11.md, its
> ladder of mould tiers in the section "Mould progression").
>
> The work under review added a per-walker life cycle to the CPU big dish
> (spores, dormancy, an alarm field the player's strike lays) and proposed
> three new ladder tiers, Sporer, Sleeper and Swarmer, each with a counter
> the player can learn. Start from Reports/mould-tiers-2026-11-11.md, then
> wiki/model/life-cycle.md, the scripts and raw output in
> Reports/mould-tiers-2026-11-11/, and the life-cycle code in
> src/mould/big.rs (`BigParams` spore_*, sleep_*, wake_*, alarm_*;
> `BigDish::life_step`, `strike`) and examples/bigdish.rs. The page is
> Reports/mould-tiers-2026-11-11/page.html. An earlier review of related
> work, and how the project reacted, is Reports/research-review-2026-11-11.md.
>
> Your main job is the big picture: tell us how to improve this work, not
> just whether the report is right.
> 1. **Approach.** Is this a good way to find game-worthy mould tiers? Are
>    the deciding measures the right ones for play? Would these tiers make
>    the player's decisions more interesting, or are they parameter tweaks
>    dressed as creatures?
> 2. **What the tests missed.** Controls and confounds: fixed walker count
>    (no births or starvation), one dish layout and body size, the strike's
>    fixed place, the 20 s runs, measures that could be satisfied the wrong
>    way. Claims that were stated but never tested (check each tier's
>    "counter"). Whether the tiers still hold at the game's operating point
>    (growth on, crystals that run dry, a squad that shoots).
> 3. **Biology and the literature as inspiration**: what real Physarum or
>    published agent models say about fruiting, sclerotia, alarm or
>    wounding responses, and what the rules got wrong or left out that would
>    make better play. Cite sources.
> 4. **Improvements, ranked** by expected value for the game, each with the
>    first experiment and the measure that would decide it.
> 5. **Correctness, secondarily.** Recompute a few sign tests from the raw
>    files; check that claims match their evidence (3-seed looks are not
>    wins); read the life-cycle code for bugs (occ bookkeeping, claims,
>    keyed draws, interplay with growth, crystals, walls and species sets);
>    check the defaults are off and the default hash 869141a7bf398adf holds
>    (`./target/release/examples/bigdish preset=dense seconds=2 blob=0
>    piles=1`, after `cargo build --release --example bigdish`).
>
> Write your review as your final message, in plain prose with short
> sections, most important first. Be specific and concrete; quote file and
> line where it helps.

## Review 2: future paths and missed directions

> You are an independent researcher advising a slime-mould game project
> (Rust), repo /home/user/Slime-Mold on `main`. Read CLAUDE.md first. The
> owner wants interesting simulations that can be turned into a game;
> biology and published Physarum models are inspiration, not the bar, but
> should not be ignored. The game is Larder World: a squad fights several
> mould species over energy crystals that run dry, in a persistent world on
> the GPU (Reports/larder-world-roadmap-2026-11-11.md). Mould types form a
> difficulty ladder that "grows as the research threads find new
> behaviours". Each needs a body, a way it reacts to the player and to
> being attacked, and a weakness the player can learn.
>
> The simulation, described neutrally: walkers on a 1024 x 640 grid each
> sense a trail field at three points ahead, turn toward the strongest,
> step, and deposit trail; the trail spreads and decays. Food sources give
> off a scent field. Options exist for several species with their own
> trails, crowd limits, grazing a regrowing field, growth and starvation per
> walker, crystals that drain, walls, and (new) a per-walker life cycle:
> spores that fly to new food, dormancy that wakes on a wave, and an alarm
> field that a strike lays (Reports/mould-tiers-2026-11-11.md,
> wiki/model/life-cycle.md). Dead ends are in Reports/dead-ends.md (grep it
> before proposing a mechanism).
>
> Your job: find the directions this work should have explored and didn't,
> and the most promising future paths. Start from the model and from what
> real Physarum does, not from the existing tiers or the player's current
> tools. Consider, among others: shuttle streaming and oscillation, memory
> and avoidance of its own extracellular slime, anticipation of periodic
> events, habituation, fusion and rejection between plasmodia, migration of
> a whole body, fragmentation and reassembly, light avoidance, sclerotia,
> sporulation, and what published agent models (Jones and others) show.
> Also say where the method itself should change (operating point, play
> probes, controls).
>
> Give 6-10 candidate directions ranked by expected value for the game.
> For each: the rule in one or two sentences (exact enough to implement as
> an off-by-default option), the creature or play it gives (body, reaction
> to attack, the player's counter), the first experiment with its deciding
> measure, its likely cost per step, and the source that inspired it. You
> may run short experiments on the CPU big dish (`examples/bigdish.rs`,
> threads=1, short runs) to check a hunch, but change no repo files.
> Write the result as your final message, most promising first.
