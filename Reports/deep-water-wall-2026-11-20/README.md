# Deep water stops the slime (2026-11-20)

The owner, 2026-11-20 05:52Z: "Deep water shouldn't be traversable by
player or slime". Soldiers already could not stand in deep water
(11-14, `map::standable`); the slime crawled through it at `deep_step`.
Now the GPU engine's slime cannot enter it either (`gpu::Settings::deep_wall`,
on by default; `deep_wall=0` is the old crawl). The CPU dish is unchanged
(the owner's card of 11-17 19:21Z: no new CPU twin).

## The rule (engine.wgsl, the water lines only)

- A walker on dry or shallow ground never steps (`sense`'s aim) or jumps
  (`jump`) into a deep cell; its trail and panic sensors read deep water
  as rock (`deep_shut`), so it turns along the bank as it does at rock.
- A walker already in deep water (flung there, or a map that starts it
  there) may still step on through deep cells at `deep_step` until it
  reaches a bank. Held in place instead, a walker a few cells into a river
  could never leave, and a planet is won only when its last walker dies.
- The slime's ways (a raid's, the charge paths', the far smell's, the
  lure's) are built on `GpuWorld::raid_ground`, which now shuts deep
  water, so raids and surges come round by the fords and bridges instead
  of pressing on the bank.

Frame cost: one more read of the ground word per walker per step for its
aim cell, and one per trail or panic sensor (three), all of them in the
cell the step already reads. Not timed (no GPU here).

## Checks

- `gpu::larder_world::tests::deep_water_is_a_wall_to_the_slime`, software
  Vulkan: the River map with the slime's start 15 dish cells from the bank,
  so 646 of 4,000 walkers start in the water. With the wall none goes in
  (the count only falls), and they swim out: 572, 480, 368, 294, 222 at
  2, 5, 10, 15, 20 s. Without it: 747, 780, 788, 788, 787.
- Soft locks, a one-off scan (`scan.rs`, not kept as a test: 100 s): every
  generated planet with deep water among planets 1 to 8, seeds 1 to 30
  (67 maps; deep water came on planets 3, 4 and 8), at scale 2. A flood
  fill over dry and shallow dish cells from each slime start's walkers
  reaches every crystal and the landing on all 67, and no living walker
  starts in deep water. This holds by construction: `levelgen::cut_fords`
  already joins every crystal and nest to the landing on soldiers' ground,
  which deep water never was.
- The twin test (`the_world_matches_the_cpu_for_one_tick`) runs its two
  soil dishes with `deep_wall` off, and still matches the CPU.

## The look

![Planet 3 seed 19 at 60 s, idle squad: with the wall the slime stops at
the lake's shallow rim and goes round; with deep_wall=0 a lobe pushes out
over the dark water](deep-wall-060s.jpg)

`larder-world look=DIR map=planet:3:19 mode=squads hand=idle at=10,60,120`,
and the same with `deep_wall=0`; whole-map stills cropped to the lake.
