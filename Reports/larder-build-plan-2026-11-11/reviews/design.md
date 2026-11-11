# Game design review (independent agent, 2026-11-11)

Prompt: see prompts.md (Reviewer 2). Verbatim report below.

1. **Baiter and Fortress have no verbs; the owner's "build defenses" brief was dropped.** Orders (plan:39-45) are move, burn line, clear pile, free fire, carry. Nothing places a decoy pile, so Baiter (:56) can't be played; the only bait is a dead soldier. The 00:18Z brief says "needs food to upgrade, build defenses" (wiki/rts/shooter.md:47); the brainstorm's turret, wall, bait and ship tools (mould-shooter:136-141) are all gone. Spending is soldiers plus two gun sidegrades: the "1-2 tools, one strategy" shape the owner rejected. Change: add a Drop bait order (spend bank to make a pile at a point) and one structure (a turret holding fire on the densest trail, or a wall of blocked ground), in M1/M2 not M4, because the counters depend on them.

2. **The M2 counter gate is ill-posed for a solo game.** "A beats B, B beats C, C beats A" (:78): in solo play every plan faces the mould only, so "Fortress beats Hunter by not paying for fire" just means Fortress scores higher, a ladder. The report says counters need the mould's answer to depend on what the squad does (mould-shooter:446-452). Change: a plan x situation matrix (layouts, drop timings, fed or starved state at a set time); pass if each of at least 3 plans is best in some situation, Holm-corrected; plus a scripted "read the mould and switch" plan must beat every fixed plan. That switch is the player's real decision.

3. **The round end is likely unreachable, so the 15-minute cap decides most rounds.** Mould is back within the second after mowing (mould-shooter:440-442); piles feed the mould on them; 23 piles. If most rounds end at the cap, share of map held is the real score, and it rewards Farmer. Change: report in M1 how rounds end per plan; give the mould a killable core, its home pile: burn it bare and the mould stops dividing (as emitters in They Are Billions or Creeper World). Shorten the cap to about 8 minutes unless long rounds stay lively.

4. **Soldiers cannot outrun a front.** Soldiers walk 8 cells/s (:37); a front crosses 256 cells in about 30 s, about 8.5 cells/s, and closes a 30-cell range in about 3.5 s. Pulling back damaged units is core RTS and Pikmin play; engulfing becomes a trap you can't react to. Change: measure front speed in M1; set soldier speed to 1.5-2x that.

5. **At 40 soldiers the squad saturates the dish.** One 30-cell range circle is about 2,800 cells; 40 are 2.7x the 40,960-cell field. 40 x 12 line cells = 480 cells of fire; a larder ring is about 88 cells. Change: keep the owner's 4-to-40, cut range to about 15 cells and line to about 6 cells per soldier, make soldiers dearer; the census should show the best plan stopping short of 40 on some seeds.

6. **The default stance burns the bank, and Clear pile measured at break-even.** Free fire by default costs per shot; firing whenever you can pay spends the bank to nothing (random ends at 20 food at price 40). Mowing at 40 gave 10,392 against idle's 10,600, 7/16 seeds; Clear pile is that mow. Change: default stance "fire only at mould touching a soldier or a held pile"; stances (hold fire, defend, free fire) and attack-move; the M1 gate needs Clear pile to beat idle at the chosen price.

7. **The mould's answers are only partly readable.** Loops versus trees shows in the trail. Larder smell and drop smell are fields the player can't see. "Starving mould spreads faster" is a new rule; its look must come from the dish. Change: a toggle drawing the real scent field (data, so allowed); a blind A/B of fed and starved pictures before tuning.

8. **Losses snowball.** Dead soldiers become food, soldiers smell like food, the squad starts at 4: one early loss feeds the mould, which rushes the other three. Fortress soldiers at the larder draw every front. Change: report P(wipe before 120 s) for idle and random in M1; tune soldier smell and corpse food so an early mistake is recoverable.

9. **The owner sees play too late; the final bar is low.** First window at M3; M5 asks only "two plans felt different". Change: a rough window at M1 (marks, move and burn only); M5 to three plans, each winning at least one of 3+ rounds.

10. **Minor.** `skirmish::HOMES` is the two home points; the field is `skirmish::FIELD` (src/mould/skirmish.rs:99, :219). Fog and scouting are missing (optional). Range circle and soldier marks are player pieces, fine; show health from the actual drain.

**Top three:** (1) Drop bait plus one structure. (2) Replace the cycle gate with a plan x situation matrix plus a read-and-switch test. (3) Reachable, readable round end: killable mould home pile, soldiers faster than fronts, how-rounds-end spread reported in M1.
