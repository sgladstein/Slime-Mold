# Round 3 films (2026-11-16): one mini-boss staged at 1:25 round world
# 310,85 on run 1, planet 3, forced to each kind; frames every 0.5 s, then
# raw/round2-clip.py at 10 fps (15 for the sapper) and crf 27.
cd "$(dirname "$0")/../../.."
run() { d=/tmp/mb/f$1; to=$2; rm -rf $d; mkdir -p $d; shift 2; ./target/release/larder-world look=$d at=$(seq -s, 85 0.5 $to) screen=960x600 run=1:3 mode=squads hand=hold boss=85,310,85 "$@" > $d/log.txt 2>&1; }
run 1 125 set=boss_kind=1 centre=375,175,2.8
run 2 175 set=boss_kind=2 centre=300,180,2.8
run 3 215 set=boss_kind=3 wall=290,150,350,150 centre=330,160,3.2
# The leech ring (after #391): the leech clip was filmed again, closer.
run 4 170 set=boss_kind=4 pump=362,256,86 set=pump_hp=5000 centre=355,235,3.2
