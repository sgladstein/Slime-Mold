# One body census raw (2026-11-12, 07:45-10:15Z)

`larderworld census` at ff9aa09 (main after #106 plus the `energy_flow=` and
`hunger_scent=` keys), all five hands, cap 900 s, CPU. `base-*` is today's
mould and `body-*` is `energy_flow=0.05 hunger_scent=8`.

- Race seeds 5-10, 11-15 and 16-20, base: run by runners 1-3 and relayed by
  message. `base-5-10` and `base-16-20` give the rules line once. `base-16-20`
  and `body-16-20` are as the runner sent them: full rows for the base, and
  rows cut at "crystals left" for the body.
- Race body 5-10 and 11-15, and Fork 5-12 both arms: run in this thread's
  container (4 cores).
- `pair.py BASE,... BODY,...` pairs the two arms by hand and seed and checks
  the gate inside each arm.
