# The review's re-check on the GTX 1070

`run-1070.ps1` (one level up) at build 9d4a2d1 on Scott-Desktop, 2026-11-11
about 01:15Z, through Remote Control. `raw/` holds its 56 output files;
`clocks.txt` the window runs with the GPU's clocks logged.

The files were copied out of the session by hand, so they were checked by
hash: for each file, the first 16 hex digits of SHA-256 over its text with
line ends normalised and trailing space trimmed, computed on the PC and
here. All 56 match (two slips in `respawn-s8.txt`, a width of 7.85 for 7.83
and a jump of 0.055 for 0.056, were found this way and corrected from the
PC's file).
