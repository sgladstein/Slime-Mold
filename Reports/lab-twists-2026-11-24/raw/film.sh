cd /home/claude/Slime-Mold
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
for a in base cold night wind; do
 ( case $a in cold) export LAB_TWIST=cold:10:60;; night) export LAB_TWIST=night:10:60;; wind) export LAB_TWIST=wind:10:60:0.35:0;; esac
   ./target/release/larder-world look=/tmp/claude-0/lab/twistfilm/$a run=1:1 mode=squads hand=idle at=5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70 > /tmp/claude-0/lab/twistfilm/$a.log 2>&1 ) &
done
wait
echo FILM_DONE > /tmp/claude-0/lab/twistfilm/done.txt
