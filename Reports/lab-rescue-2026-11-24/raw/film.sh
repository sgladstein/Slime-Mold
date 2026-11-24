cd /home/claude/Slime-Mold
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
D=/tmp/claude-0/lab/rescuefilm
seqs(){ seq -s, $1 $2; }
( LAB_RESCUE=40,150 ./target/release/larder-world look=$D/win run=1:1 mode=squads hand=rescue@40,150,90 at=$(seqs 86 152) > $D/win.log 2>&1 ) &
( LAB_RESCUE=40,150 ./target/release/larder-world look=$D/late2 run=1:1 mode=squads hand=rescue@40,150,180 at=$(seqs 184 210) centre=62,150,6 > $D/late2.log 2>&1 ) &
( LAB_RESCUE=40,150 ./target/release/larder-world look=$D/greedy run=1:1 mode=squads hand=greedy at=$(seqs 240 282) centre=40,150,6 > $D/greedy.log 2>&1 ) &
wait
echo FILM_DONE > $D/done.txt
