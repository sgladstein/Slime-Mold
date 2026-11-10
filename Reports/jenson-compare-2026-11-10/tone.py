"""Draw a raw .f32 dump (trail then walker counts, 1024 x 640 each) with a
chosen curve. Usage: tone.py file.f32 out.png field curve [param]
field: trail|count; curve: knee K (1-exp(-v/K)) | pow G (v/p99.9)^G | log"""
import sys
import numpy as np
from PIL import Image
f, out, field, curve = sys.argv[1:5]
par = float(sys.argv[5]) if len(sys.argv) > 5 else None
w, h = 1024, 640
a = np.fromfile(f, dtype='<f4')
v = a[:w * h] if field == 'trail' else a[w * h:]
v = v.reshape(h, w)
if curve == 'knee':
    g = 1 - np.exp(-v / par)
elif curve == 'pow':
    ref = np.percentile(v[v > 0], 99.9)
    g = np.clip(v / ref, 0, 1) ** par
elif curve == 'log':
    ref = np.percentile(v[v > 0], 99.9)
    g = np.clip(np.log1p(v / ref * 255) / np.log1p(255), 0, 1)
Image.fromarray((g * 255).astype(np.uint8)).save(out)
