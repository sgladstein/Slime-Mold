import re,sys,glob,math,collections
# per-kind table over a census folder, bosses formed before CUT s, rows up to CUT
CUT=float(sys.argv[2]) if len(sys.argv)>2 else 300
form=re.compile(r"boss t ([\d.]+) s: (\w+) (\w+) formed on lump (\d) at (\d+),(\d+).*grade ([\d.]+)")
row=re.compile(r"boss t (\d+) s: lump (\d) (\w+) (\w+)(?:\(\([\d.]+, [\d.]+\)\)|\(Some\(\d+\)\)|\(None\))?, (\d+) members \(first (\d+), (\d+) hurt\), at (\d+),(\d+), spread ([\d.]+), stolen ([\d.]+); nearest soldier ([\d.inf]+), (\d+) alive")
end=re.compile(r"boss t (\d+) s: lump (\d) (\w+) ended (\w+) \((\d+) members of (\d+)")
march=re.compile(r"march t (\d+) s")
K=collections.defaultdict(lambda: collections.Counter())
stuck=collections.Counter()
for f in sorted(glob.glob(sys.argv[1]+'/*/log.txt')):
  live={}; marches=set()
  for l in open(f):
    m=march.search(l)
    if m: marches.add(int(m.group(1)))
    m=form.search(l)
    if m and float(m.group(1))<=CUT:
      live[m.group(4)]=dict(kind=m.group(3),rows=[],end=None); K[m.group(3)]['formed']+=1; continue
    m=row.search(l)
    if m and m.group(2) in live and int(m.group(1))<=CUT and live[m.group(2)]['end'] is None:
      g=re.search(r"Go\(\(([\d.]+), ([\d.]+)\)",l)
      if g: live[m.group(2)].setdefault('gap',[]).append(math.hypot(float(g.group(1))-int(m.group(8)),float(g.group(2))-int(m.group(9))))
      live[m.group(2)]['rows'].append((int(m.group(1)),m.group(4),int(m.group(5)),int(m.group(8)),int(m.group(9)),float(m.group(12)),float(m.group(11))))
      continue
    m=end.search(l)
    if m and m.group(2) in live and int(m.group(1))<=CUT and live[m.group(2)]['end'] is None:
      live[m.group(2)]['end']=(m.group(4),int(m.group(1)))
      b=live.pop(m.group(2)); fin(b) if False else None
      done=b
      # store
      live['done%d'%len(live)+m.group(1)+m.group(2)]=dict(b,closed=True)
  for k,b in live.items():
    r=b['rows']; kind=b['kind']; c=K[kind]
    if not r: continue
    if min(x[5] for x in r)<20: c['reached squad (<20 units)']+=1
    if max(x[6] for x in r)>0: c['took food']+=1
    if any(x[1]=='Sit' for x in r): c['got to its errand']+=1
    if b.get('gap') and min(b['gap'])<30: c['came within 30 units of its errand']+=1
    s=sum(1 for i in range(10,len(r)) if r[i][1] in('Go','Hunt') and math.hypot(r[i][3]-r[i-10][3],r[i][4]-r[i-10][4])<3)
    c['stood still 10 s+ (seconds)']+=s
    if s>=10: c['stalled 10 s or more']+=1
    e=b.get('end')
    if e:
      c['ended '+e[0]]+=1
      if e[0]=='Broken' and r[-1][2]>0 and any(abs(t-e[1])<=2 for t in marches) and r[-1][2]>0.5*max(x[2] for x in r): c['swallowed by a march']+=1
    else: c['still alive']+=1
keys=['formed','reached squad (<20 units)','came within 30 units of its errand','got to its errand','took food','stalled 10 s or more','stood still 10 s+ (seconds)','swallowed by a march','ended Broken','ended Spent','ended Fizzled','still alive']
kinds=['hunter','leech','sapper','wanderer']
print('| | '+' | '.join(kinds)+' | all |'); print('|'+' --- |'*(len(kinds)+2))
for k in keys:
  v=[K[x][k] for x in kinds]; print(f'| {k} | '+' | '.join(map(str,v))+f' | {sum(v)} |')
