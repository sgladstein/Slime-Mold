# Planet 2: the bot or the planet? (2026-11-18)

Direction's question to the Enemy lane: the run bot loses planet 2 on every
run seed that reaches it, the Sprawler's threads eating the 9-soldier
landing. Is that the bot or the planet? If the planet, Direction sets the
Sprawler's size.

Page: https://claude.ai/artifact/129nrypmepPrPFzf1LSZjz (source `page/`; open `index.html`).
Films: `film.sh`; clips from their frames: `make_clips.py`.

**Answer: mostly the planet.** On three run seeds of four the squad lands
by small crystals while the slime, 23,000 to 25,000 walkers (planet 1's is
13,000), holds the big ones and bites the squad wherever it stands. No play
tried builds a squad there, and with the slime at 0.6 of its size the same
bot builds 18 to 24 soldiers on all three. Seed 2 lands by a big crystal
the slime does not hold and builds 30 soldiers at today's size; its loss is
the bot's known endgame (Engine and tools' handoff: the home over 200 walkers
a soldier and nothing left to drain).

## How it was looked at

The run bot (`hand=bot`) landing straight on planet 2 of run seeds 1 to 4
(`run=SEED:2`): 9 soldiers, no food, nothing bought in the shop. Software
GPU, the game as on main after #478 (`src/` unchanged through #481), one
film each. The bot logs every 10 game seconds. The deciding measure, named
before the runs: soldiers alive at 2:30, and whether the squad's food keeps
coming in after 1:00.

Four kinds of film on the three losing seeds:

1. **The run bot** as it is.
2. **Back off**: the bot looks for a crystal quiet over twice its gun's
   range (not once) and walks three ranges for it (not half a range), so
   it leaves the slime's side before it builds. A local patch to Engine and
   tools' `hands.rs`, not committed (below).
3. **Smell off**: the Sprawler's smell for soldiers turned down to the
   plain slime's (`set=smell=0.5 set=smell_r=2`; the Sprawler's is 4 and 4,
   `run::SPRAWLER_RULES`).
4. **Smaller slime**: `Level::walkers` times 0.8 or 0.6 (start walkers and
   slots, as a smaller size roll gives). A local patch to `Run::level`, not
   committed.

## Numbers

Soldiers alive at 2:30, the squad's mean health (1 is full), food harvested
by 2:30, and the slime's walkers at 2:30.

| Seed | The run bot | Back off | Smell off | Slime x0.8 | Slime x0.6 |
|---|---|---|---|---|---|
| 1 | 3, 0.22, 1,340, 24,247 | 9, 0.42, 1,963, 31,155 | 1, 0.06, 1,359, 29,984 | 14, 0.59, 3,738, 18,066 | 24, 0.74, 5,364, 12,122 |
| 3 | 9, 0.46, 2,461, 18,435 | 16, 0.48, 2,932, 24,707 | 14, 0.52, 3,601, 19,894 | 10, 0.41, 2,443, 16,906 | 18, 0.55, 4,526, 14,757 |
| 4 | 4, 0.26, 1,628, 18,694 | 4, 0.26, 1,628, 18,694 | 11, 0.45, 2,908, 15,107 | 10, 0.15, 1,825, 16,895 | 22, 0.55, 6,444, 8,513 |
| 2 | 30, 0.79, 7,356, 14,664 | | | | 28, 0.78, 7,443, 11,807 |

Slime at the landing: 23,076, 22,969, 25,448 and 23,010 walkers on seeds 1
to 4; 18,461, 20,358 and 18,409 at 0.8; 13,845, 13,782, 15,269 and 13,806
at 0.6. Planet 1 of run seed 1 lands on 13,119.

On seed 4 the back-off bot made the bot's own choices (no crystal was quiet
over two ranges either), so its row is the bot's.

### The run bot, seed 1 (the clip `seed1-worn-down.mp4`)

| Time | Soldiers | Health | Bitten | Food | Slime |
|---|---|---|---|---|---|
| 0:10 | 9 | 0.94 | 7 | 144 | 22,771 |
| 0:30 | 11 | 0.71 | 2 | 1,083 | 21,879 |
| 1:00 | 11 | 0.35 | 3 | 1,130 | 20,717 |
| 1:30 | 10 | 0.40 | 2 | 1,202 | 22,098 |
| 2:00 | 6 | 0.36 | 1 | 1,298 | 21,698 |
| 2:30 | 3 | 0.22 | 2 | 1,340 | 24,247 |

It lands on a crystal of 72,000 charge beside the slime's arm and is
bitten from 0:10. It drains it by 0:40 (1,130 food, two soldiers); the
slime holds the four big crystals (133,000 to 240,000); the free ones on
the left are 20,000 to 72,000. From 1:00 the squad is under half health,
so it steps back and heals whenever it is bitten, and the threads bite it
everywhere: food from 1:00 to 2:30 is 210. A raid of 4,135 walkers at 0:59
and a hunter mini-boss of 405 at 1:18 go for it.

### The run bot, seed 2 (the clip `seed2-builds-up.mp4`)

| Time | Soldiers | Health | Food | Slime |
|---|---|---|---|---|
| 0:30 | 12 | 0.95 | 1,112 | 22,655 |
| 1:00 | 19 | 0.93 | 3,364 | 20,802 |
| 1:30 | 25 | 0.90 | 5,701 | 16,930 |
| 2:00 | 26 | 0.87 | 5,948 | 15,714 |
| 2:30 | 30 | 0.79 | 7,356 | 14,664 |
| 3:00 | 30 | 0.49 | 8,239 | 10,877 |

The landing is by a crystal of 365,000 charge in a corner the slime does
not hold. The same threads come, but the squad drains 110 food a second at
1:00 and the volley holds them off.

## Reading

- The crystals by the landing decide planet 2 for the bot. Seed 2 lands by
  a crystal of 365,000 that the slime does not hold. On seeds 1, 3 and 4
  the squad lands by crystals of 20,000 to 120,000 charge, which give it
  1,100 to 1,900 food in the first minute and a half (three or four
  soldiers at 400 each), and every crystal over 130,000 sits inside the
  slime's body from the start (seed 4's 360,000 in a lobe of it).
- Better play helps a little and does not turn it round. Backing off gave
  more soldiers on 2 of 3 seeds (9 against 3, 16 against 9), and the same
  on the third; neither built up.
- The Sprawler's smell is not what does it. Turned down to the plain
  slime's, 2 of 3 seeds kept more soldiers (14 against 9, 11 against 4) and
  seed 1 lost more (1 against 3).
- The size is. At 0.8 seed 1 holds at 14 soldiers and seeds 3 and 4 stay
  near 10; at 0.6 all three build (24, 18, 22) and the slime ends smaller
  than it landed on two. Seed 2 at 0.6 is about as before (28 against 30):
  a size cut changes the hard landings, not the easy one.
- So for Direction: the slime's size on planet 2 is the lever, and 0.6 of
  today's (about planet 1's 13,000 walkers, against 23,000 to 25,000) is
  the first size at which the bot builds on every seed tried. Between 0.6
  and 0.8 is where the bot stops building.

## Not checked

- Winning: the films run to 3:00 and say nothing about the bot taking the
  home at 8 to 12 minutes, which is exit test 1's.
- The run as played: the run bot's run comes to planet 2 with what the shop
  sold it after planet 1 (Tinder, and the strike, rate and health levels it
  could pay for); these landings had nothing bought.
- Other seeds, and other sizes than 0.6 and 0.8. One film each.
- The owner's play. He plays differently from the bot.

## The local patches (not committed)

`src/bin/larder-world/hands.rs`, in `bot_hand`:

```rust
let far = std::env::var("BOT_FAR").is_ok();
let quiet = |c: usize| g.world.walkers_in(cs[c].0, g.world.crystal_r(c) + if far { 2.0 * range } else { range }) < 30;
// ...
let walk = if strong { 4.0 * range } else if far { 3.0 * range } else { 0.5 * range };
```

`src/mould/run.rs`, in `Run::level`: `walkers` times
`std::env::var("RUN_SIZE")` (1 when unset).
