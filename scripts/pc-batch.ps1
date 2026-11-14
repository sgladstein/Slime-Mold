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
# -Timing only once the owner has said timing may run (booking.md): a timing
# run needs the PC to itself, and the script checks that nothing else is on
# the GPU before it times anything.
#
# Every job writes one output file, and the batch ends with summary.txt: the
# lines to send back (each test binary's result line, each bench's lines,
# the shas and the driver). Send summary.txt as it is, and with -Timing
# commit every r7-*.txt file whole to a runner branch (job 7 needs the full
# per-pass tables). Keep the folder for anyone who asks for more.
#
# Why one commit for the twins: main at $Commit contains #181 (shots sink),
# #183 (fed packing) and #198 (aims that see), whose 1070 twins never ran,
# and everything merged after them, so one run covers all three.
#
# Technical lead's review (11-14 00:52Z), all in: a GPU-free check and the
# card's clocks in every timing file; 90 s cool-down before timing; A/B
# pairs interleaved (before, after, before, after) so clock drift shows as
# spread, not as a difference; one shared CARGO_TARGET_DIR so dependencies
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

# Booking rule 7 (3 GB RAM free); disk raised to 8 GB because a few release
# builds share the one target folder.
$ram = (Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory / 1MB
$disk = (Get-PSDrive C).Free / 1GB
"free RAM {0:N1} GB, free C: {1:N1} GB" -f $ram, $disk
if (-not $DryRun -and ($ram -lt 3 -or $disk -lt 8)) { throw "Not enough free RAM (3 GB) or disk (8 GB): stop and report these two numbers." }

$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$dir = Join-Path $Out $stamp
New-Item -ItemType Directory -Force -Path $dir | Out-Null
$env:CARGO_TARGET_DIR = "$Tree-target"
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
Run "twins-mouldgpu" "cargo test --release --locked --bin mouldgpu"
Run "twins-lib-gpu" "cargo test --release --locked --lib gpu::"
Run "twins-larder-world" "cargo test --release --locked --bin larder-world"
Run "machine" "$bins\lw-main.exe bench=20 hand=farm"

# Own-mould thread, PR #208 (a WGSL change): the whole suite at its head.
# New test: gpu_garden_keeps_each_sides_birth_reserve_as_the_cpu. The
# skirmish bank test and the life-cycle twin fail on main too (known).
$s = At "788db4ec" "$Tree-pr208"
$shas += "pr208 $s"
Run "twins-pr208" "cargo test --release --locked --no-fail-fast"

# ---- Timing (only with -Timing; the PC to itself, checked) ----
if ($Timing) {
    # GPU thread job 7 part 1 needs two more builds: the crater countdown
    # before (a56f644, the same GPU code as #157's first parent) and after
    # (e8d9501). Build both first so the runs can interleave.
    foreach ($pair in @(@("a56f644", "before"), @("e8d9501", "after"))) {
        $sha, $tag = $pair
        $s = At $sha "$Tree-$tag"
        $shas += "r7-$tag $s"
        Run "r7-$tag-build" "cargo build --release --locked --bin larder-world"
        if (-not $DryRun) { Copy-Item "$env:CARGO_TARGET_DIR\release\larder-world.exe" "$bins\lw-$tag.exe" }
    }
    if (-not $DryRun) { Set-Location $Tree }

    $busy = nvidia-smi --query-compute-apps=pid,process_name --format=csv,noheader
    if ($busy) {
        "GPU busy, no timing run: $busy" | Set-Content (Join-Path $dir "timing-refused.txt")
        Get-Content (Join-Path $dir "timing-refused.txt")
    } else {
        # Cool down after the test suites, so timing doesn't start hot.
        if (-not $DryRun) { Start-Sleep -Seconds 90 }
        $lw = "$bins\lw-main.exe"
        # Shapes A, C (the default) and D (shapes thread, job 12).
        Time "time-shape-a" "$lw shape=a bench=120"
        Time "time-shape-c" "$lw shape=c bench=120"
        Time "time-shape-d" "$lw shape=d bench=120"
        # Pairs interleaved, twice each (A B A B).
        foreach ($i in 1, 2) {
            # The player's own mould at 5k and 20k walkers (own-mould thread).
            Time "time-own-5k-off-$i" "$lw walkers=5000 bench=30"
            Time "time-own-5k-on-$i" "$lw walkers=5000 own=1 bench=30"
            Time "time-own-20k-off-$i" "$lw walkers=20000 bench=30"
            Time "time-own-20k-on-$i" "$lw walkers=20000 own=1 bench=30"
            # Bullets in flight against hitscan (bullets thread, #191).
            Time "time-bullet-45-$i" "$lw bullet=45 bench=30 hand=farm"
            Time "time-bullet-0-$i" "$lw bullet=0 bench=30 hand=farm"
            # GPU thread job 7 part 1, the crater countdown.
            foreach ($tag in "before", "after") {
                Time "r7-$tag-100k-$i" "$bins\lw-$tag.exe bench=60 hand=strike scale=8 walkers=100000 spread=0 passes=30"
                Time "r7-$tag-20k-$i" "$bins\lw-$tag.exe bench=60 hand=strike scale=4 walkers=20000 spread=0 passes=30"
            }
        }
        # Panic and withering on shape C (slime-enemy thread).
        Time "time-c-default" "$lw bench=30 hand=farm"
        Time "time-c-panic0" "$lw panic=0 bench=30 hand=farm"
        Time "time-c-wither0" "$lw wither=0 bench=30 hand=farm"
        # Job 7 part 2: shape C at 25k, the window's default with no
        # walkers=. idle is the mould at full growth, step not spread; farm
        # is a played game with the step spread over frames as the window does.
        Time "r7-c25k-idle" "$lw bench=20 hand=idle spread=0 passes=30"
        Time "r7-c25k-farm" "$lw bench=20 hand=farm passes=30"
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
    Select-String -Path $_.FullName -Pattern "^test result|GB free of|FAILED|panicked|^== |^free RAM|GPU busy|larder-world bench:|every frame|frames with a step|GPU \(timestamps|^ *t [0-9]+ s" | ForEach-Object { $_.Line } | Add-Content $sum
    Get-Content $_.FullName -Tail 3 | Add-Content $sum
}
# The worktrees go; the shared target and the exes stay for a rerun (delete
# "$Tree-target" and "$Tree-bins" to free the space).
foreach ($p in "$Tree-pr208", "$Tree-before", "$Tree-after") {
    if (-not $DryRun -and (Test-Path $p)) { git -C $Repo worktree remove --force $p }
}
"Done. Send back: $sum"
