# Round 2 films (2026-11-16): the round-3 mini-boss (staged at 1:25 round
# world 310,85 on run 1, planet 3, forced to each kind), each kind in its
# own look, camera following it (the oldest, live or fading) at zoom 12.
# Clips by Reports/mini-boss-2026-11-16/raw/round2-clip.py.
cd "$(dirname "$0")/../../.."
run() { d=/tmp/mbl/$1; shift; at=$1; shift; rm -rf $d; mkdir -p $d; ./target/release/larder-world look=$d at=$at screen=960x600 run=1:3 mode=squads hand=hold boss=85,310,85 follow=boss "$@" > $d/log.txt 2>&1; }
# Hunter: hunts and lunges from 91 s, shot apart at the squad 110-113 s.
run hunter $(seq -s, 92 0.25 113) set=boss_kind=1 centre=375,175,12
# Wanderer: roots on a crystal at 117 s, takes in the slime there (116 -> 233).
run wanderer $(seq -s, 112 0.5 160) set=boss_kind=2 centre=300,180,12
# Sapper: on the wall 100-165 s, through and hunting, spent at 182 s (196
# members): its look drains over the 1.5 s fade.
run sapper $(seq -s, 100 1 159),$(seq -s, 160 0.25 186) set=boss_kind=3 wall=290,150,350,150 centre=330,160,12
# Leech: on the pump from 115 s; its take 10 -> 257 food by 165 s.
run leech $(seq -s, 112 1 170) set=boss_kind=4 pump=362,256,86 set=pump_hp=5000 centre=345,200,12
