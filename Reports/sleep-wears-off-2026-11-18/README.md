# Sleep wears off (2026-11-18)

Squad and arsenal work, from the owner's playtest of 2026-11-18 (run seed
1795038793, build 2190:d066132; logs on the `play-logs` branch, the
`2026-11-18T22-10-01Z-planet-3-...` and `T22-19-21Z-planet-4-...` files).

His notes, 22:33Z: "planet 3 ended up way too easy. Not sure if it was
because I had sleep on my gun or the slime on that planet hardened into
walls, but the slimed didn't really attack me at all. Also on planet 4 I put
sleep on my turrets and they were too OP. They basically stopped
everything. gun/turrets can become OP, but if you review how that happens
in noita, there are no single OP spells. Any single spell that is
incredibly powerful is also really dangerous. Most of the OP wands come from
interesting combinations of spells and often require some more rare
spells." 22:35Z: "dont automatically rebalance planet 3, it could have just
been the sleep. review first"; "or it is the hardening slime, which i don't
think is specifically a planet 3 thing".

## What his logs say

- Planets 3 and 4: no soldier lost (planet 2 lost 20). Planet 3's slime
  never grew past its start (42,355 walkers) and was worn to 1 in 443 s.
- The planet 4 replay has `load 255 mist`: every turret fired Mist (bullets
  leave a thin sleeping gas) and nothing else. The army round (his gun's
  cards) is not in the play log or the replay, so which sleep card his gun
  carried on planet 3 is not recorded (Gas or Mist; he calls both "sleep").
  The play log should carry the army round: an Engine and tools job.

## Planet 3: sleep, not the hardening

Planet 3 of run 1795038793 (his map) and of runs 1 and 2, `hand=greedy`,
150 game seconds, the squad's deck loaded at 0 s (`load=0,CARDS`). "Off" is
the Trunk's trait switched off (`setting=0 crust=0 fire_fed=0 set=fort=0
set=set_block=0`): no setting, crust, walls or fort. Deciding measure, named
before the runs: the squad's mean health at 150 s, and soldiers lost. Raw:
`raw/planet3-summary.txt` (scripts `raw/run.sh`, `raw/sum.py`; job lists
`raw/jobs*.txt`). The container restarted during the last batch, so some
of the "new" runs stop at 60 s.

| Gun | Trunk | seed 1795038793 | seed 1 | seed 2 |
|---|---|---|---|---|
| plain | on | health 60; 11 soldiers at 60 s, 1 at 120 s | 53; 11 -> 8 | 61 |
| plain | off | wiped at 100 s | 57 | 86 |
| gas | on | 92; 16 -> 36 soldiers | 97; 19 -> 31 | 96 |
| gas | off | 89; 18 -> 48 | 95 | 98 |
| mist | on | 60 | 78 | 82 |
| mist | off | 70 | 74 | 86 |

Gas alone turns the planet: health 92-97 against 53-61 with the plain gun,
on every seed, with no soldier lost. In those 150 s the gas put walkers to
sleep 300,928 times on his seed, about seven times over a 43,000-walker
slime. Switching the Trunk's hardening off made the plain gun's game no
easier (his seed: wiped at 100 s; seeds 1 and 2: 57 and 86 against 53 and
61). With three seeds this is a direction, not a sign test (p = 0.25 at
best for 3 of 3). Mist on the gun is weaker than gas (60-82).

The Trunk is meant to be the slime that holds its ground
(`Reports/sim-use-2026-11-17/e2a-setting.md`: set slime does not come on),
so a quieter planet 3 is partly its design. Planet 3's numbers were not
changed; that waits on the owner.

## The rule: a sleep is a window, not a lock

`KitRules::gas_immune` (20 s), `BurstRules::gas_immune`,
`Engine::gas_immune`, engine.wgsl's `gas_resting` (header word 6): a walker
that is gassed is not gassed again while it sleeps (no gas or cloud
lengthens its sleep), and once it wakes, gas and cloud do not take it for
20 s. Before, a fresh gas restamped the sleep and the cloud's `atomicMax`
held a walker asleep for as long as the cloud was refilled: a turret firing
Mist laid a cloud along every shot, so the slime in front of it never woke.

The same for the Mind statuses (`RoundRules::mind_rest`, 20 s; header word
7, `mindlay`): a walker that has a status, or had it fewer than 20 s ago,
is not given the same status again. Still (no charge, no eating) on a
turret was the same lock as Mist.

GPU engine only (the owner's 11-17 card: no new CPU twin). The CPU world's
bursts ignore both numbers; 0 is the old rule. Test:
`sleep_wears_off_on_the_gpu_world` (race world seed 5, a 5-cell gas of 60
fine steps on the home crystal, rest 120 fine steps): gassed each time
54, 0, 25, 73 against 54, 54, 71, 76 with no rest.

With the rule (gas on the gun, Trunk on): seed 1 health 81 at 60 s and 77
at 150 s (against 97); his seed 71 at 60 s with the slime touching up to
26 soldiers at once (against 90 and 3). The slime reaches the squad again.
The burst counter drops from 134,945 to 2,906 gassed on seed 1, because
the cloud's short sleep (3 s, `gas_linger`) now uses up a walker's sleep and
its rest; the payload's 20 s sleep lands on walkers the cloud has not
touched.

## Planet 4: inconclusive scene

His seven turrets and two walls (from his replay) standing from 0 s, the
squad idle, an early raid forced (`set=raid_first=30`), turrets loaded
plain, Mist (old rule) and Mist (new rule), 180 s (`raw/t4.sh`). The one
raid went for a turret and every turret stood at full health in all three
(`raw/p4-*-turrets.txt`): this scene does not stress the turrets, so it
says nothing about Mist either way. The rule's test above is the evidence
that a turret's mist no longer holds a walker past one sleep.

## The audit: single cards against the Noita rule (proposals, not built)

The rule as he put it: no single card is a win on its own; a very strong
one carries a real danger; strong builds come from combinations and rarer
cards. Today's single cards, by what they do alone:

1. **Gas** (common, sold from planet 1, budget 3): a 12-cell sleep for
   20 s. Even with the rest it is the strongest single card. Proposal: make
   it uncommon (planet 2 on), and give it its danger: the cloud puts
   soldiers standing in it to sleep too (they stop firing until it thins).
   Needs a read of the cloud at the soldiers (a small read-back like
   `calm`).
2. **Mist** (uncommon): with the rest, a slowdown on the gun. On turrets
   it is now a 3 s stumble per walker every 23 s. Proposal: rare (planet
   4 on), so a sleep wall is a find, not a default.
3. **Still** (uncommon): with the rest, no longer a lock. Leave it.
4. **Blast** (common): kills a 6-cell disc and keeps a crater 20 s, with
   no risk. Proposal: Noita's explosions hurt the caster; a blast within
   its radius of a soldier hurts him. A hit at the squad's feet (slime on
   top of you) becomes a choice.
5. **Rate** (common, doubles the fire): a plain stat card; fine as it is.
6. **Multicast, Trigger, Bomb, Germ, Detonate** are the combination and
   rare cards already (tier 3 or need a partner).

These go to Direction (tuning and offers are theirs) and the owner.
