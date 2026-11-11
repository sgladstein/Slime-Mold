---
id: primordia
title: Primordia (Rust/wgpu Physarum sandbox, with the "Symbiosis" mode)
authors: skulitom (GitHub)
year: not recorded
url: https://github.com/skulitom/primordia
read: WGSL agent and trail shaders and the presets, by a research helper on 2026-11-11 (shallow clone in a scratch folder, not kept)
copy: none
licence: MIT (code may be reused with its notice)
used-in: ../../Reports/outside-rules-2026-11-11.md
---

# Primordia

A GPU Physarum sandbox in our own stack (Rust and wgpu). The rules
below are as the helper read them in the code
([survey](../../Reports/outside-rules-2026-11-11/code-survey.md)).

## Key facts for us

- Per-agent size: a fixed factor 2^(spread x u), with u in [-1, 1] hashed
  from the agent's index. It scales sensor reach and stride. We built this
  as `BigParams::size_spread`.
- Up to four species, each with a signed row of a 4 x 4 matrix; sensing is
  dot(trail_rgba, row). The cyclic preset has self 1.0, next +0.6 and
  previous -1.0, with SA 20, SO 32, RA 20, step 2 and keep 0.9. We built
  this as `BigParams::species_cycle`.
- Soft crowding on deposit n / (1 + n / 8); sensing saturates as
  t / (1 + t / K). We did not build either.
- Also a constant curl turn, a fertility terrain that scales decay and size,
  and swirl advection.
- "Symbiosis" couples agents to a Gray-Scott chemical that they graze, plus
  a slow soil layer worn down by traffic. We did not build it.
