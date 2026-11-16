# Independent review 1 of the mini-boss look (2026-11-16, before the fixes)

A subagent with an art-direction and graphics-engineering brief read the
diff, the engine's lump code, `bosses.rs` and six stills (Ember tune, zooms
6/12/24, and the hunter drawn plain). It edited nothing. Its report, as
given:

## Verdict
It doesn't read as a creature yet. At zoom 12 it looks like a slab of raw liver or pepperoni inside a gold crust. At zoom 24 it is a blurry maroon wall with orange confetti on it. It is darker, flatter and less glossy than the slime it came from, so the boss is now the lowest-contrast thing on screen. Most of it is honestly from the data, but the main signals are weak or misleading:
- **The "core" is shot noise.** At about 0.5 members a cell, the sharp kernel (σ 0.6) holds about 1.1 members and the wide one about 3.8. The near/wide ratio swings by about ±0.9, so roughly 30% of the body passes the 1.5 threshold by chance and re-rolls every frame; the 0.27 s strip boils. The same ratio feeds `rise` (the dome's height), so the bumps and glints boil too.
- **The gold crust is the members' own trail drawn as slime trail.** That is honest, but it says "slime".
- **The body itself is a thin cloud** (0.5 members a cell, a loose leash). No look will make a sparse cloud read as a solid predator; the creature has to be in the simulation.

## Directions (data → look)
1. **Flow (view only).** Add up each member's heading per cell; packing it as wrapping fixed-point adds works. How well the headings agree gives a striated, directional sheen while it marches and a mottled one while it mills, so the phase shows without a flag. Bring the streaks back, sized to the walker's real step each tick: stragglers the leash pulls in run fast, so they become long streaks converging on the body, which makes the leash visible.
2. **Wake (engine).** Members lay a trail tagged with their lump, so it leaves a coloured scar across open ground and the gold crust goes away.
3. **Release (engine).** On Broken, `form_lump(None)` clears the member bits, so the surviving half or more turns back to gold in a single frame. The climax is a pop. Keep a fading "was lump k" counter in the bits that frees up, so the colour drains out of each walker as it disperses.
4. **A body plan per kind (engine).** Make kinds differ in density profile and rhythm. For the hunter: denser at the front, a tight clench, and a periodic burst of speed. The dome then rises as it clenches and the streaks lengthen on each lunge, a real pulse rather than a painted one. Grade can then mean more members or a tighter leash. But `LUMP_N=0.5` saturates `cover` (0.86 at 1 member a cell, 0.98 at 2), so a denser boss would look the same; retune it.
5. **Crispness at zoom 24 can't come from `prep`.** Its material is two samples a cell, blended bilinearly under a σ 0.6-cell blur, so nothing it makes is sharper than about 14 px. Sharpness has to come from per-walker splats and from lighting: a taller, steeper, glossier dome gives a sharp highlight at the rim. The white highlight arc at zoom 12 is the best thing in these shots.

## Zoom and palettes
- **Zoom 6:** the older bright tune with streaks reads far better than any of the zoom 12/24 shots. The current tune has no zoom 6 shot. Its dots are about a pixel there (r = 0.75), so render it.
- **Zoom 24:** the boss fills the screen, so there is no silhouette and the surface has to carry it.
- **Ember is the worst choice.** Its dots (255,170,90) are nearly the leaning walkers' amber (255,178,40) and the spray's orange, so the body reads as a pile of panicking walkers. Its white-hot hit colour is the same `DROP_HOT` as shot spray, so hits get lost. Red already means dying here. Dark crimson on dark olive also collapses for red-green colour-blind players.
- **Violet is the one I expect to work.** It is the only unused hue, it is complementary to gold (so it stands out while gathering inside the slime), and it survives red-green colour blindness. Lift its skin's brightness.
- **Acid** fails: teal-black disappears on olive ground and trees, the green matches foliage, and teal sits next to the cyan crystals.
- **Gold** is the slime's own colour, so it isn't distinct.
- Later kinds should differ by brightness and structure (ink-black, bone-pale), not hue; the free hues are used up.

## Shader review
1. **Bug: condensed members.** Members keep and gain the condensed bit. They count 2 in `nb` but 1 in `lcb`, so `share` ≤ 0.5, the skin covers only about 77%, and bronze shows through. `dcnt += 15` also feeds the bronze plateau and height.
2. **Honesty bug: members still age on food news.** The age still drives the ash fade (`cnt` bits 16+). Below zoom 3 the member branch is skipped, so it also drives the amber and red panic specks.
3. **Nearest-centre look.** Two lumps get a Voronoi seam (a drawn edge), and overlapping lumps get the wrong look.
4. **`atomicMax` ordering.** Energy (bits 10-13) sits above hits, so a hit member's white dot loses to a better-fed neighbour.
5. **`LUMP_TAIL=0`.** The length becomes 0.01, so each dot is a forward half-disc: an accidental shape.
6. **Uniform: correct.** The "(44 words)" comment on `PARAMS_BYTES` is stale; add `debug_assert_eq!(b.len(), PARAMS_BYTES)`.
7. **Workgroup memory.** 2304 + 2304 + 10752 = 15,360 B against WebGPU's 16,384 B default, so one more channel won't fit. Measure `prep`'s cost.
8. **Bit budget.** 12 bits holds 273 condensed walkers a fine cell; with crowd 0 (unlimited, used in tests) it can overflow.
9. **Frames with no boss.** `dense = e.grows` now binds the dense words with condensing and thickening off. The test compares no lumps against a Plain lump in the new code, never against the base branch.
10. **Zoom 3 threshold.** Below zoom 3 members fall back to ordinary specks, so the colour pops as you zoom across it.

## Tests and checks to add
A byte-identical frame against the base branch with no boss; a lump of condensed members; a cut-off member at zoom 2; two lumps side by side; flicker over the body; the colour jump across the Broken tick; colour distance of member dots from amber, red and spray, and a colour-blindness simulation; grade 0 against 1; zoom 6 and gathering; a histogram of member energies; the 1070 cost of `prep` and `draw_specks` with and without a boss.

## What was done with it (same day)
Fixed: 1 (members ignore DENSE and ripeness), 2 (members draw no news), 3 (per-lump flags), 4 (hits on top), 5 (round dots), 6, 8 (dense field to 13 bits), 10 (dots at every zoom); core from the wide count (shot noise); `LUMP_N` 0.35; Violet the default, Acid and Gold dropped, Ink added. Tests added for 1-3 and the no-boss frame against the base branch (run by hand). Engine items 2-4 went to the mini-boss thread. Open: flow sheen (needs a channel the workgroup memory cannot hold; a separate buffer), flicker measure, Broken-tick jump (waits for the fade), colour-blindness check, 1070 cost.
