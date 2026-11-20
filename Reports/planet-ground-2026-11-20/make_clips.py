"""Joins two films' near frames (larder-world look=DIR ... at=40,40.2,..,44
centre=X,Y,Z, one with main's view shader and one with this change's, by
LARDER_VIEW_WGSL) into one side-by-side mp4 at real speed, before on the
left. Usage: python3 make_clips.py BEFORE_DIR AFTER_DIR OUT.mp4"""
import os, re, subprocess, sys

a, b, out = sys.argv[1], sys.argv[2], sys.argv[3]
def t(f):
    m = re.search(r'-(\d+)(ms|s)-near', f)
    v = int(m.group(1))
    return v / 1000 if m.group(2) == 'ms' else v
def lst(d):
    fs = sorted((f for f in os.listdir(d) if f.endswith('-near.png')), key=t)
    p = os.path.join(d, 'list.txt')
    with open(p, 'w') as l:
        for f in fs:
            l.write(f"file '{os.path.abspath(os.path.join(d, f))}'\nduration 0.2\n")
        l.write(f"file '{os.path.abspath(os.path.join(d, fs[-1]))}'\n")
    return p
subprocess.run(['ffmpeg', '-y', '-loglevel', 'error', '-f', 'concat', '-safe', '0', '-i', lst(a), '-f', 'concat', '-safe', '0', '-i', lst(b),
                '-filter_complex', '[0]crop=1280:700:0:40,scale=640:-2[l];[1]crop=1280:700:0:40,scale=640:-2[r];[l][r]hstack,fps=15,format=yuv420p',
                '-c:v', 'libx264', '-crf', '24', out], check=True)
subprocess.run(['ffmpeg', '-y', '-loglevel', 'error', '-sseof', '-1', '-i', out, '-frames:v', '1', '-q:v', '4', out.replace('.mp4', '.jpg')], check=True)
