#!/usr/bin/env python3
"""Mechanical checks for the mould game's wiki (wiki/).

    python3 wiki/lint.py            # from the repo root; exit 1 on any error
    python3 wiki/lint.py --base=SHA # also: every page whose source report
                                    # changed since SHA must change too (CI
                                    # does this on PRs via GITHUB_BASE_REF)

Errors (fail): a relative link that does not resolve; a page that
`wiki/index.md` does not link; a `code:` symbol in a page's frontmatter that
no longer appears anywhere under `src/`, `examples/` or the scripts kept in
`Reports/`; a page missing a required frontmatter key; a line under
"Owner's word" that
does not start with a date; `index.md` over 150 lines; a dish, tool or
mechanism page that `HANDOFF.md` does not link.

A top-level report under `Reports/` that no page cites is an error too,
unless it is listed in `NOT_YET_INGESTED` (reports from before the wiki).
So is a note (.md) inside a report folder that cannot be reached by links
from the folder's parent report or a wiki page, and a folder of notes with
no parent; folders in `PRE_WIKI_FOLDERS` only warn.

Warnings (print only): a page with a Numbers table, a page over 12 KB, a Now over 2 KB, an index line
over 200 characters; a page whose `updated` date (or UTC time, such as
2026-11-11T15:30Z) is older than the last commit to one of its `sources`; a
page that links a report its `sources` do not list; a listed pre-wiki report
still uncited.

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
NOT_YET_INGESTED = set()  # rts-design-2026-11-10.md: cited by decisions.md since 2026-11-10
# Report folders from before the wiki whose notes are not all reached from
# a cited parent. They only warn. A new folder must have a parent (Reports/
# <folder>.md, or <folder>/README.md cited by a page) that links every note
# in it, so a sub-agent's full notes are always one hop from the wiki.
PRE_WIKI_FOLDERS = {
    "mould-game-2026-11-08",  # archive with its own README; no page cites the README yet
    "new-game-brainstorm-2026-11-07",  # background, linked from Reports/README.md
}
INDEX_CAP = 150
# Size caps, in bytes, since lines vary from 60 to 1,000+ characters (a
# Numbers row is one line): the owner asked whether 300 lines was right
# (2026-11-11 15:16Z), and a 255-line page was then 27 KB, about 7k tokens.
PAGE_CAP = 12_000      # a page past this splits (warning)
NOW_CAP = 2_000        # a page's Now past this is being appended to (warning)
INDEX_LINE_CAP = 200   # one index line is one sentence (warning)
STATUSES = {"current", "open", "decided", "paused", "retired"}

LINK = re.compile(r"\[[^\]]*\]\(([^)\s]+)\)")


def pages():
    for d, _, fs in os.walk(WIKI):
        for f in sorted(fs):
            if f.endswith(".md") and not f.startswith("_"):
                yield os.path.join(d, f)


def frontmatter(text):
    """The `---` block as a dict. `sources:` and `code:` are YAML block
    lists, one `  - entry` a line (inline `[a, b]` still parses, so lint can
    report it); the key `_inline` lists keys written inline."""
    if not text.startswith("---\n"):
        return {}
    end = text.find("\n---\n", 4)
    if end < 0:
        return {}
    out = {"_inline": []}
    key = None
    for line in text[4:end].splitlines():
        if line.startswith("  - ") and key is not None:
            if not isinstance(out.get(key), list):
                out[key] = []
            out[key].append(line[4:].strip().strip("`"))
            continue
        if ":" not in line or line.startswith(" "):
            continue
        k, v = line.split(":", 1)
        key = k.strip()
        v = v.strip()
        if v.startswith("[") and v.endswith("]"):
            v = [x.strip().strip("`") for x in v[1:-1].split(",") if x.strip()]
            out["_inline"].append(key)
        out[key] = v
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
    """The last commit to `path` as a UTC `YYYY-MM-DDTHH:MMZ` string."""
    try:
        r = subprocess.run(
            ["git", "log", "-1", "--date=format-local:%Y-%m-%dT%H:%MZ",
             "--format=%cd", "--", path],
            env=dict(os.environ, TZ="UTC"),
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
        # Merge-friendly frontmatter (integration thread with the technical
        # lead, 2026-11-12 21:35Z): `updated` carried minutes and `sources:`
        # was one line, so any two PRs touching a page conflicted there, and
        # green PRs waited a CI cycle each (#143 three times). A date-only
        # `updated` and sorted one-per-line lists merge cleanly; the
        # same-commit rule below still catches a page left behind its source,
        # which the same-day time check used to.
        upd = fm.get("updated", "")
        if upd and not re.fullmatch(r"\d{4}-\d{2}-\d{2}", upd):
            errors.append(f"{rel}: `updated: {upd}` must be a date, YYYY-MM-DD")
        for k in ("sources", "code"):
            if k in fm.get("_inline", []):
                errors.append(f"{rel}: `{k}:` must be a block list, one `  - entry` a line")
            v = fm.get(k)
            if isinstance(v, list) and v != sorted(v):
                errors.append(f"{rel}: `{k}:` entries must be sorted")
        for sym in fm.get("code", []) or []:
            name = re.split(r"::|\.", sym)[-1]
            if not re.search(r"\b" + re.escape(name) + r"\b", corpus):
                errors.append(f"{rel}: code symbol `{sym}` not found in src/ or examples/")
        for section, rows in sections(text):
            if section == "Numbers":
                # Owner's card, 2026-11-11 15:28Z: short pages, the numbers
                # stay in the reports. A warning while open PRs still add one.
                warnings.append(f"{rel}: has a Numbers table; pages keep numbers in their "
                                "reports (wiki/README.md rule 2)")
            if section == "Owner's word":
                for r in rows:
                    if r.startswith("- ") and not re.match(r"- (\*\*)?(20\d\d-\d\d-\d\d|None)", r):
                        errors.append(f"{rel}: owner quote without a date: {r[:60]}")
        if fm.get("status") and fm["status"] not in STATUSES:
            errors.append(f"{rel}: status `{fm['status']}` is not one of {sorted(STATUSES)}")
        # The decision logs only grow; they split by subject, not by this cap.
        if len(text.encode()) > PAGE_CAP and not os.path.basename(p).startswith("decisions"):
            warnings.append(f"{rel}: {len(text.encode()) // 1000} KB, over the "
                            f"{PAGE_CAP // 1000} KB cap; split it (wiki/README.md, *Growth*)")
        for section, rows in sections(text):
            size = len("\n".join(rows).strip().encode())
            if section == "Now" and size > NOW_CAP:
                warnings.append(f"{rel}: Now is {size // 100 / 10} KB, over {NOW_CAP // 1000} KB; "
                                "rewrite it short and move detail to a section or sub-page")
        updated = fm.get("updated")
        for s in fm.get("sources", []) or []:
            sp = os.path.normpath(os.path.join(ROOT, s))
            if not os.path.exists(sp):
                errors.append(f"{rel}: source {s} does not exist")
                continue
            cited.add(sp)
            d = last_commit_date(sp)
            # `updated` may be a date or a UTC time (2026-11-11T15:30Z). A
            # date compares by day, so a source changed later the same day
            # does not warn; on a busy day write the time (added 2026-11-11,
            # when ten reports a day made the day check miss same-day edits).
            if updated and d and d[:len(updated)] > updated:
                warnings.append(f"{rel}: updated {updated}, but {s} changed {d[:len(updated)]}")
        # A report linked in the body but missing from `sources:` escapes
        # the check above, so the page can go stale without a warning.
        # decisions.md is a log of rulings whose links are pointers, not a
        # summary of those reports, so it is left out.
        srcs = {os.path.normpath(os.path.join(ROOT, s)) for s in fm.get("sources", []) or []}
        for target in sorted(set(LINK.findall(body))) if not os.path.basename(p).startswith("decisions") else []:
            path = os.path.normpath(os.path.join(os.path.dirname(p), target.split("#", 1)[0]))
            if os.path.dirname(path) == os.path.join(ROOT, "Reports") and path.endswith(".md") \
                    and os.path.basename(path) not in NOT_SOURCES and path not in srcs \
                    and os.path.exists(path):
                warnings.append(f"{rel}: links {os.path.relpath(path, ROOT)} but `sources:` does not list it")

    for p in all_pages:
        if p in (index,):
            continue
        if p not in linked_from_index:
            errors.append(f"{os.path.relpath(p, ROOT)}: not linked from wiki/index.md")

    with open(index, encoding="utf-8") as h:
        n = sum(1 for _ in h)
    with open(index, encoding="utf-8") as h:
        for i, line in enumerate(h, 1):
            if line.startswith("- [") and len(line.rstrip()) > INDEX_LINE_CAP:
                warnings.append(f"wiki/index.md:{i}: {len(line.rstrip())} characters, over "
                                f"{INDEX_LINE_CAP}; one sentence an entry")
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

    # Notes inside report folders (a sub-agent's full notes, a brainstorm, a
    # review). They are not each ingested: the parent report summarises them
    # and a page cites the parent. Lint checks they are reachable: every .md
    # in Reports/<folder>/ is linked, directly or through notes the parent
    # links, from the parent or a wiki page (a link to a sub-folder covers
    # what is in it). Owner's question, 2026-11-10 18:48Z.
    for d in sorted(os.listdir(reports)):
        folder = os.path.join(reports, d)
        if not os.path.isdir(folder):
            continue
        notes = []
        for dirpath, _, files in os.walk(folder):
            notes += [os.path.join(dirpath, f) for f in files if f.endswith(".md")]
        if not notes:
            continue
        top, readme = folder + ".md", os.path.join(folder, "README.md")
        parent = top if os.path.exists(top) else readme if os.path.exists(readme) else None
        say = warnings.append if d in PRE_WIKI_FOLDERS else errors.append
        if parent is None:
            say(f"Reports/{d}/: notes but no parent report (Reports/{d}.md or {d}/README.md)")
            continue
        if parent == readme and readme not in cited:
            say(f"Reports/{d}/README.md: no wiki page cites it")
        reached, covered = set(), set()
        todo = [parent] + sorted(cited & set(notes))
        while todo:
            f = todo.pop()
            if f in reached:
                continue
            reached.add(f)
            with open(f, encoding="utf-8", errors="ignore") as h:
                for target in LINK.findall(h.read()):
                    if re.match(r"[a-z]+:", target) or target.startswith("#"):
                        continue
                    full = os.path.normpath(os.path.join(os.path.dirname(f), target.split("#", 1)[0]))
                    if full in notes:
                        todo.append(full)
                    elif os.path.isdir(full) and full.startswith(folder + os.sep):
                        covered.add(full.rstrip(os.sep))
        missing = []
        for n in notes:
            if n == parent:
                continue
            if n not in reached and not any(n.startswith(c + os.sep) for c in covered):
                missing.append(n)
        if missing:
            say(f"Reports/{d}/: {len(missing)} note(s) no parent or page links, e.g. "
                f"{os.path.relpath(missing[0], ROOT)}")

    # Same-commit rule (README rule 9), enforced on a PR: a report the PR
    # changes must come with a change to every page that lists it in
    # `sources:`. The age warnings above never failed CI, and on 2026-11-11
    # four pages had drifted behind reports changed hours earlier; the owner
    # asked how to stop that recurring (15:18Z). A change that leaves a page
    # true still bumps its `updated`, which says someone checked.
    base = next((a.split("=", 1)[1] for a in sys.argv[1:] if a.startswith("--base=")), None)
    # In CI, GitHub Actions sets GITHUB_BASE_REF on pull requests, and the
    # lint step checks out full history, so the check runs there without a
    # workflow change (one would make every wiki PR run the build jobs).
    if base is None and os.environ.get("GITHUB_BASE_REF"):
        base = "origin/" + os.environ["GITHUB_BASE_REF"]
    if base:
        r = subprocess.run(["git", "diff", "--name-only", base + "...HEAD"],
                           cwd=ROOT, capture_output=True, text=True, check=False)
        changed = {os.path.normpath(os.path.join(ROOT, f)) for f in r.stdout.split()}
        for p in all_pages:
            if p in changed:
                continue
            with open(p, encoding="utf-8") as h:
                fm = frontmatter(h.read())
            for s in fm.get("sources", []) or []:
                sp = os.path.normpath(os.path.join(ROOT, s))
                if sp in changed and sp.startswith(os.path.join(ROOT, "Reports")):
                    errors.append(f"{os.path.relpath(p, ROOT)}: this PR changes {s}, a source "
                                  "of this page, but not the page; ingest it, or bump `updated` "
                                  "if the page is still true")

    for w in warnings:
        print("warning:", w)
    for e in errors:
        print("error:", e)
    print(f"{len(all_pages)} pages, {len(errors)} errors, {len(warnings)} warnings")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
