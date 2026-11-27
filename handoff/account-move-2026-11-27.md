# Account move, 2026-11-27

The owner moved the work to a new claude.ai account again on 2026-11-27
(23:36Z: "Work is moving to a new account so don't start new threads. Make
sure everything is up on the repo."). The new account sees only this
repository. Start from [`CLAUDE.md`](../CLAUDE.md) and [`NOW.md`](../NOW.md)
(Milestone 2, a whole run); each lane's state is in `handoff/<lane>.md`. The
first move, on 11-17, is recorded in
[`Reports/handoff-2026-11-17.md`](../Reports/handoff-2026-11-17.md); its
copy of the old project memory is in `handoff/project-memory-2026-11-17/`.

Main at the move: `5cd97aba` (#876). Everything below is merged unless it
says otherwise.

## What landed 11-19 to 11-27

- **Milestone 1 (three planets) done**, his word 11-24 03:09Z. Milestone 2
  and its six exit tests are on `NOW.md`.
- **Hearts are not the win** (#858, his ruling 11-26 00:08Z): a planet ends
  with the slime wiped out; a heart is a crystal mini-boss.
- **The star map** (his pick 11-25 23:45Z; #856 record, #860 build, #868,
  #869): about twelve steps with Safe/Fair/Hard forks, contracts and heat;
  harder forks pay more (Hard +2.5, #870); picked from side-by-side cards.
- **Slime toughness by planet** (#862, #864 levels by crystal coloured,
  #865 mini-bosses stronger and smarter, #867 home world slime takes three
  hits, more on dark crystals, #873 harder from planet 2: tougher slime,
  bigger raids, sooner mini-bosses). Reviews of "too easy": #854, #866.
- **Playtest 11-27 fixes**: #870 (slime keeps unguarded crystals when small,
  so planet 2 no longer starves; Hard forks +2.5) and #873 (above).
- **Slime gauge HUD** (Look and interface): #872 the run's top bar shows the
  slime as a bar, all pills in one row; #874 alerts at most three, 4 s,
  repeats counted, long texts cut short. **Next, after the owner plays it:**
  the Sleeper's share and the heart shown in the bar.
- **Turrets bought like guns** (Squad and arsenal): #871 turrets fire their
  loadout's gun and cost less than a soldier; #875 fixes from the
  adversarial review; #876 rolled turret designs, none replaced, three
  quick-build slots. **Open:** the run bot still builds plain turrets (teach
  it the rolled designs); the turret-vs-soldier value test (what a turret's
  price buys against a soldier's) has not been run.
- Also: a counter for every slime (#845, #849), change keys (#846), kit
  sounds (#853), play this run again and a frame readout (#852), pause menu
  planet card (#842), wounded pace from the Lab's rescue (#850, #851).

## Loadout page redesign

Done at the move in PR #880 (queued 11-27); its notes are in
[`look-and-interface.md`](look-and-interface.md). If #880 is not on main,
label it `queue` again.

## Ideas to carry

- **Rising soldier cost** if big armies still win: each soldier bought costs
  more than the last (or price by army size). Only if the curve (test 3) or
  his play shows big armies still winning after #873; it is a tuning number,
  so Direction owns it.
- The Lab's planet twists (Wind, Cold snap) on about one planet in three,
  the rescue planet, Night (on its branch) — see `handoff/lab.md`.

## Branches with work not on main (11-27)

Changed files that still differ from main, branches touched since 11-19
(older ones: [`branches.md`](branches.md)). Raw output branches are records,
not code.

- `claude/project-thread-2o4cic` (32 files; 2026-11-25: LAB: rescue planet: timed alarm (ALARM > 1 seconds), LAB_RESCUE_HOME pick-up point; batch 5 lines,)
- `claude/project-thread-4tc75h` (7 files; 2026-11-19: Lab: Trunk veins from the simulation (docs))
- `claude/project-thread-5qskxs` (37 files; 2026-11-19: Look B baseline clips: ten 4 s clips, planets 1-3, zoom 6 and 12 (film runner))
- `claude/project-thread-8grg6d` (665 files; 2026-11-27: Strategy league: results from SCOTT-DESKTOP)
- `claude/project-thread-9y1l65` (1 files; 2026-11-19: Handoff: the play log names the loadout (#507))
- `claude/project-thread-at1i5p` (6 files; 2026-11-22: Merge remote-tracking branch 'origin/main' into claude/project-thread-at1i5p)
- `claude/project-thread-belenh` (1 files; 2026-11-20: Merge remote-tracking branch 'origin/main' into claude/project-thread-belenh)
- `claude/project-thread-noq3o4` (6 files; 2026-11-25: Mirror the lump guard on the CPU twin for the one-tick twin test)
- `runner-out-1070-followup` (40 files; 2026-11-20: GTX 1070 follow-up batch output (20261120-221333))
- `runner-out-1070-m1` (15 files; 2026-11-20: GTX 1070 M1 timing batch output (20261120-184852))
