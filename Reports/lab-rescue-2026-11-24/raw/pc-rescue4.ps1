# LAB (never merges): the rescue planet, round two, third batch (after
# pc-rescue3.ps1), on Scott's PC; same build as a7b9204e.
# Planet 5's wounded died on every march home (7 of 7), mostly beside help.
# Two softer arms on planet 5, two dug-outs, rings 3 thick, wound 0.5:
#   N: no alarm (strikers not forced), reek 3;  Q: no alarm, reek 1.
# Hands greedy, rescue@60, rescue@150, rescue@240. Plus planet 4's greedy hand
# with no rescue at all (did the Swarmer wipe it before 1:00 anyway?).
# 9 games, each stopped by 10:00.
$ErrorActionPreference = "Continue"
$out = "C:\Users\Scott\Code\pc-batch-out\lab-rescue4-20261124"
New-Item -ItemType Directory -Force -Path $out | Out-Null
$games = @(@{n="P4_none_greedy"; p=4; a=$null; b=$null; h="greedy"})
foreach ($arm in @(@{n="N"; a="536,278,3,300,3,0,7,0.5,3"}, @{n="Q"; a="536,278,3,300,1,0,7,0.5,3"})) {
  foreach ($h in @("greedy", "rescue@60", "rescue@150", "rescue@240")) {
    $games += @{n="P5_$($arm.n)_$($h -replace '@', '')"; p=5; a=$arm.a; b="463,151"; h=$h}
  }
}
foreach ($g in $games) {
  $dir = Join-Path $out $g.n
  New-Item -ItemType Directory -Force -Path $dir | Out-Null
  if ($g.a) { $env:LAB_RESCUE = $g.a } else { Remove-Item Env:LAB_RESCUE -ErrorAction SilentlyContinue }
  if ($g.b) { $env:LAB_RESCUE_MORE = $g.b } else { Remove-Item Env:LAB_RESCUE_MORE -ErrorAction SilentlyContinue }
  $t0 = Get-Date
  & .\target\release\larder-world.exe "look=$dir" "run=1:$($g.p)" mode=squads "hand=$($g.h)" at=30,60,120,180,300,420,600 *> (Join-Path $dir "log.txt")
  $log = Get-Content (Join-Path $dir "log.txt")
  $res = ($log | Select-String -Pattern 'lab rescue t' | ForEach-Object { $_.Line.Trim() }) -join "`n    "
  $st = ($log | Select-String -Pattern '^\s*t \d+ s: mould' | ForEach-Object { ($_.Line.Trim() -replace ', rendered \d+', '' -replace ', took \d+%', '' -replace '; squad at.*', '') }) -join "`n    "
  "== $($g.n) ($([int]((Get-Date) - $t0).TotalSeconds) s wall)`n    $res`n    $st"
}
Remove-Item Env:LAB_RESCUE, Env:LAB_RESCUE_MORE -ErrorAction SilentlyContinue
