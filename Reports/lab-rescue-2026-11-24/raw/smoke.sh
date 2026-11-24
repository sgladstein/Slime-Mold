# The smoke games (planet 1, run seed 1, lavapipe; the lab branch's larder-world).
cd /home/claude/Slime-Mold
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
run(){ d=/tmp/claude-0/lab/rescue16/$1; mkdir -p $d; LAB_RESCUE=40,150 ./target/release/larder-world look=$d run=1:1 mode=squads hand=$2 at=60,240,480 > $d/log.txt 2>&1 & }
run idle idle; run greedy greedy; run go0 rescue@40,150; run go90 rescue@40,150,90; run go180 rescue@40,150,180
wait
