# LAB (never merges): lap two's smoke checks, round two, on Scott's PC. Run seed 1,
# LAB_LAP2_SIZE 0.7, bank 3000. Run from a clone of claude/project-thread-2o4cic after
#   cargo build --release --locked --bin larder-world
# Not a timing run. 18 games, each 2:00 at most:
#  A. What wipes the greedy squad on 12 and 15 alone: as is, no mini-bosses, no
#     strikers, the tier's trait off, slow surges; and lap one's planets 4 and 7.
#  B. Planets 10 and 14, alone and paired, a pump at the slime's edge (pump=X,Y),
#     the squad idle at its landing far off.
#  C. Planet 15 alone and paired with LAB_DUMP_LIFE: walkers by state each second.
$ErrorActionPreference = "Continue"  # "Stop" turns the game's stderr into an error under PowerShell 5.1
$out = "C:\Users\Scott\Code\pc-batch-out\lab-lap2-fix-20261124"
New-Item -ItemType Directory -Force -Path $out | Out-Null
$pairs = @{}
Get-Content "Reports\lab-lap-two-2026-11-24\raw\pairs.txt" | Where-Object { $_ -match '^\d+\|' } | ForEach-Object { $f = $_.Split('|'); $pairs[$f[0]] = $f[3].Split(' ', [System.StringSplitOptions]::RemoveEmptyEntries) }
$games = @(
  @{n="A12_asis"; p=12; h="greedy"; k=@()},
  @{n="A12_noboss"; p=12; h="greedy"; k=@("set=boss_first=9999")},
  @{n="A12_nostrike"; p=12; h="greedy"; k=@("set=strikers=0")},
  @{n="A12_notrait"; p=12; h="greedy"; k=@("surge_haste=1", "set=panic_margin=0", "set=last_stand=0")},
  @{n="A12_slowsurge"; p=12; h="greedy"; k=@("surge=2400")},
  @{n="A15_asis"; p=15; h="greedy"; k=@()},
  @{n="A15_noboss"; p=15; h="greedy"; k=@("set=boss_first=9999")},
  @{n="A15_nostrike"; p=15; h="greedy"; k=@("set=strikers=0")},
  @{n="A15_notrait"; p=15; h="greedy"; k=@("scar_weight=0")},
  @{n="A15_slowsurge"; p=15; h="greedy"; k=@("surge=2400")},
  @{n="A4_lapone"; p=4; h="greedy"; k=@()},
  @{n="A7_lapone"; p=7; h="greedy"; k=@()},
  @{n="B10_base"; p=10; h="idle"; k=@("pump=275,340")},
  @{n="B10_pair"; p=10; h="idle"; k=@("pump=275,340") + $pairs["10"]},
  @{n="B14_base"; p=14; h="idle"; k=@("pump=645,234")},
  @{n="B14_pair"; p=14; h="idle"; k=@("pump=645,234") + $pairs["14"]},
  @{n="C15_base"; p=15; h="greedy"; k=@(); dump=1},
  @{n="C15_pair"; p=15; h="greedy"; k=$pairs["15"]; dump=1}
)
foreach ($g in $games) {
  $dir = Join-Path $out $g.n
  New-Item -ItemType Directory -Force -Path $dir | Out-Null
  if ($g.p -ge 9) { $env:LAB_LAP2 = "1"; $env:LAB_LAP2_SIZE = "0.7" } else { Remove-Item Env:LAB_LAP2, Env:LAB_LAP2_SIZE -ErrorAction SilentlyContinue }
  if ($g.dump) { $env:LAB_DUMP = (Join-Path $dir "dump"); $env:LAB_DUMP_LIFE = "1" } else { Remove-Item Env:LAB_DUMP, Env:LAB_DUMP_LIFE -ErrorAction SilentlyContinue }
  $k = $g.k
  $t0 = Get-Date
  & .\target\release\larder-world.exe "look=$dir" "run=1:$($g.p)" mode=squads "hand=$($g.h)" bank=3000 at=1,30,60,90,120 @k *> (Join-Path $dir "log.txt")
  $log = Get-Content (Join-Path $dir "log.txt")
  $lines = ($log | Select-String -Pattern '^\s*t \d+ s: mould' | Select-Object -First 5 | ForEach-Object { ($_.Line.Trim() -replace ', rendered \d+', '' -replace ', took \d+%', '' -replace '; squad at.*', '') }) -join "`n    "
  $fell = ($log | Select-String -Pattern 'bossfell').Count
  $formed = ($log | Select-String -Pattern 'boss t [\d.]+ s: .*formed' | ForEach-Object { $_.Line.Trim().Substring(0, [Math]::Min(70, $_.Line.Trim().Length)) }) -join ' | '
  $strk = ($log | Select-String -Pattern '^\s*strikers t' | Select-Object -Last 1).Line
  $pump = ($log | Select-String -Pattern '^\s*Pump at|^\s*pump at' | ForEach-Object { $_.Line.Trim() }) -join ' | '
  "== $($g.n) ($([int]((Get-Date) - $t0).TotalSeconds) s wall)`n    $lines`n    bossfell $fell; formed: $formed`n    last strikers: $($strk)`n    pumps: $pump"
  if ($g.dump) { "    life (sec walker spore dormant dead):"; Get-Content (Join-Path $dir "dump\life.txt") | Where-Object { $_ -match '^(1|30|60|90|119) ' } | ForEach-Object { "      $_" } }
}
Remove-Item Env:LAB_LAP2, Env:LAB_LAP2_SIZE, Env:LAB_DUMP, Env:LAB_DUMP_LIFE -ErrorAction SilentlyContinue
