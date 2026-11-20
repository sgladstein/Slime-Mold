#!/bin/sh
# Reads a Strategy league run's games from its job logs (the LEAGUEROW
# lines) into OUT, one JSON line a game: fetch.sh RUN_ID OUT
set -e
run=$1; out=$2
: > "$out"
for id in $(gh api "repos/sgladstein/Slime-Mold/actions/runs/$run/jobs?per_page=50" --jq '.jobs[] | select(.status=="completed") | .id'); do
  gh api "repos/sgladstein/Slime-Mold/actions/jobs/$id/logs" | grep -a "LEAGUEROW " | sed 's/^.*LEAGUEROW //' >> "$out" || true
done
wc -l "$out"
