---
name: bomb-rounds
description: Explosions as a gun card (bomb, cluster, blowout) merged as PR #225 on 2026-11-14; engine words it holds; 1070 twin pending
metadata:
  type: project
  modified: 2026-11-14T07:15:49.174Z
---
Owner asked 2026-11-14 01:59Z: "Do we have explosions yet ... as part of the flashes of chaos gun system". Built as the `bomb` payload card (PR #225, merged 07:15Z through the merge queue). A bomb kills a 4-cell core and throws every living walker out to 10 cells straight outward. The thrown walkers are real walkers (flung spores) that land and live on. Loadouts: bomb, cluster (scatter+bomb), blowout (wide+bomb). Report: Reports/explosions-2026-11-14/. Films page: artifact Q177REGwpKfP1vyKeZtmRr.

Engine words #225 holds:
- Spec bit 13 is FLUNG.
- Params word 233 is fling_st.
- The fling row is FLING_AT 896, with fling_dep and 3 spare words (PARAMS_BYTES 912).
- Pass "fling" is 54.
- Gas header word 3 is the flung count.
- bdep block 2 adds 65536 per flung walker (laid_kind splits it).

Pending: the GTX 1070 twin, which the PC batch picks up. Tuning note: a rush with bombs wiped C at 21 s on one seed.

**How to apply:** new Params rows go after FLING_AT, and new passes go after 54. Payload checks should use `Card::kind()`, not lists of named cards. See [[gun-rounds]] and [[squad-toolkit]].
