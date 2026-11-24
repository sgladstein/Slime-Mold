# LAB (never merges): lap two's planets 9 to 15, a 1:00 smoke test each, on Scott's PC.
# Run from a clone of claude/project-thread-2o4cic after
#   cargo build --release --locked --bin larder-world
# Not a timing run. Run seed 1, each planet's pairing (pairs.txt) at LAB_LAP2_SIZE 0.7,
# the pumps hand where the second trait is the pump smell (10, 14), else greedy; bank 3000.
$ErrorActionPreference = "Continue"  # "Stop" turns the game's stderr into an error under PowerShell 5.1
$out = "C:\Users\Scott\Code\pc-batch-out\lab-lap2-smoke-20261124"
New-Item -ItemType Directory -Force -Path $out | Out-Null
$pairs = Get-Content "Reports\lab-lap-two-2026-11-24\raw\pairs.txt" | Where-Object { $_ -match '^\d+\|' }
$env:LAB_LAP2 = "1"; $env:LAB_LAP2_SIZE = "0.7"
foreach ($row in $pairs) {
  $f = $row.Split('|')
  $p = $f[0]
  $hand = if ($p -eq "10" -or $p -eq "14") { "pumps" } else { "greedy" }
  $keys = $f[3].Split(' ', [System.StringSplitOptions]::RemoveEmptyEntries)
  foreach ($arm in @("base", "pair")) {
    $name = "p$($p)_$arm"
    $dir = Join-Path $out $name
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    $k = if ($arm -eq "pair") { $keys } else { @() }
    $t0 = Get-Date
    & .\target\release\larder-world.exe "look=$dir" "run=1:$p" mode=squads "hand=$hand" bank=3000 at=1,30,60 @k *> (Join-Path $dir "log.txt")
    $log = Get-Content (Join-Path $dir "log.txt")
    $lines = ($log | Select-String -Pattern '^\s*t \d+ s: mould' | Select-Object -First 3 | ForEach-Object { ($_.Line.Trim() -replace ', rendered \d+', '' -replace '; squad at.*', '') }) -join "`n    "
    $boss = ($log | Select-String -Pattern 'formed' | Select-Object -First 2 | ForEach-Object { $_.Line.Trim().Substring(0, [Math]::Min(100, $_.Line.Trim().Length)) }) -join ' | '
    "== $name ($([int]((Get-Date) - $t0).TotalSeconds) s wall)`n    $lines`n    $boss"
  }
}
Remove-Item Env:LAB_LAP2 -ErrorAction SilentlyContinue
Remove-Item Env:LAB_LAP2_SIZE -ErrorAction SilentlyContinue
