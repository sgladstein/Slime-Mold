# LAB (never merges): the rescue planet, round two, second batch (after
# pc-rescue2.ps1), on Scott's PC. Run from a clone of claude/project-thread-2o4cic
# after cargo build --release --locked --bin larder-world.
# Changes from the first batch: planet 3's second dug-out moved off the greedy
# hand's farm (250,70 was reached by chance at 0:13) to 42,139; planets 4 and 5
# get rings 3 levels thick and a wounded pace of 0.5 (the first batch: rings
# fell at about 2:00 on planet 4, and every march home on planet 5 lost them).
# Same five hands; planet 3 two dug-outs only, planets 4 and 5 one and two.
# 25 games, each stopped by 10:00. -Planets 3 (or 4, 5) runs one planet.
param([int[]]$Planets = @(3, 4, 5))
$ErrorActionPreference = "Continue"
$out = "C:\Users\Scott\Code\pc-batch-out\lab-rescue3-20261124"
New-Item -ItemType Directory -Force -Path $out | Out-Null
# X,Y,N,HIDE,OUT,ALARM,RING,WOUND,THICK for dug-out A; LAB_RESCUE_MORE for B.
$arms = @{
  3 = @(@{n="two"; a="154,102,3,300,3,1,7,0.3,1"; b="42,139"});
  4 = @(@{n="one"; a="209,290,3,300,3,1,7,0.5,3"; b=$null}, @{n="two"; a="209,290,3,300,3,1,7,0.5,3"; b="60,185"});
  5 = @(@{n="one"; a="536,278,3,300,3,1,7,0.5,3"; b=$null}, @{n="two"; a="536,278,3,300,3,1,7,0.5,3"; b="463,151"})
}
$hands = @("idle", "greedy", "rescue@60", "rescue@150", "rescue@240")
foreach ($p in $Planets) {
  foreach ($arm in $arms[$p]) {
    foreach ($h in $hands) {
      $n = "P$($p)_$($arm.n)_$($h -replace '@', '')"
      $dir = Join-Path $out $n
      New-Item -ItemType Directory -Force -Path $dir | Out-Null
      $env:LAB_RESCUE = $arm.a
      if ($arm.b) { $env:LAB_RESCUE_MORE = $arm.b } else { Remove-Item Env:LAB_RESCUE_MORE -ErrorAction SilentlyContinue }
      $t0 = Get-Date
      & .\target\release\larder-world.exe "look=$dir" "run=1:$p" mode=squads "hand=$h" at=60,180,300,420,600 *> (Join-Path $dir "log.txt")
      $log = Get-Content (Join-Path $dir "log.txt")
      $res = ($log | Select-String -Pattern 'lab rescue t' | ForEach-Object { $_.Line.Trim() }) -join "`n    "
      $end = ($log | Select-String -Pattern '^\s*t \d+ s: mould' | Select-Object -Last 1).Line
      if ($end) { $end = ($end.Trim() -replace ', rendered \d+', '' -replace ', took \d+%', '' -replace '; squad at.*', '') }
      "== $n ($([int]((Get-Date) - $t0).TotalSeconds) s wall)`n    $res`n    last: $end"
    }
  }
}
Remove-Item Env:LAB_RESCUE, Env:LAB_RESCUE_MORE -ErrorAction SilentlyContinue
