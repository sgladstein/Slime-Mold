# Mini-boss look, round 1 (2026-11-16)

The owner, 2026-11-16 05:21Z (mini-boss thread): "We need to make these mini
bosses look cool." In this thread, 05:28Z: "This should not just be random
animation dropped on top of the slime blob. It should be based on or
inspired on the simulation data if possible. If you need changes in the
situation/mechanics for that to work suggest it to the other mini boss
thread." 05:28Z: "But obviously you can take liberties if it helps."

Page: https://claude.ai/artifact/8yuU2udSVYGZMLwuBUAibb, source [`page/`](page/index.html), a blind
A/B of three skins (key [`raw/ab-key.txt`](raw/ab-key.txt)) with today's look
as the reference.

## What a mini-boss is to the view

A lump (`mould::big` lumps, mini-boss thread, branch
claude/project-thread-g8thj2 at bcdbb59): ordinary walkers whose dense word
holds lump + 1 in bits 25-27, held together by a leash and steered as one;
hits taken in bits 28-31. `Game::lump_looks()` gives each lump's kind,
phase and grade. The hunter in these films (run 1, planet 3, squad holding)
forms at 91 s with 426 members inside the main body, hunts from 97 s,
crosses open ground at about 2.3 cells a second as a body 25 x 35 cells
(about 0.5 members a cell, spread 1.07 at 120 s), and is shot apart at
140-143 s (118 of 426 left). Boss lines: [`raw/boss-lines-run1-3.txt`](raw/boss-lines-run1-3.txt).

## What was built (view only)

- `draw_specks` counts members on their own in the fine grid's second
  half (condensed 0-12, members 13-21, a flag a lump 22-25 by `atomicOr`,
  absorptions 26+). Members draw no news (no ash, amber or red), no
  condensing and no ripeness: the engine spares them withering and panic.
- `prep` (`lump_here`, `lump_mat`): the members' share of the walkers
  under the body's wide Gaussian sets how much of the lump's material
  replaces the slime's; the members' own wide density gives the dome
  (`LUMP_RISE`), a darker wet membrane where their cover falls off, and the
  core light where they are thickest (`LUMP_HOT` 0.55-1.3 members a cell).
  The look is the flagged lump's, or the nearest flagged centre where two
  meet.
- Each member is its own dot in the core colour (`base`, the specks' third
  screen, layer 3), bright by its energy (`alive` / `LUMP_FULL` 1.5) and
  cold white by hits taken (hits on top of the `atomicMax` word).
- `View::set_lumps` (centre, `LumpLook`, strength) in a uniform (240 bytes
  now); Larder World binds the dense words whenever the world grows.
  Keys: `boss_look=plain|ember|violet|ink`, `follow=boss`.
- Hunter default: Violet (`HUNTER_LOOK`).

## Checks

- Tests `a_mini_boss_body_takes_its_look_and_plain_changes_nothing`,
  `two_lumps_keep_their_looks_and_members_show_no_panic_or_bronze`
  (llvmpipe, `MOULDGPU_TEST_GPU=1`): all 14 view tests pass.
- No boss: `larder-world map=fork hand=hold at=30,45` from the base branch
  and this one: both 30 s pictures byte-identical. At 45 s the base
  branch differs from itself between two runs (19 pixels of the whole-map
  picture, up to 59 levels), so 45 s is not a clean comparison (128 pixels
  against this branch, 4 in the near view, up to 2 levels).

## Tried and dropped

- **Core light from the members' clumps** (sharp over wide count, 1.0-1.5):
  at 0.5 members a cell the sharp window holds about one member, so the
  ratio is shot noise; the independent review estimated about 30% of the
  body crossing the threshold by chance, re-rolled every frame (the 0.27 s
  strip boiled). Now the wide count alone.
- **Look by the nearest lump centre** alone: two lumps would meet on a
  drawn line between their centres. Now per-lump flags in the fine grid.
- **Members as streaks along their heading** (0.3-0.7 cells): a milling
  body's headings are random, so it read as confetti. Now round dots.
- **Ember (crimson) as the default**: its dots sat by the panic's amber
  (255,178,40) and its red by the bolt's; dark crimson on olive collapses
  for red-green colour blindness. Kept as option B.
- **Skin edge by the sharp Gaussian**: it changed within a fine cell and
  drew as stairs at zoom 24. Now the wide one. (Plain slime also shows
  cell stairs at zoom 24 on this run; not from this change.)

## Independent review (11-16)

One reviewer read the diff, engine and stills before the fixes:
[`raw/review-1.md`](raw/review-1.md). Its engine suggestions went to the
mini-boss thread, which agreed (06:32Z) to a lunge (haste and pull jump for
about a second every few seconds; tighter clench; grade tightens the
leash) and a 1.5 s fade on Broken or Spent (`lump_looks` grows to u16:
fading flag and progress); a lump-tagged wake later.

## Open

- The owner's letter.
- The fade on release and the lunge (mini-boss thread's next change), then
  grades (strength) on screen.
- Violet sits near the core planet's purple flora (hue 290-320; the
  hunter's skin is about 265, its core 280): look at it on planet 8.
- 1070 frame cost.

## Round 2: a look a kind, and the fade (2026-11-16)

The mini-boss engine's round 3 (PR #391, merged 11:14Z) gave lumps four
kinds and a 1.5 s fade, and `Game::lump_looks()` became a `u16`: bits
0-2 kind, 3 not gathering, 4-7 grade, 8 fading, 9-15 fade progress
(0-127). Page: https://claude.ai/artifact/HHYzZqj7Pt8iH1Znd8moVy, source
[`round2/`](round2/index.html); films by
[`raw/round2-films.sh`](raw/round2-films.sh) (the round-3 boss staged at
1:25 on run 1, planet 3, forced to each kind; zoom 12, camera on the
oldest boss, live or fading). Boss lines: `raw/round2-boss-lines-*.txt`;
from 85 s to the end of each film they match the mini-boss thread's
round-3 logs line for line (the view does not touch the walkers).

### What was built

- **A look a kind** (Larder World's `kind_look`; `boss_look=` still
  overrides every kind): hunter Violet, wanderer `Bone` (new: a pale
  grey-lilac skin, a faint warm core, rose hits since cold white would
  not show on it), sapper Ember, leech Ink. Dark, mid and pale, so the
  kinds part by value as well as hue. On film the wanderer reads as pale
  pewter, liquid-metal under the sheen.
- **The fade**: `View::set_lumps` takes a `LumpBody` a lump (centre,
  look, strength, `remain`, `fill`); Larder World passes the fading
  bosses too, `remain` = 1 - progress / 127. In `lump_here` the share
  and dome scale by it and the core light by it twice over, so the light
  goes out first and the colour and dome sink into the sheet as the
  members spread; each member's dot carries its lump in bits 4-5 and
  fades with it. On film the spent sapper (196 members at 182 s) is half
  drained at 182.5 s and gold by 183.5 s; before, it snapped back in a
  frame.
- **A leech's take**: `fill` = take / (take + 150) (`LEECH_HALF`); its
  glow and its dots turn toward the charge's cyan (92, 224, 232) by it,
  and it glows wherever its members are, not only where they pack
  (`LUMP_DRINK` 1.5 over `LUMP_CORE`). On film 0% at 1:55, 38% at 2:13,
  55% at 2:31, 65% at 2:49 (take 277.6).
- **Softer core light**: `LUMP_CORE` 0.9 -> 0.4, `LUMP_HOT` (0.55, 1.3)
  -> (0.65, 1.5). A boss that sits (the sapper at its wall, the wanderer
  rooted) packs past the old full point all over, and the light swamped
  the skin: the crimson read salmon, the first pale skin (188, 180, 194)
  clipped white on its crystal. Bone's skin is now (140, 134, 152).
- `follow=boss` follows the oldest boss, live or fading, so a film stays
  on one while it drains.

### Checks

- View tests (llvmpipe, `MOULDGPU_TEST_GPU=1`): 15 pass, the first now
  also checking half and full fade (half lies between, gone is within 8
  levels of the plain gold) and that Ink lights lilac, a full leech
  cyan and Bone stays an unsaturated pale grey.
- The lunge shows: the hunter's spread goes 1.27 -> 1.57 -> 1.17 over
  101-103 s and its body flings out in tendrils and pulls back.

### Open

- The leech sits under the pump box, rig and crystal, so little of it
  shows (a cyan haze round the box). Sent to the mini-boss thread, which
  took it (11:56Z): the leech will hold its members in a band just
  outside the crystal round the rig.
- A hunter shot to pieces fades with about 7 members left, so its fade
  hardly shows; the mini-boss thread keeps that (the kill working).
- The owner's round-1 letter for the hunter; if B (Ember), the sapper
  needs another look.
- 1070 frame cost (unchanged passes; one more uniform read a lump cell).
