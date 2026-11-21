# One batch for every queued GTX 1070 job (cloud plan C6, 2026-11-14;
# Reports/cloud-plan-2026-11-14.md). The PC runner runs it with no choices
# to make:
#
#   powershell -ExecutionPolicy Bypass -File scripts\pc-batch.ps1            # twins only
#   powershell -ExecutionPolicy Bypass -File scripts\pc-batch.ps1 -Timing    # twins, then timing
#   powershell -ExecutionPolicy Bypass -File scripts\pc-batch.ps1 -DryRun    # first: print every step, run nothing
#
# Run -DryRun first (seconds, nothing built): nobody can parse-check this
# file in the cloud, so a syntax slip shows there, not an hour into twins.
#
# -Timing only once the owner has said timing may run (booking.md; standing
# OK for tests, 11-11): a timing run needs the PC to itself, so stop the
# strategy bots' league worker first, and the script checks that nothing
# else is on the GPU before it times anything. About 45 minutes of timing.
#
# Every job writes one output file, and the batch ends with summary.txt: the
# lines to send back (each test binary's result line, each bench's lines,
# the shas and the driver). Send summary.txt as it is, and with -Timing
# commit every m1-*.txt file whole to a runner branch (the per-pass tables
# are in them, not in the summary). Keep the folder for anyone who asks.
#
# Why one commit for the twins: the whole suite at one main covers every
# GPU rule merged since the last batch (11-14), each tested on the card.
#
# Technical lead's review (11-14 00:52Z), all in: a GPU-free check and the
# card's clocks in every timing file; 90 s cool-down before timing; A/B
# runs interleaved (1, 2, 3, 1, 2, 3) so clock drift shows as spread, not
# as a difference; one shared CARGO_TARGET_DIR so dependencies
# build once, with each commit's exe copied out under its sha; the shas and
# driver in the summary header.

param(
    [switch]$Timing,
    [switch]$DryRun,
    [string]$Commit = "origin/main",
    [string]$Repo = "C:\Users\Scott\Code\Slime Mold",
    [string]$Tree = "C:\Users\Scott\Code\sm-batch",
    [string]$Out = "C:\Users\Scott\Code\pc-batch-out"
)
# Continue, not Stop: Windows PowerShell 5 turns a native tool's stderr
# (cargo's progress lines) into errors under Stop.
$ErrorActionPreference = "Continue"

# Booking rule 7 asked 3 GB RAM free, but the owner's own apps leave 2.0-2.3
# GB (the runner's dry run at 004b7cb, 11-14), so the gate is 2 GB and builds
# run two jobs at a time (CARGO_BUILD_JOBS=2) to stay inside it. Disk raised to 8 GB because a few release
# builds share the one target folder.
$ram = (Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory / 1MB
$disk = (Get-PSDrive C).Free / 1GB
"free RAM {0:N1} GB, free C: {1:N1} GB" -f $ram, $disk
if (-not $DryRun -and ($ram -lt 2 -or $disk -lt 8)) { throw "Not enough free RAM (2 GB) or disk (8 GB): stop and report these two numbers." }

$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$dir = Join-Path $Out $stamp
New-Item -ItemType Directory -Force -Path $dir | Out-Null
$env:CARGO_TARGET_DIR = "$Tree-target"
$env:CARGO_BUILD_JOBS = "2"
$bins = "$Tree-bins"
New-Item -ItemType Directory -Force -Path $bins | Out-Null
$env:MOULDGPU_TEST_GPU = "1"

function Run([string]$name, [string]$cmd) {
    if ($DryRun) { "dry: $name :: $cmd"; return }
    $file = Join-Path $dir "$name.txt"
    "== $name ($cmd) started $(Get-Date -Format o)" | Tee-Object -FilePath $file
    cmd /c "$cmd 2>&1" | Tee-Object -FilePath $file -Append | Out-Null
    "== $name exit $LASTEXITCODE at $(Get-Date -Format o)" | Tee-Object -FilePath $file -Append
}

# A timing run: the card's clocks, temperature and power before it, then
# the bench.
function Time([string]$name, [string]$cmd) {
    if ($DryRun) { "dry: $name :: card clocks, then $cmd"; return }
    $file = Join-Path $dir "$name.txt"
    "== card before: " + (nvidia-smi --query-gpu=clocks.gr,clocks.mem,temperature.gpu,power.draw --format=csv,noheader) | Set-Content $file
    $file2 = Join-Path $dir "$name-run.txt"
    Run "$name-run" $cmd
    Get-Content $file2 | Add-Content $file
    Remove-Item $file2
}

# A worktree at $sha (fresh), entered; returns its short sha.
function At([string]$sha, [string]$path) {
    if ($DryRun) { "dry: worktree $path at $sha" | Out-Host; return $sha }
    if (Test-Path $path) { git -C $Repo worktree remove --force $path }
    git -C $Repo worktree add --detach $path $sha | Out-Null
    Set-Location $path
    return (git -C $path log -1 --format=%h).Trim()
}
$shas = @()

if (-not $DryRun) { git -C $Repo fetch origin }
# Pin the batch to one sha, so the summary names exactly what ran.
$Commit = (git -C $Repo rev-parse "$Commit^{commit}").Trim()
$main = At $Commit $Tree
$shas += "batch main $main"

Run "build" "cargo build --release --locked --bins"
if (-not $DryRun) { Copy-Item "$env:CARGO_TARGET_DIR\release\larder-world.exe" "$bins\lw-main.exe" }

# ---- Twins (compute; allowed whenever the PC is on and quiet) ----
Run "machine" "$bins\lw-main.exe bench=20 hand=farm"

# The whole suite on the card: every GPU test since 11-14 (the Lure, the
# fort, walls, tether, march, condensing, the heart, fire) ran only on
# llvmpipe until now. Known on Windows since 11-12: the life-cycle twin.
Run "twins-all" "cargo test --release --locked --no-fail-fast"

# ---- Timing (only with -Timing; the PC to itself, checked) ----
# Milestone 1's batch (Engine and tools, job 7; 2026-11-20). The 11-14
# batch's timing (shapes, own mould, the crater countdown A/B at a56f644
# and e8d9501) is recorded in Reports/pc-batch-1070-2026-11-14.md and
# needs no rerun. This one times the game the owner plays: planets 1 to 3
# of a run in the squad mode, with every rule since then on by default
# (the Lure's field, the Trunk's fort, raised walls, the tether and the
# raid march, condensing and the heart, fire, starfall), played by the
# strategy bot's rush. Each planet twice, interleaved (1 2 3 1 2 3), so
# clock drift shows as spread.
if ($Timing) {
    if (-not $DryRun) { Set-Location $Tree }

    # Is anything else using the card? Not --query-compute-apps: on Windows
    # (WDDM) it lists every desktop app with "N/A" memory, so it always
    # refused (dry run at 004b7cb). The card's own load instead: five
    # samples a second apart, busy if any is over 5%. The strategy bots'
    # league worker (slime-league) must be stopped first: it plays games
    # on this card all day.
    $util = @()
    if (-not $DryRun) {
        foreach ($k in 1..5) { $util += [int](nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader,nounits | Select-Object -First 1).Trim(); Start-Sleep -Seconds 1 }
    }
    $busy = if (($util | Measure-Object -Maximum).Maximum -gt 5) { "GPU load % over 5 s: $($util -join ', ')" } else { $null }
    # The league worker between its games shows no load: look for its game
    # too (pause it with an empty PAUSE file in slime-league, never by
    # stopping its Python).
    if (-not $DryRun) {
        $league = @(Get-CimInstance Win32_Process -Filter "Name='larder-world.exe'" | Where-Object { $_.CommandLine -match "league=" })
        if ($league.Count -gt 0) { $busy = "a league game is running (pid $($league[0].ProcessId)): pause the worker first" }
    }
    "GPU load % before timing: $($util -join ', ')"
    if ($busy) {
        "GPU busy, no timing run: $busy" | Set-Content (Join-Path $dir "timing-refused.txt")
        Get-Content (Join-Path $dir "timing-refused.txt")
    } else {
        # Cool down after the test suites, so timing doesn't start hot.
        if (-not $DryRun) { Start-Sleep -Seconds 90 }
        $lw = "$bins\lw-main.exe"
        $hand = "hand=strat:rush,aim=6"
        # The played game, as the window plays it (steps spread over
        # frames): frame p50/p95 against the W2c gate (10 ms p95). 300 game
        # seconds reach the first raid (mostly 3:00 to 5:00).
        foreach ($i in 1, 2) {
            foreach ($p in 1, 2, 3) {
                Time "m1-planet$p-play-$i" "$lw run=1:$p mode=squads bench=300 $hand"
            }
        }
        # Each pass's GPU ms a step, timed alone (spread=0: whole steps, so
        # the step timestamps are whole too), 30 steps at 120 s.
        foreach ($p in 1, 2, 3) {
            Time "m1-planet$p-passes" "$lw run=1:$p mode=squads bench=120 $hand spread=0 passes=30"
        }
        # The draw at the review's two zooms, on the squad's first view
        # (planet 3, the biggest map, on the larder), with each view pass timed.
        foreach ($z in 6, 12) {
            Time "m1-planet3-zoom$z" "$lw run=1:3 mode=squads bench=60 $hand centre=larder,$z viewpasses=30"
        }
    }
}

# ---- What to send back ----
Set-Location $Out
$sum = Join-Path $dir "summary.txt"
"batch $stamp, timing $Timing, dry run $DryRun" | Set-Content $sum
"shas: " + ($shas -join "; ") | Add-Content $sum
"driver: " + (nvidia-smi --query-gpu=name,driver_version --format=csv,noheader) | Add-Content $sum
Get-ChildItem $dir -Filter "*.txt" | Where-Object { $_.Name -ne "summary.txt" } | Sort-Object Name | ForEach-Object {
    "--- $($_.BaseName)" | Add-Content $sum
    Select-String -Path $_.FullName -Pattern "^test result|GB free of|FAILED|panicked|^== |^free RAM|GPU busy|larder-world bench:|every frame|frames with a step|GPU \(timestamps|^ *t [0-9]+ s|^  view:|GPU load" | ForEach-Object { $_.Line } | Add-Content $sum
    Get-Content $_.FullName -Tail 3 | Add-Content $sum
}
# The shared target and the exes stay for a rerun (delete "$Tree-target"
# and "$Tree-bins" to free the space).
"Done. Send back: $sum"
