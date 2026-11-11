# Larder harness: code review (independent agent, 2026-11-11 ~02:00Z)

Asked for by the owner (01:51Z). The reviewer read `examples/larderprobe.rs`
and the snip code, and ran single seeds. Its prompt named where to look for
the wound refresh, which the author had just found, so it was not blind on
that point. A summary of its report:

1. **The merged harness (2c38252) panics on main.** The field's rival home
   moved from 216 to 215.5 (addebdb), and `larder_index` needs a match
   within 0.5. The first census ran on the tree before that move, where it
   matched. Fixed: `LARDER` reads `skirmish::HOMES[1]`.
2. **Firing a snip again over an open wound did not keep it open**
   (`stroke_dab` wounded only agar). Fixed in `src/mould/mod.rs`. On seed 1,
   before against after: pile 1,135 -> 116 stolen; near 11,610 -> 10,227 (first
   bite 8.3 -> 18.6 s); idle and aim unchanged. The first fire census is
   invalid for the hands that re-fire on the same cells.
3. **The pile hand never wounds the larder itself:** `stroke_dab` skips pile
   cells, so on the pile it only kills walkers and clears trail. Its effect
   is culling, not blocking.
4. **Fort picked its arc from the default shot interval**, so other
   `every=` values could fire one arc only; the doc said four arcs where
   five are used (five is right). Fixed.
5. Smaller: the clear gun's band is 3 cells against the snip's 5; loop counts
   treat wounds as holes (fine while only idle runs are compared); fields
   named for 90 s were taken at 45 s (renamed); `par` panicked on no seeds
   (now an assert); no correction printed; the DEBUG check read the
   environment every step (reordered); the dish has no late food, so it is
   not the skirmish's own dish.

Checked and fine: the rival and the level are really gone; the larder's
index stays fixed; nothing crosses wounded ground (walkers, births, trail and
scent spread and the feeding search all need agar); the sign test and the
direction of "better"; `par` pairs seeds and results; the loop counter's
connectivity; first bite; the 4.4 s first bite is real growth.
