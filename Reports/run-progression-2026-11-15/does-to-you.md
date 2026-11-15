# What each planet's slime does to you (D3 test, 2026-11-15)

Owner, 2026-11-15 21:41Z, on the whole-game review's D3 ("each tier gets one
thing it does to you"): "Agreed. We can test it out." On D10 (the outpost
leaves the run shop; the lure and siren stay out until they visibly draw the
slime): agreed. On D7 (a brake on the 40-soldier rush): "Play first. No
changes yet", so the rush is untouched here.

Page: https://claude.ai/artifact/GEMb4jx9chFLC7nxmXkUZu; its source is
`does.html` here, with the films in `does/` (before on the left, with the
planet's thing on the right, 4 s a frame).

## What was built

- `run::does_to_you(tier)`: one row a rung of `LADDER`, a sentence for the
  planet card and the landing alert, game rules (`Rules::set` names, set in
  `Run::rules`, not in the gun range) and engine keys (`gpu::set` names, set
  over the tier's own when the window makes the slime). Tests:
  `every_ladder_tier_does_something_to_you` (every rung says something and
  changes something; its keys parse after the tier's; the range is left
  alone).
- The planet card in the shop adds the sentence after the tier's name; on
  landing the alert reads "Drifter: It smells your pumps and comes to eat
  their crystals" (warning colour).
- D10: `ITEMS` no longer sells the lure, siren or outpost, and the command
  card hides them in a run (`run_hidden`); test
  `the_run_shop_sells_no_outpost_lure_or_siren`.

## The rows, and the films behind them

Run seed 1, one seed each (a look, not a census: no p-values). Health is the
squad's mean. "Before" is main's run; "with" adds only the row. Hands:
`pumps` (builds a pump on each crystal it drains), `idle`, `stand`
(attack-move to the core with the depth's average rifle).

| Planet | Tier | What it does to you | Setting | Before | With |
|---|---|---|---|---|---|
| 1 | Drifter | It smells your pumps and comes to eat their crystals | pump smell 20 over 15 cells (`pump_smell` 0, `lure_r` 6) | far pump 197/200 hp at 2:00, squad 4 at 41 health | far pump 23 hp at 2:00, first pump down by 1:00, squad lost at 2:21 |
| 2 | Sprawler | Its fast threads smell your soldiers from far off | soldier smell 4 over 4 cells (`smell` 0.5, `smell_r` 2) | idle squad 2 soldiers, 13 health at 2:00 | squad gone by 1:30 |
| 3 | Trunk | It runs its cords to your pumps | as planet 1 | first two pumps 189 and 182 hp at 2:00 | 163 and 161 |
| 4 | Swarmer | Its surges charge your squad from far off, and fast | surge reach 25 past the gun's range (`panic_margin` 5), lunge 3x (`surge_haste` 2) | health 74 at 2:00 | health 29 |
| 5 | Sporer | Struck, it scatters spores that start new slime | bursts at alarm 0.3 (tier 1), 0.2 a step (tier 0.05) | 4 soldiers, health 27 | none left at 2:00 |
| 6 | Sleeper | It wakes at your first shots, and all of it comes | wakes at alarm 0.3 (tier 1) | 1 soldier, 14 health | squad gone by 1:30 |

The branch's own binary with no extra keys gave the same lines as the films
run with the keys by hand, so the run applies the rows (planets 2-6
identical to the line).

**Tried and rejected** (same seed and hand, 2:00):

- Drifter, pump smell 2 to 8: too weak to see.
- Swarmer, a louder alarm (`alarm_relay` 20, `alarm_weight` 6): health 74 ->
  95 and kills 4,537 -> 878. The body went to the wounds, not the squad.
- Sporer, burst 0.2 alone: health 27 -> 20, 4 -> 3 soldiers (weak). Burst
  0.5 with `spore_rate` 1e-4: health 74 (backfired, the spores left the
  fight). Burst 0.3 with `spore_alarm` 3: 1 soldier at 13 (works, kept the
  simpler 0.3 bar). Burst 0.2 with surge reach 20: squad gone at 2:00 too.
- Sleeper, `wake_alarm` 3 with `alarm_relay` 40: a higher bar is a deeper
  sleep, 4 soldiers at 99 health and kills stuck at 459 from 1:00.

## Reading

Every row makes the planet harder in a way you can see on the dish: the
pumps planets lose pumps sooner, the squad planets lose the squad sooner.
Three rows now end the four-soldier test squad before 2:30 that survived it
before (planets 1, 5, 6) and one ends the idle squad 30 s sooner (planet 2).
That is a big step on the early planets; the owner's playtest decides the
size, and every number is one `Rules` or engine key in `does_to_you`.
Frame cost: none; each row sets numbers the game already uses.

The Trunk row is modest (the cords turn slowly). The review's own wording
had the Sprawler "cut supply"; no rule makes the slime go for the line
between a pump and the squad yet, so it smells soldiers instead. A rule
like that, or slime that bites buildings other than pumps, would go to the
slime-enemy thread.

## Not done here

- Several seeds, deeper planets 7-8, a bought loadout.
- The 40-soldier rush (D7): held for the owner's play.
