"""Each film's near frames, in time order, to an mp4 (15 frames a game
second, played at real time), the HUD's bottom panel cropped off.
Usage: python3 clip.py OUT PAGE_DIR"""
import glob, os, re, subprocess, sys, tempfile, shutil

def t(f):
    m = re.search(r'-(\d+)(ms|s)-near', f)
    v = int(m.group(1))
    return v / 1000 if m.group(2) == 'ms' else v

out, page = sys.argv[1], sys.argv[2]
for d in sorted(glob.glob(os.path.join(out, '*'))):
    fs = sorted(glob.glob(os.path.join(d, '*-near.png')), key=t)
    if not fs:
        continue
    tmp = tempfile.mkdtemp()
    for i, f in enumerate(fs):
        os.symlink(os.path.abspath(f), os.path.join(tmp, f'{i:04d}.png'))
    name = os.path.basename(d)
    subprocess.run(['ffmpeg', '-loglevel', 'error', '-y', '-framerate', '15', '-i', os.path.join(tmp, '%04d.png'),
                    '-vf', 'crop=960:450:0:0', '-c:v', 'libx264', '-crf', '23', '-pix_fmt', 'yuv420p',
                    os.path.join(page, name + '.mp4')], check=True)
    subprocess.run(['ffmpeg', '-loglevel', 'error', '-y', '-i', fs[len(fs) // 3], '-vf', 'crop=960:450:0:0,scale=480:-2',
                    '-q:v', '5', os.path.join(page, name + '.jpg')], check=True)
    shutil.rmtree(tmp)
    print(name, len(fs), 'frames')
