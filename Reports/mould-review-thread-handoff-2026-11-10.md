# The review thread's hand-off (2026-11-10)

**Status: hand-off note. The mould work is moving to a new account and
project (owner, 2026-11-10 16:01Z); this is what the independent review
thread knew that was not yet written in this repo.** Nothing here is built.

The review thread never changed the game. It reviewed it three times and
answered the owner's design questions; the mould thread and the looks thread
did the building. A new session picking this up has none of the old project's
chat or memory, so the owner's words are quoted here with their times.

## Where the review work is

| What | Where |
|---|---|
| Review 1: fun and looks, design writing, comparable games | [mould-review-2026-11-10.md](mould-review-2026-11-10.md) |
| Review of the looks pass (page only; findings below) | [`mould-review-2026-11-10/pages/looks-review/`](mould-review-2026-11-10/pages/looks-review/looks-review.html) |
| The from-the-simulation look, mocked for the owner | [`mould-review-2026-11-10/pages/sim-true-look/`](mould-review-2026-11-10/pages/sim-true-look/sim-true-look.html) |
| Review 2: Keep It Alive, water and sleep, The Wild One | [mould-review-2-2026-11-10.md](mould-review-2-2026-11-10.md) |
| Review 2's raw results, commands, traces, probe patch | [`mould-review-2-2026-11-10/`](mould-review-2-2026-11-10/) |
| Harnesses for review 1 (play a dish from a written hand; draw the raw fields) | `mould-review-2026-11-10/mouldhand.rs.txt`, `mouldraw.rs.txt` |

The Artifact links in [pages.md](pages.md) belong to the old account. The
HTML and images beside each are the copies to rely on.

## The one decision that is open

Asked on a card 2026-11-10 15:12Z, **not answered** when the work moved:
*"Pick what the mould thread fixes first after this review."*

| Option | What it means |
|---|---|
| Wild One: score food eaten | The Wild One is scored by who ate more of the food, so the crash in the last ten seconds stops deciding the stars. |
| Keep It Alive: find the luck | Trace why some games die near 4:30 and others pass 8:00, then turn that into a choice the player makes. |
| **Both, in that order (recommended)** | The scoring change first because it is small, then the trace. |
| Wait until I have played | Nothing changes until the owner has tried both dishes. |

Ask it again before starting either; do not take the recommendation as his
answer.

## Next steps, in the order the review recommends

From review 2 (numbers and reasons are in its report):

1. **The Wild One: score the share of food eaten**, or ring the bell at about
   2:15. Today the bell rings while four of the seven piles are running dry
   and the player's mould loses 28-54% in the last ten seconds.
2. **Keep It Alive: trace individual games, not the pooled table**, to find
   what puts a game in the short hump (dead by 5:05) or the long one (past
   7:20). The same seed lands in either depending on the hand, and a clumsier
   hand is not worse. The untested guess: whether sleeping mould lies within
   reach of where a far pile lands at about the twelfth pile, when the gap
   passes 140 cells. Then give that choice to the player (show the next
   pile's landing spot earlier; let sleepers last longer than 20 s).
3. **The Wild One: make seconds 30 to 150 matter at a person's aim.** One
   star is the opening salt wall alone; the second needs the lamp, snips and
   oats within about 2 cells. `HJIT` moved all three together, so which of
   them needs the precision is not known. Separate them first.
4. **The snip on The Wild One: make a cut-off piece of wild mould visibly
   starve, or take the snip off that dish.** Three cuts through the nest
   changed nothing on 8 of 8 seeds.
5. **Star bars last**, once the above have changed what a game is.
6. Four small things in the two-mould step, none a bug today: salt kills of
   wild walkers are counted as `Kill::Snip`; both moulds can take a pile's
   last bite in one step; `wild_left` is one step stale; the window keeps
   stepping after the bell so the bar's live "You / wild" drifts from the
   card.

Still unbuilt from review 1 ([its ranked list](mould-review-2026-11-10.md)):
marks that stay (lights set down and lifted, salt that can be rubbed out),
oats as a nudge instead of a ruler, a scored amount with the do-nothing
result beside it, and the three 45-degree dishes kept as a side pack.

Agreed by the owner as the order for depth (below), not yet reached:
**melting salt, spores, a rival the player steers, same-kind moulds
joining.** Only "if those play well", meaning after water and sleep and the
wild mould are judged fun in his hand.

## What the owner said in this thread

All 2026-11-10, UTC. "Card" means he tapped an option.

- 01:41, 01:42: asked for an independent review of the mould game for fun
  and looks, and of game design, philosophy and similar existing games.
- 02:57, card: **the keep-it-alive endless dish first.**
- 02:58: "Looks too."
- 02:58: "We also need to add complexity to the game. The tools are quite one
  dimensionally right now"
- 03:00: "Also wondering about having multiple molds that can fight or steal
  resources from each other"
- 03:06: "Agreed", to this order: (1) the endless dish; (2) **not more tools,
  but tools that work on each other through damp ground**: a water tool, the
  lamp dries, dry mould sleeps, water wakes it; (3) **a wild second mould**,
  another colour, same rules, nobody steering, that steals from the same
  piles and then fights at the seam. Later, if those play well: the list
  above. The bars agreed with it: a good game uses three or more tools and a
  random hand still loses to a plan; the player's hand must change which
  mould ends bigger, paired over seeds.
- 03:20 (in the looks thread): "Not sure look alive but look good", and to
  improve where the graphics look pixelated: "I know this is a pixel
  simulation but we can still make things look better on top of that".
- 04:08 (in the looks thread): "After you finish all your work, have the
  fable thread review it". That produced the looks-pass review and review 2.
- 13:51: "I don't like the sharp edge between the orange and black
  background. That also doesn't look real simulation"
- 13:52: "And the sharp edge between the orange and gray slime"
- 13:53: "I want all the visuals to look like they're coming from a real
  simulation and not just graphics animations. What makes a visual look that
  way?"
- 13:58, card: **build the from-the-simulation look.** This is the ruling in
  `CLAUDE.md` ("no thresholded tube edge or fed/unfed colour line; the
  picture *is* the data"). The looks thread built it; F7 flips back.

What was told to him about why a picture reads as a real simulation, since
the answer is not otherwise in the repo: the old drawing picked one trail
height, painted everything above it flat orange and everything below black,
and turned "how long since fed" into fed or not fed. Both are cuts the
simulation does not hold. A picture reads as data when brightness follows
the quantity through one fixed curve, when a count shows as density (a dot
per walker), and when nothing is drawn that the simulation is not holding.
The mock: brightness = 1 − exp(−0.3 × trail), hunger = (seconds since fed −
0.5) / 6 sliding gold to grey, one additive dot per walker.

## The looks-pass review, in words

The page has the pictures; these were its findings (looks pass at
Pixel_Physics PR #1057). The looks thread reported all of them fixed in
PR #1058. **That fix was never re-checked by the review thread.**

- Smoothing was a clear gain.
- The dish rim showed as a staircase under the tube.
- Starving mould stayed orange for up to 6 s before greying.
- At 22.5 degrees the stain changed about 30% of the dark agar.
- With flow colour on, the tube went muddy brown.
- The lamp read as a grey puddle.
- Walker specks read as pepper to the reviewer; left to the owner's eye. His
  pick on specks on or off was still open.

## Re-running review 2's probe

`mould-review-2-2026-11-10/mouldrev.patch` is a diff against
`examples/mouldprobe.rs` at Slime-Mold main `e242d2d`, with the `INVARSHOW`
line the older patch lacked. Checked on that commit: it applies, builds, and
the control prints live counts.

```
cp examples/mouldprobe.rs examples/mouldrev.rs
patch examples/mouldrev.rs < Reports/mould-review-2-2026-11-10/mouldrev.patch
cargo build --release --example mouldrev
HJIT=2 ./target/release/examples/mouldrev level=5 players=warden seeds=1
INVAR=1 INVARSHOW=1 ./target/release/examples/mouldrev level=5 players=warden seeds=3
```

Do not commit `examples/mouldrev.rs`; it is a scratch copy of the probe.
`commands.txt` lists the 139 job lines as they were run (there the binary is
`./mouldrev`, and results went to `abl/`); the 16-seed baselines and the
traces used the unpatched probe, the traces with `LEVELTRACE=1`.

The review measured at Pixel_Physics `b3bd36ac1`. Two baselines re-run on
Slime-Mold `e242d2d` give the same line to the walker: warden seed 1 (783 /
204, 2 stars) and sleeper seed 3 (lived 4:11). So the tables still describe
this repo's game as of that commit. Re-run the baselines after any change to
the mould itself before leaning on them.

## What nobody has checked

- **Either dish in a hand.** Every result is a scripted hand. Mouse feel and
  sound are unjudged. The owner had not reported playing Keep It Alive or
  The Wild One when the work moved.
- Frame cost in review 2 (the machine was busy with the batches).
- Finer tubes (sensor reach 6 to 8) and the from-the-simulation look as
  built, against either dish's numbers.
- Whether review 2's directions hold on more seeds. About 14 comparisons on 8
  seeds; nothing passes the strict bar of 0.004, and an 8-0 is p 0.008.

## Mistakes the review made, so they are not repeated

- Review 1 said the tool bar overlapped the dish. That was the probe drawing
  at scale 3. **The window always draws at scale 4** (`SCALE` in
  `src/bin/mould.rs`); render at 4 before judging the bar.
- The looks review asked for hunger to show at once. That was about timing.
  The fix made the orange-to-grey line sharper in space, which the owner then
  disliked. Say which of *when* and *where* a complaint is about.
- The review harness first drew only at the moments it took a still, so the
  stain was under-sampled. Draw every frame, as the window does, when a look
  builds up over time.
