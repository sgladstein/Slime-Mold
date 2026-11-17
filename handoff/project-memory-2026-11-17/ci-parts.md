---
name: ci-parts
description: CI tests run as four parallel matrix jobs since PR #346 (2026-11-15): lib, window, window smoke, rest; tracked-ignored-file check; replays untracked
metadata:
  type: project
  modified: 2026-11-15T12:51:37.846Z
---

PR #346 merged 2026-11-15 12:51Z. `.github/workflows/ci.yml` job `test` is a matrix: `lib` (--lib), `window` (larder-world --skip the_window_plays), `window smoke` (larder-world tests named the_window_plays*), `rest` (other bins + integration tests from cargo metadata, examples, doc tests, clippy). One rust-cache key per part. First run (cold caches) 9.5 min total vs ~13 min serial; lib is the slowest part.

**Why:** serial `cargo test` was ~11 min (lib 230 s, window 196 s, GPU 128 s). Repo is public, so extra runners are free.

**How to apply:** a new larder-world smoke test named `the_window_plays_*` lands in "window smoke"; any other name goes to "window". A new binary is picked up by "rest" automatically. The "what changed" job fails if a tracked file matches .gitignore (`git ls-files -ci --exclude-standard`). play.ps1 sets local replay files aside while pulling. Reports/README.md and Reports/pages.md merge with `merge=union`. larder-world `Args` derives Default: write `Args { ..., ..Default::default() }`. New test the_window_plays_a_run_in_squad_mode_at_depths_3_and_6: planet 6 hand=every ends Stuck at 34 s, 0 shots (stuck rule open item). See [[pr-merging]].
