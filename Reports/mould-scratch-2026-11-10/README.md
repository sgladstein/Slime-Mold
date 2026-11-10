# Mould scratch, 2026-11-10 (Keep It Alive, water and sleep, The Wild One, finer tubes)

One tarball (the mount has a file cap): `mould-scratch-2026-11-10.tgz`.

- `alive/`, `water/page/`, `wild/page/`, `fine/page/`: HTML sources and images of the four Artifact pages
  (Keep It Alive, Water and Sleep https://claude.ai/artifact/5KN1DQDHhqV6faXFwx737Y,
  The Wild One https://claude.ai/artifact/QDgynHJg8dPRKuhZf54sf9,
  Finer Mould Tubes https://claude.ai/artifact/5JT3D1sv7CwSRmYfkR6VTW).
- `tune/run.sh`, `tune/run5.sh`: census runners (level 4 / level 5), `NAME "extra args" "hands" "seeds"`,
  4-wide at RAYON_NUM_THREADS=1, one `mouldprobe` per hand x seed. Paths inside point at the old scratchpad; edit S=.
- `tune/w5.py DIR`: your share = mine/(mine+wild) per seed, stars, paired sign tests between hands.
- `tune/*/a16-HAND-SEED.txt`: raw probe output of every Wild One tuning arm (G16 = shipped settings, 16 seeds;
  G8-lamp/oats/snip/salt = one tool only; F0.6/F1.0 fights + wild repel; R0.3/R0.0 no fights).
- `fine/runs`, `fine/mid`, `fine/best`: sensor reach 9/8/7/6 census (`level=all sensor_offset=N`).
- `water/*.txt`: water-and-sleep census outputs.
