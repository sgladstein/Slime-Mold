# The looks on the GTX 1070 (Scott-Desktop). PowerShell, from the repo root,
# with two builds: target\release\mouldgpu-main.exe from main before the
# looks (cf06d9c) and target\release\mouldgpu.exe with them.
# (1) What the looks' code costs when every look is off: keys 1 and 4 on
# both builds, alternating, three rounds. (2) The seven looks presets
# (keys 9, 0, Shift+1-5 are preset=9..15), once each with the per-pass
# profile. (3) Key 1 (100,000 walkers, the owner's pick) with each kind of
# look switched on.
$new = ".\target\release\mouldgpu.exe"
$old = ".\target\release\mouldgpu-main.exe"
$out = "target\sandbox-looks\raw"
$pics = "target\sandbox-looks\pics"
New-Item -ItemType Directory -Force $out, $pics | Out-Null
function Run($bin, $name, [string[]]$a) {
    & $bin @a "seed=1" "look=$pics" "seconds=10" "tag=$name" *> "$out\$name.txt"
}
foreach ($r in 1..3) {
    foreach ($p in 1, 4) {
        Run $old "main-p$p-r$r" @("preset=$p")
        Run $new "looks-p$p-r$r" @("preset=$p")
    }
}
foreach ($p in 9..15) { Run $new "p$p" @("preset=$p", "twice=1") }
Run $new "p1-trailset" @("preset=1", "trail_scale=0.02", "turn_by=-20:1", "step_by=1:1")
Run $new "p1-floor-wander" @("preset=1", "floor=0.25", "wander=0.01")
Run $new "p1-contagion" @("preset=1", "sick=120", "immune=480", "catch=0.5", "sick_trail=3", "seed_at=4")
