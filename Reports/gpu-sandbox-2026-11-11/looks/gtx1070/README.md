# The looks on the GTX 1070

`run-1070.ps1` (one level up) on Scott-Desktop, 2026-11-11 about 03:15Z,
through Remote Control: build b1f0383 (`mouldgpu.exe`, with the looks)
and build cf06d9c (`mouldgpu-main.exe`, main just before them). `raw/`
holds its 22 output files: `main-*` and `looks-*` are keys 1 and 4 on
each build, three rounds alternating; `p9.txt` to `p15.txt` the seven
looks presets (keys 9, 0, Shift+1-5); `p1-*` key 1 with one kind of look
switched on.

The files came back as text, so they were checked by hash: for each file,
the first 16 hex digits of SHA-256 over its text with line ends normalised
and trailing space trimmed, computed on the PC and here. All 22 match.
