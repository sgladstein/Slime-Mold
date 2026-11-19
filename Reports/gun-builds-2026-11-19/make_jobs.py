"""Job lists for the gun lab (2026-11-19). Each line: DEPTH SEED SECS GUN DECK
ARGS, read by .github/workflows/gun-lab.yml (ARGS: extra keys, such as
arena=9 tail=30).

python3 make_jobs.py singles > jobs/singles.txt
"""
import sys
SETS = ["standard", "slug", "needle", "buckshot"]
SHELLS = ["plain", "blast", "bomb", "gas", "germ"]
SPECIAL = ["blind", "sour", "spin", "provoke", "still", "shatter", "detonate", "flare", "torch"]
FLIGHT = ["pierce", "split", "scorch", "bait", "mist", "range", "rate", "smother", "seek", "tinder"]
NEEDS_PAYLOAD = ["wide", "scatter", "quiet", "heave", "pull", "thrift", "delay", "thick", "trigger", "expire", "multicast"]

def singles():
    decks = ["none"]
    decks += [f"{s}.{t}" for s in SHELLS for t in SETS if not (s == "plain" and t == "standard")]
    decks += SPECIAL + FLIGHT
    decks += [f"{m}+blast" for m in NEEDS_PAYLOAD]
    return decks

def main():
    stage = sys.argv[1]
    seeds = range(1, int(sys.argv[2]) + 1) if len(sys.argv) > 2 else range(1, 7)
    extra = sys.argv[3] if len(sys.argv) > 3 else "arena=9 tail=30"
    secs = sys.argv[4] if len(sys.argv) > 4 else "60"
    decks = {"singles": singles}[stage]()
    for d in decks:
        for depth in (1, 2, 3, 4):
            for s in seeds:
                print(depth, s, secs, "rifle", d, extra)

main()
