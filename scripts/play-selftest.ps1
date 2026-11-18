# Checks play.ps1's own functions on Windows (CI: .github/workflows/
# windows-build.yml, in Windows PowerShell 5.1, the shell play.cmd uses).
# Nobody can test a double-click script from a cloud session, and the one
# PC that runs it is the owner's, where a broken script costs him a play
# session. So the parts that can go wrong unseen are run here against
# throwaway repositories: sending the play logs to a branch, and falling
# back to a local build when CI's build cannot be used.
#
#   powershell -NoProfile -ExecutionPolicy Bypass -File scripts\play-selftest.ps1

$env:LARDER_PLAY_FUNCTIONS_ONLY = "1"
. (Join-Path $PSScriptRoot "..\play.ps1")
Remove-Item Env:LARDER_PLAY_FUNCTIONS_ONLY

$script:failed = $false
function Check($ok, $what) {
    if ($ok) { Write-Host "ok: $what" } else { Write-Host "FAILED: $what" -ForegroundColor Red; $script:failed = $true }
}

$tmp = Join-Path ([IO.Path]::GetTempPath()) ("larder-selftest-" + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Path $tmp | Out-Null
$remote = Join-Path $tmp "remote.git"
$work = Join-Path $tmp "work"
git init --quiet --bare $remote
git init --quiet $work
git -C $work remote add origin $remote
$logs = Join-Path $work "play-logs"
function Log($name, $text) { Set-Content -LiteralPath (Join-Path $logs $name) -Value $text }

# No folder and an empty folder: nothing is said and nothing is made.
Check ((@(Send-PlayLogs $work)[-1]) -eq "") "no play-logs folder: nothing said"
New-Item -ItemType Directory -Path $logs | Out-Null
Check ((@(Send-PlayLogs $work)[-1]) -eq "") "an empty folder: nothing said"

# Two logs, and no play-logs branch yet: the branch is made.
Log "2026-11-17T21-03-11Z-planet-1-7.txt" "log map=planet:1:7 seed=7`nt=1 mould=5"
Log "2026-11-17T21-15-40Z-planet-2-7.txt" "log map=planet:2:7 seed=7`nt=1 mould=9"
$said = @(Send-PlayLogs $work)[-1]
Check ($said -like "Sent 2 play log*") "two logs sent: '$said'"
$first = git -C $remote rev-parse --verify --quiet refs/heads/play-logs
Check ([bool]$first) "the play-logs branch exists on the remote"
$files = @(git -C $remote ls-tree -r --name-only play-logs)
Check (($files -join ",") -eq "logs/2026-11-17T21-03-11Z-planet-1-7.txt,logs/2026-11-17T21-15-40Z-planet-2-7.txt") "the branch holds the two logs: $($files -join ',')"
Check ((git -C $remote show "play-logs:logs/2026-11-17T21-03-11Z-planet-1-7.txt" | Select-Object -First 1) -eq "log map=planet:1:7 seed=7") "a log reads back as written"
Check ((git -C $remote log -1 --format=%an play-logs) -eq "Larder World play log") "the commit carries the fixed name, not the PC's"
Check (@(Get-ChildItem -LiteralPath $logs -Filter *.txt -File).Count -eq 0) "the sent logs left the folder"
Check (@(Get-ChildItem -LiteralPath (Join-Path $logs "sent") -Filter *.txt -File).Count -eq 2) "and are kept in play-logs\sent"
Check (-not (git -C $work status --porcelain --untracked-files=no)) "the checkout's own index is untouched"
Check (-not $env:GIT_INDEX_FILE) "no index setting is left behind"

# A later round: one commit on top, the earlier logs still there.
Log "2026-11-18T09-00-00Z-planet-3-7.txt" "log map=planet:3:7 seed=7"
$said = @(Send-PlayLogs $work)[-1]
Check ($said -like "Sent 1 play log*") "a later log sent: '$said'"
Check ((git -C $remote rev-parse "play-logs^") -eq $first) "it sits on top of the first commit"
Check (@(git -C $remote ls-tree -r --name-only play-logs).Count -eq 3) "the branch holds all three"

# A remote that cannot be reached: it says so, and the log stays.
git -C $work remote set-url origin (Join-Path $tmp "nowhere.git")
Log "2026-11-18T10-00-00Z-planet-1-9.txt" "log map=planet:1:9 seed=9"
$said = @(Send-PlayLogs $work)[-1]
Check ($said -like "Couldn't send the play logs: GitHub could not be reached*kept in play-logs*") "an unreachable remote is said plainly: '$said'"
Check (Test-Path (Join-Path $logs "2026-11-18T10-00-00Z-planet-1-9.txt")) "and the log is kept for next time"

# CI's build is not used for a checkout it was not built from (this
# throwaway one), with or without a network: the answer is "build here".
Set-Content -LiteralPath (Join-Path $work "a.txt") -Value "x"
git -C $work add a.txt
git -C $work -c user.name=t -c user.email=t@example.com commit --quiet -m "a"
Check ($null -eq (@(Get-CiBuild $work)[-1])) "CI's build is refused for other code"

Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
if ($script:failed) { Write-Host "play.ps1 self-test FAILED" -ForegroundColor Red; exit 1 }
Write-Host "play.ps1 self-test passed"
