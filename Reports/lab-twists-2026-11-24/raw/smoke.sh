cd /home/claude/Slime-Mold
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
one() { a=$1; h=$2; ( case $a in cold) export LAB_TWIST=cold:20:60;; night) export LAB_TWIST=night:20:60;; wind) export LAB_TWIST=wind:20:60:0.35:0;; esac
   ./target/release/larder-world look=/tmp/claude-0/lab/twistsmoke/$a-$h run=1:1 mode=squads hand=$h at=1,20,60,90,120,180 > /tmp/claude-0/lab/twistsmoke/$a-$h.log 2>&1 ) }
for a in base cold night wind; do one $a greedy & done; wait
for a in base cold night wind; do one $a idle & done; wait
echo SMOKE_DONE > /tmp/claude-0/lab/twistsmoke/done.txt
