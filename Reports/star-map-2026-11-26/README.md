# The star map: are a fork's planets clearly ordered? (2026-11-26)

The owner's pick (2026-11-25 23:45Z): "I choose 1 + 4 and each of those
planet options should have clearly different difficulties with greater
rewards on the harder planets." The build is `src/mould/route.rs` and the
star map half of `src/mould/run.rs` (`Run::curve`, `Run::choose`); the
brief is [`../review-2026-11-25/route-choice-brief.md`](../review-2026-11-25/route-choice-brief.md).

## The question and its measure (named before the run)

Does the run bot find a step's Hard planet costlier than its Fair one, and
the Fair one costlier than the Safe one? Measure: on paired seeds (the same
run seed and step, the route before it Fair, the bot's shop before it the
same), soldiers lost (`fell_end`) and time to win (`secs`, a loss counted as
costlier than any win). The brief's bar: each pair ordered on at least 5 of
6 seeds, else widen `Pip::offset`.

## Method

`check.sh`: the run bot's plan (`scripts/run-bot-plan.txt`) with
`league=STEP:SEED:0:PIP`, steps 3, 6 and 9 of twelve, run seeds 1 to 8,
on Mesa's software Vulkan in a cloud session (games, not timing). A step
with two planets has no third pip; those games are skipped. Raw lines are
in `raw/`.

## Result

Pending: the games are running.
