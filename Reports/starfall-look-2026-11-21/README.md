# Starfall and core look (2026-11-21)

The owner asked for a visual-design pass, as an artist would do it, on
starfall (the shadow before a fall, the impact, the ore on the ground) and
on boss cores (the drop, its pickup ring, the drain), 2026-11-21 03:36Z.
Page: https://claude.ai/artifact/QnxvV1wp4nX9iJtntaBVYT (source in `page/`).

**How it was filmed.** Planet 1 (run 7:1) in the slime viewer
(`viewer=1`), squad idle at 52.8,125, with
`LARDER_VIEWER_SPAWN="2:ore@60,140;13:core@49,127"`: ore warned at 0:02
and landing at 0:12, a hunter's core dropped by the squad at 0:13 (it
slides home a little and the squad drains it, 40 points, by about 0:21).
Frames every 0.2 s from 0:02 to 0:22 at zoom 12, every 0.25 s from 0:08
at zoom 6, the same command with the build before and after
(`centre=55,133,12` and `55,133,6`), cropped clear of the viewer's panel.
Software Vulkan, so no frame rate is quoted from these films.

**What it is.** One new mark shape (`Shape::Star`, `Marks::star`, drawn
in `view.wgsl`'s `star_at`) with six kinds: the warning (shadow, dashed
red ring at the kill radius, closing ticks), the meteor, the ore (cut rock,
molten cracks cooling with its warmth, gold flecks), the core (cut crimson
gem, heart beating faster as its life runs out, cracks in its last third),
the dial (the ring to stand in, filling gold with what was taken and dark
with what was eaten; it replaces the bar), and a burst (impact, core
breaking out, cashing). Gold motes (`Marks::streak`) run from a drop to
each soldier draining it. The window's code is in `draw.rs`, the block
after the buildings.

**Cost.** At most four marks a drop and one a mote; each kind's pixels are
its own box (the ore's 4 ore radii across, the dial's 2 reaches), so a
planet's few drops cost a few thousand pixels of shader work twice (claim
and compose) a frame. Not timed on the GTX 1070.

**Tuning from the films.** First cut: the meteor's halo and the drops'
glows reached past their marks' boxes and showed square edges, the ore was
a black lump crossed by a white net of small cells, and a dial drawn under
its drop was hidden by the drop's glow. Now: halos end inside the box,
the ore has three or four big cells of cracks over lighter cut faces, the
dial is drawn over the drop and claims only its band, and the impact's
light was cut from 5 to 2.2 so the landing does not white out the squad.
