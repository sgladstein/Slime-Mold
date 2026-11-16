import sys, re, os, subprocess, glob
src, out, fps = sys.argv[1], sys.argv[2], float(sys.argv[3])
tag = sys.argv[4] if len(sys.argv) > 4 else 'near'
lo = float(sys.argv[5]) if len(sys.argv) > 5 else 0
hi = float(sys.argv[6]) if len(sys.argv) > 6 else 1e9
def t(p):
    m = re.search(r'-(\d+)(s|ms)-' + tag + r'\.png$', p)
    v = int(m.group(1))
    return v / 1000 if m.group(2) == 'ms' else v
fs = sorted((p for p in glob.glob(os.path.join(src, '*-' + tag + '.png')) if lo <= t(p) <= hi), key=t)
lst = out + '.txt'
with open(lst, 'w') as f:
    for p in fs:
        f.write(f"file '{p}'\nduration {1/fps}\n")
    f.write(f"file '{fs[-1]}'\n")
subprocess.run(['ffmpeg', '-y', '-loglevel', 'error', '-f', 'concat', '-safe', '0', '-i', lst, '-c:v', 'libx264', '-pix_fmt', 'yuv420p', '-crf', '23', '-r', str(max(fps, 10)), out], check=True)
print(out, len(fs), 'frames', t(fs[0]), '-', t(fs[-1]))
