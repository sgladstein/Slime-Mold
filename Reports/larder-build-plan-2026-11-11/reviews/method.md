# Test-method review (independent agent, 2026-11-11)

Prompt: see prompts.md (Reviewer 3). Report below, lightly condensed.

1. **The M2 counter-cycle gate cannot pass significantly.** Each plan has one score per seed, so on one seed plans fall in a straight order and A>B, B>C, C>A can't all hold. If the links hold on k1, k2, k3 seeds, (16-k1)+(16-k2)+(16-k3) >= 16, so one link holds on 10 of 16 or fewer (p >= 0.45). A significant cycle on paired seeds is impossible; one counted by majority is noise. The counters on plan line 56 are Versus counters; in solo, Hunter and Farmer never meet. Change: in a one-player game, "plans counter each other" means the best plan depends on a state the player can see. Gate M2 on a crossover: pre-name 2-3 readable conditions (drop placement, mould fat or lean, a second layout); A > B on >= 15/16 under X and B > A on >= 15/16 under Y, both surviving Holm; plus an adaptive reader plan that picks from the visible state and beats every fixed plan.

2. **"No plan best on more than 12 of 16" passes while the game is bad.** Near-identical plans split seeds; ties at the floor (price 120: every plan 40-60). Change: "best on a seed" needs a pre-named margin over the runner-up; the second-best must not be the best one with a different parameter.

3. **Plans are under-specified; tuning budget unsaid.** Which piles Farmer holds, where Baiter's decoy goes, when soldiers are bought, when plans switch. The most-tuned plan tops the ladder; tuning and gating on the same 16 seeds overfits. Change: before M1, commit each plan as a decision rule with named parameters, equal tuning budget, tune on seeds 1-16, gate on fresh seeds 17-32.

4. **The outcome spread has no single number for the sign test.** Change: one ordered scalar per run: squad wins +(1 + soldiers left/40 + (900-T)/900); stalemate at the cap share held - 0.5; squad wiped -(1 + (900-T)/900). Ties dropped. Check saturation first (the old theft measure was capped by `Params::table`): if one band holds >= 14/16 of all runs, change the dish before the census.

5. **The M1 control is too weak.** Random lost to idle 16/16 at every price, so "beats random 16/16" passes for any plan that doesn't waste food. Change: every plan must beat idle and a spend-matched random (same food spent per second), each >= 15/16 after Holm; write idle's expected loss as a number.

6. **The M2 ablation is run but never gated.** 6 hands x 15 pairs x 6 conditions ~ 90 comparisons; the first Holm threshold is 5.6e-4, so only 16/16 or 15/16 survive. Change: name the family before the run, pair only plans that matter; each answer must change which plan wins somewhere, or it is cut.

7. **Census run time unpriced.** 528 s was 288 runs x 7,200 steps on four cores (7.3 s a run). A 15-minute round is 13,500 steps, ~13.7 s a run before soldiers. M2 at 576 runs is ~33 min; a price sweep ~1.6 h; held-out seeds double it. Change: quote soldier step cost from M1; screen on 8 seeds, one condition, 300 s; full census only for the gate, across Haiku runners.

8. **Look-first is missing.** Change: each milestone opens with look=DIR renders on 2 seeds per plan; a front gate before M3: a front visible and travelling >= 10 s before it touches a soldier or the larder, >= 14/16.

9. **The M4 gate measures the script, not the upgrade.** Change: each plan with and without each upgrade on paired seeds; a sidegrade passes if it significantly helps one plan and not another.

10. **The M6 gate is not an equivalence test.** Change: twin test on the GPU at 256 x 160 with a pre-named equivalence bound (median within +-5%, best-plan counts within +-2 seeds); 1024 x 640 is a new census.

11. **Smaller.** Census and window must replay to the same score. Define "share of map held" in cells or piles, and when "food stops arriving late".

**Top three:** (1) crossover gate across visible conditions plus a reader plan. (2) Parameterised plans, equal tuning budget, held-out seeds. (3) Pre-named ordered scalar with a saturation check; beat idle and spend-matched random.
