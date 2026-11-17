# The cut count: is there slime worth cutting off?

2026-11-17. Step D of the [gun-system review](../gun-system-review-2026-11-16/README.md),
the cut family, which the plan put last (*Cut goes last*): "Before any Cut
card is built, count how often a run planet has a part worth cutting (the
mass one thin cut would leave without food)." The owner, 11-16 03:44Z:
"there are always crystals all over and that is where the slime so
concentrated, so I don't often find a large group of slime that can be
separated ... I haven't given up on this feature yet. My only point was
that this may not be useful." The game designer's rule (11-17 12:23Z): "If
play rarely leaves cuttable necks, because slime sits on crystals, say so
and skip the card. Withering plus panic already make the cut through walls
and positioning, so a cut bullet only earns a slot if the shapes are
there."

Page: [Slime Cut Count](https://claude.ai/artifact/7h8b6QmZCrwtEJvaNJUzfu) (source in `page/`): each planet's cut map
beside the game's frame at its worst count, the Sleeper before and after
the fix below, and every count.

Status: **no Cut card.** One thin cut rarely pays: the biggest part a
single cut could leave without food is under 2% of the slime in 21 of 22
counts (necks 4 dish cells wide; the median share behind such necks is
1.1%). The one exception is the Swarmer's first minute (planet 4 at 60 s):
arms round the rock hold 19% of the slime behind necks, the biggest part
9%, and by 120 s it is 1.5%. A wall or a squad in a gap already makes that
cut, and withering and panic make it show. The count tool stays in the
tree for when the slime's shape changes.

## The count

- **What a cut part is** (`mould::necks`, new; a headless key only, so
  nothing runs in play and there is no frame cost). On the dish's walker
  grid (a cell a dish cell), the slime is every cell with a walker; its
  food is every cell within a crystal's radius plus 1.5 dish cells of a
  crystal that still holds charge. The slime joined to food at all is
  eroded by `r` (a cell stays if every cell within `r` is slime), so every
  neck `2r` cells wide or less vanishes and the bodies either side fall
  apart; each body takes back the slime nearest it, and a body whose share
  touches no food is a part a cut there would starve. Counted at `r` 1
  (necks 2 dish cells wide) and 2 (necks 4). A part tied to food by two thin
  necks counts too, though it takes two cuts, so this is an upper bound on
  one cut. Slime already without food is counted apart ("no food").
- **Pinholes filled first** (added after the first census; below). An empty
  cell on open ground (agar under at least half its fine cells) with 5 or
  more slime cells of its 8 counts as slime for the shape, twice over; it
  adds no walkers, and rock is never filled, so a gap in the rock stays a
  gap.
- **Tests**: `a_blob_behind_a_thin_neck_is_cuttable_and_a_thick_one_is_not`
  (two 10 x 10 blobs, one on food, joined by a neck 6 long: 2 wide parts
  the far blob off at `r` 1, 4 wide does not until `r` 2; a lone blob is "no
  food"; the map marks the right cells) and
  `pinholes_in_a_sheet_are_not_necks_but_a_gap_in_rock_is` (a sheet with a
  seam of pinholes is not cut once they are filled, and is cut with them
  left open; behind a rock wall with one gap 2 wide the far side is cut).
- **The key**: `larder-world look=DIR run=SEED:PLANET necks=T,...` prints
  the count at each T (filled, and with pinholes left open to compare) and
  writes `...-TTTs-necks.png`, the cut map, 3 pixels a dish cell. The count
  runs after the step, so a T that is also the run's last frame counts.

## The first census, and what was wrong with it

`raw/necks.sh`: planets 1-8 of the run (`run=1:P`, run seed 1), idle (the
slime as it grows with nobody shooting) and squadwork (squads take crystals
and fight), counted at 60, 120 and 180 s; 38 counts, since many squadwork
games ended early with the squad wiped. The 300 s count it asked for never
printed (the count ran before the step, and the run stops at its last
frame; fixed). **It took about 90 minutes on four cores, against my 40
minute estimate**: planets 5-8 hold 80-190k walkers and run slowly on
llvmpipe. Table: `raw/necks.txt`.

It said the Sleeper's planet (6) hung 30-56% of its slime on necks 4 wide
at 2-3 minutes, in 10-11 parts of 500+ walkers. Its map showed why: the
Sleeper's sheet is speckled with empty cells, and eroding by 2 through the
speckle broke a sheet joined to its crystals along a long front into
"parts". With pinholes filled the same moment is 0.9%. The two hands gave
nearly the same counts wherever both still played (the slime's shape, not
the squad, sets it), so the recount is idle only.

## The recount

`raw/pictures.sh` (idle, planets 1-8, 60, 120 and 180 s, run seed 1, with a
map at each count; about 30 minutes for planets 1-7 on four cores; my
session's 30 minute limit stopped planet 8 after its 60 s count, and its
120 and 180 s counts are a second run of the same command for those times,
about 23 minutes alone). The runs are deterministic: every count with pinholes left open
matches the first census exactly. Table: `raw/recount.txt`; the lines
behind both tables: `raw/necks-lines.txt`, `raw/recount-lines.txt`.

Shares are of all the slime's walkers at that moment.

| | Necks 2 cells | Necks 4 cells |
|---|---|---|
| Median share behind such necks | 1.6% | 1.1% |
| Counts with a part of 500+ walkers | 5 of 22 | 6 of 22 |
| Counts whose largest part is 2%+ of the slime | 1 of 22 | 1 of 22 |
| Largest part seen | 5,031 (8.9%, planet 4 at 60 s) | 5,051 (9.0%, the same) |

Each planet at its worst count (necks 4; first census, pinholes open, in
brackets):

| Planet | When | Walkers | Necks 4 share | Parts of 500+ | Largest part |
|---|---|---|---|---|---|
| 1 Drifter | 120 s | 17,720 | 0.6% (0.0%) | 0 | 110 |
| 2 Sprawler | 60 s | 24,653 | 1.4% (1.9%) | 0 | 229 |
| 3 Trunk | 120 s | 27,257 | 4.7% (1.5%) | 0 | 318 (1.2%) |
| 4 Swarmer | 60 s | 56,274 | 19.0% (18.7%) | 3 | 5,051 (9.0%) |
| 5 Sporer | 60 s | 82,804 | 5.8% (13.0%) | 3 | 1,367 (1.7%) |
| 6 Sleeper | 180 s | 94,836 | 1.1% (55.7%) | 0 | 162 |
| 7 Grudge | 120 s | 106,127 | 1.1% (1.1%) | 0 | 347 |
| 8 late mix | 120 s | 164,845 | 4.8% (3.9%) | 6 | 829 (0.5%) |

The Swarmer's 60 s is the one count where one cut could starve a big part:
its arms reach round the rock to far crystals, and by 120 s they have
merged into the body (1.5%). Planets 5 and 8 hold several parts of 500+
but each is under 2% of the slime: many small cuts, not one big one. On
planet 3 filling raised the share (1.5% to 4.7% at 120 s), most likely
because a speckled sheet erodes into nothing (as in the test's seam) where a
filled one leaves bodies to part, so filling is not only a cut in the
count. Planet 8 is one of the last three tiers, picked by the seed
(`run::LADDER`'s mix past planet 7).

## The call

- **No Cut card**, by the designer's rule. A cut bullet pays by one
  severed neck starving one part; the largest such part is under 2% of the
  slime in 21 of 22 counts, so most cuts would starve a few hundred
  walkers while the rest of the body sits on its crystals, as the owner
  found in play.
- **Where cutting does pay, walls already do it.** The one big case is the
  Swarmer's first minute, whose arms reach round the rock for far crystals;
  a wall in the gap, or a squad standing in it, cuts as well as a bullet
  and holds, and withering (a part with no news of food for 5 s thins) and
  panic (it then charges or runs for food) make that cut show. A thin cut
  made by a bullet refills in about 2 s unless held
  ([one body](../one-body-2026-11-12.md)), the other reason a bullet is the
  wrong tool.
- **If the slime's shape changes** (a new species that spreads in long
  veins, a rule that thins the body between crystals), re-run
  `raw/pictures.sh` with that change before reopening the card: the count
  would say yes if the largest part were 5% or more of the slime in most
  counts.
- **Thinnest-vein aim** (plan step C, which waited on this count) waits on
  the same condition.

## For the next playtest pack

The designer, 11-17 12:23Z: "Put the Heave/Corral merge question in the
next playtest pack as an owner pick rather than a separate card." Heave and
Corral are one shove in two cards ([gun cards](../../wiki/world/gun-cards.md),
*Merge or cut candidates*). The integration thread added the pick to the
pack (#433's pick page, with a line linking the
[shove page](https://claude.ai/artifact/2xhG8dfegCSryLGDPexpdE)).

## Rerun

```
cargo build --release --bin larder-world
bash Reports/gun-cut-count-2026-11-17/raw/pictures.sh OUT      # about 30-45 min, 4 cores
python3 Reports/gun-cut-count-2026-11-17/raw/necks_table.py OUT
python3 Reports/gun-cut-count-2026-11-17/raw/make_page.py OUT OPEN_MAP PAGE_DIR
```

`raw/necks.sh` is the first census (both hands, to 300 s); with today's
build it prints the filled count and the open one side by side.
