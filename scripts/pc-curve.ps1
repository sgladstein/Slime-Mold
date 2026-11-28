# The run bot's curve on the owner's PC (Engine and tools, 2026-11-28;
# Milestone 2 test 3: "The run bot plays the twelve steps, and one page
# shows time to win, soldiers, income and losses by step and pip"). The
# owner's rule for long runs (11-28 03:34Z): "If my PC is on and running and
# connected, then use my PC if not then github" (.github/workflows/curve.yml
# plays the same rows there).
#
#   powershell -ExecutionPolicy Bypass -File scripts\pc-curve.ps1 -DryRun   # first: print every game, run nothing
#   powershell -ExecutionPolicy Bypass -File scripts\pc-curve.ps1           # the whole curve
#   ... pc-curve.ps1 -Seeds 1 -Steps 1,2,3                                 # a part
#
# Each game is one planet: `larder-world league=STEP:SEED:SECS:PIP`, the
# steps before it taken as won with two stars on the Fair route and shopped
# by the plan, then STEP's planet of that pip (a step with no such planet is
# skipped: step 1 and the home world have only a Fair one). Two arms: the run
# bot's plan (scripts/run-bot-plan.txt) and the builder arm, the same plan
# with a pump guarded by a turret (`pumps=1,guard=1,turrets=1,hold=20`;
# since #887 the turret is the best rolled design the shop sells).
#
# Its own clone (never the owner's play folder) at -Tree, on -Commit. One
# game at a time; before each game it waits while the owner plays (a
# larder-world.exe from his play folder) or while gpu-timing.lock exists
# (the GPU timing rule, project memory gtx1070-batch). The strategy league
# keeps its PAUSE file while this runs.
#
# Rows go to -Out\curve-<sha>.txt, one `CURVE arm=... league ...` line a
# game, and with -Push are committed to branch `curve-out` every -Every
# games and at the end (scripts/curve_table.py turns them into the table).
# A row already in the file is not played again, so a run stopped by sleep
# picks up where it was.

param(
    [switch]$DryRun,
    [switch]$Push,
    [int[]]$Seeds = @(1, 2, 3),
    [int[]]$Steps = @(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12),
    [string[]]$Pips = @("safe", "fair", "hard"),
    [string[]]$Arms = @("main", "builder"),
    [int]$Secs = 0,
    [int]$Every = 6,
    [string]$Commit = "origin/main",
    [string]$Play = "C:\Users\Scott\Code\Slime Mold",
    [string]$Tree = "C:\Users\Scott\Code\slime-curve",
    [string]$Out = "C:\Users\Scott\Code\slime-curve-out"
)
# Continue, not Stop: Windows PowerShell 5 turns cargo's stderr into errors
# under Stop (pc-batch.ps1's note).
$ErrorActionPreference = "Continue"
$lock = "C:\Users\Scott\Code\gpu-timing.lock"
$builder = "pumps=1,guard=1,turrets=1,hold=20"

if (-not (Test-Path $Tree)) {
    if ($DryRun) { "dry: git clone https://github.com/sgladstein/Slime-Mold $Tree" } else { git clone https://github.com/sgladstein/Slime-Mold $Tree }
}
if (-not $DryRun) {
    git -C $Tree fetch origin
    git -C $Tree checkout --detach $Commit
    Push-Location $Tree
    cargo build --release --locked --bin larder-world
    Pop-Location
}
$sha = if ($DryRun) { "dry" } else { (git -C $Tree rev-parse --short=8 HEAD).Trim() }
$exe = Join-Path $Tree "target\release\larder-world.exe"
New-Item -ItemType Directory -Force -Path $Out | Out-Null
$rows = Join-Path $Out "curve-$sha.txt"
if (-not (Test-Path $rows)) { New-Item -ItemType File -Path $rows | Out-Null }
$plan = (Get-Content (Join-Path $(if ($DryRun) { $Play } else { $Tree }) "scripts\run-bot-plan.txt") | Where-Object { $_ -notmatch '^#' -and $_.Trim() } | Select-Object -First 1).Trim()

function PlanOf([string]$arm) {
    if ($arm -eq "builder") { return (($plan -split '\|') | ForEach-Object { "$_,$builder" }) -join '|' }
    return $plan
}

function WaitTurn {
    while ($true) {
        $playing = Get-CimInstance Win32_Process -Filter "Name='larder-world.exe'" | Where-Object { $_.ExecutablePath -and $_.ExecutablePath.StartsWith($Play) }
        if (-not $playing -and -not (Test-Path $lock)) { return }
        "waiting: $(if ($playing) { 'the owner is playing' } else { 'gpu-timing.lock' }) $(Get-Date -Format o)"
        Start-Sleep -Seconds 60
    }
}

function PushRows {
    if (-not $Push -or $DryRun) { return }
    $git = Join-Path $Out "git"
    if (-not (Test-Path $git)) { git clone --branch curve-out https://github.com/sgladstein/Slime-Mold $git 2>$null; if (-not (Test-Path $git)) { git clone https://github.com/sgladstein/Slime-Mold $git; git -C $git checkout --orphan curve-out; git -C $git rm -rf --quiet . } }
    git -C $git pull --quiet origin curve-out 2>$null
    Copy-Item $rows (Join-Path $git "curve-$sha.txt") -Force
    git -C $git add "curve-$sha.txt"
    git -C $git commit --quiet -m "Curve rows from $env:COMPUTERNAME on $sha"
    git -C $git push --quiet origin curve-out
    if ($LASTEXITCODE -ne 0) { "push failed: the rows stay in $rows" }
}

$done = @{}
foreach ($l in Get-Content $rows) { if ($l -match '^CURVE arm=(\S+) step=(\d+) seed=(\d+) pip=(\S+)') { $done["$($Matches[1]) $($Matches[2]) $($Matches[3]) $($Matches[4])"] = 1 } }
$played = 0
foreach ($seed in $Seeds) {
    foreach ($step in $Steps) {
        foreach ($pip in $Pips) {
            foreach ($arm in $Arms) {
                $key = "$arm $step $seed $pip"
                if ($done.ContainsKey($key)) { continue }
                $p = PlanOf $arm
                $cmd = "league=${step}:${seed}:${Secs}:${pip}"
                if ($DryRun) { "dry: $key :: $exe $cmd plan=$p"; continue }
                WaitTurn
                $t0 = Get-Date
                $out = & $exe $cmd "plan=$p" 2>&1
                $wall = [int]((Get-Date) - $t0).TotalSeconds
                $line = $out | Where-Object { "$_" -match '^league ' } | Select-Object -First 1
                if (-not $line) {
                    $why = ($out | Where-Object { "$_" -match 'no (safe|fair|hard) planet' } | Select-Object -First 1)
                    $line = if ($why) { "skipped $why" } else { "failed $($out | Select-Object -Last 1)" }
                }
                "CURVE arm=$arm step=$step seed=$seed pip=$pip wall=$wall $line" | Tee-Object -FilePath $rows -Append
                $played++
                if ($played % $Every -eq 0) { PushRows }
            }
        }
    }
}
PushRows
"done: $played games, rows in $rows"
