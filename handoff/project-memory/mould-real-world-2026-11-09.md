---
name: mould-real-world-2026-11-09
description: Mould out of the dish (Scott 11-09): side-on in lab box = runner on surface, roots only with buried food; options A side-on probe (rec), B top-down floor, C outdoor journey; page + PR #1055
metadata:
  type: project
  modified: 2026-11-09T04:18:28.103Z
---
Thread "Imagine the slime mould in a real world" cmsg_016bYH3Ybq4z6xkGqHujo4HW37y2bNuD4kkg2maKLqQBXD (Scott 11-09 03:48Z). Design exploration only. Page https://claude.ai/artifact/Uda48Hzdse6pSLWWZEBGb1; report Reports/mould-real-world-2026-11-09.md + examples/mould_world_sketch.rs in PR #1055 (MERGED 11-09 04:43Z). Follows [[mould-depth-2026-11-09]], [[mould-everything-simulated]].

FINDINGS (shipped mould, unchanged, one seed): side-on lab played_bed aged 20k frames: food lies on the surface, so the mould grows ONE RUNNER along the ground, 4/5 piles beyond home in 10 s untouched; 60-row soil with no buried food still a runner (+1 loop); 5 buried piles -> roots, 9/10. Top-down forest floor = dish network with scenery, 2/7 alone. sensor_offset 4 gave thinner branches (one seed). Jones walkers can't crawl ON a surface side-on (dead-ends §13b), only inside soil/litter + thin air film.
BUG FOUND: Dish::spread skips border cells, so trail on a livable edge never fades (harmless for the ellipse dish); told the mould thread.

OPTIONS: A "mould under the garden" side-on probe in its own binary on a lab-built world (RECOMMENDED; food up/water down/light pushes down), B forest floor from above (fallback, own world), C outdoor journey with spores on real wind (later). Note: reverses Scott's 11-08 06:44Z "don't bring the world in" if he picks A.
PENDING SCOTT: which option to prototype (decision card posted).
