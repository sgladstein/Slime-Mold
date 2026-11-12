# Two independent reviews of the mould tiers (2026-11-11)

The owner, 20:11Z, in the mould tiers thread: "run two agent reviews of your
work: 1) one review what you already did. 2) Explore future paths or missed
directions that you should have explored." And 20:12Z: "Also make sure you
work is being integrated into the game". The prompts were shown to the owner
first (project rule, `wiki/decisions.md`), and the owner said to start at
20:18Z.

Both reviewers were fresh agents with a clean checkout of `main` at 3dabcb7
(after #70, the tiers). They changed no repo files and ran short reruns with
`threads=1`. A container restart at about 20:31Z stopped both; they were
resumed from their transcripts and finished at 20:40Z and 20:42Z.

- Review 1, the work done: [`review-1.md`](mould-tiers-review-2026-11-11/review-1.md)
  (verbatim), probes in [`review-1-probes/`](mould-tiers-review-2026-11-11/review-1-probes/).
- Review 2, missed directions: [`review-2.md`](mould-tiers-review-2026-11-11/review-2.md)
  (verbatim but one line about a search service's quota), pulse prototype and
  runs in [`review-2-probes/`](mould-tiers-review-2026-11-11/review-2-probes/).

The thread's response, and what it changed, is at the end.

## Prompt 1: the work that was done

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

## Prompt 2: future paths and missed directions

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

## What each review found, in short

**Review 1.** The engineering holds: rules deterministic and off by default,
the default hash holds, seed-1 reruns match the raw files, all five sign
tests recompute. But each 8-of-8 test shows a rule does what it was built to
do on a 20 s dish with a fixed walker count, under a bomb-sized strike; no
test checks the counter it names, and none ran at the game's operating
point. Found: the game's shots laid no alarm, so all three answers to attack
were inert in Larder World; the test strike kills 21-33 times a shot's area
(though a bullet-sized strike still sets off the burst and the wake); in the
world probe the Sleeper froze into an immortal crust (dormant walkers pay no
upkeep). The Sporer is a real creature, the Swarmer a modifier, the Sleeper
interesting only when woken. Also: T2 is a mould that spores only when
struck, not the T1 Sporer, and its strike was radius 10, not 8; the bigdish
header lacked the new keys; M3's spore and crust dead ends were not cited;
the life-cycle fields are global, not per species.

**Review 2.** Ranked directions: (1) supply lines, traffic-made arteries
that carry food, which a cut starves; (2) habituation, a mould that learns
to ignore a repeated trick; (3) fusion and rejection between clones, with a
scar; (4) anticipation of rhythmic fire; (5) pacemaker waves (prototyped: a
heartbeat that pushes the mould outward, not a moving body); (6) day and
night light; (7) a nomad that avoids its own slime (check grazing first);
(8) two foods. Method: judge new rules in the world race, test counters
against an equal-cost random action and against other tiers' counters, run
grids of player choices to see agency, give every tier a pace number, 16+
seeds with Wilcoxon and Holm.

## The thread's response

Changed in the same PR as this record (the owner's second ask, "make sure
you work is being integrated into the game"), with the numbers in
[mould-tiers-2026-11-11.md, *In the Larder world*](mould-tiers-2026-11-11.md):

- **Shots lay alarm** (review 1, section 2, its first improvement).
  `squad::shots` lays `SHOT_ALARM` (50) within twice a hit's radius when the
  alarm field is on; off by default, so the GPU twin is unchanged.
- **Named tiers for the game.** `mould::tiers::Tier` (Sporer, Sleeper,
  Swarmer) sets each tier's keys on the CPU dish and refuses on the GPU,
  naming the missing key, until the GPU port lands. `bigdish tier=`.
- **The game's operating point** (review 1, sections 2 and 4; review 2,
  method 1). The three tiers re-tested in the Larder world's race (growth,
  23 crystals that run dry), 8 paired seeds: the Swarmer (8 of 8) and the
  Sleeper's bait (8 of 8) hold; the Sporer's edge does not (more walkers at
  60 s on 7 of 8, p 0.07; its struck burst 4-2-2).
- **The frozen Sleeper** (review 1, section 4). Confirmed on 4 seeds of the
  reviewer's lean world. Paying upkeep while asleep was tried at a half and
  a tenth and lost the bait counter (a dead end now); a dormant walker now
  dies with chance `sleep_death` (1e-4 a fine step in the tier), which keeps
  the bait and lets the crust fade.
- **Report errors** (review 1, section 9): T2 relabelled as a mould that
  spores only when struck, its radius corrected to 10, in a dated note; the
  bigdish header lists the life-cycle keys.
- **The counters are proposals** (review 1, section 3): the report, the
  life-cycle page and the ladder page now say so.

Not done here, and where each goes:

- Counters as plans against random and idle hands, shot-sized and sustained
  fire, 16 seeds with Wilcoxon and Holm (review 1's improvements 1-2;
  review 2's method 2-3): these need the player AI in the world, so they go
  with W2b (Larder thread), using `Tier`.
- Life-cycle fields per species (review 1, section 6): for mixes (W3), with
  the creature and GPU threads.
- Each tier beating a tool (spores over walls, waking to the soldiers'
  smell, a Swarmer that drains soldiers) and a cost and trigger for fruiting
  (review 1's improvements 3-4): next rule work on the tiers.
- Species conversion of dormant walkers and spores (review 1, smaller
  issues): noted on the life-cycle page, left as it is.
- Review 2's eight directions go to the ladder page's *Open* list, supply
  lines and habituation first, each with the reviewer's first experiment.
