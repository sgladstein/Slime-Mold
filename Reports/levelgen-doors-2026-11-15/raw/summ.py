"""Per biome: nests with a direct way <=10 wide, some way >=16 wide, and both (localchoke.tsv)."""
import sys, csv
bio = {1: 'meadow', 2: 'barrens', 3: 'fen', 4: 'valley', 5: 'jungle', 6: 'highlands', 7: 'caves', 8: 'core'}
rows = list(csv.DictReader(open(sys.argv[1]), delimiter='\t'))
tot = [0, 0]
for d in range(1, 9):
    rs = [r for r in rows if int(r['name'].split('-')[1]) == d]
    nar = sum(0 < float(r['direct_width']) <= 10 for r in rs)
    wide = sum(float(r['widest']) >= 16 for r in rs)
    both = sum(0 < float(r['direct_width']) <= 10 and float(r['widest']) >= 16 for r in rs)
    tot[0] += both; tot[1] += len(rs)
    print(f"{d} {bio[d]:9} nests {len(rs):2}: direct <=10 {nar:2}; some way >=16 {wide:2}; both {both:2} | " + ' '.join(f"{r['direct_width']}/{r['widest']}" for r in rs))
print('all nests with both:', tot)
