# LAB (never merges): the rescue planet, round three (Direction 2026-11-24
# 23:57Z: make it possible on planets 4 and 5), on Scott's PC. Run from a clone
# of claude/project-thread-2o4cic after cargo build --release --locked --bin
# larder-world.
# New: the careful hand rescuetight@T[,FARM[,FIRE]]: stands at the landing on
# Defend (or drains the crystal nearest the larder within FARM cells) until
# game second T, then all rescuers walk as one merged blob by one squad order
# on hold fire (FIRE 1: Defend, the control) to the dug-out; free fire after
# the first dug-out is reached. Planet 4 keeps round two's dug-outs; planet 5's
# move to the body's west edge (403,312 and 420,150; rings build 8 of 8 on
# lavapipe) with the alarm kept on. Every arm: rings 3 thick, wound 0.5.
# 32 games, each stopped by 10:00. -Planets 4 (or 5) runs one planet.
param([int[]]$Planets = @(4, 5))
$ErrorActionPreference = "Continue"
$out = "C:\Users\Scott\Code\pc-batch-out\lab-rescue5-20261125"
New-Item -ItemType Directory -Force -Path $out | Out-Null
# X,Y,N,HIDE,OUT,ALARM,RING,WOUND,THICK for dug-out A; LAB_RESCUE_MORE for B.
$arms = @{
  4 = @(@{n="one"; a="209,290,3,300,3,1,7,0.5,3"; b=$null}, @{n="two"; a="209,290,3,300,3,1,7,0.5,3"; b="60,185"});
  5 = @(@{n="edge1"; a="403,312,3,300,3,1,7,0.5,3"; b=$null}, @{n="edge2"; a="403,312,3,300,3,1,7,0.5,3"; b="420,150"})
}
$hands = @("idle", "greedy", "rescue@60", "rescuetight@60", "rescuetight@60,0,1", "rescuetight@60,40", "rescuetight@120", "rescuetight@180")
foreach ($p in $Planets) {
  foreach ($arm in $arms[$p]) {
    foreach ($h in $hands) {
      $n = "P$($p)_$($arm.n)_$($h -replace '[@,]', '_')"
      $dir = Join-Path $out $n
      New-Item -ItemType Directory -Force -Path $dir | Out-Null
      $env:LAB_RESCUE = $arm.a
      if ($arm.b) { $env:LAB_RESCUE_MORE = $arm.b } else { Remove-Item Env:LAB_RESCUE_MORE -ErrorAction SilentlyContinue }
      $t0 = Get-Date
      & .\target\release\larder-world.exe "look=$dir" "run=1:$p" mode=squads "hand=$h" at=60,180,300,420,600 *> (Join-Path $dir "log.txt")
      $log = Get-Content (Join-Path $dir "log.txt")
      $res = ($log | Select-String -Pattern 'lab rescue t|lab rescuetight' | ForEach-Object { $_.Line.Trim() }) -join "`n    "
      $end = ($log | Select-String -Pattern '^\s*t \d+ s: mould' | Select-Object -Last 1).Line
      if ($end) { $end = ($end.Trim() -replace ', rendered \d+', '' -replace ', took \d+%', '' -replace '; squad at.*', '') }
      "== $n ($([int]((Get-Date) - $t0).TotalSeconds) s wall)`n    $res`n    last: $end"
    }
  }
}
Remove-Item Env:LAB_RESCUE, Env:LAB_RESCUE_MORE -ErrorAction SilentlyContinue
