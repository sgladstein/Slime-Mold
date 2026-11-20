# Starfall and boss cores (2026-11-20)

The owner's pick of 19:06Z ("I like starfall with boss cores. The amount of
star fall should have a minimum but can sometimes happen a lot. Random
selection from distribution as normal"), after two rounds of options in the
thread "Brainstorm how battles earn shop money"
([doc](https://claude.ai/artifact/XHLmAP2RFnSAHba6orJLjW)). Code:
`src/mould/larder_world/starfall.rs`; what it does is in
[the run's wiki page](../../wiki/world/run.md), *Starfall and boss cores*.

Page: [Starfall and Boss Cores](https://claude.ai/artifact/NswkCQSmzpxvpAKWwcqZBF),
source in `page/` (open `index.html`).

## What was filmed

Planet 1, run seed 1, `hand=greedy`, the falls forced early with
`set=star_first=12 set=star_last=26 set=star_gap=10 set=star_tail=0
set=star_near=8 set=star_far=30`; zoom 6 at `centre=236,78,6` (0:14 to
1:30), zoom 12 at `centre=241,97,12` (0:18 to 0:40). Three falls landed at
about 0:22, 0:32 and 0:42, 40 points each. The greedy bot never goes for a
drop, so the slime ate them: at 1:10 they held 8, 16 and 2.

## Numbers changed while filming

The slime's bite into the ore (`StarRules::eat`) went from 0.25 to 1 a
walker a second (times the slime's mass): at 0.25 the slime sat on a lump
for 40 s and took 6 of its 40 points, which is no race. The zoom 12 clip was
filmed at 0.25; it ends before the slime arrives.

## Not done

- No bot goes for drops yet, so league and run bot numbers show what
  drops pay only by chance.
- A core does not re-form its boss; the helper thread "Build boss cores for
  shop money" builds that, with raids' cores.
- The win's base pay is unchanged, so a run now has more shop money;
  Direction retunes.
