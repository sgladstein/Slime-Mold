# Provoke and Still: the Mind family's last two cards

2026-11-17. The gun-card plan's Mind family
([review](../gun-system-review-2026-11-16/README.md), step D), finished with
the two statuses the simulation-use thread built for them
([B1 and B2](../sim-use-2026-11-17/b-provoke-still.md)). The owner ruled at
01:13Z that these tools are both carried devices and gun cards, with the
gun thread making the cards. The owner, 03:07Z: "Continue without my
guesses. I am busy". The game designer agreed the order at 15:27Z: Provoke
and Still, then step E.

Page: [Provoke and Still](https://claude.ai/artifact/KeLdsKF61XaDDdESBLEdGF) (source in `page/`).

Status: built and on. Finished by the Squad and arsenal lane, 11-17 (the
branch `claude/project-thread-vx9t3p-wip` rebased onto main, filmed and
measured). No card family follows it until Milestone 1 passes.

## What was built

- **`Card::Provoke`** (`big::MIND_DEAF`) and **`Card::Still`**
  (`big::MIND_STILL`). Both are payloads that kill nothing. They lay their
  status on the walkers within `RoundRules::mind_r` (8 dish cells, wide by
  `Wide`), as Blind, Sour and Spin do, and count in the Mind slot.
  - **Provoke**: the walkers hear no food news, so once their news is older
    than the wither lag they believe they are cut off. They turn the
    cut-off grey, panic (charge a soldier within `panic_r`, or run for a
    crystal) and wither, as a part a wall cut off does. It is the cut
    family's answer where the slime's shape leaves nothing to cut
    ([cut count](../gun-cut-count-2026-11-17/README.md)): it fakes the cut.
  - **Still**: no panic, surge, raid or breakaway, and no crystal taken.
    The walkers keep walking the network.
- **Their seconds are the devices'** (`KitRules::provoke_secs` 12,
  `still_secs` 10), not the 6 s of the other Mind statuses. A provoked
  walker panics only after the 5 s wither lag and a 1.5 s lean, so 6 s
  would end the status just as it acted. `BurstRules::deaf_steps` and
  `still_steps` carry the lengths, and both engines' bursts read them
  through one function (`BurstRules::mind_steps_for`), so the CPU and GPU
  cannot differ. `set=provoke_secs=` and `set=still_secs=` tune them.
- **Cost 2 and rarity 2**, as the other Mind cards. As rolled bullets
  (`Shell::Provoke`, `Shell::Still`), the shop sells them from planet 2,
  in the standard set only (`Shell::special`).
- **Faces**: the icons are a body with its vein broken and the charge it
  makes (Provoke), and a rough sea gone flat (Still). Shots use the Mind
  family's rose-magenta tracer. The card text gives the reach and seconds.
- **Test decks** (the window's H): `provoke`, `still`, `provoke-plain`.

## Tests

- `a_mind_bullet_casts_its_status_and_kills_nothing`, extended: each new
  card is a Mind-slot payload of cost 2 and rarity 2. Its burst carries its
  status, with no kill and no alarm. The burst rules give Provoke 12 s and
  Still 10 s while Spin keeps 6 s, and `provoke_secs` tunes Provoke's.
- `provoke_and_still_rounds_stamp_the_devices_seconds` (CPU race world): a
  round cast at a walker stamps the status on every walker within
  `mind_r`, to the step its seconds end. It counts one Mind cast and kills
  nothing.
- The engine twins for the two statuses are the simulation-use thread's
  (`the_world_matches_the_cpu_for_one_tick`, "mind: deaf and still
  walkers").

## In play

Two paired clips from planet 1 (`run=1:1`), each one game filmed twice
from the second the deck loads, plain bullets against the card in every
other shot, at zoom 6 and 12 (`raw/film.sh`, counts in `raw/counts.json`).

- **Provoke** (squad mode, `hand=hold@222,70`, deck at 14 s, 16 to 32 s). A
  squad of eight holds beside a crystal the slime's stream feeds. It only
  defends itself (the commanded squad's guns are the player's), so with
  plain bullets the slime's film grows over the crystal beside it. With
  Provoke the slime it hits goes grey about 5 s on, lets go of the crystal
  and runs at the squad: the crystal is clear at 27 s. Shots 42 against 23
  (21 of them Provoke), kills 387 against 319, slime 12,980 to 12,608
  against 12,899, all 8 soldiers at health 98 either way.
- **Still** (`hand=squadwork` without `mode=squads`, so the eight stand as
  one group in the open; deck at 50 s, surge at 54 s, raid at 59 s, 56 to
  72 s). The raiders it hits lose the charge and drift past. Soldiers 8 of
  8 at health 87 against 7 of 8 at 71; the most walkers on one soldier 8
  against 13; kills 320 against 585. In squad mode the same hand splits up
  round a crystal and the pair read alike (health 85 against 84 at 70 s),
  so that scene is not the film.

**Paired 20 s windows** (`raw/windows.sh`, 18 runs, about 20 min on 4
cores; a tuning note, not a gate; table `raw/windows.txt`, logs
`raw/windows/`). Run planets 1 to 3, each played to 50 s and to 70 s, then
20 s with plain, provoke+plain or still+plain. Named before the run:
soldiers left (then health), kills, slime at the end.

| Window | plain: killed, soldiers, health | provoke | still |
|---|---|---|---|
| p1-50 | 526, 8, 67 | 552, 8, 68 | 298, 8, 88 |
| p1-70 | 432, 7, 51 | 437, 7, 46 | 325, 8, 60 |
| p2-50 | 1,030, 6, 39 | 1,103, 6, 39 | 879, 7, 18 |
| p2-70 | 1,166, 0 | 949, 0 | 714, 0 |
| p3-50 | 31, 8, 100 | 44, 8, 99 | 36, 8, 100 |
| p3-70 | 908, 6, 67 | 916, 6, 66 | 342, 8, 87 |

- **Still** against plain: the squad better off in 4, worse in 1 (sign p
  0.38; 39 soldiers against 35); kills fewer in 5 of 6 (p 0.22; 2,594
  against 4,093); more slime at the end in 6 of 6 (p 0.031). Over the six
  comparisons (two cards, three measures) none survives a Bonferroni
  correction (0.05 / 6 = 0.008). It reads as a trade: safer, fewer kills.
- **Provoke** against plain: soldiers better in 1, worse in 4 (p 0.38; 35
  and 35); kills more in 5 of 6 (p 0.22; 4,001 against 4,093 in all). In
  these windows the slime already charges the squads, and Provoke adds
  nothing to a charge. Its use is slime that sits on something.
- These windows play `hand=squadwork` without `mode=squads` (the earlier
  thread's script), so they are the eight as one group, not the squad
  mode's hand.

## Frame cost

Nothing new runs every step. A cast is one Mind burst, as Blind's is. The
statuses' own cost is the simulation-use thread's (one more read of the
walker's Mind word in `panic_to` and the life pass). It has not been timed
on the 1070.
