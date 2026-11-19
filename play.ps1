# One-click Larder World: update to the newest main, get or build the game,
# open the start menu, and send the play logs when the game quits.
# Double-click play.cmd (it runs this). Extra arguments go to the game, e.g.
#   play.cmd map=nest mode=squads
# Why a script: playing took four commands (checkout, pull, two builds, run);
# the owner asked for one thing to click (2026-11-14).
#
# Windows PowerShell 5.1 safe: native commands are checked by $LASTEXITCODE,
# never by ErrorActionPreference=Stop, because 5.1 turns git's and cargo's
# progress lines on stderr into errors.


function Say($text, $colour = "Cyan") { Write-Host ""; Write-Host "== $text" -ForegroundColor $colour }

# The files whose change changes the game: with none of them different
# between two commits, a build of one is a build of the other.
$GameFiles = @("src", "assets", "news", "Cargo.toml", "Cargo.lock", "build.rs")

# Where CI publishes its Windows build of main (.github/workflows/
# windows-build.yml): one release that every new main overwrites.
$BuildUrl = "https://github.com/sgladstein/Slime-Mold/releases/download/windows-build"

# Gets CI's Windows build if it is the code in $repo, into target\ci.
# Returns the folder holding larder-world.exe, or $null to build here.
#
# Why (the plan of 2026-11-17, the Engine and tools lane's job 3): every
# play began with a cargo build, minutes long after most updates, on the
# one PC that plays. CI builds each new main once and the script fetches
# that. It falls back to a local build whenever it cannot be sure the
# fetched build is this code: no network, CI not finished for this main
# yet, local edits, or a download that does not check out.
function Get-CiBuild($repo) {
    try {
        if (-not (Get-Command git -ErrorAction SilentlyContinue)) { return $null }
        # Local edits to the game are not in CI's build.
        $dirty = git -C $repo status --porcelain --untracked-files=no -- $GameFiles
        if ($dirty) { return $null }
        $ProgressPreference = "SilentlyContinue"   # 5.1 draws a progress bar that slows a download many times over
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
        $dir = Join-Path $repo "target\ci"
        New-Item -ItemType Directory -Force -Path $dir | Out-Null
        $stamp = Join-Path $dir "build.txt.new"
        Invoke-WebRequest -UseBasicParsing -TimeoutSec 20 -Uri "$BuildUrl/build.txt" -OutFile $stamp
        $built = (Get-Content -LiteralPath $stamp -TotalCount 1).Trim()
        if ($built -notmatch '^[0-9a-f]{40}$') { return $null }
        # CI's build is this code if no game file differs between its
        # commit and the one checked out (most updates bring only notes).
        git -C $repo cat-file -e "$built^{commit}" 2>$null
        if ($LASTEXITCODE -ne 0) { return $null }
        git -C $repo diff --quiet $built HEAD -- $GameFiles
        if ($LASTEXITCODE -ne 0) { return $null }
        $have = Join-Path $dir "build.txt"
        $exe = Join-Path $dir "larder-world.exe"
        if ((Test-Path $have) -and (Test-Path $exe) -and ((Get-Content -LiteralPath $have -TotalCount 1).Trim() -eq $built)) { return $dir }
        Say "Getting the game CI built for this code"
        $zip = Join-Path $dir "larder-world-windows.zip"
        Invoke-WebRequest -UseBasicParsing -TimeoutSec 300 -Uri "$BuildUrl/larder-world-windows.zip" -OutFile $zip
        $fresh = Join-Path $dir "unpacked"
        if (Test-Path $fresh) { Remove-Item -Recurse -Force $fresh }
        Expand-Archive -LiteralPath $zip -DestinationPath $fresh
        # The zip names its own commit: the release's two files are
        # uploaded one after the other, so a new build.txt can sit beside
        # the old zip for a moment.
        $inside = Join-Path $fresh "build.txt"
        if (-not (Test-Path $inside) -or ((Get-Content -LiteralPath $inside -TotalCount 1).Trim() -ne $built)) { return $null }
        if (-not (Test-Path (Join-Path $fresh "larder-world.exe"))) { return $null }
        Copy-Item -Force (Join-Path $fresh "*") $dir
        Remove-Item -Recurse -Force $fresh
        Remove-Item -Force $zip
        return $dir
    } catch {
        return $null
    }
}

# Sends the play logs in $repo\play-logs to the branch `play-logs` on
# $remote, then moves them to play-logs\sent. Returns a line saying what
# happened. It never throws and never asks for anything: the owner's rule
# for it is that it says plainly when it could not send and never blocks
# play (it runs after the game has quit).
#
# Why a branch (the owner, 2026-11-17 17:39Z: "yes play logs is a great
# idea"): threads can read what a play session did, by the build and the
# switches in each log's first line, without asking him for files.
#
# How: a commit made from the log files alone, on top of the branch's last
# one, with git's plumbing and an index file of its own. Nothing in the
# working folder or the real index is touched, so it cannot disturb the
# checkout the game is built from. The commit carries a fixed name, not
# git's configured one: the repository is public.
#
# $ask: let git ask for a GitHub sign-in (`play.cmd send-logs`, run once on
# a PC that has never pushed). After a game it is never set.
function Send-PlayLogs($repo, $remote = "origin", $ask = $false) {
    $oldIndex = $env:GIT_INDEX_FILE
    try {
        $dir = Join-Path $repo "play-logs"
        if (-not (Test-Path $dir)) { return "" }
        $logs = @(Get-ChildItem -LiteralPath $dir -Filter *.txt -File)
        if ($logs.Count -eq 0) { return "" }
        $kept = "The $($logs.Count) play log(s) are kept in play-logs and will be sent next time."
        if (-not (Get-Command git -ErrorAction SilentlyContinue)) { return "Couldn't send the play logs: git not found. $kept" }
        # Never stop to ask for a password: fail and say so instead.
        if (-not $ask) {
            $env:GIT_TERMINAL_PROMPT = "0"
            $env:GCM_INTERACTIVE = "never"
        }
        git -C $repo fetch --quiet $remote "+refs/heads/play-logs:refs/remotes/$remote/play-logs" 2>$null
        $parent = git -C $repo rev-parse --verify --quiet "refs/remotes/$remote/play-logs"
        $env:GIT_INDEX_FILE = Join-Path (git -C $repo rev-parse --absolute-git-dir) "play-logs-index"
        if (Test-Path $env:GIT_INDEX_FILE) { Remove-Item -Force $env:GIT_INDEX_FILE }
        if ($parent) { git -C $repo read-tree $parent | Out-Null } else { git -C $repo read-tree --empty | Out-Null }
        if ($LASTEXITCODE -ne 0) { return "Couldn't send the play logs: git could not read the play-logs branch. $kept" }
        foreach ($f in $logs) {
            $blob = git -C $repo hash-object -w -- $f.FullName
            if ($LASTEXITCODE -ne 0) { return "Couldn't send the play logs: git could not store $($f.Name). $kept" }
            git -C $repo update-index --add --cacheinfo "100644,$blob,logs/$($f.Name)" | Out-Null
        }
        $tree = git -C $repo write-tree
        $what = "Play logs: $($logs.Count) round(s), $($logs[0].BaseName) to $($logs[-1].BaseName)"
        $who = @("-c", "user.name=Larder World play log", "-c", "user.email=play-log@users.noreply.github.com")
        if ($parent) { $commit = git -C $repo @who commit-tree $tree -p $parent -m $what } else { $commit = git -C $repo @who commit-tree $tree -m $what }
        if ($LASTEXITCODE -ne 0 -or -not $commit) { return "Couldn't send the play logs: git could not make the commit. $kept" }
        git -C $repo push --quiet $remote "${commit}:refs/heads/play-logs" 2>$null
        if ($LASTEXITCODE -ne 0) {
            # Which of the two it was, so the line says what to do.
            git -C $repo ls-remote --quiet $remote HEAD 2>$null | Out-Null
            if ($LASTEXITCODE -ne 0) { return "Couldn't send the play logs: GitHub could not be reached. $kept" }
            return "Couldn't send the play logs: git on this PC is not signed in to GitHub. To sign in once, run: play.cmd send-logs  $kept"
        }
        $sent = Join-Path $dir "sent"
        New-Item -ItemType Directory -Force -Path $sent | Out-Null
        foreach ($f in $logs) { Move-Item -Force -LiteralPath $f.FullName -Destination $sent }
        return "Sent $($logs.Count) play log(s) to the play-logs branch."
    } catch {
        return "Couldn't send the play logs: $($_.Exception.Message) They are kept in play-logs."
    } finally {
        if ($oldIndex) { $env:GIT_INDEX_FILE = $oldIndex } else { Remove-Item Env:GIT_INDEX_FILE -ErrorAction SilentlyContinue }
    }
}

# scripts\play-selftest.ps1 loads the functions above and stops here.
if ($env:LARDER_PLAY_FUNCTIONS_ONLY) { return }

$repo = $PSScriptRoot

# `play.cmd send-logs`: only send the logs, and let git ask for a GitHub
# sign-in, which it then remembers. For the PC's first time.
if ($args.Count -gt 0 -and $args[0] -eq "send-logs") {
    $said = @(Send-PlayLogs $repo "origin" $true)[-1]
    if (-not $said) { $said = "There are no play logs to send." }
    Say $said
    exit 0
}

Set-Location -LiteralPath $repo
$host.UI.RawUI.WindowTitle = "Larder World"

# cargo from a fresh rustup install is on the user PATH, but a double-click
# can start with an older PATH; fall back to rustup's own folder.
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
}

# 1. Update. Never touch local edits: with uncommitted changes, or with no
#    network, it says so and plays the code that is already here.
if (Get-Command git -ErrorAction SilentlyContinue) {
    # The game rewrites its replay (larder-world-replay-MAP-SEED.txt) beside
    # it. Two were committed by mistake (#192) and untracked again on
    # 2026-11-15, so playing Flats seed 1 or planet 2 left a "local change"
    # and updates stopped. Set this copy's replays aside while updating and
    # put them back after, so they never block an update and are not lost.
    $replays = @(git ls-files -m -- 'larder-world-replay-*.txt')
    foreach ($f in $replays) { Copy-Item -Force $f "$f.keep"; git checkout -- $f }
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
            $before = (Get-FileHash -LiteralPath $PSCommandPath).Hash
            git pull --ff-only origin main
            $pulled = $LASTEXITCODE -eq 0
            if (-not $pulled) { Say "Couldn't update (offline?). Playing the last code you had." Yellow }
            # This script changed in the pull: run the new one. PowerShell
            # read the old one whole at the start, so without this the
            # first launch after an update runs the old steps. (11-18: the
            # owner's first launch after the play-log send landed ran a
            # script with no send, and his four logs stayed on his PC.)
            if ($pulled -and (Get-FileHash -LiteralPath $PSCommandPath).Hash -ne $before -and -not $env:LARDER_PLAY_RESTARTED) {
                foreach ($f in $replays) { if (Test-Path "$f.keep") { Move-Item -Force "$f.keep" $f } }
                Say "play.cmd itself was updated; starting the new one"
                $env:LARDER_PLAY_RESTARTED = "1"
                & powershell -NoProfile -ExecutionPolicy Bypass -File $PSCommandPath @args
                exit $LASTEXITCODE
            }
        }
    }
    foreach ($f in $replays) { if (Test-Path "$f.keep") { Move-Item -Force "$f.keep" $f } }
} else {
    Say "git not found; playing the code as it is." Yellow
}

# Logs a last send left behind (the game closed with no network, a
# window shut early, an old script): sent now, before playing.
$left = @(Send-PlayLogs $repo)[-1]
if ($left) {
    if ($left.StartsWith("Sent")) { Say $left } else { Say $left Red }
}

# 2. The game and the map editor (the menu's Edit maps button starts
#    larder-edit from beside the game): CI's build when it is this code
#    (Get-CiBuild), else a build here. Cargo draws its own progress bar; the
#    first build takes a few minutes, later ones only rebuild what changed.
# (The last thing a function writes is its answer; anything a command
# inside it let slip comes before.)
$bin = @(Get-CiBuild $repo)[-1]
if ($bin) {
    Say "Using the game CI built for this code"
} else {
    Say "Building (the first time takes a few minutes)"
    cargo build --release --bin larder-world --bin larder-edit
    if ($LASTEXITCODE -ne 0) {
        Say "The build failed. The messages above say why." Red
        exit 1
    }
    $bin = Join-Path $repo "target\release"
}

# 3. Play. Run from the repo root so files the game reads and writes land
#    here; LARDER_ROOT tells a build made on another machine (CI's) where
#    the maps and missions are.
Say "Starting Larder World" Green
$env:LARDER_ROOT = $repo
& (Join-Path $bin "larder-world.exe") @args
$code = $LASTEXITCODE

# 4. Send the play logs. After the game has quit, so it never holds up
#    play, and it says plainly when it could not send.
$said = @(Send-PlayLogs $repo)[-1]
if ($said) {
    # A failure waits for Enter, so it is read: the logs are how the
    # threads see a play session (11-18: a silent miss lost one).
    if ($said.StartsWith("Sent")) { Say $said } else { Say $said Red; Read-Host "Press Enter to close" | Out-Null }
}
exit $code
