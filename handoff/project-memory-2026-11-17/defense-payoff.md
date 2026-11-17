---
name: defense-payoff
description: Thread "Make building defenses worth it" (11-15): owner's turtle stalemate; options raids / kill box pays / hold ground; raids + render upgrade + last stand (off) built, PR #368 merged 11-15 23:15Z (raids walk round walls)
metadata:
  type: project
  modified: 2026-11-15T20:59:19.420Z
---

Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXEnjrVh9DUyQJjbvFwpmKRG, branch claude/project-thread-rb5511, started 2026-11-15 20:56Z.

Owner (Scotttt, 20:56Z): "I could build a huge defense, but that just ends with the mold clustering around its own crystals and not doing anything. even if i build lures, only a thin trickle moves into my kill box not enough to even counteract the mold growing in size."

Causes found in code (11-15): surges/panic only reach units within gun range + 5, so a far defense is never attacked; lure scent is 8x a soldier's over only 6 cells (BuildRules::lure_r) and gets eaten; pump_smell 0 so the economy draws nothing; surplus food goes to thickening at home.

Options put to owner 11-15 ~21:15Z: (1) raids: surplus food sends waves at your pumps/buildings from anywhere (Factorio pollution, They Are Billions noise), lures divert raids; (2) kill box pays: slime killed near turrets returns food; (3) hold ground: covered ground can't regrow, slime capped by held ground (Creeper World). Recommended 1+2.

Owner's pick (21:15Z): "Raids is the number 1 option. Also consider that a slime that is losing might rush you will everything it has. Not sure if this is a good idea or not. 2) the kill box pays but only with an upgrade". Plan: last stand built as an option, off (stated uncertainty), filmed for owner.

Slime thread (11-15 21:00Z): nothing open on surges; build raids on Game::time_surges + BigParams::surge_at (GPU word 3 of surge row); raids need their own target and pull, not panic_r; film with flinch=0 and default (flinch pushes back under fire).

Built (PR #368, branch claude/project-thread-rb5511, report Reports/raids-2026-11-15/, page https://claude.ai/artifact/8qKTwT5RmZbKeUQF4GZYG6): Rules::raid (raids.rs; 400 food, 15% mass, share 0.6, lure_r 40), BigParams::raid_* both engines (GPU row 976-1007, DRAW_RAID 406), turret render upgrade 600 food / 0.25 food a kill, last stand off. Fork turtle 4:00: kills 10,462 vs 7,850, 2 of 3 pumps lost, slime same size. Raiders walk round walls (11-15 22:5x): big::raid_field_for on 2x2 blocks built on a worker thread at decision, raid launches next game second; planet 8: 6 ms off-frame, 0.22 ms on frame; no paths.rs/squads.rs changes. Render upgrade is on the selected turret's card (main's building panel). Turtle rerun: killed 10,867 vs 7,850. Not timed on the 1070.

**How to apply:** slime behaviour is shared with the slime-enemy thread ([[slime-enemy-review]]); pathing with levelgen ([[levelgen]]). See [[larder-buildings]], [[pump-economy]].

Random raids (owner 11-16 01:48Z, PR #373): raids "regardless" of whether the kill box wins; each raid's wait N(60 s, 20) floor 20 (first N(75,15)) and size N(15%, 6%) within 4-40%; food meter removed; own Rng stream in Raids.
