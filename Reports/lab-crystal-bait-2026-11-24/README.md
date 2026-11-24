# Lab: a charged crystal as bait for a mini-boss (2026-11-24)

The Lab lane's thirteenth job, on the lab branch `claude/project-thread-2o4cic`
only (feeding `14ba8f8f`, the `drain@` hand `8573ff93`), not in the game.
Brief from Direction 2026-11-24 10:34Z, after [the march counter](../lab-march-counter-2026-11-24/README.md)
found no march wall: test feeding as a mini-boss counter, framed as a
player choice (a crystal left charged is bait that starves a passing boss,
against draining it for income), on planets 4 and 7 (7 is the bot's wall
now), with the same boss in both arms.

Page: "Crystal Bait" (source in `page/`, listed in `Reports/pages.md`).

## The rule and the choice

Feeding (`LAB_FEED=1.0`): a lump member (a march's or a mini-boss's)
under its birth energy that stands on a charged crystal leaves its lump and
eats, losing the lump's orders and armour. A wanderer goes crystal by
crystal toward the player's things, and only to a fresh crystal (charge
at least half its start, `BossRules::fresh`), so a charged crystal on its
way is where it stops, and with feeding where it thins out. A drained
crystal is not on its way at all.

Three arms a map, the squad of 8 (bank 3,000) on the crystal by its
landing, every mini-boss forced to a wanderer (`set=boss_kind=2`) so both
arms get the same kind, the game's raid clock, 3:00, lavapipe
(`raw/runb.sh`, `raw/queue.txt`):

- **off**: hold on the crystal, no feeding.
- **bait**: hold on the crystal, left charged; feeding on.
- **drain**: drain that crystal and buy soldiers with it (`drain@X,Y`);
  feeding on, so other crystals still bait.

## Results

| Planet, seed | Off | Bait | Drain |
|---|---|---|---|
| 4, 1 | 8 alive (the wanderer stayed far off) | 8 alive | 19 alive |
| 4, 2 | **wiped 2:35**, 7 to the wanderer | **8 alive**; broken 2:36 (473 → 212 at the crystal at 437,283) | 16 alive; broken 2:32 |
| 4, 3 | 8 alive; the wanderer roots, swells to 5,312, 130 away at 3:00 | 8 alive; **starved to broken at 2:01**, 207 from the squad (400 → 100 at 303,350) | 19 alive; broken 2:58 |
| 7, 1 | 8 alive | 8 alive | 13 alive (its wanderers fizzled at birth) |
| 7, 2 | **wiped 1:44**, the wanderer (592) straight onto the squad's crystal | **8 alive**; broken 1:23 at that crystal (315 → 89) | 13 alive (its wanderers fizzled at birth) |
| 7, 3 | 8 alive (the wanderers spent far off) | 8 alive (the same) | dropped for time |

Paired off against bait: the squad was wiped in 2 of 6 off runs and in
none of the 6 bait runs (sign test 2 of 2, p 0.5: not significant). The
wanderer broke in 3 of 6 bait runs against 0 of 6 off. In every bait
break the boss thinned on a charged crystal before or as it met the guns;
on 4/3 it starved to broken with no soldier near.

**The choice.** Draining the squad's own crystal paid 2,180 to 3,270 food,
8 to 11 more soldiers by 3:00, and those squads lived too (planet 4). On
planet 4 the crystal that starved the boss was never the squad's own but
the next one on its way (437,283; 303,350), so draining the squad's
crystal cost no bait, and drain was the better play. On 7/2 the bait was
the squad's own crystal, so its drain run was to be the real test of the
trade; but in both planet 7 drain runs the wanderer formed elsewhere and
fizzled at birth (0 members), the game having gone another way once the
squad spent its income, so no boss came and the trade is still untested.

## Caveats

- Six pairs, a wanderer forced in all; two of them decide. A lead.
- Feeding changes the boss before it meets the squad too (7/2: it formed
  at 315 members fed against 592 off), so the bait is not only the
  crystal by the squad.
- A rooted wanderer still takes in the slime round it and swells (4/1
  off: 16,203 members); feeding does not stop that, only thins the hungry.
- Other kinds: a hunter, Pinwheel or Mortar goes for the squad, not a
  crystal, so it meets bait only by crossing one. Not tested here.

## Recommendation

**A real counter for the wanderer, worth building as its trait.** Fed
wanderers broke on charged crystals and no bait squad was wiped; it is
the slime's own food rule, and a player can read it: a boss heading for
a crystal thins out on it. The choice Direction framed exists only when
the crystal the boss goes for is one the player would drain; on planet 4
it mostly was not. Before it lands:

- Enemy: feeding for the wanderer only (its lump), so marches keep their
  armour; tune by play. Its look is the walkers stopping on the crystal.
- Lab, if wanted: the trade itself, with the boss placed (`boss=` staging)
  so the drain arm meets the same wanderer; more seeds. Planet 7 runs
  about 25 minutes a game on lavapipe.

Card: "Its wanderers are always hungry: leave a crystal in their way
and they stop to eat."

## Raw

`raw/runb.sh` (one run: arm, crystal, `boss_kind=2`), `raw/queue.txt`
(the eighteen; 7/3 drain dropped for time). Each line of the table is the
log's last `t N s:` line, its `bossfell` count and its `boss t` lines.
