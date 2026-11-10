# Firedamp playtest (2026-11-08)

`cargo run --release --bin firedamp`. Played through the code: the builder's harness, plus my own scripted players (`examples/zzreview_firedamp.rs`, since deleted). Each strategy ran on seeds 1-6, paired.

## 1. The open questions and my answers

**Before you light, can you tell where the flame will stop? Mostly yes.** The flame burnt 100% of the yellow patch it was lit in (27,001 of 27,019 cells over six seeds). I left out the cells inside a bang's own radius, and about 1 burnt cell in 5 still lay outside the yellow, not 1 in 2. Of those, 79% were purple when the match landed: the fan stirred them into the burning band during the burn. There are three catches:
- The whole burn is over in under a second. You see a flash, not a front you could watch stop.
- Thin yellow ribbons run along the edge of the green and join places far apart. In picture 2 the patch runs as a ribbon most of the way to the low face.
- Nothing on screen shows which yellow is joined to which. The pointer only gives one sentence, and with the match held, the thin and thick readings are cut off ("GAS HERE IS TOO THICK TO").

**Does herding gas feel crisp or mushy? Pushing is crisp; holding is mushy, and the game barely asks for either.** A fan clicked without a drag points right, toward the faces. In the fresh-player run it shoved the sump's gas up to the low face's lamp in about 9 s. That face was lost at 24-28 s, sooner than doing nothing (36-38 s). A fan used as a wall on the climb only delayed the loss to 81-93 s. A door holds the gas, but it also cuts both faces' air ("NO FRESH AIR"). **The builder's 2-star plan never taps a door** (door taps = 0): one fan and a match whenever there is yellow.

**Is "too thick to burn, so thin it first" learnt from one fizzle? Partly.** The fizzle message says the rule plainly ("THE GREEN IS TOO THICK TO BURN. STIR IT WITH A FAN TO THIN IT."), and doing what it says works. But it works slowly: the sump turned yellow 14-22 s after the fizzle, even with the fan placed at once. On 5 of 6 seeds a face blew up a few seconds after that. A planned player never fizzles at all (0 thick fizzles), because the fan running from the start stops the green ever forming.

## 2. What I played and what happened (seeds 1-6)

| strategy | stars per seed | ore per seed | first face lost |
|---|---|---|---|
| do nothing | 0 0 0 0 0 0 | 37 38 38 38 36 37 | 36-38 s |
| builder's plan (1 fan, match at yellow ≥30) | 2 2 2 2 2 2 | 194 193 194 194 198 195 | never |
| fresh player (click, match the gas, door when a face stops) | 0 0 0 0 1 0 | 36 35 34 36 74 34 | 24-28 s (62 s on seed 5) |
| clever: fan on the climb blowing back down, no door | 1 1 1 1 2 1 | 132 139 137 139 141 135 | 81-93 s |
| eager: light any yellow at all | 2 2 2 2 2 2 | 195 194 195 193 198 195 | never |
| greedy: wait for a big pocket | 0 0 0 0 0 0 | 12 12 10 10 12 10 | 47-50 s (the lamp lit it first) |
| cheese: a match at the leak every 1.5 s | 0 0 0 0 0 0 | 56 53 52 53 54 56 | 62-65 s, 0 bangs |
| cheese: one fan, nothing else | 0 0 0 0 0 0 | 39 38 39 37 35 35 | 39-42 s |

No cheese beats the plan, and random loses to it, as the builder found. But **waiting for a bigger bang never pays**: eager equals the plan, and greedy loses. Every winning run blasted 75-80 ore and then stopped, which looks like the seam running out, so **3 stars (220) was out of reach in every strategy.** The worst frame (tick plus drawing) was 40.9 ms during a burn; the mean tick was about 1 ms.

## 3. Captions

1. `1-nothing.gif`: doing nothing, seed 1. The pool fills the sump and climbs to the low face, which goes dark and then blows up at 37 s. The high face is lost later, which ends the shift early. 0 stars, 37 ore, 0 bangs.
2. `2-plan-light.png`: the builder's plan, first match, seed 1. Top: the magenta outline (my overlay, not in the game) is the yellow patch the match will burn. Bottom, 1 s later: that patch is gone, "+31 ORE". 1 light, 1 bang. Over the whole run: 32 bangs, 194 ore, 2 stars.
3. `3-plan-bang.gif`: the same bang in real time. You see the flash, the fire running through the yellow, the crater in the seam and the ore count.
4. `4-fresh-player.png`: the fresh player, seed 1. Top, 9 s: the first thick fizzle and its message, with the low face already reading GAS NEAR because of the fan pointing right. Bottom: the end. The low face blew up at 27 s, and the door slammed on the climb starved the high face ("NO FRESH AIR") for 2.5 minutes. 0 stars, 36 ore, 1 bang, 1 thick fizzle.

## 4. What feels good / what feels dead

Good:
- The bang. Flash, crater, falling ore and "+31 ORE" all land within a second. It is the best moment in the game.
- The three gas colours read clearly over the dark rock, and doing nothing visibly loses in about 37 s.
- The fizzle message is plain and correct.

Dead:
- Doors and herding take no part in winning. The plan is one fan and a match on a timer.
- Greed against safety, which the pitch promised, is flat: small pops earn as much as big bangs.
- A face lost to "NO FRESH AIR" is never tied to the door that caused it.
- The default aim of a fan quietly kills a new player.

## 5. Improvements, ranked

1. **(Do first) Show the patch the match will burn.** With MATCH or FUSE held, outline the joined yellow under the pointer. Label it SMALL POP / BANG / BIG BANG, draw the blast circle, and turn the outline red if the patch reaches a lamp. That answers "where will it stop" before the click, and makes a ribbon running to a lamp visible.
2. **Make greed pay.** Put more ore in the seam, or give ore that grows faster than the bang's size, so that 3 stars needs one or two deliberately large bangs. Today lighting every speck scores the same as the plan, and a big patch never forms before it spills.
3. **Fans: no silent default aim.** On placing a fan, show its cone and arrow and require the drag, or by default point it away from the nearest lamp.
4. **Make herding necessary and legible.** Add a second leak or a side gallery where the winning plan needs a door. When a face stops for want of air, draw the air path from the shaft to the face and highlight the door that cut it.
5. **Make thinning pay off in seconds, not 14-22 s.** Stir harder inside a fan's cone, so the green visibly turns yellow within about 3 s of the hint being followed.
6. Put the pointer reading on its own line so it is never cut off.

## 6. Promise score: 3 / 5

The bang and the colour band work and feel good. But winning comes down to one fan and a timed match, so the two hooks, herding and greed, are not in play yet.
