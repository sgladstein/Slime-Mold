# Larder tests: write-up review (independent agent, 2026-11-11 ~01:53Z)

Asked for by the owner ("Have 1-3 independent agents review your work",
01:51Z). The reviewer read the *Results of the three tests* section, the four
raw files, `examples/larderprobe.rs` and `CLAUDE.md`, and was not given the
author's conclusions. Its report, verbatim:

---

**What holds up.** Every median, count and p-value quoted in the section matches the raw output: 12,273 / 12,213 / 12,175 / 12,264 / 1,220; 4.6 to 4.1 s; loops 1 against 10; walkers 341 against 6,900; the 1% saving at 8/12. The Holm conclusions are also right: everything at 16/16, 15/16 or 12/12 survives, and near against idle (p 0.021) does not. The pile result (16/16, about ten times less stolen) is solid, and so is lean against fat on loops.

**Issues, most important first**

1. **The deciding measure is saturated, and the write-up doesn't say so.** Stolen is about 12,200-12,300 in every condition except pile. That includes the lean dish, where 341 walkers steal 12,296 against 12,273 for 6,900 walkers. Theft seems capped by how fast the pile yields food, not by how many walkers eat it, so only "zero eaters" can move it. That is a better explanation for the ring failing than "the mould on a pile is fed by that pile", which the section states as a mechanism but never measured. It also undercuts "lean against fat is real" as support for the Farmer-then-Hunter design: lean mould steals just as much.

2. **The ring was tested after the mould arrived, not as a defence.** Fire starts when mould comes within 50 cells. Near, rand and pile fire about 345 of a possible 360 shots, so firing starts around 3.6 s, half a second before the 4.2 s first bite. "Cutting a supply line cannot protect a pile" only holds for cuts made once the mould is already arriving. The harness has a `fort` hand (ring held from 0 s), but there is no output for it and the write-up doesn't mention it. That is the missing control. Rewrite the claim as "fails when the ring goes up once the mould is near".

3. **Shot budgets differ.** Aim fires about 240-280 shots against about 345 for near, rand and pile, because it holds fire when no tube crosses the ring. Aim against near is 8/16 (p 1.0), so aiming adds nothing over firing blind on the ring. That line isn't reported.

4. **The comparison count is wrong.** The files print 24 comparisons (9 + 9 + 4 + 2), not 26. The conclusions don't change.

5. **A significant result is left out.** On mould eaten across the whole dish, aim beat rand on 0 of 16 (p 3e-5, survives Holm). Random fire anywhere cuts the dish-wide eating (272,197 against about 274,400), lowers walkers and raises loops (12-13 against 10). A plan-loses-to-random result on a secondary measure belongs in the write-up.

6. **The smell test has no plain-pile baseline.** Gain 0 (median first bite 4.2 s) was run but never compared. Gain 0.3 is slower than no smell at all (4.6 s), so "smell 3 sooner on 16/16" is measured against an odd reference: against plain the gain is 4.2 to 4.1 s. "The dish is too fast" is a plausible reading: first bite sits near the crossing time, so there is a floor effect. But it is a hypothesis, not a finding, until the slowed-walker re-run happens.

7. **Lean dish exhausted.** Lean ate exactly 86,250 in every run, so every lean pile was emptied. Walkers at the end (341) measure starvation once the food is gone, not a lean economy. At 45 s it is about 900 against 6,900. The "grey unfed branches" claim rests on pictures, not on numbers in the raw files.

8. **The pile hand was added after looking.** The write-up discloses this, but it was added after seed 1 only, with no held-out check. Pile with the clear gun matches idle (12,270), so the effect comes from killing walkers or wounding the ground, not from clearing trail. The write-up doesn't separate kill from wound. If wounded ground on the pile blocks eating, the squad is closing its own larder, and the game cost of that is unknown.

9. **Reproducibility.** `LARDER` now reads `HOMES[1]` = (215.5, 80); the runs used (216, 80). Re-running the current harness may not reproduce the files exactly. The 11 s-per-run cost can't be checked from the output, and shots were never timed separately, which the method rule on frame cost asks for.

**What a designer needs that isn't in the write-up:** theft rate is roughly fixed per pile whatever the mould's size; random fire hurts the mould's whole economy; the only verb that worked denies the pile to everyone; and nothing at all was tested before the mould arrived.
