# One batch for every queued GTX 1070 job (cloud plan C6, 2026-11-14;
# Reports/cloud-plan-2026-11-14.md). The PC runner runs it with no choices
# to make:
#
#   powershell -ExecutionPolicy Bypass -File scripts\pc-batch.ps1            # twins only
#   powershell -ExecutionPolicy Bypass -File scripts\pc-batch.ps1 -Timing    # twins, then timing
#
# -Timing only once the owner has said timing may run (booking.md): a timing
# run needs the PC to itself, so nothing else may run while this does.
#
# Every job runs in its own worktree at one pinned commit, writes one output
# file, and the batch ends with summary.txt: the lines to send back (each
# test binary's result line, each bench's machine line and last lines).
# Send summary.txt as it is, and with -Timing commit every r7-*.txt file
# whole to a runner branch (job 7 needs the full per-pass tables). Keep the
# folder for anyone who asks for more.
#
# Why one commit for the twins: main at $Commit contains #181 (shots sink),
# #183 (fed packing) and #198 (aims that see), whose 1070 twins never ran,
# and everything merged after them, so one run covers all three.

param(
    [switch]$Timing,
    [string]$Commit = "origin/main",
    [string]$Repo = "C:\Users\Scott\Code\Slime Mold",
    [string]$Tree = "C:\Users\Scott\Code\sm-batch",
    [string]$Out = "C:\Users\Scott\Code\pc-batch-out"
)
# Continue, not Stop: Windows PowerShell 5 turns a native tool's stderr
# (cargo's progress lines) into errors under Stop.
$ErrorActionPreference = "Continue"

# Booking rule 7: start only with more than 3 GB of RAM and 3 GB on C: free.
$ram = (Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory / 1MB
$disk = (Get-PSDrive C).Free / 1GB
"free RAM {0:N1} GB, free C: {1:N1} GB" -f $ram, $disk
if ($ram -lt 3 -or $disk -lt 3) { throw "Not enough free RAM or disk: stop and report these two numbers." }

$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$dir = Join-Path $Out $stamp
New-Item -ItemType Directory -Force -Path $dir | Out-Null

# The worktree, fresh at $Commit. It never touches anyone else's checkout.
git -C $Repo fetch origin
# Pin the batch to one sha, so the summary names exactly what ran.
$Commit = (git -C $Repo rev-parse "$Commit^{commit}").Trim()
if (Test-Path $Tree) { git -C $Repo worktree remove --force $Tree }
git -C $Repo worktree add --detach $Tree $Commit
Set-Location $Tree
$env:MOULDGPU_TEST_GPU = "1"

function Run([string]$name, [string]$cmd) {
    $file = Join-Path $dir "$name.txt"
    "== $name ($cmd) started $(Get-Date -Format o)" | Tee-Object -FilePath $file
    cmd /c "$cmd 2>&1" | Tee-Object -FilePath $file -Append | Out-Null
    "== $name exit $LASTEXITCODE at $(Get-Date -Format o)" | Tee-Object -FilePath $file -Append
}

Run "build" "cargo build --release --locked --bins"

# ---- Twins (compute; allowed whenever the PC is on and quiet) ----
Run "twins-mouldgpu" "cargo test --release --locked --bin mouldgpu"
Run "twins-lib-gpu" "cargo test --release --locked --lib gpu::"
Run "twins-larder-world" "cargo test --release --locked --bin larder-world"
Run "machine" "cargo run --release --locked --bin larder-world -- bench=20 hand=farm"

# Own-mould thread, PR #208 (a WGSL change): the whole suite at its head, in
# a worktree of its own. New test: gpu_garden_keeps_each_sides_birth_reserve_as_the_cpu.
# The skirmish bank test and the life-cycle twin fail on main too (known).
$own = "$Tree-pr208"
if (Test-Path $own) { git -C $Repo worktree remove --force $own }
git -C $Repo worktree add --detach $own 788db4ec
Push-Location $own
Run "twins-pr208" "cargo test --release --locked --no-fail-fast"
Pop-Location
git -C $Repo worktree remove --force $own

# ---- Timing (only with -Timing; each run alone, nothing else on the PC) ----
if ($Timing) {
    $lw = "target\release\larder-world.exe"
    # Shapes A, C (the default) and D (shapes thread, job 12).
    Run "time-shape-a" "$lw shape=a bench=120"
    Run "time-shape-c" "$lw shape=c bench=120"
    Run "time-shape-d" "$lw shape=d bench=120"
    # The player's own mould at 5k and 20k walkers (own-mould thread).
    Run "time-own-5k-off" "$lw walkers=5000 bench=30"
    Run "time-own-5k-on" "$lw walkers=5000 own=1 bench=30"
    Run "time-own-20k-off" "$lw walkers=20000 bench=30"
    Run "time-own-20k-on" "$lw walkers=20000 own=1 bench=30"
    # Bullets in flight against hitscan (bullets thread, #191).
    Run "time-bullet-45" "$lw bullet=45 bench=30 hand=farm"
    Run "time-bullet-0" "$lw bullet=0 bench=30 hand=farm"
    # Panic and withering on shape C (slime-enemy thread).
    Run "time-c-default" "$lw bench=30 hand=farm"
    Run "time-c-panic0" "$lw panic=0 bench=30 hand=farm"
    Run "time-c-wither0" "$lw wither=0 bench=30 hand=farm"
    # GPU thread, job 7 part 1: the crater countdown before (a56f644, same GPU
    # code as #157's first parent) and after (e8d9501). walkers= picks the
    # plain race mould at both commits, so the two match.
    foreach ($pair in @(@("a56f644", "before"), @("e8d9501", "after"))) {
        $sha, $tag = $pair
        $wt = "$Tree-$sha"
        if (Test-Path $wt) { git -C $Repo worktree remove --force $wt }
        git -C $Repo worktree add --detach $wt $sha
        Push-Location $wt
        Run "r7-$tag-build" "cargo build --release --locked --bin larder-world"
        Run "r7-$tag-100k" "target\release\larder-world.exe bench=60 hand=strike scale=8 walkers=100000 spread=0 passes=30"
        Run "r7-$tag-20k" "target\release\larder-world.exe bench=60 hand=strike scale=4 walkers=20000 spread=0 passes=30"
        Pop-Location
        git -C $Repo worktree remove --force $wt
    }
    # Job 7 part 2: shape C at 25k, the window's default with no walkers=.
    # idle is the mould at full growth, step not spread; farm is a played
    # game with the step spread over frames as the window does.
    Run "r7-c25k-idle" "$lw bench=20 hand=idle spread=0 passes=30"
    Run "r7-c25k-farm" "$lw bench=20 hand=farm passes=30"
}

# ---- What to send back ----
$sum = Join-Path $dir "summary.txt"
"batch $stamp at $Commit, timing $Timing" | Set-Content $sum
Get-ChildItem $dir -Filter "*.txt" | Where-Object { $_.Name -ne "summary.txt" } | Sort-Object Name | ForEach-Object {
    "--- $($_.BaseName)" | Add-Content $sum
    Select-String -Path $_.FullName -Pattern "^test result|GB free of|FAILED|panicked|^== .* exit|^free RAM|larder-world bench:|every frame|frames with a step|GPU \(timestamps|^ *t [0-9]+ s" | ForEach-Object { $_.Line } | Add-Content $sum
    Get-Content $_.FullName -Tail 3 | Add-Content $sum
}
"Done. Send back: $sum"
