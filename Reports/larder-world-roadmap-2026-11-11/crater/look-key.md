# Key to the three looks on the crater page

Kept off the page (blind comparison). Order drawn with Python's
`random.shuffle` on 2026-11-12.

- Look 1: the scattered smaller strike, `aim=1.5,0.44,2.5`
  (`Rules::strike` 1.5, `shot_every` 0.44, `spread` 2.5).
- Look 2: the crater as it ships (strike 2.5, one shot a second, flat
  disc, `scorch_keep` 0.1, `scorch_full` 30).
- Look 3: the cone, `scorch_cone=1 scorch_full=150 scorch_keep=0`.

Window commands (Fork, seed 1, rush hand, software Vulkan):

```
larder-world look=DIR hand=rush at=5,7,9,11,13 aim=1.5,0.44,2.5
larder-world look=DIR hand=rush at=5,7,9,11,13
larder-world look=DIR hand=rush at=5,7,9,11,13 scorch_cone=1 scorch_full=150 scorch_keep=0
```

Frames are the `near` view cropped to (250, 110)-(900, 510).

The plain-soil frames (`soil-N-TTs.jpg`) use the same key, from the race
world with no map ground (`map=noise`), seed 1, rush hand,
`at=20,23,26`, the `near` view cropped to (330, 150)-(980, 550).
