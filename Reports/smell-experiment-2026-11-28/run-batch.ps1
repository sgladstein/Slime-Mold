# The smell experiment's batch on the owner's PC (2026-11-28, the scent
# thread; his rule 11-28 03:34Z: long bot runs on his PC when it is on and
# connected). Run from the root of a clone of this branch, never his play
# folder:
#
#   powershell -ExecutionPolicy Bypass -File Reports\smell-experiment-2026-11-28\run-batch.ps1 -DryRun
#   powershell -ExecutionPolicy Bypass -File Reports\smell-experiment-2026-11-28\run-batch.ps1
#
# It builds larder-world, plays every line of queue.txt (`ARM PLANET SEED
# SECS PLAN [ARGS]`, as `league=PLANET:SEED:SECS plan=PLAN ARGS`),
# `-Parallel` games at a time, and writes rows.txt beside the queue: one
# `ARMS game=N arm=... wall=... league ...` line a game as each ends, as the
# Arms workflow's rows. Games already in rows.txt are skipped, so a run cut
# short goes on where it stopped.

param(
    [int]$Parallel = 2,
    # A later batch: queue<Batch>.txt into rows<Batch>.txt (batch 2,
    # 11-28: the same games on the build without the charges' gate).
    [string]$Batch = "",
    [switch]$DryRun
)
$ErrorActionPreference = "Continue"
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$queue = Join-Path $here "queue$Batch.txt"
$rows = Join-Path $here "rows$Batch.txt"
$work = Join-Path $env:TEMP "smell-exp-games$Batch"
New-Item -ItemType Directory -Force -Path $work | Out-Null
$exe = "target\release\larder-world.exe"

if (-not $DryRun) {
    cargo build --release --locked --bin larder-world
    if ($LASTEXITCODE -ne 0) { throw "build failed" }
}

$done = @{}
if (Test-Path $rows) {
    Get-Content $rows | ForEach-Object { if ($_ -match '^ARMS game=(\S+)') { $done[$Matches[1]] = $true } }
}

$games = @()
$i = 0
foreach ($line in Get-Content $queue) {
    if ($line -match '^\s*#' -or $line.Trim() -eq '') { continue }
    $w = $line.Trim() -split '\s+'
    $i++
    $name = "{0:D3}" -f $i
    $gameArgs = @("league=$($w[1]):$($w[2]):$($w[3])", "plan=$($w[4])")
    if ($w.Count -gt 5) { $gameArgs += $w[5..($w.Count - 1)] }
    $games += [pscustomobject]@{ Name = $name; Arm = $w[0]; Args = $gameArgs }
}
"{0} games in the queue, {1} already done" -f $games.Count, $done.Count

$running = @()
function Collect {
    param($r)
    $wall = [int](($r.P.ExitTime - $r.T0).TotalSeconds)
    $out = Get-Content (Join-Path $work "$($r.G.Name).out") -ErrorAction SilentlyContinue | Where-Object { $_ -like 'league *' }
    if (-not $out) { $out = @("league failed exit=$($r.P.ExitCode)") }
    foreach ($o in $out) { "ARMS game=$($r.G.Name) arm=$($r.G.Arm) wall=$wall $o" | Add-Content -Path $rows }
    "{0} {1} done in {2} s" -f $r.G.Name, $r.G.Arm, $wall
}
foreach ($g in $games) {
    if ($done.ContainsKey($g.Name)) { continue }
    if ($DryRun) { "dry: $($g.Name) $($g.Arm) :: $exe $($g.Args -join ' ')"; continue }
    while (@($running | Where-Object { -not $_.P.HasExited }).Count -ge $Parallel) { Start-Sleep -Seconds 2 }
    foreach ($r in @($running | Where-Object { $_.P.HasExited })) { Collect $r }
    $running = @($running | Where-Object { -not $_.P.HasExited })
    $p = Start-Process -FilePath $exe -ArgumentList $g.Args -NoNewWindow -PassThru `
        -RedirectStandardOutput (Join-Path $work "$($g.Name).out") -RedirectStandardError (Join-Path $work "$($g.Name).err")
    # Windows PowerShell 5 keeps ExitCode and ExitTime only once the handle
    # has been read.
    $null = $p.Handle
    $running +=[pscustomobject]@{ P = $p; G = $g; T0 = Get-Date }
}
foreach ($r in $running) { $r.P.WaitForExit(); Collect $r }
"rows in $rows"
