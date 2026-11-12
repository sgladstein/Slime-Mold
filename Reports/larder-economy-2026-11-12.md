# Larder World economy: rations, and the overrun on (PR 4), 2026-11-12

The Larder thread's PR 4. Soldiers eat from the bank, a squad the bank can't feed loses health, and the larder overrun is on by default. The PR also takes out the regrowing core (`CoreKind::Regrow`, PR #163) after the owner's no-king ruling. Earlier Larder World work is in the [roadmap report](larder-world-roadmap-2026-11-11.md); raw output is in [larder-economy-2026-11-12/](larder-economy-2026-11-12/).

**Why.** Integration's findings from the one-body thread's slime-enemy review (17:10Z):
- Idle survived 46 of 48 games, but the owner ruled on 11-11 that an idle player should lose ("if you do nothing, the slime should get much stronger than you and kill you").
- Food didn't bind: farm, wall and fort ended with 83-85% of their harvest unspent.
- Strike won by banking behind a 30-soldier army it never paid to keep.

Plan agreed with integration at 19:59Z, built under the owner's 17:26Z testing rule (build first; the census answers a question and gates nothing).

**What.**
- **Rations** (`Rules::ration` 1): every live soldier eats 1 food a game second from the bank (`Game::eaten`). Farm's 8 soldiers eat 8 a second of the about 55 they bank; strike's 30 eat 30.
- **Hunger** (`Rules::hunger` 1): while the bank can't pay, every soldier loses 1 hit point a second (a share for a part-paid step; `Game::hungry`), so a full-health soldier lasts 100 s of an empty bank.
  - Hunger costs only hit points. A starving soldier walks, fires and harvests at full speed, so a farm that runs dry recovers when a carrier banks, and the need falls as the squad shrinks. Slowing hungry soldiers would spiral, since slower carriers bank less and so starve more (integration's check, 19:59Z).
- **The overrun is on** (`Rules::overrun` 20 s; it had been a campaign switch): the round is lost once the mould has held the larder with the bank empty for 20 s.
- The window tells the player once, each time the bank runs dry: "Out of food: the squad is starving (N food a second to feed it)". Soldiers' health falls where the player can see it.
- Census keys `ration=`, `hunger=` and `overrun=`; each row ends with the food eaten.
- Test: `rations_feed_the_squad_and_an_empty_bank_starves_it_until_food_comes` (paid, half paid, recovery when food comes, death, off at 0). The books, stuck and pump tests now count rations.
- **The game designer's three checks** (21:51Z: a squad that slowly dies on an empty bank and can do nothing is the first playtest's soft lock again):
  - *The drain shows.* The HUD's food rate was already the bank's net change, so it counts rations. Its tooltip now names what the squad eats a second, and when the bank is falling and would run dry within a minute the rate says when ("-6/s, dry in 40 s").
  - *Starving soldiers can fix it.* `a_starving_squad_harvests_as_a_fed_one_and_feeds_itself`: two games on one seed, with the same rations and the hunger on or off, harvest step for step the same until a soldier dies, and the starving squad banks a load and eats again.
  - *The reserve recruit isn't fed into starvation.* In the same test, a wiped squad on an empty bank gets its recruit at full health. The recruit walks to a crystal, banks food and eats within 60 s. A recruit on an empty bank has 100 s of health to make that trip.

**Smoke runs** (race seeds 5-8, raw [economy/](larder-economy-2026-11-12/smoke-ration1-race-5-8.txt)), choosing the ration:
- **At 2**: idle starves by 340-350 s on all 4. But strike starves its 30 soldiers on 3 of 4 and fort on 1 of 4: an army would cost 4.5 times its price over a game.
- **At 1**: idle starves by 570-600 s on all 4, and farm and fort hold to the cap. Strike wins 2, loses 1 and stalemates 1 (before rations it stalemated all 4). 1 is the default.
- **The rush** dies in 43-110 s on these seeds with or without rations: the hand walks its squad into the body. Without rations, 2 of its 4 games then sat at 0 soldiers to the cap. With rations, the bank drops under a soldier's price and the game ends.

**The census** (cd56a52, race seeds 5-20, seven hands, 15-minute games, about 30 min on this machine; raw [economy/](larder-economy-2026-11-12/census-ration1-race-5-20.txt)). The question, named before the run: does idle lose, does food bind, and does strike stop dominating? The comparison column is the PR 3b gate (90ca407), which also had the 10 s crater, not 4.5 s, so it is a guide, not a pairing.

| hand | ends, rations (mould wiped / squad wiped / cap) | PR 3b gate | harvest unspent at the end (median), rations / PR 3b |
| --- | --- | --- | --- |
| idle | 0 / 16 / 0, median 586 s | 0 / 0 / 16 | |
| random | 0 / 14 / 0 (2 overrun) | 1 / 14 / 1 | 0% / 2% |
| farm | 0 / 0 / 16 | 0 / 0 / 16 | 62% / 74% |
| wall | 0 / 0 / 16 | 0 / 0 / 16 | 58% / 74% |
| fort | 0 / 0 / 16 | 0 / 0 / 16 | 43% / 80% |
| rush | 0 / 11 / 4, plus 1 overrun | 1 / 5 / 10 | |
| strike | 6 / 3 / 7 | 4 / 0 / 12 | 17% / 64% |

- **Idle loses, 16 of 16**: its squad starves at a median 586 s.
- **Food binds more**: fort keeps 43% of its harvest unspent against 80%, and strike 17% against 64%. Farm and wall still bank over half, since 8 soldiers cost little; buildings and tools are their sinks.
- **Strike no longer dominates**: it beats farm 12-4 (p 0.08, not under Holm; 26-6 at PR 3b, held). It is decisive in 9 of 16 games, 6 wins and 3 losses, where it was 4 wins and 12 stalemates. Its three losses are 30-soldier armies that starved on seeds with a thin harvest. The player AI buys to 30 regardless of what it can feed, which is a tuning note for the player AI thread.
- Every plan but the rush beats idle and random 16-0 (all hold under Holm over 12). The rush loses to idle 4-12 and ties random 7-9.
- **Smoke checks**:
  - No game ends in under a minute; the shortest is a rush at 83 s.
  - Farm and wall end every game with all 8 soldiers, so they never starved.
  - The rush hand's 4 cap games sit at 0 soldiers with 411-491 food, which is over a soldier's price. The hand keeps a reserve and won't buy; a player could.

**Smoke run under the headless default** (after #164 made every mould headless; race seeds 5-8, five hands, about 4 min; raw [smoke-headless-race-5-8.txt](larder-economy-2026-11-12/smoke-headless-race-5-8.txt)):
- Idle starves in 4 of 4 games, at 522-597 s.
- Farm reaches the cap in 4 of 4 with all 8 soldiers.
- Strike reaches the cap in 2. On seed 7 the mould overran the larder at 778 s with the bank empty, and on seed 8 its 30 soldiers starved by 783 s. That is the player AI's 30-soldier army again, which it buys whatever it can feed.
- Random loses all 4 (three squads wiped, one overrun) at 179-289 s.
- The shortest game is the rush on seed 7 at 58 s. Its squad died in the body with 397 food still banked, so rations played no part. The rush hand walks into the body: on the PR 4 smoke runs before #164 it died at 43-110 s with or without rations.
- No soft lock: farm and strike are stuck 0 and 3 s a game.

**The regrowing core is taken out.** The owner ruled at 20:21Z: "remove the king rule. As long as the slime has food/crystal energy it can make walkers. Headless." The tiers thread confirmed at 21:00Z that no tier will use `CoreKind::Regrow`, so PR 4 removes it, along with `Game::mould_ate`, `World::start_births`, the census key `regrow=` and the window's "grew back" alert. It worked as built ([*A core that grows back*](larder-world-roadmap-2026-11-11.md) in the roadmap report), so it is recorded in `dead-ends.md` as dropped, not rejected, with what to restore if a core comes back. `core=fixed|headless` stays for tests.

**Cost.** Per step on the CPU: one multiply and a subtraction, and a loop over the soldiers while hungry. Nothing on the GPU.
