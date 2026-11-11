# The review re-check on the GTX 1070 (Scott-Desktop): run.sh's arms, plus
# every preset once with the per-pass profile. PowerShell, from the repo
# root, after `cargo build --release --locked --bin mouldgpu`.
$bin = ".\target\release\mouldgpu.exe"
$out = "target\sandbox-review\raw"
$pics = "target\sandbox-review\pics"
New-Item -ItemType Directory -Force $out, $pics | Out-Null
function Run($name, $seed, [string[]]$a) {
    & $bin @a "seed=$seed" "look=$pics" "seconds=10" "tag=$name-s$seed" *> "$out\$name-s$seed.txt"
}
foreach ($p in 1..8) { Run "p$p" 1 @("preset=$p", "twice=1") }
foreach ($s in 1..16) { Run "fed32k" $s @("preset=2") }
foreach ($s in 1..8) { Run "crowd1" $s @("preset=1", "food=0") }
foreach ($s in 1..8) { Run "crowd0" $s @("preset=3") }
foreach ($s in 1..8) { Run "respawn" $s @("preset=5") }
foreach ($s in 1..8) { Run "jenson" $s @("preset=4") }
