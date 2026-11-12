#!/usr/bin/env bash
# The tier census: each new tier against the hands, on the race world under
# the Larder rules of PR 1 (free shots; the flee from gunfire is opt-in, not
# the default, since it failed the gate), held-out seeds. A first run at
# 8ec3e81 had the flee on by default and is void (random beat every plan on
# the Trunk, the Larder thread's flee failure).
#   bash Reports/mould-tiers-campaign-2026-11-12/census.sh TIER [OUT] [SEEDS] [THREADS]
# TIER is none (the plain mould), drifter, sprawler, trunk, wary or grudge.
# From the repo root after cargo build --release --example larderworld.
#
# Deciding measures, named before the run (seeds 201-216, this thread's
# held-out block, never used before; dev seeds 1-4):
#  C1 the gate, per tier: every plan beats idle and random on paired seeds
#     (larderworld's own sign tests, Holm over its comparisons).
#  C2 the payoff table: each plan's mean score v2 by tier, and which plan
#     is best on the most seeds. The designer's bar: the best plan changes
#     between tiers.
#  C3 against the plain mould, per tier: idle's end time (the Drifter must
#     still wipe an idle squad, by about 660 s, the campaign's 11:00) and
#     the best plan's score, paired by seed, sign test.
set -u
TIER=${1:?tier}; OUT=${2:-Reports/mould-tiers-campaign-2026-11-12/census}; SEEDS=${3:-201-216}; THREADS=${4:-4}
mkdir -p "$OUT"
T=""; [ "$TIER" != none ] && T="tier=$TIER"
./target/release/examples/larderworld census seeds=$SEEDS threads=$THREADS hands=idle,random,farm,wall,boom,claim,fort,strike $T > "$OUT/$TIER-$SEEDS.txt" 2>&1
# One short row a game for the runners' messages: hand seed score end time soldiers mould.
grep ' seed ' "$OUT/$TIER-$SEEDS.txt" | sed -E 's/^ *([a-z:=+0-9_]+) seed +([0-9]+): +(.*) at +([0-9]+) s, score ([-+.0-9]+); soldiers +([0-9]+), mould +([0-9]+).*/\1 \2 \5 \3 \4 \6 \7/' > "$OUT/$TIER-$SEEDS.rows"
