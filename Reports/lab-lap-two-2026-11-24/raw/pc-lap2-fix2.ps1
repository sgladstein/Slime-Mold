# LAB (never merges): lap two's smoke checks, round two, part 2, on Scott's PC. Run seed 1,
# LAB_LAP2_SIZE 0.7, bank 3000. Run from a clone of claude/project-thread-2o4cic after
#   cargo build --release --locked --bin larder-world
# Not a timing run. 10 games, each 2:00 at most:
#  D. 12 and 15 alone with the strikers' trigger lowered (strikers_below 0.45, 0.5;
#     the game's is 0.6), and at LAB_LAP2_SIZE 0.5; 15 paired at 0.5.
#  E. Planet 10 with the pump 25 units outside the threads (214,680), alone and paired;
#     planet 14 paired with a stronger pump smell (40, the pair's is 20).
$ErrorActionPreference = "Continue"  # "Stop" turns the game's stderr into an error under PowerShell 5.1
$out = "C:\Users\Scott\Code\pc-batch-out\lab-lap2-fix2-20261124"
New-Item -ItemType Directory -Force -Path $out | Out-Null
$pairs = @{}
Get-Content "Reports\lab-lap-two-2026-11-24\raw\pairs.txt" | Where-Object { $_ -match '^\d+\|' } | ForEach-Object { $f = $_.Split('|'); $pairs[$f[0]] = $f[3].Split(' ', [System.StringSplitOptions]::RemoveEmptyEntries) }
$games = @(
  @{n="D12_below45"; p=12; h="greedy"; k=@("set=strikers_below=0.45")},
  @{n="D12_below50"; p=12; h="greedy"; k=@("set=strikers_below=0.5")},
  @{n="D12_size50"; p=12; h="greedy"; k=@(); size="0.5"},
  @{n="D15_below45"; p=15; h="greedy"; k=@("set=strikers_below=0.45")},
  @{n="D15_below50"; p=15; h="greedy"; k=@("set=strikers_below=0.5")},
  @{n="D15_size50"; p=15; h="greedy"; k=@(); size="0.5"},
  @{n="D15_pair_size50"; p=15; h="greedy"; k=$pairs["15"]; size="0.5"},
  @{n="E10_base"; p=10; h="idle"; k=@("pump=214,680")},
  @{n="E10_pair"; p=10; h="idle"; k=@("pump=214,680") + $pairs["10"]},
  @{n="E14_pair40"; p=14; h="idle"; k=@("pump=645,234", "set=pump_smell=40", "set=lure_r=15")}
)
foreach ($g in $games) {
  $dir = Join-Path $out $g.n
  New-Item -ItemType Directory -Force -Path $dir | Out-Null
  if ($g.p -ge 9) { $env:LAB_LAP2 = "1"; $env:LAB_LAP2_SIZE = $(if ($g.size) { $g.size } else { "0.7" }) } else { Remove-Item Env:LAB_LAP2, Env:LAB_LAP2_SIZE -ErrorAction SilentlyContinue }
  if ($g.dump) { $env:LAB_DUMP = (Join-Path $dir "dump"); $env:LAB_DUMP_LIFE = "1" } else { Remove-Item Env:LAB_DUMP, Env:LAB_DUMP_LIFE -ErrorAction SilentlyContinue }
  $k = $g.k
  $t0 = Get-Date
  & .\target\release\larder-world.exe "look=$dir" "run=1:$($g.p)" mode=squads "hand=$($g.h)" bank=3000 at=1,30,45,60,90,120 @k *> (Join-Path $dir "log.txt")
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
