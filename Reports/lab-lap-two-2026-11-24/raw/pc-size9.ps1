# LAB (never merges): planet 9's first size against planet 8, the run bot, on Scott's PC.
# Run from a clone of claude/project-thread-2o4cic after
#   cargo build --release --locked --bin larder-world
# Not a timing run. 12 games: run seeds 1 to 3, planet 8 as it is, and planet 9
# (LAB_LAP2=1: the Drifter with the Sporer's bursts) at three sizes, LAB_LAP2_SIZE
# 0.55, 0.65 and 0.8 (1.0 starts about twice planet 8: 330,000 against 173,000 on seed 1).
# hand=bot, bank=3000 (11 soldiers at landing), to a win, a loss or 10:00.
$ErrorActionPreference = "Continue"  # "Stop" turns the game's stderr into an error under PowerShell 5.1
$out = "C:\Users\Scott\Code\pc-batch-out\lab-size9-20261124"
New-Item -ItemType Directory -Force -Path $out | Out-Null
$spores = @("alarm_keep=0.98", "spore_rate=0.00001", "spore_burst=0.2", "spore_alarm=0.3")
$at = "at=1,60,120,180,240,300,360,420,480,540,600"
foreach ($s in 1..3) {
  foreach ($arm in @("p8", "p9x0.55", "p9x0.65", "p9x0.8")) {
    $name = "s$($s)_$arm"
    $dir = Join-Path $out $name
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    Remove-Item Env:LAB_LAP2 -ErrorAction SilentlyContinue
    Remove-Item Env:LAB_LAP2_SIZE -ErrorAction SilentlyContinue
    $t0 = Get-Date
    if ($arm -eq "p8") {
      & .\target\release\larder-world.exe "look=$dir" "run=$($s):8" mode=squads hand=bot bank=3000 $at *> (Join-Path $dir "log.txt")
    } else {
      $env:LAB_LAP2 = "1"; $env:LAB_LAP2_SIZE = $arm.Substring(4)
      & .\target\release\larder-world.exe "look=$dir" "run=$($s):9" mode=squads hand=bot bank=3000 $at @spores *> (Join-Path $dir "log.txt")
    }
    $log = Get-Content (Join-Path $dir "log.txt")
    $lines = ($log | Select-String -Pattern '^\s*t \d+ s: mould' | ForEach-Object { ($_.Line.Trim() -replace ', rendered \d+', '' -replace '; squad at.*', '') }) -join "`n    "
    "== $name ($([int]((Get-Date) - $t0).TotalSeconds) s wall)`n    $lines"
  }
}
Remove-Item Env:LAB_LAP2 -ErrorAction SilentlyContinue
Remove-Item Env:LAB_LAP2_SIZE -ErrorAction SilentlyContinue
