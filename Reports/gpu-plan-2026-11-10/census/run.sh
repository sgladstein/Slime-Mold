#!/usr/bin/env bash
# The GPU plan's phase 1 arms: every scored dish, paired seeds, each arm run
# against today's rules. One process per seed, arm and dish group, 4 at a time.
#   cargo build --release --example mouldprobe
#   Reports/gpu-plan-2026-11-10/census/run.sh 16 today late together fixed gpu
# Arms:
#   today     the game
#   late      the census every 30 steps (4 a second), landing 4 steps (two
#             frames) late
#   together  walkers move all at once (`Params::all_at_once`)
#   fixed     positions, headings, trail and scent on fixed point
#             (`Params::fixed_point`)
#   gpu       all three: together, fixed and late, as the GPU design runs
set -euo pipefail
cd "$(dirname "$0")/../../.."
out=Reports/gpu-plan-2026-11-10/census/raw
mkdir -p "$out"
probe=./target/release/examples/mouldprobe
n="${1:-16}"
shift || true
[ $# -eq 0 ] && set -- today late
jobs=()
for seed in $(seq 1 "$n"); do
  for arm in "$@"; do
    case "$arm" in
      today) extra="" ;;
      late) extra="census_every=30 census_lag=4" ;;
      together) extra="all_at_once=1" ;;
      fixed) extra="fixed_point=1" ;;
      gpu) extra="all_at_once=1 fixed_point=1 census_every=30 census_lag=4" ;;
      *) echo "unknown arm $arm" >&2; exit 1 ;;
    esac
    jobs+=("$probe level=1,2,3 seeds=$seed $extra > $out/$arm-puzzles-$seed.txt")
    jobs+=("$probe level=4 players=idle,random,plan,fresh,intended,sleeper seeds=$seed $extra > $out/$arm-alive-$seed.txt")
    jobs+=("$probe level=5 players=idle,random,plan,fresh,intended,warden seeds=$seed $extra > $out/$arm-wild-$seed.txt")
  done
done
printf '%s\n' "${jobs[@]}" | xargs -P 4 -I{} bash -c '{}'
