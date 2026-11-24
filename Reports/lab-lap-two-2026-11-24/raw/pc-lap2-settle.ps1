# LAB (never merges): the strikers' settled peak (set=strikers_settle=60, code on the lab
# branch) and planet 10's new second trait, on Scott's PC. Bank 3000, greedy squad.
# Run from a clone of claude/project-thread-2o4cic after
#   cargo build --release --locked --bin larder-world
# Not a timing run. 24 games:
#  F. Planets 12 and 15 alone, LAB_LAP2_SIZE 0.7, run seeds 1 and 2, 2:00: settled peak on
#     (no strikers_below patch), and seed 2 as it is.
#  G. Planets 2 to 8 (lap one, the game's own rows), run seed 1, 3:00: as on main and with
#     the settled peak on, paired.
#  H. Planet 10, run seeds 1 and 2, 2:00: alone and with its new second trait (the
#     Drifter's spore bursts, pairs.txt), with the life rows.
$ErrorActionPreference = "Continue"  # "Stop" turns the game's stderr into an error under PowerShell 5.1
$out = "C:\Users\Scott\Code\pc-batch-out\lab-lap2-settle-20261124"
New-Item -ItemType Directory -Force -Path $out | Out-Null
$pairs = @{}
Get-Content "Reports\lab-lap-two-2026-11-24\raw\pairs.txt" | Where-Object { $_ -match '^\d+\|' } | ForEach-Object { $f = $_.Split('|'); $pairs[$f[0]] = $f[3].Split(' ', [System.StringSplitOptions]::RemoveEmptyEntries) }
$settle = "set=strikers_settle=60"
$two = "at=1,30,45,60,90,120"
$three = "at=1,30,60,90,120,150,180"
$games = @(
  @{n="F12_s1_settle"; p=12; s=1; k=@($settle); at=$two},
  @{n="F15_s1_settle"; p=15; s=1; k=@($settle); at=$two},
  @{n="F12_s2_asis"; p=12; s=2; k=@(); at=$two},
  @{n="F12_s2_settle"; p=12; s=2; k=@($settle); at=$two},
  @{n="F15_s2_asis"; p=15; s=2; k=@(); at=$two},
  @{n="F15_s2_settle"; p=15; s=2; k=@($settle); at=$two}
)
foreach ($p in 2..8) {
  $games += @{n="G$($p)_main"; p=$p; s=1; k=@(); at=$three}
  $games += @{n="G$($p)_settle"; p=$p; s=1; k=@($settle); at=$three}
}
foreach ($s in 1..2) {
  $games += @{n="H10_s$($s)_base"; p=10; s=$s; k=@(); at=$two; dump=1}
  $games += @{n="H10_s$($s)_spores"; p=10; s=$s; k=$pairs["10"]; at=$two; dump=1}
}
foreach ($g in $games) {
  $dir = Join-Path $out $g.n
  New-Item -ItemType Directory -Force -Path $dir | Out-Null
  if ($g.p -ge 9) { $env:LAB_LAP2 = "1"; $env:LAB_LAP2_SIZE = "0.7" } else { Remove-Item Env:LAB_LAP2, Env:LAB_LAP2_SIZE -ErrorAction SilentlyContinue }
  if ($g.dump) { $env:LAB_DUMP = (Join-Path $dir "dump"); $env:LAB_DUMP_LIFE = "1" } else { Remove-Item Env:LAB_DUMP, Env:LAB_DUMP_LIFE -ErrorAction SilentlyContinue }
  $k = $g.k
  $t0 = Get-Date
  & .\target\release\larder-world.exe "look=$dir" "run=$($g.s):$($g.p)" mode=squads hand=greedy bank=3000 $g.at @k *> (Join-Path $dir "log.txt")
  $log = Get-Content (Join-Path $dir "log.txt")
  $lines = ($log | Select-String -Pattern '^\s*t \d+ s: mould' | Select-Object -First 8 | ForEach-Object { ($_.Line.Trim() -replace ', rendered \d+', '' -replace ', took \d+%', '' -replace '; squad at.*', '') }) -join "`n    "
  $fell = ($log | Select-String -Pattern 'bossfell').Count
  $formed = ($log | Select-String -Pattern 'boss t [\d.]+ s: .*formed' | ForEach-Object { $_.Line.Trim().Substring(0, [Math]::Min(70, $_.Line.Trim().Length)) }) -join ' | '
  $first = ($log | Select-String -Pattern '^\s*strikers t' | Where-Object { $_.Line -notmatch ': 0 condensed holding 0, 0 taken' } | Select-Object -First 1).Line
  $strk = ($log | Select-String -Pattern '^\s*strikers t' | Select-Object -Last 1).Line
  "== $($g.n) ($([int]((Get-Date) - $t0).TotalSeconds) s wall)`n    $lines`n    bossfell $fell; formed: $formed`n    first strikers: $($first)`n    last strikers: $($strk)"
  if ($g.dump) { "    life (sec walker spore dormant dead):"; Get-Content (Join-Path $dir "dump\life.txt") | Where-Object { $_ -match '^(1|15|30|45|60|90|119) ' } | ForEach-Object { "      $_" } }
}
Remove-Item Env:LAB_LAP2, Env:LAB_LAP2_SIZE, Env:LAB_DUMP, Env:LAB_DUMP_LIFE -ErrorAction SilentlyContinue
