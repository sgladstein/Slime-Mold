"""Joins a film's near frames (larder-world look=DIR ... at=...) into an mp4:
half a second a frame, a quarter round the burst (59-67 s). Usage:
python3 make_clips.py DIR OUT.mp4"""
import os, re, subprocess, sys

d, out = sys.argv[1], sys.argv[2]
def t(f):
    m = re.search(r'-(\d+)(ms|s)-near', f)
    v = int(m.group(1))
    return v / 1000 if m.group(2) == 'ms' else v
fs = sorted((f for f in os.listdir(d) if f.endswith('-near.png')), key=t)
with open(os.path.join(d, 'list.txt'), 'w') as lst:
    for f in fs:
        lst.write(f"file '{os.path.abspath(os.path.join(d, f))}'\nduration {0.25 if 59 <= t(f) <= 67 else 0.5}\n")
    lst.write(f"file '{os.path.abspath(os.path.join(d, fs[-1]))}'\n")
subprocess.run(['ffmpeg', '-y', '-loglevel', 'error', '-f', 'concat', '-safe', '0', '-i', os.path.join(d, 'list.txt'),
                '-vf', 'fps=12,format=yuv420p', '-c:v', 'libx264', '-crf', '26', out], check=True)
