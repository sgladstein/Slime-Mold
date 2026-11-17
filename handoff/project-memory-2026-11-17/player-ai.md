---
name: player-ai
description: Larder player AI (larder_world_ai): seven styles incl. cut/take (PR #213 merged 11-14); C4 on shape C stalemates; take_claim wipes plain C 6/8 (#242); censuses paused until playtest
metadata:
  type: project
  modified: 2026-11-14T02:21:35.427Z
---

Owner asked 2026-11-11 for a player AI so threads can test Larder without playtests; later two soldier teams vs several moulds with AI soldiers.

Code: `src/mould/larder_world_ai.rs` (plays only via Cmd/Game::apply, reads View). Hands in `examples/larderworld.rs census` (knobs `boom:spend=2+turrets=8`); `scripts/plan_matrix.py run|table` (paired sign test, Holm). Small-dish AI (#65) is legacy, no small-dish tuning.

Styles: boom, claim, late, fort, strike, cut (strikers Fight to mid-road core->4 held crystals nearest home), take (cut then claim taken crystals, spend=2, 8 turrets). Census rows end `; peak N; built N; trace t:bank/soldiers/mould` every 30 s.

History: #88 world port; #156 matrix (strike leads Drifter/Sprawler/Trunk); #178 army sized to income (rations); #201 View::weigh (walkers x Rules::mass); #213 merged 2026-11-14 02:2xZ (7df2182): cut/take, spend bank on turrets, rich bank buys to max_squad when ration=0, trace logging, unit test a_rich_hand_spends_its_bank.

C4 (shape C 25k, seeds 301-308, with rations): no plan wipes the mould; mould drops 25k->6-10k in 30 s then plateaus; strike squad wiped 7/8, cut beats strike 7-1 (n.s.). No-rations dev seeds 1-4: strike best (+0.36-0.43, mould down to 2-5k) but every plan still hits the cap.

Owner (11-14 01:08Z): easily beats the game by hand; don't make the slime easier for the AI. Owner (01:49Z): soldiers must not need feeding; integration sets Rules::ration 0. Asked owner how they win (no answer yet).

Tier matrix (11-14, PR #234 queued, Reports/plan-matrix-tiers-2026-11-14.md): census-size C on 2371bf0, no rations, seeds 301-308, 9 plans x 8 columns, 1h26 local (no runners). Nothing wipes plain C (deciding measure failed); take best by median 7/8 (strike on Drifter), take wipes Swarmer 4/8; boom/claim/fort never attack -> stalemate +0.15; D3 dominance gates fail; Holm 120 none survive.

Take specialties (11-14, PR #242 queued, Reports/plan-matrix-take-2026-11-14.md): take_fort/take_boom/take_claim/take_strike (knob core_after). On main after #227 elbow room (lifted take a lot). take_claim (16 turrets, keep 4) wipes plain C 6/8 and is best on all 8 columns; others lose to take. Designer: no more censuses until owner's playtest; score stays (missions hold the real goals).

**How to apply:** no censuses until the owner's playtest; rebuild commands for squad control (no larder) when that lands. Owner asked how they win (no answer). Merging is by `queue` label (CLAUDE.md). Later: two AI teams, exploiter search, GPU census. Owner squad-control thread: cmsg_01PcwPdssrGxiXYvyCG8WaUXRjKGDpWRiGwPp1xy9E2dBJ. Related: [[mould-behaviours]], [[slime-enemy]], [[testing-proportion]].
