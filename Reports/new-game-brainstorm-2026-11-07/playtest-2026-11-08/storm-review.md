# Storm Glass (`--bin storm`): playtest review, 2026-11-08

## 1. The open question and my answer
**Is aiming lightning fun in the hand, or does it feel like dice?**

**It's not dice. The aim is honest, but the level gives you too little to decide.** The dotted line you see while holding predicts where the bolt lands to within a median of **1 to 2 cells** (90% within 3 to 5) for every player I scripted. From where you point to where it lands is a median of 2 to 3 cells. I reproduced the builder's aim table exactly: from 20 cells off, a tap lights the lamp 4/10 times and a hold 7/10; over open sand a tap misses by a median of 10 cells and a hold by 5. About 1 bolt in 10 still strays 30 or more cells over open ground, whatever the hold. The lamps pull bolts in, so play near them is forgiving: a careless fresh player scores **2 stars on 10 of 10 storms**. The weakness is the opposite of dice: one right-click solves the level (see "cheese").

## 2. What I played and what happened (stars per seed, seeds 1-10)
| Strategy | Stars per seed | Total /30 |
|---|---|---|
| Do nothing (builder probe) | 0 0 0 0 0 0 0 0 0 0, 8 bell hits each | 0 |
| Random (builder probe) | 0 0 0 0 0 0 1 1 1 1 | 4 |
| Builder's plan (builder probe) | 2 1 3 3 3 1 0 2 3 3 | 21 |
| Builder's plan (my replay) | 3 3 3 2 3 3 1 2 1 2 | 23 |
| Fresh player (aim jitter of ±3 cells; tries the middle, then field lamp, then rock lamp, water, rod) | 2 on every seed | 20 |
| Tap-spammer (8 quick taps on the field lamp) | 1 1 2 1 1 1 1 1 0 1 | 10 |
| Clever: rod at the ledge mouth, water bridge to the post | 2 3 1 2 2 1 2 3 2 1 | 19 |
| **Cheese: right-click the rock lamp, then strike the ledge** | **3 on every seed** | **30** |
| Same cheese with sloppy 0.3 s taps and ±3 aim | 3 3 3 2 3 3 3 3 3 3 | 29 |

The cheese works because a rod driven with the pointer on the overhang lamp lands on the lamp's head and stands up into the rock ledge. After that, every bolt on the ledge reads "LAMP UNDER THE ROCK LIT THROUGH THE SCORCH LINE", from the first bolt on, with no scorch line at all. A fresh player clicking "the obvious thing" finds this by accident. My fresh player lit the rock lamp exactly this way on all 10 seeds.

## 3. Captions
- `1-nothing.png`: do nothing. The cloud strikes the bell after 4 s. Seed 3: 8 of 8 bolts hit the bell, 0 stars.
- `2-plan-one-held-bolt.gif`: one held bolt from the plan (seed 3). The leader creeps down, the dotted line shows where it will land, and on release it lands where the line pointed. Dotted-line error is 1 to 2 cells (median over 80 bolts). The bolt lights the field lamp.
- `3-plan-end.png`: builder's plan, seed 3, last bolt. Both lamps are full: 3 stars, 0 bell hits, 2 lamps lit through wiring, 96 scorch cells. **The scorch wiring is almost invisible on the loam.**
- `4-cheese-rod-on-lamp.png`: right-click on the rock lamp. The rod spikes up into the ledge, and the first bolt on the ledge lights the lamp. 3 stars on 10 of 10 seeds.

## 4. What feels good / what feels dead
Good:
- Hold, creep, release is legible. The bar warns "LET GO NOW OR IT HITS THE BELL", and the dotted line keeps its promise.
- Every bolt answers within a second: a flash, a burst where it lands, and plain words for what it hit.
- Doing nothing loses cleanly (0 stars, 8 bell hits), and a plan beats random 9 of 10.

Dead:
- **There are few real decisions.** The field lamp fills in 2 heavy bolts, which leaves 6 of 8 bolts spare. Once you play, the bell is never at risk: 0 bell hits in all 70 of my active storms.
- **The "growing wiring" pitch doesn't show on screen.** Scorch is dark on dark loam at dusk. "Lit through the scorch line" is read, not seen.
- Glass and sand: no strategy needs them. The "blocks bolts" rule never matters.
- A tap aims nearly as well as a hold. Holding mainly buys gain 2 instead of 1.

## 5. Improvements (ranked)
1. **(Do first) Close the rod-on-lamp cheese and add more targets than bolts.** Let a rod stand only on open ground under sky, never on a lamp or under rock. Then use about 4 lamps with 8 bolts, so every bolt is a choice. On screen, the right-click should refuse with "NO SKY ABOVE - THE ROD WON'T STAND HERE".
2. **Make the scar seen.** Draw fresh scorch as glowing embers for about 2 s, then charcoal with a light edge. When a lamp lights through wiring, run a bright pulse along the network cells, which `Strike.network` already lists.
3. **Put the bell in play for an active player.** Let the cloud drift toward the tower during the storm, so that later calls compete with the bell's pull. The dotted line would show the bend toward the tower. Aiming would then trade risk, not just distance.
4. **Make the hold a real trade.** Let a tap cost half a bolt or recharge faster, while a hold digs a longer scar (wiring reach). Then "tap to be quick, hold to build wiring" becomes a choice.
5. **Use the glass.** Put one lamp behind the sand strip, so careless misses turn into glass walls you must route around.

## 6. Promise: 3/5
The verb is honest and readable, which answers the risky part of the question. The level is too small and has an exploit, so it's not yet clear that choosing where to strike is fun over a whole storm.
