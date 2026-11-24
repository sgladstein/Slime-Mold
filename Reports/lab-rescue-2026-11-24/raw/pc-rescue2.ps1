# LAB (never merges): the rescue planet, round two (Direction 2026-11-24 19:37Z),
# on Scott's PC. Run from a clone of claude/project-thread-2o4cic after
#   cargo build --release --locked --bin larder-world
# Not a timing run of the frame; the measure is game time to the win or loss.
# Planets 3, 4 and 5, run seed 1, defaults (3 stranded a dug-out, hidden 5:00,
# ring of doors 7 cells, wounded 0.3, heart cannot break). Each planet twice,
# paired: one dug-out (A) and two (A then B), five hands each: idle, greedy,
# and rescue@T (greedy until T, then everyone to the dug-outs and home) at
# T = 60, 150, 240. 30 games; every game stops by 10:00 (the last picture).
# Pass -Planets 3 (or 4, 5) to run one planet's ten.
param([int[]]$Planets = @(3, 4, 5))
$ErrorActionPreference = "Continue"
$out = "C:\Users\Scott\Code\pc-batch-out\lab-rescue2-20261124"
New-Item -ItemType Directory -Force -Path $out | Out-Null
$spots = @{ 3 = @("154,102", "250,70"); 4 = @("209,290", "60,185"); 5 = @("536,278", "463,151") }
$hands = @("idle", "greedy", "rescue@60", "rescue@150", "rescue@240")
foreach ($p in $Planets) {
  foreach ($two in @($false, $true)) {
    foreach ($h in $hands) {
      $n = "P$($p)_$(if ($two) { 'two' } else { 'one' })_$($h -replace '@', '')"
      $dir = Join-Path $out $n
      New-Item -ItemType Directory -Force -Path $dir | Out-Null
      $env:LAB_RESCUE = $spots[$p][0]
      if ($two) { $env:LAB_RESCUE_MORE = $spots[$p][1] } else { Remove-Item Env:LAB_RESCUE_MORE -ErrorAction SilentlyContinue }
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
