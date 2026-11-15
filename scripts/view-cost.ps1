# What each part of the view's shading costs on the GTX 1070 (performance
# thread, 2026-11-15; Reports/view-cost-2026-11-15/). The PC runner runs it
# with no choices to make:
#
#   powershell -ExecutionPolicy Bypass -File scripts\view-cost.ps1 -DryRun   # first: print every step, run nothing
#   powershell -ExecutionPolicy Bypass -File scripts\view-cost.ps1
#
# It builds larder-world once at -Commit, then for each view below times the
# view's passes alone (`viewpasses=`) with the shader as it is ("none") and
# with one part of `shaded` left out at a time (knockouts.json: each entry
# is an exact text swap in view.wgsl, loaded through LARDER_VIEW_WGSL, so
# nothing is rebuilt). A part's cost is "none" minus its knockout. Two
# rounds, interleaved, so clock drift shows as spread. With -Compare SHA it
# also times that commit's own shader (its exe) beside this one, A/B.
#
# Send back summary.txt; commit the whole output folder to a runner branch.
# A timing run needs the PC to itself (booking.md).

param(
    [switch]$DryRun,
    [string]$Commit = "origin/main",
    [string]$Compare = "",
    [string]$Repo = "C:\Users\Scott\Code\Slime Mold",
    [string]$Tree = "C:\Users\Scott\Code\sm-perf",
    [string]$Out = "C:\Users\Scott\Code\view-cost-out",
    [int]$Rounds = 2
)
$ErrorActionPreference = "Continue"

$ram = (Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory / 1MB
$disk = (Get-PSDrive C).Free / 1GB
"free RAM {0:N1} GB, free C: {1:N1} GB" -f $ram, $disk
if (-not $DryRun -and ($ram -lt 2 -or $disk -lt 4)) { throw "Not enough free RAM (2 GB) or disk (4 GB): stop and report these two numbers." }

$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$dir = Join-Path $Out $stamp
New-Item -ItemType Directory -Force -Path $dir | Out-Null
$env:CARGO_TARGET_DIR = "$Tree-target"
$env:CARGO_BUILD_JOBS = "2"
$bins = "$Tree-bins"
New-Item -ItemType Directory -Force -Path $bins | Out-Null

function Step([string]$what, [scriptblock]$do) {
    if ($DryRun) { "dry: $what"; return }
    "== $what"
    & $do
}

# One worktree of our own (never another thread's checkout), one exe a
# commit, copied out under its sha.
Step "fetch" { git -C $Repo fetch origin }
if (-not (Test-Path $Tree)) { Step "worktree add $Tree" { git -C $Repo worktree add --detach $Tree $Commit } }
function Build([string]$rev) {
    Step "build $rev" {
        git -C $Tree checkout --detach $rev
        $sha = (git -C $Tree rev-parse --short=8 HEAD).Trim()
        Push-Location $Tree
        cargo build --release --locked --bin larder-world 2>&1 | Select-Object -Last 3
        Pop-Location
        Copy-Item "$env:CARGO_TARGET_DIR\release\larder-world.exe" "$bins\lw-$sha.exe" -Force
        $script:exe = "$bins\lw-$sha.exe"
        $script:sha = $sha
    }
}
$exe = ""; $sha = ""
if ($Compare -ne "") {
    Build $Compare
    $cmpExe = $exe; $cmpSha = $sha
}
Build $Commit
"commit $sha; compare $Compare" | Tee-Object -FilePath (Join-Path $dir "shas.txt")

# The knockout shaders, from this commit's view.wgsl (line ends made LF so
# the swaps' newlines match a CRLF checkout).
$variants = @("none")
if (-not $DryRun) {
    $src = (Get-Content -Raw "$Tree\src\world\view.wgsl") -replace "`r`n", "`n"
    $ko = Get-Content -Raw "$Tree\Reports\view-cost-2026-11-15\knockouts.json" | ConvertFrom-Json
    [IO.File]::WriteAllText((Join-Path $dir "v-none.wgsl"), $src)
    foreach ($p in $ko.PSObject.Properties) {
        $s = $src
        $ok = $true
        foreach ($e in $p.Value) {
            if (-not $s.Contains($e.old)) { "knockout $($p.Name): text not found, skipped"; $ok = $false; break }
            $s = $s.Replace($e.old, $e.new)
        }
        if ($ok) {
            [IO.File]::WriteAllText((Join-Path $dir "v-$($p.Name).wgsl"), $s)
            $variants += $p.Name
        }
    }
}
"variants: $($variants -join ', ')"

# The views: the fork (meadow everywhere) at zoom 1, 6 and 24, and a
# planet (soils, water, rock, plants) at zoom 6 and 24 on the squad.
$views = [ordered]@{
    "fork-z1"    = "map=fork bench=3 hand=farm centre=175,215,1 viewpasses=120"
    "fork-z6"    = "map=fork bench=3 hand=farm centre=175,215,6 viewpasses=120"
    "fork-z24"   = "map=fork bench=3 hand=farm centre=175,215,24 viewpasses=120"
    "planet-z6"  = "map=planet:8:1 bench=3 hand=farm centre=559,364,6 viewpasses=120"
    "planet-z24" = "map=planet:8:1 bench=3 hand=farm centre=559,364,24 viewpasses=120"
}

if (-not $DryRun) {
    "cool-down 60 s"
    Start-Sleep -Seconds 60
}
$summary = Join-Path $dir "summary.txt"
for ($r = 1; $r -le $Rounds; $r++) {
    foreach ($v in $views.Keys) {
        $runs = @()
        foreach ($k in $variants) { $runs += ,@($k, $exe, (Join-Path $dir "v-$k.wgsl")) }
        if ($Compare -ne "") { $runs += ,@("commit-$cmpSha", $cmpExe, "") }
        foreach ($run in $runs) {
            $name = "r$r-$v-$($run[0])"
            $cmd = "$($run[1]) $($views[$v])"
            if ($DryRun) { "dry: $name :: LARDER_VIEW_WGSL=$($run[2]) $cmd"; continue }
            $env:LARDER_VIEW_WGSL = $run[2]
            if ($run[2] -eq "") { Remove-Item Env:LARDER_VIEW_WGSL -ErrorAction SilentlyContinue }
            $file = Join-Path $dir "$name.txt"
            "== card before: " + (nvidia-smi --query-gpu=clocks.gr,clocks.mem,temperature.gpu,power.draw,utilization.gpu --format=csv,noheader) | Set-Content $file
            cmd /c "$cmd 2>&1" | Add-Content $file
            $lines = Get-Content $file | Select-String -Pattern "machine at the start|view, every pass|^\s+(base|prep|marks|compose|specks|drain)\s"
            "== $name" | Add-Content $summary
            $lines | ForEach-Object { $_.Line } | Add-Content $summary
        }
    }
}
Remove-Item Env:LARDER_VIEW_WGSL -ErrorAction SilentlyContinue
if (-not $DryRun) { "done: $dir"; Get-Content $summary | Select-Object -First 40 }
