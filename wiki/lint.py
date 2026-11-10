#!/usr/bin/env python3
"""Mechanical checks for the mould game's wiki (wiki/).

    python3 wiki/lint.py            # from the repo root; exit 1 on any error

Errors (fail): a relative link that does not resolve; a page that
`wiki/index.md` does not link; a `code:` symbol in a page's frontmatter that
no longer appears anywhere under `src/`, `examples/` or the scripts kept in
`Reports/`; a page missing a required frontmatter key; a row of a page's
Numbers table with no link to its source; a line under "Owner's word" that
does not start with a date; `index.md` over 150 lines; a dish, tool or
mechanism page that `HANDOFF.md` does not link.

A top-level report under `Reports/` that no page cites is an error too,
unless it is listed in `NOT_YET_INGESTED` (reports from before the wiki).

Warnings (print only): a page whose `updated` date is older than the last
commit to one of its `sources`; a listed pre-wiki report still uncited.

Why a script and not an LLM pass: these checks are exact and cheap, so a
session runs them before every wiki commit and spends its judgement on the
part a script cannot do (pages that contradict each other or a newer
report). Stdlib only, so it runs anywhere Python 3.8+ does.
"""

import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
WIKI = os.path.join(ROOT, "wiki")
REQUIRED = ("title", "kind", "status", "updated", "sources")
# Reports that are indexes or records, not sources a page summarises.
NOT_SOURCES = {"README.md", "dead-ends.md", "pages.md"}
# Reports that predate the wiki (2026-11-10) and no page cites yet. They only
# warn. Any report not listed here must be cited by a page, or lint fails:
# the repo is new and will grow, and a wiki that skips new reports goes stale
# without anyone noticing. Remove a name once a page cites it.
NOT_YET_INGESTED = {
    "mould-review-2026-11-10.md",
    # Merged (PR #10) minutes before the wiki and its lint landed (PR #9).
    "rts-design-2026-11-10.md",
}
INDEX_CAP = 150

LINK = re.compile(r"\[[^\]]*\]\(([^)\s]+)\)")


def pages():
    for d, _, fs in os.walk(WIKI):
        for f in sorted(fs):
            if f.endswith(".md") and not f.startswith("_"):
                yield os.path.join(d, f)


def frontmatter(text):
    """The `---` block as a dict; `[a, b]` values become lists."""
    if not text.startswith("---\n"):
        return {}
    end = text.find("\n---\n", 4)
    if end < 0:
        return {}
    out = {}
    for line in text[4:end].splitlines():
        if ":" not in line or line.startswith(" "):
            continue
        k, v = line.split(":", 1)
        v = v.strip()
        if v.startswith("[") and v.endswith("]"):
            v = [x.strip().strip("`") for x in v[1:-1].split(",") if x.strip()]
        out[k.strip()] = v
    return out


def sections(text):
    """(heading, lines) for each `## ` section."""
    out, name, rows = [], None, []
    for line in text.splitlines():
        if line.startswith("## "):
            if name:
                out.append((name, rows))
            name, rows = line[3:].strip(), []
        elif name:
            rows.append(line)
    if name:
        out.append((name, rows))
    return out


def last_commit_date(path):
    try:
        r = subprocess.run(
            ["git", "log", "-1", "--format=%cs", "--", path],
            cwd=ROOT, capture_output=True, text=True, check=False)
        return r.stdout.strip() or None
    except OSError:
        return None


def code_corpus():
    text = []
    for top in ("src", "examples"):
        for d, _, fs in os.walk(os.path.join(ROOT, top)):
            for f in fs:
                if f.endswith(".rs"):
                    with open(os.path.join(d, f), encoding="utf-8") as h:
                        text.append(h.read())
    for d, _, fs in os.walk(os.path.join(ROOT, "Reports")):
        for f in fs:
            if f.endswith((".py", ".sh")):
                with open(os.path.join(d, f), encoding="utf-8", errors="replace") as h:
                    text.append(h.read())
    return "\n".join(text)


def main():
    errors, warnings = [], []
    corpus = code_corpus()
    all_pages = list(pages())
    index = os.path.join(WIKI, "index.md")
    linked_from_index = set()
    cited = set()

    for p in all_pages:
        rel = os.path.relpath(p, ROOT)
        with open(p, encoding="utf-8") as h:
            text = h.read()
        # Links (code spans and fenced blocks are not links).
        body = re.sub(r"```.*?```", "", text, flags=re.S)
        body = re.sub(r"`[^`\n]*`", "", body)
        for target in LINK.findall(body):
            if re.match(r"[a-z]+:", target) or target.startswith("#"):
                continue
            path = target.split("#", 1)[0]
            full = os.path.normpath(os.path.join(os.path.dirname(p), path))
            if not os.path.exists(full):
                errors.append(f"{rel}: broken link {target}")
                continue
            if p == index:
                linked_from_index.add(full)
            if full.startswith(os.path.join(ROOT, "Reports")):
                cited.add(full)

        if p in (index, os.path.join(WIKI, "README.md")):
            continue
        fm = frontmatter(text)
        for k in REQUIRED:
            if k not in fm:
                errors.append(f"{rel}: frontmatter has no `{k}`")
        for sym in fm.get("code", []) or []:
            name = re.split(r"::|\.", sym)[-1]
            if not re.search(r"\b" + re.escape(name) + r"\b", corpus):
                errors.append(f"{rel}: code symbol `{sym}` not found in src/ or examples/")
        for section, rows in sections(text):
            if section == "Numbers":
                for r in rows:
                    if r.startswith("|") and not re.match(r"\|[-| ]+\|$", r) \
                            and "](" not in r and "same" not in r \
                            and not r.startswith("| Measure") and not r.startswith("| Hand"):
                        errors.append(f"{rel}: Numbers row has no source link: {r[:60]}")
            if section == "Owner's word":
                for r in rows:
                    if r.startswith("- ") and not re.match(r"- (\*\*)?(20\d\d-\d\d-\d\d|None)", r):
                        errors.append(f"{rel}: owner quote without a date: {r[:60]}")
        updated = fm.get("updated")
        for s in fm.get("sources", []) or []:
            sp = os.path.normpath(os.path.join(ROOT, s))
            if not os.path.exists(sp):
                errors.append(f"{rel}: source {s} does not exist")
                continue
            cited.add(sp)
            d = last_commit_date(sp)
            if updated and d and d > updated:
                warnings.append(f"{rel}: updated {updated}, but {s} changed {d}")

    for p in all_pages:
        if p in (index,):
            continue
        if p not in linked_from_index:
            errors.append(f"{os.path.relpath(p, ROOT)}: not linked from wiki/index.md")

    with open(index, encoding="utf-8") as h:
        n = sum(1 for _ in h)
    if n > INDEX_CAP:
        errors.append(f"wiki/index.md: {n} lines, over the {INDEX_CAP}-line cap; "
                      "split it per folder (wiki/README.md, *Growth*)")

    # HANDOFF.md is the front page: every dish, tool and mechanism page must
    # be linked from it, so a new page cannot be missing from the summary.
    with open(os.path.join(ROOT, "HANDOFF.md"), encoding="utf-8") as h:
        handoff = h.read()
    for p in all_pages:
        with open(p, encoding="utf-8") as h:
            kind = frontmatter(h.read()).get("kind")
        rel = os.path.relpath(p, ROOT)
        if kind in ("dish", "tool", "mechanism") and f"]({rel})" not in handoff:
            errors.append(f"{rel}: not linked from HANDOFF.md's status section")

    reports = os.path.join(ROOT, "Reports")
    for f in sorted(os.listdir(reports)):
        full = os.path.join(reports, f)
        if f.endswith(".md") and f not in NOT_SOURCES and full not in cited:
            if f in NOT_YET_INGESTED:
                warnings.append(f"Reports/{f}: no wiki page cites it yet (predates the wiki)")
            else:
                errors.append(f"Reports/{f}: new report that no wiki page cites; ingest it")

    for w in warnings:
        print("warning:", w)
    for e in errors:
        print("error:", e)
    print(f"{len(all_pages)} pages, {len(errors)} errors, {len(warnings)} warnings")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
