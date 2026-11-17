---
name: mini-boss-look
description: Mini-boss look thread (11-16): view draws a lump from its own members; round 2 = a look a kind + fade drain + leech cyan fill (PR #393 merged); round 2b held-body look PR #397 merged; round-1 A/B letter awaits owner
metadata:
  type: project
  modified: 2026-11-16T12:23:02.554Z
---

Thread "Make mini-boss slimes look cool" (cmsg_01PcwPdssrGxiXYvyCG8WaUX8BMLnYUy7fAXhh635oaWM9, branch claude/project-thread-w9vv1m). Report Reports/mini-boss-look-2026-11-16/ (round 1 + round 2 sections).
- Round 1 (PR #386 merged 11-16 09:00Z): blind A/B page https://claude.ai/artifact/8yuU2udSVYGZMLwuBUAibb (key raw/ab-key.txt: A violet, B ember, C ink); awaits the owner's letter.
- Round 2 (PR #393, merged 11-16 12:32Z): page https://claude.ai/artifact/HHYzZqj7Pt8iH1Znd8moVy. Kind looks in larder-world `kind_look`: hunter Violet, wanderer Bone (new), sapper Ember, leech Ink; `LumpBody` (at, look, strength, remain, fill) in `View::set_lumps`; fading bosses drawn with remain = 1 - progress/127; leech fill = take/(take+150) turns glow to charge cyan. LUMP_CORE 0.4 (was 0.9), LUMP_HOT (0.65,1.5).

Owner 11-16 05:28Z: the look must be "based on or inspired on the simulation data", not "random animation dropped on top"; suggest mechanics changes to the mini-boss thread; "you can take liberties if it helps"; 1-3 independent review subagents allowed.

View: fine grid 2nd half = condensed 0-12, members 13-21, lump flags 22-25 (atomicOr), absorbs 26+. Uniform 272 bytes. Member dot word: hits<<14 | e8<<10 | 3<<8 | lump<<4 | look. Keys `boss_look=` (overrides every kind), `follow=boss` (oldest boss, live or fading).

Mini-boss thread took the suggestion: leech rings its pump (#396 merged ~20:45Z, band 14-18 cells, ~0.36 members a cell in thick sheet). Round 2b (PR #397, merged 11-16 21:16Z): `LumpBody::held` (lump_looks bit 3) lets a held body's own member cover carry its look (`LUMP_OWN` smoothstep(0.2,0.8,cover)); gathering ones still seep by share. Uniform 288 bytes.

**How to apply:** if the owner's letter is B (Ember) for the hunter, the sapper needs another look. 1070 frame cost not timed. Violet sits near the core planet's purple flora (hue 290-320). Related: [[mini-boss]], [[slime-look]], [[art-direction]].
