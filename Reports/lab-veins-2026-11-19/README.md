# Lab: veins the simulation makes, for the Trunk (2026-11-19)

The Lab lane's fifth prototype, on the lab branch `claude/project-thread-4tc75h`
only (code `042b8a9`, on main's code of 11-19), not in the game. Brief from
Direction 2026-11-19 09:30Z: Look found nothing in the simulation to draw as
veins inside the Trunk's thick body ([dead end](../dead-ends.md), the
Trunk's body as cords); the owner wants one look per slime, the Trunk's its
cords. Half a day, cheap pictures, a recommendation.

Page: "Trunk Veins", https://claude.ai/artifact/9xqBqE9DYxxfq8LMAvyac1 (source in `page/`).

## From the model

The Trunk (planet 3) is the plain rules with a narrow sensor angle (22.5
degrees), a long sensor reach (18) and a slow-fading trail (keep 0.95). Out
of its body those make long straight cords: walkers stream along a trail
ridge and the ridge is where they stream.

Inside the body nothing makes a direction:

- A fine cell holds at most one walker, and the body is close to full, so a
  walker inside is mostly blocked: it gets a new heading and stays.
- Every crowded cell lays the same capped deposit, and the trail spreads
  and fades slowly, so the trail is flat across the body and the three
  sensors read the same.
- So headings are random and there is no flow to see (Look's finding).

A vein needs something that moves inside the body and builds on itself.
Two rules try to make the walkers' own motion that thing. Both read the
**flow** on each 2 x 2 block: every walker adds its step's heading (0 when
blocked) and the block keeps three quarters of it each tick, so it is the
last few ticks' motion. Milling cancels and a stream adds up; its coherence
runs 0 to 1.

1. **Vein deposit** (`LAB_VEIN` v): a walker's trail deposit x (1 - v + 2v x
   coherence). A stream lays a ridge and milling lays little, so the
   sensors would find ridges and follow them (the trail reinforced along
   the direction of travel).
2. **Align** (`LAB_ALIGN` a): a calm walker turns a x coherence of the way
   to its block's flow (walkers lining up with their neighbours).

## Results

Planet 3, run seed 1, greedy hand, `run=1:3 mode=squads hand=greedy`, one
seed. Stills at zoom 12 on the body at 150,110 and zoom 6 at 130,120.
`LAB_ALIGN=0.0001` (the flow kept but no effect) is the control: it matches
the base picture.

| Arm | Slime 1:00 | Killed 1:00 | Slime 2:00 | Slime 3:00 | Killed 3:00 |
|---|---|---|---|---|---|
| Base | 40,427 | 3,678 | 17,414 | 15,192 | 7,724 |
| Control (flow kept, no effect) | 40,474 | 3,424 | | | |
| Vein 0.5 | 40,126 | 4,168 | 18,982 | 17,881 | 7,922 |
| Vein 1 | 39,611 | 4,498 | squad wiped at 1:25 | | |
| Align 1 | 40,196 | 3,716 | | | |

- **No veins inside the body** at either rule. The middle of the body stays
  a flat, even film; the picture changes only at its edges.
- **The body gives ground where nothing flows.** At zoom 12 both rules open
  a bare patch in the body's middle by 1:00 where base is full: the still
  walkers there lay little (vein) or drift together (align), and the body
  draws back.
- **Vein 0.5 makes its cords stronger.** At 2:00 a thick cord runs out of
  the body across the meadow where base has a thin trickle; the slime is
  9% bigger at 3:00 (17,881 against 15,192) and kills a little more.
  Vein 1 wiped the squad at 1:25 (one seed; base kept it to 3:00).
- Speed and crystals held: the vein arms ran the squad's harvest about the
  same (crystals left at 3:00 0.476 against 0.427); the slime's speed was
  not measured.
- A first version stored the flow in the dense region's fourth quarter,
  which is the fort's mask (`fort_at`): the whole body set into amber
  walls in a maze. Not a vein: those pictures were discarded.

Frame cost (not timed): one atomic add per walker step, a fade of each
2 x 2 block once a tick, and one read per deposit (nine per trail cell).

## Recommendation

**Drop both rules as the Trunk's veins.** A walker-motion rule cannot draw
veins inside a body that is packed a walker a cell, because nothing there
moves; it only acts on the edges and the cords (vein 0.5 makes the cords
bolder and the slime fiercer, which is a tuning question, not a look).

What does flow inside a packed body is **food**: the game's shared food
(`energy_flow` 0.05, in shape C, the game's slime) passes each walker's energy along the
body from the crystals it eats. That flow is simulation and needs no walker
to move, and the view can already receive each walker's energy (`alive`, `View::set_alive`). A
vein drawn from it (where energy changes along the body, from the crystals
out) would be the picture of real data. Untested: the lab has not looked at
whether the energy has vein structure or is a smooth gradient. That is a
Look question first (draw it once) and, if it is only a gradient, an Enemy
question (share food along the trail ridges, so food runs in channels).
