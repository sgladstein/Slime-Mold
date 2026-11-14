# One-click Larder World: update to the newest main, build, open the start menu.
# Double-click play.cmd (it runs this). Extra arguments go to the game, e.g.
#   play.cmd map=nest mode=squads
# Why a script: playing took four commands (checkout, pull, two builds, run);
# the owner asked for one thing to click (2026-11-14).
#
# Windows PowerShell 5.1 safe: native commands are checked by $LASTEXITCODE,
# never by ErrorActionPreference=Stop, because 5.1 turns git's and cargo's
# progress lines on stderr into errors.

$repo = $PSScriptRoot
Set-Location -LiteralPath $repo
$host.UI.RawUI.WindowTitle = "Larder World"

function Say($text, $colour = "Cyan") { Write-Host ""; Write-Host "== $text" -ForegroundColor $colour }

# cargo from a fresh rustup install is on the user PATH, but a double-click
# can start with an older PATH; fall back to rustup's own folder.
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
}

# 1. Update. Never touch local edits: with uncommitted changes, or with no
#    network, it says so and plays the code that is already here.
if (Get-Command git -ErrorAction SilentlyContinue) {
    $dirty = git status --porcelain --untracked-files=no
    if ($dirty) {
        Say "You have local changes, so I'm not updating. Playing the code as it is." Yellow
        $dirty | ForEach-Object { Write-Host "   $_" }
    } else {
        $branch = git rev-parse --abbrev-ref HEAD
        if ($branch -ne "main") {
            Say "Switching from '$branch' to main"
            git checkout main
            if ($LASTEXITCODE -ne 0) { Say "Couldn't switch to main; playing '$branch' instead." Yellow }
        }
        if ((git rev-parse --abbrev-ref HEAD) -eq "main") {
            Say "Getting the newest code"
            git pull --ff-only origin main
            if ($LASTEXITCODE -ne 0) { Say "Couldn't update (offline?). Playing the last code you had." Yellow }
        }
    }
} else {
    Say "git not found; playing the code as it is." Yellow
}

# 2. Build the game and the map editor (the menu's Edit maps button starts
#    larder-edit from beside the game). Cargo draws its own progress bar; the
#    first build takes a few minutes, later ones only rebuild what changed.
Say "Building (the first time takes a few minutes)"
cargo build --release --bin larder-world --bin larder-edit
if ($LASTEXITCODE -ne 0) {
    Say "The build failed. The messages above say why." Red
    exit 1
}

# 3. Play. Run from the repo root so files the game reads and writes land here.
Say "Starting Larder World" Green
& (Join-Path $repo "target\release\larder-world.exe") @args
exit $LASTEXITCODE
