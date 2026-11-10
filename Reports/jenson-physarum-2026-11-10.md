# Sage Jenson's physarum and other slime mould simulations (2026-11-10)

Research only, nothing built. Asked for by the owner on 2026-11-10: a deeper
look at Jenson's physarum work, what "contagion" is, whether code or more
writing exists, and a survey of other slime mould simulations. This file is
the repo copy of the live doc shown in the thread,
https://claude.ai/artifact/8WdSBQL1rwX6gKfABDtWtc . It builds on the full
read of Jenson's post, Cherouvim and Jones 2010/2015 in
[`mould-game-2026-11-08/depth-2026-11-09/sources-research.md`](mould-game-2026-11-08/depth-2026-11-09/sources-research.md).

**How it was read.** Jenson's post (cargocollective.com/sagejenson/physarum)
and sagejenson.com/physarum refuse automated reading (robots.txt). The owner
saved the page as a PDF and then the post's GIFs, kept here in
[`jenson-physarum-2026-11-10/`](jenson-physarum-2026-11-10/) with contact
sheets of eight evenly spaced frames per GIF (`*_sheet.png`). The text
matches the 2026-11-09 full read. A first pass judged the pictures from the
PDF's single frames and called network_2 "no bigger than ours"; the owner
disagreed, and the full GIF shows the owner was right (below). Also opened: sagejenson.com, sagejenson.com/36points (a JavaScript
viewer, no model text), the xCoAx 2020 paper (full text), This is Paper's
*Primordium* article, raster.art's artist page, the MCPM paper (arXiv
2204.01256, full text), the Polyphorm paper (arXiv 2009.02441), and the
READMEs of Bleuje/interactive-physarum, SebLague/Slime-Simulation,
CreativeCodingLab/Polyphorm, PolyPhyHub/PolyPhy and
jasonwebb/morphogenesis-resources. Not opened: the Dismantling video,
Lague's video, the gpu-io source (refused), Jones 2011 and 2012, Jones's
2015 book, Tero 2010 beyond its PubMed entry. The pages that describe Jenson
say "their", so this file does too.

## Summary

Jenson runs the same Jones 2010 rules we do, but with 5-10 million particles on a GPU, no one-particle-per-cell limit, and the trail drawn at full resolution. That, not a better rule, is most of why the clips are so much more interesting to watch than ours (400-2,200 walkers on a 256 x 160 grid). One clip grows from 3-5 loops to roughly 180-340 in nine seconds; our dish holds a few dozen at most.

- **Code:** Jenson has never published the physarum code. The closest readable code is [Bleuje's interactive-physarum](https://github.com/Bleuje/interactive-physarum), which ports Jenson's later *36 Points* parameter presets.
- **More writing:** one paper, [Jenson & Kuksenok, xCoAx 2020](https://2020.xcoax.org/pdf/xCoAx2020-Jenson.pdf), about a live stylus-driven performance, plus its [video](https://youtu.be/hEbbmJaYHAc). Neither gives numbers.
- **Contagion:** a second state that particles carry and pass on through an "infected trail", so a change can be seen travelling along the network at walker speed. It is the simulated pulse this repo has wanted since 2026-11-09, and is not built yet.
- **Wider field:** the [Monte Carlo Physarum Machine](https://arxiv.org/abs/2204.01256) behind the NASA cosmic-web story credits Jenson and is open source. Tero's 2010 Tokyo-rail model is a different, graph-based engine.

## Jenson's model, and what exists beyond the post

Jenson never published the physarum code. The post (February 2019) is still the most technical thing Jenson has written about it, and the "CPU version… soon" it promises was never posted. What Jenson has published since is art, one academic paper and one video, plus code by others who re-built the later work.

**What the post says the model is.** It is Jones 2010 with two changes. Each particle has a position, a heading and three sensors (front-left, front, front-right). It deposits onto a trail map, the trail gets a 3x3 mean blur and a multiplicative decay, and particles turn toward the strongest sensor. Every parameter is configurable (sensor distance, sensor size, sensor angle, step size, rotation angle, deposit amount, deposit size, diffuse size, decay), but the post gives no numbers.

- **Change 1: no one-particle-per-cell rule.** "For my implementations I usually ignored this step, preferring the patterns that arose without it." Jenson notes it is "crucial for exact mimicry" of the organism, because it approximates conservation of matter. Dropping it also makes every particle independent, which is what lets a GPU run millions.
- **Change 2: contagion** (next section).
- **Scale.** C++ and GLSL in openFrameworks, all on the GPU, real time on a GTX 1070, "with most of the examples comprised of between 5 and 10 million particles".

**What the post's pictures show** (the PDF of the page, then the GIFs themselves, stepped through frame by frame on 2026-11-10). All are grey trail on black, no colour map. Judged by what is interesting to watch and could become play, not by how much they look like the organism.

| Clip | What happens | Why it is interesting for a game |
| --- | --- | --- |
| network_2.gif (130 frames, about 9 s) | Opens on one coarse network of 3-5 loops in a disc, with feathery fronts at the rim. The fronts then branch into fine dendrites, the dendrites close into loops, and a fine mesh floods the whole dish while the first big ring stays as a thick trunk. Enclosed loops counted per frame: about 3-5 at the start, 30-45 at a quarter, 125-260 at half way, 180-340 at the end (range across two brightness cut-offs). | Visible expansion with a hierarchy: old trunks plus new capillaries. The fronts are the most alive part, and they are where a player's food or light would matter. |
| explosion.gif (220 frames, about 9 s) | A sparse, speckled flower of faint loops with a bright ring in the middle. The ring shrinks to a bright clot, then bursts outward as thick, bright, looping tubes that sweep across the old faint network until they fill it. | An event with a before and after: one state spreading through another from a point. Either this or the web clip is the contagion the post describes; the post does not caption them. |
| ef_9.gif and web_800.jpg (61 frames, about 4 s) | Inside a rough circle, fibrous cords of hair-thin strands merge and slide; outside, a straight square lattice. Pixels change about twice as much inside as in the corners (mean change 42 against 22 of 255 from first to last frame); the circle's edge barely moves in this clip. | Two pattern regimes side by side with a sharp border: a territory you could push. |
| Header still | Bundles of straight parallel diagonal lines over a fine square lattice | A grid-locked regime. Shows how far one rule set can swing with its parameters. |
| Diagram | Sense, rotate, move, deposit, diffuse, decay. Rotate: front strongest, keep going; front weakest, turn randomly; otherwise turn toward the stronger side | Exactly Jones's rule, and ours. |

Loop counts: pixels brighter than 110 (or 150) of 255 count as tube; dark regions inside the central disc of 15 to 20,000 pixels count as loops. Script: `jenson-physarum-2026-11-10/gif_frames.py`.

**36 Points, the idea worth stealing.** Each particle reads the trail value s where it stands, then sets its own sensor distance, sensor angle, turn angle and step length from it, each as base + amplitude x s^exponent. So one rule gives thin fast wanderers in sparse areas and slow tight bundles in dense ones, in the same frame. Particles also respawn at random after a while, so the picture never runs down. This is most of why the later work looks so varied from one preset to the next.

## Contagion: a state that travels through the network

Contagion is a second state that particles carry and pass on through the trail, so a change can be seen moving along the tubes. Jenson's whole description is two sentences: "I also added a contagion behavior that alters properties of the simulation. When a particle is infected, it lays down an infected trail which then spreads to other particles." The post's GIF shows it spreading out from the centre of the picture. Two clips fit that description: in explosion.gif a bright state bursts from the middle and sweeps over a faint old network, and in ef_9.gif a fibrous region sits inside a straight lattice. Either way, infection changes how particles move, not just their colour. No paper, video or port documents it further, so the mechanism below is a reconstruction from those two sentences and the clips, not Jenson's code.

The likely loop, one step at a time:

1. Some particles start infected (at a point, or where something happens).
2. An infected particle deposits into a second trail channel, the "infected trail", besides or instead of the normal one.
3. That channel diffuses and decays like the normal trail, so it lives only where infected particles have recently walked.
4. A healthy particle that senses or stands on enough infected trail becomes infected.
5. Being infected "alters properties": it might sense further, turn harder, step faster, or simply draw in another colour.

What that gives on screen, and why it is interesting for us:

- **It travels at the particles' own speed, along the network.** The front follows tubes and cannot jump a gap, because only walkers carry it. A cut tube stops it dead; a loop lets it arrive from two sides.
- **It is the simulation, not a drawing.** A visible wave made this way meets the owner's rule that everything on screen is simulated. Our repo's 2026-11-09 sources report already names contagion as the best candidate for a pulse the simulation makes (`Reports/mould-game-2026-11-08/depth-2026-11-09/sources-research.md`, sections 5.6 and 6.1). Nothing in `src/mould/` builds it yet.
- **If infection changes behaviour, the network rewires as the wave passes.** That is where the feedback loops and "tipping point" in the 2020 paper probably come from: a state change that spreads through particles and changes how they move.

Open questions the post leaves: whether particles recover (an SIR-style wave that passes, versus a one-way takeover), and which properties change. Both are design choices for us, not facts to look up.

## How our mould compares

Jenson's model is the same as ours step for step; the difference is about 5,000 times more particles, no one-per-cell limit, and a picture drawn at full resolution from the trail. Our rule set already matches Jones's Table 1 in every agent value (sensor reach 9, turn 45, deposit 5, 3x3 blur), so what you are seeing is scale and rendering, not a smarter rule.

|  | Jenson, physarum (2019) | Our mould today |
| --- | --- | --- |
| Particles | 5-10 million | 400-2,200 walkers (about 1-7% of cells) |
| Grid | Not stated; a screen-sized texture, about 2 million pixels at 1080p (my assumption) | 256 x 160 = 40,960 cells, about 30,000 inside the dish rim (`src/mould/mod.rs:71`) |
| Particles per cell | Several per pixel at 1080p (assumption above) | 0.01-0.07; about 0.2-0.3 on tube cells |
| One per cell | Off: particles pile onto ridges | On (`occ`); a blocked walker is Jones's "gel" |
| Where it runs | GPU (GLSL compute, openFrameworks), GTX 1070 | CPU, single thread, about 1.4 ms of frame per 1,500 walkers |
| Picture | The trail, at full resolution, grey on black | The trail as brightness at 4x pixel scale, hunger blended, a dot per walker |
| Tube width | A few pixels, with finer veins inside | About 12 cells (`TUBE_WIDTH`), so roughly 20 tubes fit across the dish |
| Extras | Contagion; later, parameters set by the sensed trail (36 Points) | Food, lamp push, salt, snips, water and sleep, a rival mould, walker food economy |

**Why Jenson's clips are more interesting to watch than our dish**, in order of how much I think each matters:

1. **Far more network.** network_2 ends with roughly 180-340 loops in one dish, with tubes about 1-2% of the dish's width. Ours are about 5% (12 cells of 256), so only about 20 tubes fit across and the dish holds a few dozen loops at most. That is the bigger scale the owner saw; the first pass was wrong to judge it from the first frame.
2. **It keeps changing.** Fronts branch, branches close into loops, a mesh fills in behind, and old trunks persist under new capillaries. Our default 45/45 setting is Jones's "minimising" regime and settles; a tube left alone moved 0 cells in 60 s in the organ prototype.
3. **Events sweep the whole dish.** The explosion clip turns one state into another from a point outward. We have nothing that travels through the network like that.
4. **A smooth haze and bright tube cores.** Several particles per pixel and no one-per-cell limit give a soft glow where the mould has been and tubes with a bright ridge. Ours are grainy and flat-topped.
5. **Resolution.** Each clip is drawn one pixel per trail cell; ours is a 256-wide field shown at 4x.

The header still and the lattice outside the web clip show the same rules can also lock onto the pixel grid in straight lines, so the settings matter as much as the particle count. The post gives none of them.

**What it would take here.** Scale is the only expensive part. On our CPU path, 5 million walkers would cost several seconds a step, so a Jenson-scale dish means moving the walkers to a GPU compute shader. Our window already draws through `pixels`, which sits on wgpu, so a compute path is in our stack rather than a new dependency. The game-side cost is bigger than the code: every per-walker rule (food stores, starvation, sleep, the rival's fight, the census's tube joins) would have to run on the GPU too, and constants tuned at sensor reach 9 on a 256-wide dish would all move. A cheaper middle step is a bigger grid (say 1024 x 640) with tens of thousands of walkers and one-per-cell off, as a look experiment before any GPU work. None of these has been measured here.

## Other slime mould simulations worth knowing

There are two families of model. Almost every artwork and game-like project, ours and Jenson's included, is Jones's particle-and-trail model. The famous Tokyo-railway result is a different model, a flow network whose tubes thicken with use. The table runs from the source papers to the implementations, with the one thing each adds for us.

| Source | What it is | What it adds for us |
| --- | --- | --- |
| [Jones 2010](http://eprints.uwe.ac.uk/15260/1/artl.2010.16.2.pdf), *Artificial Life* 16(2) | The particle-and-trail model everyone uses | Already read in full in `Reports/mould-game-2026-11-08/depth-2026-11-09/sources-research.md`. Sensor angle 22.5 never settles; 45 (ours) settles into a minimal net. |
| Jones 2011 (*Natural Computing*) and Jones & Adamatzky 2012 | Growth and shrinkage by local crowding; oscillation that emerges from jammed particles | Thinner, self-sizing tubes; a pulse and migration the simulation makes itself. Not read in full yet. |
| [Jones 2015, *From Pattern Formation to Material Computation*](https://link.springer.com/book/10.1007/978-3-319-16823-4) (Springer book) and the [2015 survey](https://arxiv.org/abs/1511.05774) | The model used as a "virtual material" | Shrink-wrap, inflate, sclerotium (drying into a ball), Voronoi borders between colonies, piles that smell only when touched. The survey is read in the repo; the book is not. |
| [Tero et al. 2010, *Science*](https://pubmed.ncbi.nlm.nih.gov/20093467/), "Rules for biologically inspired adaptive network design" | The Tokyo-rail experiment and its flow model: tubes carry flux, used tubes thicken, unused ones wither | A different engine: the network is a graph, not particles. Good for "which tube survives" rules; it does not make the organic look. |
| [Monte Carlo Physarum Machine](https://arxiv.org/abs/2204.01256) (Elek et al., *Artificial Life* 2022) and [Polyphorm](https://github.com/CreativeCodingLab/Polyphorm) / [PolyPhy](https://github.com/PolyPhyHub/PolyPhy) | Jones made stochastic and 3D, fitted to galaxy data (the NASA dark-matter story) | Agents pick a direction by chance, weighted by trail^exponent; as the exponent grows it becomes Jones's model. No per-cell limit. 10 million agents on a 1024^3 grid on one GPU. Attractive agents make stable nets, repulsive ones transient cells. PolyPhy is MIT-licensed Python on Taichi. |
| [Bleuje, interactive-physarum](https://github.com/Bleuje/interactive-physarum) (2024) | Installation piece on GPU compute shaders, gamepad-driven | The readable version of Jenson's *36 Points*: 17 of its parameter presets, blended across space under a pen, particle inertia, spawning, waves, 9 colour modes. CC BY-NC-SA. Closest thing to Jenson's code. |
| [Sebastian Lague, Slime Simulation](https://github.com/SebLague/Slime-Simulation) | Unity project with the video *Coding Adventure: Ant and Slime Simulations* (2021) | A clear GPU walk-through of Jones. The video shows several species that follow their own trail and avoid the others' (from memory of the video; the README does not say). GPL-3.0. |
| [Amanda Ghassaei, gpu-io physarum](https://apps.amandaghassaei.com/gpu-io/examples/physarum/) | Browser demo on WebGL fragment shaders | Touch adds attractant, presets in a menu. Credits Jones and Jenson. Source link refused my fetch. |
| [Jason Webb, morphogenesis-resources](https://github.com/jasonwebb/morphogenesis-resources) | Curated list | Points to more ports: [Ken Voskuil, WebGL](https://kaesve.nl/projects/mold/summary.html), [Nicolas Barradeau, three.js](https://github.com/nicoptere/physarum), [Deniz Bicer, Unity](https://github.com/DenizBicer/Physarum) and an [explainer](https://denizbicer.com/202408-UnderstandingPhysarum.html), [Jan Ivanecky, C++ 2D and 3D](https://github.com/janivanecky/Physarum), [Softology's write-up](https://softologyblog.wordpress.com/2019/04/11/physarum-simulations/). |
| [Georgios Cherouvim](http://ch3.gr/2018/slime-mold/) (2017-2018) | The posts that sent Jenson to Jones | 300,000 particles at 60 fps in TouchDesigner; agents steered by optical flow from video. Read in the repo's 2026-11-09 report. |

Every source above is an artwork, a demo or science; none is a game with goals and tools.

## What to try here

Five experiments come out of this, cheapest and most useful first. Each would be judged the repo's way: a look on paired seeds first, then counts against idle and random hands, with the frame cost quoted.

1. **Contagion as the pulse the simulation makes.** Food, a cut or a tap marks walkers; marked walkers lay a short-lived signal field; walkers that cross it catch the mark. The wave runs at walker speed and stops at a cut, so it shows the player which parts are connected. Cost: one bit per walker and one more 3x3 pass on a 41k-cell field. Deciding measure: does the front stop at a snip on every paired seed, and can you see it at 4x.
2. **One-per-cell off, as a blind look A/B.** Jenson's change, and the cheapest way to get bright vein cores. Risk: it removes Jones's "gel" and our blocked-walker turn, and the growth window counts walkers per cell, so food and crowding numbers move.
3. **Reach set by density, from 36 Points.** Let a walker's sensor reach shrink where the trail is thick and stay long where it is thin. That could give fine tubes without the loss recorded in `Reports/dead-ends.md` (reach 9 to 6 everywhere made the mould sense food from less far and every planned hand got worse), because searching walkers in thin trail would keep reach 9. Cost: one lookup and a power per walker.
4. **A bigger dish on the CPU before any GPU work.** For example 1024 x 640 with tens of thousands of walkers, to see how much of Jenson's look is density alone. Measure the frame cost first; the current rate (about 1.4 ms per 1,500 walkers) says 30,000 walkers is near 28 ms, too slow, so this is a picture experiment through `mouldlooks`, not a playable setting. If the look is worth it, the playable route is a wgpu compute shader.
5. **Stochastic steering from MCPM.** Pick the turn by chance in proportion to sensed trail to a power, instead of always the strongest sensor. The power is one dial from soft and wandering to Jones-sharp. Cost: one random number per walker.
