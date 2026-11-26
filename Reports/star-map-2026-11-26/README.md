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
`league=STEP:SEED:0:PIP`, steps 3 and 6 of twelve, run seeds 1 to 6 (the
script's default of steps 3, 6 and 9 and seeds 1 to 8 ran at about four
games an hour per core, too slow for one session),
on Mesa's software Vulkan in a cloud session (games, not timing). A step
with two planets has no third pip; those games are skipped (an empty file
in `raw/`). Raw lines are
in `raw/`.

## Result

**First pass (`raw-before/`, 24 games, the species at the pip's place):**
step 3's Safe planet met the Drifter and its Fair one the Sprawler, whose
`tier_size` is 0.45, so Safe had more slime than Fair (walkers 0.53-0.78
against 0.37-0.55) and took longer to win on 5 of 6 seeds. Fix, now in the
build: every planet of a fork meets the step's own species (`Run::level`),
so the pip changes only how much slime, its threats, the map and the twist.

**Second pass (`raw/`, 36 games, `python3 order.py`).** Paired counts are
"harder pip costlier / cheaper / tied"; one-sided sign test, ties dropped.

| Step | Pair | Soldiers lost | Time to win (a loss worst) |
|---|---|---|---|
| 3 | Safe < Fair | 3 / 1 / 1, p 0.31 | 0 / 5 / 0 |
| 3 | Fair < Hard | 4 / 0 / 0, p 0.06 | 4 / 0 / 0, p 0.06 |
| 6 | Safe < Fair | 5 / 0 / 0, p 0.03 | 5 / 0 / 0, p 0.03 |
| 6 | Fair < Hard | 2 / 3 / 0, p 0.81 | 2 / 3 / 0 |

Slime size (walkers) is cleanly ordered on every seed: step 3 about 0.3,
0.45 and 0.95; step 6 about 1.45, 2.3 and 5.5. Games lost: step 3 Hard
3 of 4, every other cell 0 or 1 (step 6 Hard 1 of 5). Totals of soldiers
lost on the seeds with all three pips: step 3 Safe 3, Fair 17, Hard 118;
step 6 Safe 3, Fair 95, Hard 161.

None survives a multiple-comparison correction over the eight tests
(Bonferroni needs p < 0.006), and only step 6 Safe < Fair meets the bar
named above (5 of 6 seeds; 5 of 5 here). Read as: the bot sees **three
clearly different planets** where it is not at a floor or a ceiling. At
step 3 Safe and Fair both cost it almost nothing (the bot wins with 0-4
soldiers lost either way); at step 6 Fair and Hard both bleed it (23-57
lost on four Fair seeds). Safe at step 3 takes longer to win than Fair on
every seed although its slime is smaller: its map is the first planet's
(`map_depth` 1), and the bot clears more slime on it (11,572 killed
against 7,128 on seed 3). Not followed further.

**Decision:** offsets stay Safe -1, Fair 0, Hard +1.5. The measure
saturates at both ends, so widening would not show here what it does in
play, and the next pass belongs to the owner's play (build first, balance
later). The first retune if Hard reads too close to Fair later in a run:
Hard +2 (`Pip::offset`).

## Pictures

[`index.html`](index.html) (page https://claude.ai/artifact/Hk2ou8TJHFpbCgJxL8ZBjh): the
map at step 3 and at the end of a run, and step 3 of run 3 landed Safe,
Fair and Hard side by side from 10 s to 2:00 (`run=3:3s`, `3f`, `3h`,
`hand=greedy`).
