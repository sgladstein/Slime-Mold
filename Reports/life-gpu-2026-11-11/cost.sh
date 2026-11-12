#!/bin/sh
# What the life cycle costs a frame on the GPU (2026-11-11): the sandbox's
# default dish (100,000 walkers, one a cell, food), a frame of two ticks,
# headless, 5 s a run, three rounds alternating. BASE is mouldgpu built
# from the species port (PR #82) before the life cycle, NEW this branch's.
# Pairs:
#   plain: no outside rule (the plain pipelines), BASE against NEW;
#   forced: the outside rules on with nothing kept (blocked_keep with a
#     reset every tick, as compiling_the_outside_rules_out does), BASE
#     against NEW (the life branches, off);
#   then NEW, forced, with spores, with dormancy and the alarm, and with
#     both: these change the dish (spores fly, the dormant do not move), so
#     the passes' own times come from the per-pass lines below.
# From the repo root. Output on the software GPU: raw/cost.txt.
B=${BASE:?set BASE to mouldgpu built from the species port}
N=${NEW:-target/release/mouldgpu}
D=${OUT:-/tmp/life-cost}
F="blocked_keep=1 blocked_reset=1"
S="spore_rate=0.01 spore_scent=-1 spore_steps=20 spore_reach=8 spore_land=1 spore_deposit=0.2 spore_rest=10"
Z="sleep_rate=0.05 wake_scent=50 sleep_deposit=0.3 alarm_keep=0.95 alarm_relay=20 wake_alarm=1 alarm_weight=2 sleep_rest=10 wake_steps=10"
for r in 1 2 3; do
  echo "== round $r"
  echo "plain base:     $($B look=$D seconds=5 tag=c | grep '^wall ms')"
  echo "plain new:      $($N look=$D seconds=5 tag=c | grep '^wall ms')"
  echo "forced base:    $($B look=$D seconds=5 tag=c $F | grep '^wall ms')"
  echo "forced new:     $($N look=$D seconds=5 tag=c $F | grep '^wall ms')"
  echo "spores:         $($N look=$D seconds=5 tag=c $F $S | grep '^wall ms')"
  echo "dormancy:       $($N look=$D seconds=5 tag=c $F $Z | grep '^wall ms')"
  echo "both:           $($N look=$D seconds=5 tag=c $F $S $Z | grep '^wall ms')"
done
# The passes, each tick submitted alone.
echo "== passes"
echo "forced: $($N look=$D seconds=2 tag=c $F | grep 'a pass')"
echo "both:   $($N look=$D seconds=2 tag=c $F $S $Z | grep 'a pass')"
echo "== states at the end"
$N look=$D seconds=5 tag=c $F $S $Z | grep '^life'
