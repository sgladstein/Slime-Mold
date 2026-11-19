# run_jobs.ps1 JOBS OUT [PARALLEL]: plays every line of a gun-lab job list
# (DEPTH SEED SECS GUN DECK [KEYS...]) with target\release\larder-world.exe,
# PARALLEL at a time (4), appending each game's `gunlab` line to OUT. Lines
# For Scott's PC (the 1070: about 4 s an
# arena game, 6 s a full planet-1 game).
param([string]$Jobs, [string]$Out, [int]$Parallel = 4)
$exe = Join-Path (Get-Location) 'target\release\larder-world.exe'
if (-not (Test-Path $Out)) { New-Item -ItemType File -Path $Out | Out-Null }
$todo = @()
foreach ($line in Get-Content $Jobs) {
  if ($line -match '^\s*(#|$)') { continue }
  $todo += $line
}
$running = @()
$n = 0
foreach ($line in $todo) {
  $w = $line -split '\s+'
  $a = @("gunlab=$($w[0]):$($w[1]):$($w[2])", "gun=$($w[3])", "deck=$($w[4])")
  if ($w.Length -gt 5) { $a += $w[5..($w.Length - 1)] }
  while (($running | Where-Object { -not $_.HasExited }).Count -ge $Parallel) { Start-Sleep -Milliseconds 200 }
  $tmp = [System.IO.Path]::GetTempFileName()
  $p = Start-Process -FilePath $exe -ArgumentList $a -NoNewWindow -PassThru -RedirectStandardOutput $tmp -RedirectStandardError "$tmp.err"
  $p | Add-Member -NotePropertyName Tmp -NotePropertyValue $tmp
  $running += $p
  $n++
  foreach ($r in @($running | Where-Object { $_.HasExited })) {
    Get-Content $r.Tmp | Where-Object { $_ -match '^gunlab ' } | Add-Content $Out
    Remove-Item $r.Tmp, "$($r.Tmp).err" -ErrorAction SilentlyContinue
  }
  $running = @($running | Where-Object { -not $_.HasExited })
  if ($n % 50 -eq 0) { Write-Host "$n of $($todo.Count) started" }
}
$running | ForEach-Object { $_.WaitForExit() }
foreach ($r in $running) {
  Get-Content $r.Tmp | Where-Object { $_ -match '^gunlab ' } | Add-Content $Out
  Remove-Item $r.Tmp, "$($r.Tmp).err" -ErrorAction SilentlyContinue
}
Write-Host "done: $n games"
