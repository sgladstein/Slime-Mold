# Which bullet a gun card changes (options page, 2026-11-24)

The owner asked (11-24 14:59Z) for a card look that shows which cards a
modifier works on, floated putting modifiers *on* a bullet instead of in
their own slots, asked whether the row wraps like Noita, and asked for
several ideas, not only his.

**Wrap:** no. In `rounds::casts` a modifier, Delay or Thick changes the
next payload to its right; leftovers go on a trailing plain cast (the
gun's own bullet, balance review 11-20); a leftover Trigger, Expire or
Multicast carries nothing (and since #806 the shop holds carriers back
until two bullets are owned); flight ("Always") cards ride on every bullet.

**Options** (mock-ups in the game's colours, nothing built): A shot brackets
under the row; B fused shots, with the plain bullet as a ghost card; C
modifiers socket onto a bullet and Always cards onto a gun strip (a rules
change: slots become bullets); D a hover trace with arcs and a line of
words (live on the page); E a "fires" line like Noita's spell summary;
F shot colours. Squad and arsenal's pick: B with D. Wrapping is a separate
yes or no. Direction put B+D, C and E on a card for the owner.

Files: `index.html` (the page), `opt-A.png` to `opt-F.png` (each option),
`today.png` (the run shop on main, `run=1 menu=shop-mid at=1`).
Page: https://claude.ai/artifact/H9TpzW4Kes2b5V4pfodLrT
