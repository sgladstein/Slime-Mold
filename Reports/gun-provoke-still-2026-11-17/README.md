# Provoke and Still: the Mind family's last two cards

2026-11-17. The gun-card plan's Mind family
([review](../gun-system-review-2026-11-16/README.md), step D), finished with
the two statuses the simulation-use thread built for them
([B1 and B2](../sim-use-2026-11-17/b-provoke-still.md)). The owner ruled at
01:13Z that these tools are both carried devices and gun cards, with the
gun thread making the cards. The owner, 03:07Z: "Continue without my
guesses. I am busy". The game designer agreed the order at 15:27Z: Provoke
and Still, then step E.

Page: [PAGE](PAGE) (source in `page/`).

Status: STATUS

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

PLAY

## Frame cost

Nothing new runs every step. A cast is one Mind burst, as Blind's is. The
statuses' own cost is the simulation-use thread's (one more read of the
walker's Mind word in `panic_to` and the life pass). It has not been timed
on the 1070.
