---
name: player-mould
description: Player's own mould (gardeners) thread 2026-11-12: probe #105, CPU build #119, cap #124, craters/front #132, hunt+cord #149; next a front hand
metadata:
  type: project
  modified: 2026-11-12T17:23:39.980Z
---

Thread "Design the player's own mould species" (cmsg_01PcwPdssrGxiXYvyCG8WaUXBmPwzNsk24pDonTtxjxzQ3), started 2026-11-12 04:35Z. Owner chose **gardeners** (card, 04:45Z): soldiers feed, road and cut a second species (OWN = 3) that fights the enemy by conversion. Proposal doc: https://claude.ai/artifact/LBDQgb8x9RoH6bE11G8H3m. Report: Reports/player-mould-2026-11-12.md; wiki: wiki/world/own-mould.md (bumping the report also needs wiki/model/outside-looks.md `updated` bumped, else lint fails).

- #105 form probe (page H4Pd9hmsLHGgjqEciuNBmE): network out-eats hunks 14/16; dense forms win contact 7/7. Eat-vs-fight trade, not a cycle. `species_convert_per` (off).
- #119 CPU build (`own_mould.rs` Garden, `World::tend` hook, `examples/garden.rs`; page 7Z4b4NB1JG1H7QEf9TrUer): sowing grows own mould 13x farm (32/32), enemy not smaller.
- #124 `species_cap` (half the slots a side; taken core stops only enemy births): no change yet.
- #132 garden shots lay craters; `garden front`: road +886 own cells in 20 s, sow +609, fire -396 enemy (needs craters), cut radius 3 nothing.
- #149 (17:30Z): enemy hunt (`species_avoid_of`, off) feeds the player: hunters convert on arrival (net turned to own 27-30 of 32). ~7k walkers churn each way at the front anyway, so contact was never missing; the enemy needs an attacker's edge in conversion. One det cord (`Garden::cord`) about equals 20 s of fire at +20 s (-289 vs -354) but doesn't hold.

Reviewers: tech lead withdrew census gates after the 17:26Z testing rule (build, show in play, retune); still wants species slots (gpu::MAX_SPECIES 4) agreed with the tiers thread before fixing an id, and the GPU port measured at 20k total (10k a side; 1070 p95 ~8.5 ms) and 5k. Designer: find a third relation (something beats pure network), order-to-visible-change under ~5 s, soldiers keep a job. Next (designer's order): a front hand that picks road/sow/fire at the line, vs random and idle. Later: rerun with the one-body keys; crust/flier forms need per-species life rates. All results so far are CPU, 2,500 walkers a side. Related: [[one-body]], [[squad-toolkit]], [[larder-world-merge-order]].
11-12 20:00Z PR #158: garden hand "front" (sow2 + front team of soldiers 2-3 picking fire/sow/road every 20 s); enemy smaller than sow2 6/8 seeds; page https://claude.ai/artifact/Rv4ZqMJ1hQDnJKiVcAXihx. Built under the owner's 17:26Z rule (build first; census not a gate). Next: attacker's edge in conversion, GPU port.
11-12 23:13Z PR #165 merged (b17485d): GPU garden + window `larder-world own=1`. Keys: Sow W (main card, slot 1,3), Road Y and det cord C on the tools page (E). Bursts/det cord hit both moulds on both engines (design: the cord is the player's cut). Shots and squad strike spare species 3. Page https://claude.ai/artifact/SLmH8k3C797UCo3949RcMn. 1070 twin passed (skirmish bank test + life-cycle twin fail on main too). 1070 timing job 9 held for a free PC, at b17485d. Next: species_cap on GPU (shared birth pool), attacker's edge in conversion.
11-14 01:02Z PR #208 merged (13211cc): species_cap on the GPU garden (capclear/capcount into wstat 6/7 before breed; cap in each Set's +44 word; stop_births zeroes only the enemy cap). Needed for campaign mission 8 (own=1 vs Grudge). 1070 twin pending in the integration PC batch. Next: an attacker's edge in conversion.
