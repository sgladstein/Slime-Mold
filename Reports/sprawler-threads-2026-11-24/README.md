# The Sprawler's threads in the game (2026-11-24)

The Enemy lane built the Lab's threads into planet 2, the Sprawler. The Lab's prototype is [`../lab-threads-2026-11-24/`](../lab-threads-2026-11-24/README.md). Direction's word was to land it on and to retune planet 2's size if it played harder.

Page: "Sprawler Threads In Game". Its source is in `page/`, and it is listed in `Reports/pages.md`. The page shows main before and the change, in a clip at zoom 8 (0.2 s frames), a clip at zoom 6 (1:00 to 2:00), stills at zoom 6 and 12, and the whole map.

## The rule

The threads are two rules, on the GPU engine only:
- **Lanes:** a calm walker may step into a full cell when that cell holds as much trail as its own, up to 3 more walkers there.
- **The shuttle:** a walker past 0.9 of its birth energy walks down the crystals' scent at 3, and back up it when hungry.

With them, the trail is kept at 0.92 and the sensors sit at 6. The keys are `thread_lanes`, `thread_trail`, `thread_shuttle`, `thread_fed`, `keep` and `sensor_offset`, set in `run::SPRAWLER_KEYS`.

The view's thread veil is now gated by the walkers nearby (Look's `smoothstep(0.02, 0.12, wide)`), so the gaps between the spokes show bare ground.

## Counts

Run seed 1, planet 2, an idle squad of 4, measured on lavapipe:

| Arm | Size | Landed | Walkers at 1:00 | Walkers at 2:00 | Alone or in twos | Killed | Soldiers at 2:00 |
|---|---|---|---|---|---|---|---|
| main | 0.6 | 13,845 | 12,788 | 14,317 | 26% | 6,223 | 4 |
| threads | 0.6 | 13,845 | 19,827 | 26,725 | 11% | 5,352 | 3 |
| threads | 0.45 | 10,384 | 15,493 | 15,593 | 14% | 4,646 | 4 |

The threaded body packs more walkers into the same ground. At 0.6 it filled its birth ceiling of 27,691. The packed body stays near 14,000, held by its room.

So `run::tier_size` for the Sprawler goes from 0.6 to 0.45. The spokes read the same, and the walker count by 2:00 is close to main's. An earlier siege by the run bot at 0.6, on two maps, won 1 of 2 with threads against 2 of 2 without.

## Cost

Each step a calm walker moves under a crowd limit reads two trail cells (`thread_room` in `walk`), and each walker compares its energy once in `sense` (the shuttle's sign). The GTX 1070 time is in the 4K thread's next planet 2 batch. Lavapipe is not for timing.
