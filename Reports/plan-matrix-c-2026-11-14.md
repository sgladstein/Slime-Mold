# Strategy check on shape C before the playtest (2026-11-14)

The cloud plan's C4 (Reports/cloud-plan-2026-11-14.md): a small plan-by-tier matrix on today's default, shape C (25,000 walkers, panic, withering, bullets). It answers one named question: *does any plan dominate before the next playtest, and does anything beat walking in now that it is deadly?* It is judged against the designer's counter table (D3), whose plain-C line is: cut should be best, the strike should lose.

Measures were written before the run ([raw/MEASURES.txt](plan-matrix-c-2026-11-14/raw/MEASURES.txt)). Race world, `larderworld census walkers=25000`, cap 900 s, unused seeds 301-308, main 323832e plus the new cut style. Runner time: 15 minutes on one machine. Raw output is in [raw/](plan-matrix-c-2026-11-14/raw/), and the table is [matrix.md](plan-matrix-c-2026-11-14/matrix.md) (`scripts/plan_matrix.py table`).

**The new plan, cut** (`Style::CUT` in `src/mould/larder_world_ai.rs`). It has the strike's army and timing: 30 soldiers, and from 6:00 all but 8. Each striker attack-moves to the middle of the road between the mould's home and one of the 4 mould-held crystals nearest the larder, spread evenly. With no crystal held, it strikes the home.

| idle | random | boom | claim | fort | strike | cut |
| --- | --- | --- | --- | --- | --- | --- |
| −1.33 0/0/8 | −1.83 0/0/8 | +0.33 0/8/0 | +0.34 0/8/0 | +0.33 0/8/0 | −1.22 0/1/7 | +0.34 0/7/1 |

Median score; the ends are mould wiped / cap / squad wiped.

- **Walking in is now a losing plan.** The strike lost its whole squad on 7 of 8 seeds, between 589 and 891 s.
- **Cut beats the strike** on 7 of 8 seeds (p 0.07). It loses its squad once, against the strike's seven. It is what the designer's table wanted, but it doesn't break the mould.
- **Nothing beats the mould.** No plan wiped it on any seed. Boom, claim, fort and cut all stalemate at the cap at +0.33 to +0.34. The best by median, claim over cut 6-2 (median +0.01), is a tie.
- **No plan dominates.** Four plans tie for best. Every plan beats idle and random on 8 of 8, apart from the strike, which beats idle on only 5. Idle's squad dies on all 8 (it can't feed itself under rations).
- **Holm:** with 8 seeds the smallest p is 0.0078, so after the correction over the 12 tests nothing is significant. Read it as a spread.
- **Designer's gates (D3) for plain C:** idle loses (yes); cut beats random on at least 6 of 8 (8 of 8, yes); cut is best (tied with claim, boom and fort, not alone); the strike loses (yes).

**What it means for the playtest:** the deadly walk-in works as intended. Nothing in the AI's book can kill the default mould in 15 minutes, though. A playtest on C will feel like a stalemate unless the player finds something the AI doesn't do. For the numbers' owners, this is a tuning note: cut pieces wither and panic, but cutting four roads is not enough to break the body.

**Not run:** the other seven tier columns. At this run's 15 minutes a column, they would take about 2 hours on one machine (more on the Sporer and Sleeper, whose life cycles cost more a tick), over the 1 h rule without the owner's OK. Bait-then-strike (Sleeper) is also untested, as the designer allowed.
