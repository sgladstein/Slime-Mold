---
name: lean-reading
description: Owner wants threads to read only what their task needs; CLAUDE.md is the only always-read file (2026-11-10)
metadata:
  type: feedback
  modified: 2026-11-10T20:01:21.292Z
---

On 2026-11-10 (19:59Z) Scotttt asked whether every thread really needs to read README, Reports/README and the wiki, and how many tokens that costs. They also pushed back when a CLAUDE.md review added HANDOFF and ROADMAP to the list everyone reads. They approved making the reading depend on the task (PR #25). Now CLAUDE.md (about 1k tokens) is the only file every thread reads. The old list everyone read was about 12k tokens.

**Why:** the owner cares about how many tokens each thread spends on context that does nothing for its task.

**How to apply:** don't add files to the list in CLAUDE.md that every thread reads. Briefs for runner or other narrow threads should name only the files the task needs. Search the big indexes (dead-ends, Reports/README) for what you need rather than reading them whole. Related: [[haiku-runners]], [[roadmap]].
