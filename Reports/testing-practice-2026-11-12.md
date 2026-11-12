# How we use balance testing (review, 2026-11-12)

The owner asked (11-12 17:00Z) whether running a balance census for every
feature, holding work in one area for balance in another, and picking a
worse-looking mould to keep the numbers is how games are made; at 17:02Z he
asked for the whole project to be looked at, not only his examples. Review doc:
https://claude.ai/artifact/VUQ1oXjzRVVzdzU7itnZmH. He adopted the rule below
by card at 17:26Z ("Adopt as written").

## What the project did

About 70 of the project's first 102 threads were census runners, against
about three owner playtests; 31 runner threads ran on 11-12 alone. A 16-seed,
five-hand Larder census takes about 45 minutes on four threads
([roadmap, *W2b*](larder-world-roadmap-2026-11-11.md)), and each census reset
re-runs every baseline. Cases where the census decided play or looks:

| When | Tried | Census | Outcome |
| --- | --- | --- | --- |
| 11-12 16:20 | dense mould (25k walkers, smooth tubes) | killed the idle squad in 78 s | held as a census reset; damage to count mould mass |
| 11-12 16:25 | gun rounds | planned 14 loadouts x 6 tiers x 16 seeds (about a day) | cut to about 40 min after the owner asked |
| 11-12 15:31 | rounds on the GPU | reviewers: hold until blast-carry stops winning | port waited on another system's balance |
| 11-12 13:00 | one body at today's size | no level gives one body at today's difficulty | off by default |
| 11-12 06:20 | flee from gunfire (owner's growing-hole bar) | hole 102 v 29 cells 16/16, but random won by holding the core | opt-in, not default |
| 11-12 | four rules sending a harvester home from a dry crystal | scripted farm hand scored lower | all rejected; soldiers still wait at dry crystals |
| 11-11 | slower mould clock | did not fix the hole; also "a census reset" | dropped |

What the census did well: it found dominant strategies (strike over every
plan, food not binding, guns at the crystals, the ring of fire, free money in
Bloomfront) and a rush firing 196 of 200 shots into rock.

What went wrong: it measured the bots, not the game (a scripted hand losing
score by walking off a dry crystal says nothing about a player); it was a
release bar applied to prototypes whose numbers change weekly; and today's
difficulty froze into a constraint nobody chose for fun.

## How studios do it

Balance comes late: a find-the-fun prototype may be "ugly, broken,
unbalanced", and "only now" in a later phase does one agonise over one point
of power ([Justin Gary](https://justingarydesign.substack.com/p/leveraging-the-phases-of-design)).
Bots answer narrow questions on finished content: King runs bots on each
Candy Crush level "thousands of times", and designers judge the result
([mobilegamer.biz](https://mobilegamer.biz/how-king-balances-human-and-ai-powered-design-in-candy-crush-saga/)).

The technical lead review agreed (17:11Z), withdrew its own 32-seed push and
its hold on the toolkit's GPU port, and added the shared-default case below.

## The rule (owner's card, 11-12 17:26Z)

- **Build first, balance later.** A feature or look lands when it works and
  can be seen: unit and twin tests, plus a GIF or a playable build of a few
  seeds. A census is not a gate for a feature, a look, or a controls or
  quality-of-life fix.
- **Balance never blocks looks or features.** When a census result clashes
  with one, it stays and the numbers are retuned; the result becomes a tuning
  note to the thread that owns those numbers. Reviewers raise balance as
  tuning notes and don't hold work in one area for balance in another.
- **A census earns its cost** when it answers a strategy question that changes
  what is built next: player AI, a change to a shared default every thread
  measures against, dominant-strategy checks before a milestone or a playtest,
  mission difficulty, or a question the owner asks. Name the question and the
  estimated runner time first; keep it under about an hour of runner time and
  ask the owner before running more.
- **Smoke checks on every feature** stay cheap: idle doesn't win, the game
  doesn't end in under a minute, nothing soft-locks, the frame cost is quoted.
