#!/bin/bash
# film.sh BIN OUTDIR NAME "ARGS" START N FPS
BIN=$1; OUT=$2; NAME=$3; ARGS=$4; START=$5; N=$6; FPS=$7
T=$(python3 -c "import sys; a,n,f=float(sys.argv[1]),int(sys.argv[2]),float(sys.argv[3]); print(','.join(f'{a+i/f:.4f}' for i in range(n)))" $START $N $FPS)
D=$OUT/$NAME
rm -rf $D; mkdir -p $D
timeout 1500 $BIN look=$D $ARGS at=$T screen=960x600 > $D/log.txt 2>&1
python3 - "$D" "$FPS" <<'PY'
import sys, glob, re
from PIL import Image
d, fps = sys.argv[1], float(sys.argv[2])
def t_of(f):
    m = re.search(r'-(\d+)ms-near', f)
    if m: return int(m.group(1))
    m = re.search(r'-(\d+)s-near', f)
    return int(m.group(1)) * 1000
fs = sorted(glob.glob(d + '/*-near.png'), key=t_of)
ims = [Image.open(f).convert('RGB') for f in fs]
for i, im in enumerate(ims):
    im.save(f'{d}/f{i:03d}.png')
ims[0].save(f'{d}/film.gif', save_all=True, append_images=ims[1:], duration=int(1000 / fps), loop=0)
print(d, len(ims))
PY
grep -E "t [0-9]+ s:" $D/log.txt | tail -1
