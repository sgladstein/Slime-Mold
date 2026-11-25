# LAB (never merges): the rescue planet, round three, second batch (after
# pc-rescue5.ps1), on Scott's PC. Run from a clone of claude/project-thread-2o4cic
# after cargo build --release --locked --bin larder-world.
# pc-rescue5: the careful hand reached every dug-out on planets 4 and 5 by 1:30
# on Defend or free fire, then every march home died 6 to 20 s after the reach
# to the alarm (strikers forced on for good). New: ALARM above 1 forces them
# for that many seconds, then the planet's own come back; LAB_RESCUE_HOME=X,Y
# takes the found out there instead of at the larder (a shorter way home).
# Arms keep planet 4's dug-outs and planet 5's edge ones, rings 3 thick.
# 28 games, each stopped by 10:00. -Planets 4 (or 5) runs one planet.
param([int[]]$Planets = @(4, 5))
$ErrorActionPreference = "Continue"
$out = "C:\Users\Scott\Code\pc-batch-out\lab-rescue6-20261125"
New-Item -ItemType Directory -Force -Path $out | Out-Null
# X,Y,N,HIDE,OUT,ALARM,RING,WOUND,THICK for dug-out A; b: LAB_RESCUE_MORE; h: LAB_RESCUE_HOME.
$arms = @{
  4 = @(@{n="one_a20"; a="209,290,3,300,3,20,7,0.5,3"; b=$null; h=$null},
        @{n="one_a40"; a="209,290,3,300,3,40,7,0.5,3"; b=$null; h=$null},
        @{n="two_a20"; a="209,290,3,300,3,20,7,0.5,3"; b="60,185"; h=$null});
  5 = @(@{n="edge1_a20"; a="403,312,3,300,3,20,7,0.5,3"; b=$null; h=$null},
        @{n="edge1_a40"; a="403,312,3,300,3,40,7,0.5,3"; b=$null; h=$null},
        @{n="edge2_a20_w03"; a="403,312,3,300,3,20,7,0.3,3"; b="420,150"; h=$null},
        @{n="edge2_a1_home"; a="403,312,3,300,3,1,7,0.5,3"; b="420,150"; h="220,230"})
}
$hands = @("idle", "greedy", "rescuetight@60,0,1", "rescuetight@60,0,2")
foreach ($p in $Planets) {
  foreach ($arm in $arms[$p]) {
    foreach ($h in $hands) {
      $n = "P$($p)_$($arm.n)_$($h -replace '[@,]', '_')"
      $dir = Join-Path $out $n
      New-Item -ItemType Directory -Force -Path $dir | Out-Null
      $env:LAB_RESCUE = $arm.a
      if ($arm.b) { $env:LAB_RESCUE_MORE = $arm.b } else { Remove-Item Env:LAB_RESCUE_MORE -ErrorAction SilentlyContinue }
      if ($arm.h) { $env:LAB_RESCUE_HOME = $arm.h } else { Remove-Item Env:LAB_RESCUE_HOME -ErrorAction SilentlyContinue }
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
Remove-Item Env:LAB_RESCUE, Env:LAB_RESCUE_MORE, Env:LAB_RESCUE_HOME -ErrorAction SilentlyContinue
