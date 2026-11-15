import re,sys,collections
rows=[]
for fn in sys.argv[1:]:
    for line in open(fn):
        p=line.split()
        if not p or not p[0].isdigit() or '|' not in line: continue
        m=re.search(r'\] (\S+) (\d+) \|',line)
        if not m: continue
        parts=line.split('|')
        nums=parts[1].split(); walks=parts[2].split()
        if len(walks)<3: continue
        dial=re.search(r'\[(.*?)\]',line).group(1)
        rows.append(dict(d=int(p[0]),s=int(p[1]),att=m.group(1),ms=int(m.group(2)),cr=int(nums[0]),rock=nums[1],walls=int(nums[6]),nest=int(walks[0]),first=int(walks[1]),crys=int(walks[2]),open=parts[3].strip(),dial=dial))
print(len(rows),'planets')
W={1:800,2:912,3:1024,4:1216,5:1456,6:1712,7:2048,8:2512}
for d in range(1,9):
    rs=[r for r in rows if r['d']==d]
    if not rs: continue
    w=W[d]
    near_first=[r for r in rs if r['first']<0.65*w]
    crys6=[r for r in rs if r['cr']<=7]
    far_crys=[r for r in rs if r['crys']>150]
    lowopen=[r for r in rs if int(r['open'].rstrip('%'))<70]
    print(d,len(rs),'first<0.65w',len(near_first),'crystals<=7',len(crys6),'nearest crystal>150',len(far_crys),'open<70%',len(lowopen), 'attempts>0',sum(r['att']!='0' for r in rs))
    for r in lowopen[:4]: print('   lowopen',r['s'],r['dial'],r['open'],r['walls'])
