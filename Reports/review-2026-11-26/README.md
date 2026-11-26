# Review of 11-25 21:30Z to 11-26 14:50Z: did "too easy" get fixed? (2026-11-26)

The owner's ask (11-26 14:52Z): "Review all the recent work". Scope: what
merged since [the 11-25 review](../review-2026-11-25/README.md) (#855 to
#865), judged against his last play (run 1795651376, 11-26 04:52-05:39Z,
build 161d5bd: planets 1 to 4 won in 1:16 to 3:18, planet 5 the only fight,
thousands of energy unspent, no turret all run; "way too easy ...
progression needs to be way stronger for the slime").

## What landed

| PR | What | How it was checked |
|---|---|---|
| #855-#857 | Review steps A (threat audit, Lab brief), C (route choice), B (five plans on P2/4/6) | reports |
| #858 | Hearts are a crystal mini-boss after 2 min of packed slime; a planet is won at its last walker | GPU tests; 1-2 games |
| #860 | Star map: 12 steps, Safe/Fair/Hard forks (curve offset -1/0/+1.5), contracts, heat | 36 bot games, steps 3 and 6 |
| #861 | Lab: the hunt (slime follows the squad's trail), report only | 16 arms x 6 seeds |
| #862 | `blows`: an ordinary walker takes 1 to 3 hits by planet | 1 bot game |
| #863 | Landing card fades on first key or click | window test |
| #864 | Slime levels 0-7 by crystal (+0.5 hits, +6% speed, +0.3 bite each), coloured; raids sooner, bigger, armoured | 2 bot games |
| #865 | Mini-bosses: ward (0-65% of hits glance off), hunters stalk the lone soldier, fall back to feed | 1 forced game |

Everything after #858 is on main and **none of it has been played by the
owner**: his last run was on 161d5bd, before the star map and all three
strength changes.

## The bot check run for this review

Measure named first: does the run bot (`scripts/run-bot-plan.txt`, the
plan CI's nightly plays: rush, no cards, never builds) still win the star
map's Fair planets, paired by seed against the star map's own check of this
morning (`../star-map-2026-11-26/raw/`, build of #860, before #862, #864,
#865)? `check.sh`, lavapipe, 4 at a time, main at 8a863aa. Raw lines in
`raw/`.

| Step (curve place, old planet) | Before: won | Now: won | Now, time on wins | Paired |
|---|---|---|---|---|
| 3 Fair (2.27, ~P2) | 6 of 6 | 4 of 6 | 3:01-3:49 (was 1:42-2:25) | slower on 4 of 4 wins, 2 losses |
| 6 Fair (4.18, ~P4) | 6 of 6 | **0 of 6** | (wiped at 5:24-9:13) | 6 of 6 worse, sign p 0.016 |
| 9 Fair (6.09, ~P6) | not run | 0 of 2 | (wiped at 10:45, 11:40) | |

At step 6 the bot bought 38 to 99 soldiers and was wiped every time, a
heart alive at the end in all six; it ended each game with 186-352 energy
banked and built nothing (pumped 0, turrets 0, walls 0, as before).

Caveats: before is also before #858's win rule (it won at the heart; now it
must wipe the body), so the pairing measures #858 + #862 + #864 + #865
together. The bot is not the owner: it has no cards, no turrets and no
walls. The owner cleared old planet 4 in about 3 minutes with no losses and
a carbine with pierce, seek and split (kills a shot 7 to 35); dividing his
kill rate by the 1.75-3.25 hits of step 6's slime still leaves him a margin
the bot never had. So this says "a cliff for the plan we measure the curve
with", not "a cliff for him". It does mean the nightly Run bot will be red
from tonight and Milestone 2 test 3 (the curve) cannot be measured with
today's bot.

## Findings, big picture first

**1. The "too easy" fix is health and timers, not pressure.** Today's
answer multiplies the slime's hit points (an average crystal's slime takes
about 1.5 hits at step 3, 2.3 at step 6, 3 at step 9 and 4.4 at the end;
the home crystal up to 6.5) and makes the scheduled threats bigger (raids
every 150 s to 70 s at 15% to 30% of the slime with up to 8 hits, mini-boss
ward). That closes the arithmetic gap the play log showed (squad kill rate
up 15x, slime up 5x) and it is the right first lever. But it does not
change the shape the 11-25 review and the Lab found: the slime still does
not come for a squad (the hunt prototype made it come, and 6 rifles killed
73,000 walkers down one corridor untouched). A tougher slime that still
arrives one tongue at a time into the guns is a bigger sponge, and the run
gets longer rather than more tense. Two pieces are real pressure: hunters
that stalk the lone builder or pump guard, and armoured raids. Those are
the ones that make turrets and walls worth building.

**2. Nothing touched the unspent energy or the empty Build tab.** The
owner's other two notes were thousands of energy unspent and no turret all
run. Toughness divides every damage source alike, turrets included, so it
does not make a turret better value than another soldier. The bot's answer
on today's main is the same as before: buy soldiers, bank the rest, build
nothing. Whether the stalking hunters and armoured raids push a human to
guard pumps is the open question, and it is the one his next play answers.

**3. The star map stretched the easy part.** The 12 steps read the old
8-planet curve at 1 + 7(step-1)/11, so a Fair route spends steps 1 to 5 on
old planets 1 to 3.5, the stretch he called too easy, and a Safe pick at
steps 2 and 3 is old planet 1 again (curve 1.0 to 1.27, one hit, no
levels). Today's toughness there is 1.0 to 1.6 hits: the 22-39 s best burst
to clear the starting slime becomes about 35-60 s. Early steps are now
slower, not harder.

**4. The middle may now be a cliff, and it was stacked untested.** Three
strength changes merged in about three hours on 1, 2 and 1 games, each
multiplying the last (hits x level hits x raid armour x ward, plus hearts
now forming on every packed crystal and the win moved to the last walker).
The bot went from 6 of 6 to 0 of 6 at step 6. No one measured them
together until this check.

**5. The Lab's result was not followed up.** The hunt showed the reach
problem is half the story; the other half is arrival through the narrowest
place. Its recommended next step (mass at the end of the trail, then come
in wide) was not started, and no one tried the hunt with today's tougher
slime, the obvious combination: a 2-3 hit tongue may now break through.

**6. Small things that read wrong.**
- `news/things-to-try.txt` line 1 still says "Kill it to win the planet"
  of the heart, the rule #858 removed.
- `news/whats-new.txt`'s 11-26 toughness line says "five on the home
  world"; since #864 orange takes 3 there and the darkest crystal 6.5.
- The nightly Run bot has been red since 11-25 (stall check) and will now
  also lose from step 6.
- The step-6 losses all end with a live heart: hearts form on every packed
  crystal, so a squad that is losing meets one mini-boss after another.
  Whether that reads as a comeback or a pile-on is for play.

## Is the jump now too harsh?

For the bot, yes, from about step 6 (old planet 4). For the owner, probably
not on steps 1 to 5 and unknown after; his margin was large, and the jump
lands where he asked for one. The honest answer is that his next run is
the measure, and the build should go to him as it is (build first, balance
later). The one thing to fix before he plays is the stale text, so the game
does not tell him to win at the heart.

## Next steps, in order

1. **Fix the two stale lines** (Direction): things-to-try's heart line
   and whats-new's "five on the home world". Minutes.
2. **Let him play today's build before any more strength lands.** No new
   toughness, raids or mini-boss changes until his run; then retune from
   his log. What to watch in the log: time to win and losses at steps 3, 6
   and 9, energy banked at each planet's end, and whether he builds a
   turret or wall (`pumps`, `turrets`, `walls` on the `t=` lines).
3. **Make the curve bot able to build** (Engine and tools, with Squad):
   today's curve bot never builds, so it can only say "rush loses". Give
   the run plan a builder arm from step 4 (pumps plus a turret per pump,
   the step-B lure plan's shape) and rerun this check's 14 games, about an
   hour. That gives test 3 a bot that plays the game he is meant to play,
   and turns the nightly green or tells us where the real wall is.
4. **Front-load the early run** (Direction, a tuning number): read the
   12 steps on a curve that climbs faster early (step 3 at about old
   planet 3 rather than 2.3), or make step 2 and 3's Safe pick no easier
   than old planet 1.5. His complaint was planets 1 to 4; the map made
   that stretch longer.
5. **Give energy a sink that answers the new threats** (Squad, Direction):
   the stalking hunter and armoured raids are the right pressure; make
   their counter something you build (a pump guard turret, a wall gate)
   and price it so the bank he had runs dry. Check from his log first
   whether he already builds once hunters stalk.
6. **Lab: the hunt with today's slime**: rerun the hunt rig (lab commit
   `ec603a74` merged onto today's main) with `blows` and levels on, 6
   seeds, under an hour. If a 2-3 hit tongue now costs kiting soldiers,
   the hunt can replace some timed threats with pressure from the slime's
   own fields, the 11-25 review's first finding. If not, build the Lab's
   "mass then come in wide" step.

Rejected for now: rolling back any of today's changes (the owner asked for
them, and the bot is not his measure), and a bigger census (not needed
until the bot can build).
