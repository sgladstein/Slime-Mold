#!/bin/sh
# The cloud check of 2026-11-14: the real larder-edit window on llvmpipe
# Vulkan under Xvfb, driven by xdotool, a frame after each step
# (apt install mesa-vulkan-drivers libxkbcommon-x11-0 xdotool).
# OUT is a scratch folder; the frames became page/window.gif.
OUT=${OUT:-/tmp/edit-run}; mkdir -p $OUT/win
export DISPLAY=:99
Xvfb :99 -screen 0 1400x900x24 & sleep 2
cargo build --release --bin larder-edit --bin larder-world
./target/release/larder-edit map=nest name=nest-edit dir=$OUT/saved seed=2 &
sleep 6; WID=$(xdotool search --name "map editor" | head -1)
xdotool windowmove $WID 0 0; xdotool windowfocus $WID; sleep 1
n=0; snap(){ sleep 0.6; import -window root -crop 1280x800+0+0 $OUT/win/f-$(printf %03d $n).png; n=$((n+1)); }
snap
xdotool key 3; xdotool mousemove 500 200; snap
xdotool mousedown 1; for x in $(seq 500 20 900); do xdotool mousemove $x $((200 + (x-500)/4)); sleep 0.02; done; xdotool mouseup 1; snap
xdotool key bracketright bracketright; xdotool key 5; xdotool mousemove 600 550; xdotool mousedown 1; for y in $(seq 550 10 700); do xdotool mousemove 600 $y; sleep 0.02; done; xdotool mouseup 1; snap
xdotool key c; xdotool mousemove 1000 500; snap; xdotool click 1; snap
xdotool key w; xdotool mousemove 950 300; xdotool mousedown 1; xdotool mousemove 1100 380; sleep 0.2; snap; xdotool mouseup 1; snap
xdotool key e; xdotool mousemove 1000 500; xdotool click 1; snap
xdotool key ctrl+z; snap
xdotool key ctrl+s; snap
# Enter saves and starts larder-world on the edited map, beside the editor.
xdotool key Return; sleep 25; import -window root $OUT/win/play.png
