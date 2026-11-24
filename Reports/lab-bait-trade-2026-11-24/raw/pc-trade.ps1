# LAB (never merges): the bait-or-drain trade on planet 7, 18 games, on Scott's PC.
# Run from a worktree of claude/project-thread-2o4cic after
#   cargo build --release --locked --bin larder-world
# Not a timing run: no lock, the league may keep running.
# Each game: the squad of 8 on the crystal by its landing; one wanderer placed at
# game second 20 on the slime about 280 units away (boss=20,X,Y), no other mini-boss
# (boss_first=9999, boss_kind=2). Arms: off (hold, no feeding), bait (hold, LAB_FEED=1.0),
# drain (drain@ the crystal and buy soldiers, LAB_FEED=1.0). 2:30 a game.
$ErrorActionPreference = "Stop"
$out = "C:\Users\Scott\Code\pc-batch-out\lab-trade-20261124"
New-Item -ItemType Directory -Force -Path $out | Out-Null
$seeds = @(
  @{s=1; c="954,314"; b="682,383"}, @{s=2; c="848,93";  b="586,47"},
  @{s=3; c="952,200"; b="699,354"}, @{s=4; c="268,78";  b="586,84"},
  @{s=5; c="936,192"; b="648,332"}, @{s=6; c="858,32";  b="617,132"})
foreach ($x in $seeds) {
  foreach ($arm in @("off", "bait", "drain")) {
    $name = "p7s$($x.s)_$arm"
    $hand = if ($arm -eq "drain") { "drain@$($x.c)" } else { "hold@$($x.c)" }
    if ($arm -eq "off") { Remove-Item Env:LAB_FEED -ErrorAction SilentlyContinue } else { $env:LAB_FEED = "1.0" }
    $env:LAB_MARCH_LOG = "0.8"
    $dir = Join-Path $out $name
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    $t0 = Get-Date
    & .\target\release\larder-world.exe "look=$dir" "run=$($x.s):7" mode=squads "hand=$hand" bank=3000 at=150 set=boss_kind=2 set=boss_first=9999 "boss=20,$($x.b)" *> (Join-Path $dir "log.txt")
    $log = Get-Content (Join-Path $dir "log.txt")
    $last = ($log | Select-String -Pattern '^\s*t \d+ s: mould' | Select-Object -Last 1).Line
    $fell = ($log | Select-String -Pattern 'bossfell').Count
    $ended = ($log | Select-String -Pattern 'boss t .* ended' | ForEach-Object { $_.Line.Trim() }) -join ' | '
    $path = ($log | Select-String -Pattern '^\s*boss t (30|45|60|75|90|105|120|135|150) s: lump' | ForEach-Object { ($_.Line.Trim() -replace ', spread.*nearest soldier', '; near') }) -join "`n    "
    "== $name ($([int]((Get-Date) - $t0).TotalSeconds) s wall)`n  $($last.Trim())`n  bossfell $fell; $ended`n    $path"
  }
}
Remove-Item Env:LAB_FEED -ErrorAction SilentlyContinue
Remove-Item Env:LAB_MARCH_LOG -ErrorAction SilentlyContinue
