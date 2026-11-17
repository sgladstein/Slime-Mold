---
name: condense
description: Slime condensing (owner 11-14 08:07Z) and thickening: DELETED 11-17 (PR #422); specks and lumps keep the dense word; history of the bronze
metadata:
  type: project
---

Condense thread cmsg_01PcwPdssrGxiXYvyCG8WaUX2oAU9RdbXnDCNh179rKtwT built the owner's idea (11-14 08:07Z): slime that packs tight (>=20 of 24 cells round it) or feeds on a crystal for `condense` steps (menu 300 = 10 s) splits: half absorbed, half condensed (two lives, bite/hold x2, upkeep x2, births stop at mass). Off by default; start menu "Condense". Both engines, twins exact. Window Fork seed 1 farm: 26,904 walkers at 5:00 vs 44,806 off.
- GPU: Params 900/904/908; binding 48 after gas (`Engine::dense_at`); passes 55-59. View bronze (view.wgsl draw_specks/draw_absorbed/prep/cell_mat_c): own halves of fgrid and specks, slime-look fields untouched.
- Page https://claude.ai/artifact/7L9QoQxqnk9Cazt1qyFpvR; report Reports/condense-2026-11-14/.
- Owner 08:34Z: art director must review the transformation look ("It has to look cool"). Approved 10:58Z: implosion streaks (ABSORBED slot word 15 steps), warm flash, bronze 92,58,34; PR #266 merged 11-14 12:05Z, texture #269 merged 12:35Z; awaits owner playtest (default on in C? 10 s ripe?). Bronze texture follow-up approved 12:08Z; keep bronze luma <= ~99 (bolt red is 105).
- Open: owner playtest (on in C?), 1070 frame cost.

**Why:** later slime/look threads touch the same GPU words, binding and view lines.
**How to apply:** check this before claiming Params words or changing the slime material; related [[slime-enemy]], [[slime-look]], [[art-direction]].

Breakaway (owner 11-14 15:06Z "separate entity ... exploring further, eating crystals"; 15:21Z "not sure yet" new type vs same slime, "start building and play testing and iterate"): PR #274 merged 11-14 16:04Z, `roam=` / menu Breakaway, off by default, needs Condense. Condensed pieces run to far fresh crystals and land. Pieces stream as specks, not a blob; cohesion is the likely next step. Awaits owner playtest.
Thickening (slime-enemy thread, PR #338, 11-15): reuses DENSE for fed walkers that can't divide (`thicken`, shape C default 2). Threshold in ccrowd bits 24-31 (radius bits 16-23). Breakaway now runs with condense OR thicken (twin "thicken roam"). `ripens()` = condense-only gates ripeness/absorb/slots-full stop.

**Owner 11-16 04:58Z: "Remove the bronze, keep the specks".** Condense and thicken leave play (menu, shape C, run); the code stays as keys (`condense=`, `thicken=`) for films. Breakaway now runs from ordinary walkers too (slime-enemy thread, [[slime-enemy-review]]).

**Deleted 2026-11-17 (integration thread, PR #422, tech lead's ask):** condense and thicken gone from both engines (DENSE/ABSORBED, mass, dense_cells, massclear/masscount; passes after 55 renumbered: DCLEAR 56, DCOUNT 57). Kept: specks (`roam`, ripen, condense_crowd/condense_r in ccrowd 0-23, DRAW_ROAM 405), dense word RIPE/ROAM/crystal/LUMP/HURT bits, dclear/dcount gated on lumps_on. Freed: Params 900, ccrowd 24-31, dense header word 0 + first cell block, word bits 16-17 (view bronze shader still reads them: don't reuse until slime-look removes it). seek_turn, fed_crowd (Params 860, 892 free), body_pull, tether deleted in #421.
