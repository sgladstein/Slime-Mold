#!/usr/bin/env bash
# Sorts the repository's branches (the Engine and tools lane's job 5).
# Scott's rulings: delete old branches whose work is on main, and leave
# any with unmerged commits (2026-11-17 19:07Z, "7) Yes"); let CI do the
# deleting (decision card, 2026-11-18 13:17Z: "Let CI delete them").
#
# Run by .github/workflows/branches.yml on a full clone. A branch whose tip
# is in main loses nothing when it goes, so it is deleted. Every other
# branch is listed, with the commits main has no copy of (`git cherry`, so
# a commit merged under another hash counts as merged), and left alone.
# Never touched: main, the merge queue's own branches, and the head of an
# open pull request.
#
# Why a workflow: a cloud thread may push only its own branch, so it
# cannot delete another thread's (11-18: HTTP 403 on `git push --delete`
# and on the refs API).
#
# DELETE=false lists without deleting. The lists go to stdout and, in
# Actions, to the run's summary.
set -euo pipefail

R="${GITHUB_REPOSITORY:?}"
DELETE="${DELETE:-true}"
out="${GITHUB_STEP_SUMMARY:-/dev/null}"

git fetch -q --prune origin '+refs/heads/*:refs/remotes/origin/*'
open=$(gh api "repos/$R/pulls?state=open&per_page=100" --paginate --jq '.[].head.ref')

deleted=()
kept=()
rows=()
while IFS='|' read -r ref date subject; do
  b="${ref#origin/}"
  case "$b" in
    origin | main | HEAD | merge-queue/*) continue ;;
  esac
  if grep -qxF "$b" <<<"$open"; then
    kept+=("$b (open pull request)")
    continue
  fi
  if git merge-base --is-ancestor "$ref" origin/main; then
    if [ "$DELETE" != "true" ]; then
      deleted+=("$b (would go)")
    elif git push -q origin --delete "$b"; then
      deleted+=("$b")
    else
      kept+=("$b (in main, but the delete failed)")
    fi
    continue
  fi
  n=$(git cherry origin/main "$ref" | grep -c '^+' || true)
  rows+=("| \`$b\` | $date | $n | ${subject//|/\\|} |")
done < <(git for-each-ref --sort=-committerdate --format='%(refname:short)|%(committerdate:short)|%(subject)' refs/remotes/origin)

{
  echo "## Branches, $(date -u +%Y-%m-%dT%H:%MZ)"
  echo
  echo "**Deleted, tip already in main (${#deleted[@]}):** ${deleted[*]:-none}"
  echo
  echo "**Left alone (${#kept[@]}):** ${kept[*]:-none}"
  echo
  echo "**Not in main (${#rows[@]})**, newest first; the count is the branch's commits main has no copy of:"
  echo
  echo "| Branch | Last commit | Not in main | Its last commit |"
  echo "|---|---|---|---|"
  if [ "${#rows[@]}" -gt 0 ]; then printf '%s\n' "${rows[@]}"; fi
} | tee -a "$out"
