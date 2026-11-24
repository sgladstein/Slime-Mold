import sys, numpy as np, os
d=sys.argv[1]; S0=float(sys.argv[2]) if len(sys.argv)>2 else 50
a=open(f'{d}/dump/agar.bin','rb').read(); nl=a.index(b'\n'); w,h=map(int,a[:nl].split())
n=len([f for f in os.listdir(d+'/dump') if f.startswith('occ')])
for s in list(range(10,n,10))+[n-1]:
    o=np.frombuffer(open(f'{d}/dump/occ-{s:04d}.bin','rb').read(),np.uint8).reshape(h,w).astype(int)
    sc=np.frombuffer(open(f'{d}/dump/scar-{s:04d}.bin','rb').read(),np.float32).reshape(h,w)
    print(f"{d} t={s:3d} walkers={o.sum():6d} on scar>={S0:g}: {o[sc>=S0].sum():5d} (cells {int((sc>=S0).sum())}) near squad (60 cells of 770,400): {o[340:460,710:830].sum():5d}")
