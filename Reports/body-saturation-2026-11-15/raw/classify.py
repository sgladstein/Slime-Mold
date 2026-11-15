import sys
from PIL import Image
import numpy as np
cls = {'thin (x<1)':(30,60,200),'mid (1-3)':(40,180,60),'sat (3-6)':(230,210,40),'sat (6-12)':(230,60,30),'sat (>12)':(255,255,255)}
for f in sys.argv[1:]:
    a = np.asarray(Image.open(f).convert('RGB')).astype(int)
    a = a[60:510, :]  # drop HUD rows
    best = None
    cnt = {}
    for k,c in cls.items():
        d = np.abs(a - np.array(c)).sum(-1)
        cnt[k] = d
    names = list(cls)
    D = np.stack([cnt[k] for k in names])
    idx = D.argmin(0); ok = D.min(0) < 90
    tot = ok.sum()
    out = {names[i]: int(((idx==i)&ok).sum()) for i in range(len(names))}
    print(f.split('/')[-1], 'body px', tot, ' '.join(f"{k}:{100*v/max(tot,1):.1f}%" for k,v in out.items()))
