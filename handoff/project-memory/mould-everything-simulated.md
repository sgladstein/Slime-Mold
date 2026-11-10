---
name: mould-everything-simulated
description: Scott 11-09: everything on screen in the mould game must come from real pixel simulation, no drawn-only effects (fake pulse disliked); food must really flow
metadata:
  type: feedback
  modified: 2026-11-09T01:10:09.040Z
---
Scott 11-09 01:09Z (thread cmsg_016bYH3Ybq4z6xkGqHujo4HWKXbN7RGrsGS3sjZN1gbMSb): "I also really don't like the fact that the pulse is just drawn on screen. Everything we're seeing should be based on real pixel simulations." Also 01:09Z: food feels fake because it is "just a distance metric": food up to 70 cells along tube, then grey.

**Why:** a render-only effect (the pulse, a distance cutoff dressed as food) reads as fake; he wants the picture to be the state of the sim.
**How to apply:** in the mould game (and likely the other probes), don't add render-only effects that imply a mechanic; draw fields the sim actually holds (food in tube, flow). Render-only is fine only for UI chrome (tiles, labels, previews). Related: [[mould-game-brainstorm-2026-11-08]].

**11-10 13:51-13:54Z (review thread cmsg_016bYH3Ybq4z6xkGqHujo4HW8wgxAHEW1fvrMLaVGqw4vA), the rule extends to STYLE, not just content:** "I don't like the sharp edge between the orange and black background. That also doesn't look real simulation"; same for the edge between orange and grey slime; "I want all the visuals to look like they're coming from a real simulation and not just graphics animations." My answer: the picture is the data. Colour straight from a held quantity through one fixed ramp, no threshold-and-fill (the tube edge is `trail > tube*0.5`); soft where the sim is soft, hard only where it is hard (salt, rim, snip); visible grain (a dot per walker); brightness adds up; no timer-driven motion. Mock page https://claude.ai/artifact/Gk8QELawvDLuYMgjBrC1GM (source /mnt/project-files/game-design/mould-review-2026-11-10/mouldraw.rs: brightness = 1-exp(-0.3*trail), blended hunger, additive walker dots; strands read thinner with no sim change). SCOTT PICKED (card 13:58Z): BUILD THE FROM-THE-SIMULATION LOOK as the default, with a key back to today's. Relayed to the looks thread cmsg_016bYH3Ybq4z6xkGqHujo4HWRgLiSYry5wTdeftNdJveXA, which built it: PR #1063 (14:36Z, with the merge desk), default on, F7 back to the tube look; draw cost +0.6-0.8 ms at scale 4.
**How to apply (any visual work here):** avoid thresholded fills, outlines, flat tones and binary colour switches on graded quantities; a time-instant warning must still blend in space.
