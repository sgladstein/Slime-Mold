# Reports

Everything written for the mould game, newest thinking last within each
group. Lines marked **(from Pixel_Physics)** are the index entries as they
stood there on 2026-11-10, when the game moved; the rest were added with the
move.

## Reports and their folders

- [mould-probe-2026-11-07.md](mould-probe-2026-11-07.md) **(from Pixel_Physics)** — **probe built,
  playtest fixes in (2026-11-08), waiting on the owner's play.** A Jones-model
  slime mould herded with a lamp, oats, salt and snips (`--bin mould`). The
  lamp now pushes a tube aside instead of erasing it (join lost while held on
  it: 5 of 5 seeds before, 0 of 5 after); oats come back, food turns up at
  0:30 and 0:55, and the storm is warned 5 s ahead and aimed at the worst
  cut. Over the same seven scripted hands, doing nothing fell from 2 stars to
  0 and a random hand from 6 to 3, while the plan rose 6 to 8 and a fresh
  player's script 6 to 9. **Owner played it (2026-11-08): "the most promise
  so far", but needs instruction and a challenge.** The game now opens on a
  six-step lesson and a briefing card, labels the storm and its scar, and
  no longer flashes grey (about 420 cells a second to 0). Also records why
  the storm is one cut, not three. **Three challenge dishes and a dish
  picker (2026-11-08):** Behind the Wall, Beat the Ruler, Running Dry, each
  meeting its ship bar over 5 seeds (do nothing 0, random at most 1, the
  scripted intended hand 3); strays limited on hand-made dishes, which
  doing nothing had used to join all three in Beat the Ruler.
- [mould-keep-alive-2026-11-10.md](mould-keep-alive-2026-11-10.md) **(from Pixel_Physics)** — **built, waits on the owner's play: an endless
  dish, and water and sleep so the tools work on each other.** Keep It Alive is Running Dry at 22.5 degrees with no bell;
  piles land further each time and the score is how long it lived. The lamp dries the ground and dry mould sleeps through
  a famine; a drop of water wakes it by the next pile. Over 16 seeds a hand using lamp, oats and water lives 2:51-10:42,
  longer than an oat-only hand and random on 15 of 16 each; doing nothing and random get 0 stars on all 16.
- [mould-wild-one-2026-11-10.md](mould-wild-one-2026-11-10.md) **(from Pixel_Physics)** — **built, waits on the owner's play: a wild second
  mould that steals food and fights at the seam.** A whole second dish under the same rules, nobody steering, in violet;
  both moulds eat from one table per pile and kill each other where they touch. In THE WILD ONE, all four tools end
  bigger than it on 16 of 16 seeds (mostly 2 stars) where doing nothing and random get 0 stars on 16 and 14; lamp and salt
  each win alone, the snip does not. Costs about +1.4 ms a frame.
- [mould-game-2026-11-08/](mould-game-2026-11-08/README.md) **(from Pixel_Physics)** — **the thinking around the mould game, kept so it
  is not lost (owner, 2026-11-09):** the five-agent brainstorm after his first play, the overnight build's briefs and
  results (dishes, Wanderer, Organ, the Night look and its blind key), both adversarial reviews, the HTML of every
  picture page shown to him with links to the pages, and the 2026-11-09 depth reports: Jenson, Cherouvim and Jones 2010
  against our model, and how tools, events, ground and the mould's responses could interact for discovery. Proposals,
  not built.
- [mould-real-world-2026-11-09.md](mould-real-world-2026-11-09.md) **(from Pixel_Physics)** — **design exploration, no build: what the mould game
  looks like out of the dish.** The shipped mould dropped unchanged into a side-on slice of the lab box grows one runner
  along the ground (4 of the 5 surface piles beyond home in 10 s, untouched), because the world's dead matter lies on the
  surface; with five piles buried 12+ rows down it branches into roots (9 of 10), where the same ground without buried
  food stays a runner. From above, a forest floor keeps the dish's network but is not the engine. Records an edge-of-window
  trail bug the review caught (`Dish::spread` skips the border). Recommends a side-on "mould under the garden" probe in its
  own binary, top-down as the fallback; waits on the owner.
- [mould-review-2026-11-10.md](mould-review-2026-11-10.md) **(from Pixel_Physics)** — **independent review, no build: how to make the mould game
  fun and good-looking.** Played from written hands on the mould thread's unlanded branch. At 45 degrees the mould obeys
  every oat and then freezes (far pile joined by 15 s with 3 oats; every move the same move); at 22.5 it joins all the food
  untouched and the lamp's effect is lost in its own rewiring. Running Dry over 8 seeds: doing nothing loses at both
  settings, a random hand lives on 1 of 8 at 45 and 6 of 8 at 22.5, the scripted plan 8 of 8 at both. Recommends, ranked
  with pros and cons: a keep-it-alive dish that does not end, player marks that stay (lights set down, salt rubbed out),
  drawing the real walkers and thinner tubes; plus design writing and a table of comparable games. Waits on the owner's pick.
- [mould-looks-2026-11-10.md](mould-looks-2026-11-10.md) **(from Pixel_Physics)** — **built, drawing only: the mould game's looks pass.** A speck
  per real walker, one colour ramp for the food the tube holds (rescaled: the old reading was full on over half the tube),
  smoothing of the rim, tube insides, salt walls and cuts, a fading stain where mould has been, the lamp drawn as the light
  the simulation shines, and narrow tool tiles so the bar stops overlapping. All on by default, each behind a key (F3-F6).
  Same dish drawn both ways on the page; costs about 1.5-2.5 ms of drawing a frame at scale 4.

## Added with the move (2026-11-10)

- [new-game-brainstorm-2026-11-07/](new-game-brainstorm-2026-11-07/) — **where the game came from.** The flows,
  gas and growth brainstorm that produced all eight prototypes: the brief, five explorers, three reviews, the
  prototype briefs (`prototypes/2-mould.md` and `common.md` are the mould's), and the 2026-11-08 playtest
  (`playtest-2026-11-08/mould-review.md`). Other games' material is kept with it because the reviews compare them.
- [mould-game-2026-11-08/](mould-game-2026-11-08/README.md) now also carries the pictures that were only in the
  project folder: the toy-feel sheets (`brainstorm/toy-feel/`), the second review's stills
  (`reviews/review-2-pictures/`), and every prototype round's GIFs, PNGs, WAVs and video (`prototypes/*/`).
  [`depth-2026-11-09/PAPERS.md`](mould-game-2026-11-08/depth-2026-11-09/PAPERS.md) cites the two Jones papers.
- [mould-review-2026-11-10/](mould-review-2026-11-10/) now also carries `hand-fixed.txt`, the from-the-simulation
  look mock (`mouldraw.rs.txt`), the review hands patch for `mouldprobe` (`mouldrev.patch`: aim error, slow
  reactions, one-move openings, a two-mould invariant check), and the page sources and images of the Looks Review
  and Sim-True Look pages (`pages/`). `mouldhand` itself is now `examples/mouldhand.rs`.
- [mould-scratch-2026-11-10/](mould-scratch-2026-11-10/README.md) **(from Pixel_Physics)** — **raw census behind Keep It Alive, Water and
  Sleep, The Wild One and Finer Tubes**: the four pages' HTML and images, the census runners, `w5.py` (shares and
  paired sign tests), and every raw probe output of the wild-mould tuning arms and the sensor-reach sweep.
- [pages.md](pages.md) — every Artifact page made for the game, and where its source is kept.
- [dead-ends.md](dead-ends.md) — the mould's tried-and-reverted mechanisms.
