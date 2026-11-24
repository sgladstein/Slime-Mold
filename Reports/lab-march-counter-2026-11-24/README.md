# Lab: a counter to planet 4's armoured march, from the slime's own rules (2026-11-24)

The Lab lane's twelfth job, on the lab branch `claude/project-thread-2o4cic`
only (code `14ba8f8f`), not in the game. Brief from Direction 2026-11-24
09:14Z: the late-raid wall on planet 4; prototype one model-side counter a
player could use against a big armoured march, built from the slime's own
rules (scent, alarm, trail), not a new tool. To Direction before Enemy.

Page: "March Counter" (source in `page/`, listed in `Reports/pages.md`).

## The idea: a march is hungry, so feed it

A march is a raid lump: its members carry the lump in their dense word and
take 3 blows on planet 4 (`RAID_ARMOUR`). Read from the dump
(`LAB_MARCH_LOG`), a march is **born hungry**: the 14,311-walker march of
planet 4, seed 1, leaves at 0.57 of birth energy a member on average, 61-70%
of its members under half. The counter that uses that, from the slime's own
food rule rather than a tool:

- `LAB_FEED=F`: a march member (any lump) under F x birth energy that stands
  on a charged crystal leaves its lump and eats: it loses the march's
  orders and its armour, and becomes plain slime. So a squad that holds on
  a free crystal makes whatever reaches it stop and eat first. Run here at
  F = 1.0.
- `LAB_HUNGER=F` (tried first): a lump member under F x birth energy has no
  armour. Since marches are born hungry this is a flat cut to the armour,
  not a counter (0.5 and 0.8 gave the same run), and it is dropped.

## Results

Planet 4, bank 3,000, `hand=hold` (8 soldiers, holding still), lavapipe,
3:00 unless stated. `raw/runm.sh`, `raw/one.sh`, `raw/two.sh`, the queues
in `raw/queue*.txt`.

**The marches were not the wall in this rig.** With the mini-bosses off
(`set=boss_first=9999`), the squad of 8 lost no one to a march:

| Rig | Seed | Off | Feeding |
|---|---|---|---|
| on its crystal, raids 0:30 then every 40 s | 1 | 8 alive, 13,643 killed | 8 alive, 13,231 |
| | 2 | 8 alive, 8,042 | 8 alive, 8,145 |
| | 3 | 8 alive, 10,925 | 8 alive, 10,677 |
| raids at 2:00, 3:00, 4:00, to 4:30, at its landing | 1 | 8 alive, 13,077 | 8 alive, 13,167 |
| the same, on its crystal | 1 | 6 alive, 9,107 | 7 alive, 10,803 |

The marches (1,600 to 16,800 members) stall on the way and stream in
(`raids.rs`, `march_stall`), and a still squad of 8 shoots the stream down.
Every squad wiped in this job was wiped by a **mini-boss** (`bossfell` in
the log), never by a march.

**Feeding does starve a mini-boss that goes for a crystal.** With the
mini-bosses on:

| Seed | Mini-boss (same in both arms?) | Off | Feeding |
|---|---|---|---|
| 1, at its landing | Pinwheel (same) | wiped 1:24 | wiped 1:30 |
| 1, on its crystal | Pinwheel off, Mortar fed (not the same) | wiped 1:32, 7,850 killed | 8 alive, 13,409 |
| 2, on its crystal | wanderer (same, 507 / 499 members) | **wiped 2:42**, 7 to the boss | **8 alive**, 8,532 killed |
| 3, on its crystal | wanderer (same) | 8 alive | 8 alive |

Seed 2 is the one clean pair: the wanderer heads for the crystal at
437,283; off, it keeps 507 members, sits there, then comes on and wipes the
squad at 2:42; fed, its hungry members drop out on the crystal (499 to 336
by 2:13, 152 by 2:43) and it never reaches the squad. Seed 1's pair rolled
different mini-bosses, so it says nothing about feeding. One clean pair:
a lead, not a result.

## Not tried, and caveats

- A rig where a march does wipe the squad. The wall Direction named is
  likely the bot's planet 4 losses or a bigger army's raids drawn by pumps;
  I did not have that game to hand. Feeding should be retested there.
- Feeding counts any lump, so mini-bosses and marches alike; the slime that
  drops out on a crystal grows the slime there (seed 1 on its crystal:
  47,354 against 39,066 at the end).
- A fed walker still bites: it stops marching, not touching.
- What a player sees: the march or boss thins as it crosses a crystal. No
  look of its own; the picture is the walkers stopping.

## Recommendation

**Not yet a counter to the march; keep it as one for the wanderer.** In
this rig a still squad of 8 is never broken by planet 4's marches, only by
its mini-bosses, so there is no march wall here to counter. Feeding
starves a mini-boss that goes for a crystal (one clean pair, seed 2), which
is a counter a player can read and use: fight from a crystal the boss is
heading for. Before Enemy builds anything:

- Direction: name the game where the march is the wall (a bot league game
  or the owner's), so the Lab can rerun feeding there; or take feeding as a
  mini-boss counter (more seeds with the same boss in both arms first).
- Enemy: nothing to build yet.

A card for it, if it lands: "Its raids are starving: make them cross food
and they stop to eat."

## Raw

`raw/runm.sh` (one run, `LAB_MARCH_LOG` on), `raw/one.sh` and
`raw/queue.txt` (the 3:00 runs), `raw/two.sh` and `raw/queue2.txt` (the
4:30 runs). Logs are not kept; each line in the tables is the log's last
`t N s:` line and its `bossfell` count.
