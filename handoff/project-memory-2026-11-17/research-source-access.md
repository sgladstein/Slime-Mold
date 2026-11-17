---
name: research-source-access
description: Where outside slime-mould sources live (research/sources/ cards), which can be fetched from cloud sessions, and the rules for copies
metadata:
  type: reference
  modified: 2026-11-10T17:26:29.176Z
---

- **Start at `research/sources/README.md`** (PR #8, 2026-11-10): one card per outside source with read depth, local copy, licence, key facts, and which reports use it. Add or update a card whenever a report relies on a source. Wiki pages cite card paths in their `sources:` field; cards stay out of wiki/ (agreed with the wiki thread).
- Sage Jenson's post refuses automated fetch (robots.txt). Use the owner's saved PDF and GIFs in `Reports/jenson-physarum-2026-11-10/`. Never use reader mirrors. Jenson is they/them.
- curl through the proxy is blocked for sagejenson.com, arxiv.org and xcoax.org (no PDF downloads); WebFetch can read arXiv abs/pdf, the xCoAx PDF, sagejenson.com home and /36points, and GitHub READMEs. gpu-io source on GitHub is robots-refused.
- Our code is proprietary: Bleuje (CC BY-NC-SA) and Lague (GPL-3.0) are read-only for ideas; PolyPhy (MIT) may be reused with its notice.
- After wiki PR #9 merges, wiki/lint.py fails on any top-level Reports/*.md that no wiki page cites (or that isn't in NOT_YET_INGESTED). New reports need a wiki citation in the same PR.
