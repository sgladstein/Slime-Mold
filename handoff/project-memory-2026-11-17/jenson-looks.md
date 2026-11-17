---
name: jenson-looks
description: Jenson's three missing looks (trunks, contagion, feathery fronts) on the big dish, 2026-11-11 (PR #45): what worked and what is still open
metadata:
  type: project
  modified: 2026-11-11T02:41:08.912Z
---

PR #45 (merged 2026-11-11) added off-by-default BigParams rules for Jenson's looks; report Reports/jenson-looks-2026-11-11.md, page https://claude.ai/artifact/DqJxgjQGVgnwQNwbbzoGrg.

- Trunks: a dense centred start disc (blob_r 45, haze 0.3) keeps a thick ring at 20 s when respawn is slow (0.001). The respawn rate sets how long it lasts.
- Contagion (sick_* fields): runs along the tubes. It coarsens the mesh while it passes but leaves no mark by 20 s, and a knife cut doesn't stop it. Same lesson as Two Fevers: it needs a lasting effect to matter for play.
- Feathery fronts: still not reproduced after 7 mechanisms. With Jenson's rule any haze organises on its own.
- Findings were sent to the GPU sandbox thread for porting.

**Why:** the owner asked for the three looks and a sandbox of every option (2026-11-11 00:06Z).
**How to apply:** before trying fronts again, read the dead-ends entry; it lists the untried ideas. Related: [[rts-direction]].

Owner, 2026-11-11 04:11Z: "we don't necessarily need to ReDiscover everything ourselves" (lean on published parameter knowledge, don't re-sweep it), and "at some point this just becomes an organic pattern generator ... it generates some sort of creature for gameplay and not just random organic patterns". Judge new rules by whether they make a creature (a body with identity, that moves, fights or grows), not by pattern novelty.

Outside rules (PR #55, 2026-11-11, page https://claude.ai/artifact/VLnRN8wTX9RxniD4c8drBw): creature leads are rival species with own trails (clean territories), explorer_spread (feathery radial growth, closest to Jenson fronts so far), blocked_deposit droplets/beads (still, don't move). Open: make a droplet crawl; species that fight over a border.

Creatures (PRs #56, #58, #61; page https://claude.ai/artifact/YYZJSPdm6VAb857m7WnJLR): grazing droplets crawl (graze_weight 10). Species fight edges: a louder trail (species_boost) overreaches. Numbers (species_share) win only when species are mixed (55% becomes 75-88%, 8 of 8); sorted bodies hold. Open: keep bodies mixing at contact.

Spread/mix (PR #61, page https://claude.ai/artifact/Mp6XiXvfqo4gxc1gWaMjRz): from one spot without food only explorers cross the dish. mix_* fields: walker kinds sort into parts (outriders, splitter, roamer, grazing outriders); sent to Larder as tier candidates. Next: mixes on Larder's finite crystals (PR #64, crystals_left()) with paired seeds.

- Owner (2026-11-11 14:59Z): explorer mould filling the dish in ~10 s is far too fast for a game; a mould must not take a whole map that quickly. Plan: gate spread by food cost (W0 growth on crystals), pace via steps/s as a separate knob.
