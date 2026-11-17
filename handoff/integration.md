# Integration handoff (2026-11-17, account move)

The integration thread kept Larder World's shared rules: the merge order, the GPU
`Params` and binding queue (ROADMAP "Params list"), ROADMAP itself, the PC booking
sheet (`handoff/project-files/`, the Scott-Desktop booking), and tie-breaks between threads.

## State
- **Nothing open.** No unmerged branches or PRs. The branch `claude/project-thread-pu1x88`
  matches what was merged.
- **Merged 11-17:**
  - #419: the all-on test trimmed to 15 s.
  - #421 and #422: six off switches deleted (seek_turn, fed_crowd, body_pull/tether,
    thicken, condense), each with its line in `Reports/dead-ends.md`.
  - #427: fire shyness treated as one decision. fire_weight was then deleted in #430,
    and fire_alarm became the Sprawler's tier trait in #434.
  - #433: the playtest pack. It adds `play-new-slime.cmd`, which is play.cmd with
    `far_smell=1 set=unit_paths=1`, and the ROADMAP note "Playtest pack 11-17".
- **Free Params words:** 892 and 900, and ccrowd bits 24-31. 860 was freed again by #430.
  Don't reuse view bits 16-17 until the graphics code for bronze is removed.

## Pending owner picks
1. Play `play-new-slime.cmd`. If nothing feels wrong, `far_smell` and `unit_paths`
   become defaults in one PR.
2. The blind page "New Slime Picks", with its source in
   `Reports/sim-use-2026-11-17/page-pack/` (the key is in `pack-films.md`, not on the page):
   - setting on or off
   - set slime as a wall or a trap
   - crust pulling the slime back (+8) or pushing it on (-8)
   - fire_fed on or off
   - whether Heave and Corral merge into one shove card
     (`Reports/gun-shove-2026-11-16/`).
3. After the owner plays and picks, every row in ROADMAP's switches-off table gets
   a default or a delete, and each delete gets its dead-ends line.

## In flight elsewhere at the move
- #436 (run-progression): `run::bring`, so the shop offers Tinder before the
  Sprawler's planet. It was waiting for #434 to merge.
- The guns thread was building the Provoke and Still cards.

## Rules worth keeping
- Merge by adding the `queue` label. Don't merge main in by hand.
- Don't assign the owner or request their review on PRs.
- Every new switch gets its row in ROADMAP's switches-off table and goes into
  `all_off_switches_on_together_play_on_both_engines` in the same PR.
