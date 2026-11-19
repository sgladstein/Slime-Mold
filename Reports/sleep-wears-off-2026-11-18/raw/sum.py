import re,glob,sys,os
d=sys.argv[1]
rows=[]
for f in sorted(glob.glob(d+'/*.log')):
    n=os.path.basename(f)[:-4]
    t=open(f).read()
    lines=re.findall(r'^  t +(\d+) s: mould (\d+), soldiers (\d+).*?shots (\d+), killed (\d+), took (\d+)%(.*)$',t,re.M)
    sq=re.findall(r'squad: touching ([\d.]+) mean, (\d+) most; health (\d+) mean',t)
    gassed=re.findall(r'(\d+) walkers gassed',t)
    out=[n]
    for (tt,m,s,sh,k,tk,rest),q in zip(lines,sq):
        out.append(f"t{tt}: mould {m} sold {s} kill {k} hp {q[2]} touch {q[0]}/{q[1]}{' END' if 'ended' in rest else ''}")
    if gassed: out.append('gassed '+gassed[-1])
    print(' | '.join(out))
