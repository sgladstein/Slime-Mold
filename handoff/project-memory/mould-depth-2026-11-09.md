---
name: mould-depth-2026-11-09
description: Mould 11-09: real food flow built (flow switch, T panel), drawn pulse off; two depth reports + both Jones papers; 45/45 is Jones's settling setting, 22.5 is his live default
metadata:
  type: project
  modified: 2026-11-09T03:25:17.815Z
---
Thread cmsg_016bYH3Ybq4z6xkGqHujo4HWKXbN7RGrsGS3sjZN1gbMSb. Follows [[mould-game-brainstorm-2026-11-08]], rule [[mould-everything-simulated]].

BUILT on claude/project-thread-cek3ly (no PR yet): `Params::flow` (off by default; food carried, sap stream+beat, walls that remember), T tuning panel saved to mould-tune.txt (src/mould/tune.rs), drawn pulse OFF everywhere (`render::DRAWN_PULSE`). Commits f8df782ce, b5ed628de. Flow page https://claude.ai/artifact/UtZ3tYPdiaCiL5wLVRjr7C. Flow costs plan the tough star (storm cut really starves), mid-tube lean (far load 0.08), +1.7 ms/frame.

SAVED TO REPO (Scott 03:12Z "all sub-agent reports ... saved in the repo"): Reports/mould-game-2026-11-08/ (brainstorm, prototypes, reviews, 8 page sources, depth-2026-11-09/{sources-research,complexity}.md), commit 690948bb7. Papers (PDF+txt) only in /mnt/project-files/game-design/mould-depth-2026-11-09/papers/ (copyright; not in repo).

DEPTH REPORTS: complexity.md = water/wet field, crust (sclerotium), hunger flips lamp -> fruiting/spores, dissolving salt, salt habituation; 15x15 matrix; 3 dishes. sources-research.md = Jenson (contagion pulse, no 1-per-cell), Cherouvim, Jones.
JONES 2010 FACTS (read in full): Table 1 SA 22.5 or 45, RA 45, SO 9, depT 5, decayT 0.1, %p 3-15, wProj 0.01-0.1. SA22.5/RA45 never stabilises, sprouts bridges across gaps; 45/45 (OURS) minimises to a fixed network. Weak stimulus -> Steiner tree, strong (0.5) -> minimum spanning tree. Shrinking sheet (%p 50, remove p 0.00025/step) = repeatable network; fast = holes. Single agent shuttle-streams. 2015 paper: oscillation/travelling waves from a small movement change; rich=radial, poor=pseudopod; repellent Voronoi; meeting moulds pause then fuse; sclerotium blob -> centroid.

22.5 GOALS (Scott card "Keep 22.5, new goals"): measured 6 seeds x 7 hands: at 22.5 every hand joins all food by 55-58 s (= when the last late pile lands) and survives the storm, so QUICK and WEATHERED stars are free; ratio 1.34-2.03 for all hands; ORACLE PRUNER (probe players=prune, 150 unlimited snips that keep food joined) cannot make it lean (1.40-1.84, broke reach on 3/6): the mould regrows faster than cutting. flow=1 at 22.5: idle leaner (1.28-1.48) than plan. Join clock + oracle pruner commit 9f9cb3159. Asked Scott (card): hold it back (rec) / calming tool / explore-then-settle. BAIT PATCHES prototype commit 69df5a12e: Params::spared (N patches, off), spared_scent 0.5 default (0.1 used on page), probe players=guard; page https://claude.ai/artifact/A9PeyWDtzXzzX24m4iZCga. Smell makes patch a trap; salt ring saves 6/6; lamp fails once patch smells. Stars not built; 22.5 not yet default. 
REVIEW 11-10 (other thread, PR #1056, page https://claude.ai/artifact/W4EqSvxDWK1RC9YgpSQPTR): recommends none of my card's 3; ranks (1) keep-alive dish no bell at 22.5, fewer oats, food further; (2) player marks that stay; (3) draw walkers, sensor reach ~6, one colour ramp for flow. Against bait patches. Scott picking on that thread's card; wait for it. Hand harness /mnt/project-files/game-design/mould-review-2026-11-10/mouldhand.rs.
SCOTT ORDER (11-10 03:07Z "Agreed" in review thread): 1 keep-it-alive endless dish (+ "Looks too": walker specks, finer tubes, one flow colour ramp), 2 water tool + lamp dries + dry mould sleeps/water wakes (complexity.md items 1-2) on the endless dish, 3 wild second mould (other colour, steals food, then fights at seam). Bars: good game uses >=3 tools, random loses to plan; wild mould: player's hand changes which mould ends bigger, paired seeds. Scott: "tools are quite one dimensional".
KEEP IT ALIVE, WATER+SLEEP, NEXT STEPS: see [[mould-keep-alive-2026-11-10]].
PENDING SCOTT: grey look pick A/B/C; flow tuning verdict; OK to add SA/RA/SO/food strength to T panel + 22.5 vs 45 page (offered 03:4xZ).
