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

# Stage 2 (2026-11-19), from the singles on the 1070: the carriers are
# the best payload of each family alone (needle blast, standard blast,
# needle plain, needle germ, gas) and every card that needs or adds to a
# payload goes in front of each; then payload pairs in turn, the readers
# after their setters, and the trigger and multicast with two payloads.
CARRIERS = ["blast.needle", "blast.standard", "plain.needle", "germ.needle", "gas.standard"]
SECOND = ["wide", "scatter", "quiet", "heave", "pull", "delay", "thick", "pierce", "split", "scorch", "bait", "mist", "range", "rate", "smother", "seek", "tinder"]
PAYLOADS = ["blast.needle", "blast.standard", "plain.needle", "germ.needle", "germ.standard", "gas.standard", "gas.needle", "torch", "still", "provoke"]
TWO = [("gas.standard", "shatter"), ("gas.needle", "shatter"), ("germ.standard", "detonate"), ("germ.needle", "detonate"),
       ("blast.needle", "gas.standard"), ("blast.standard", "gas.standard"), ("blast.needle", "germ.needle"), ("blast.needle", "torch"), ("torch", "blast.standard")]

def pairs():
    decks = ["none"] + CARRIERS
    decks += [f"{x}+{c}" for x in SECOND for c in CARRIERS]
    from itertools import combinations
    decks += [f"{a}+{b}" for a, b in combinations(PAYLOADS, 2)]
    decks += [f"{a}+{b}" for a, b in TWO if f"{a}+{b}" not in decks]
    decks += [f"{t}+{a}+{b}" for t in ("trigger", "multicast") for a, b in TWO]
    decks += ["tinder+torch", "tinder+blast.standard", "tinder+torch+blast.standard"]
    seen = set()
    return [d for d in decks if not (d in seen or seen.add(d))]

def main():
    stage = sys.argv[1]
    seeds = range(1, int(sys.argv[2]) + 1) if len(sys.argv) > 2 else range(1, 7)
    extra = sys.argv[3] if len(sys.argv) > 3 else "arena=9 tail=30"
    secs = sys.argv[4] if len(sys.argv) > 4 else "60"
    decks = {"singles": singles, "pairs": pairs}[stage]()
    depths = (1, 3, 4) if stage == "pairs" else (1, 2, 3, 4)
    for d in decks:
        for depth in depths:
            for s in seeds:
                print(depth, s, secs, "rifle", d, extra)

main()
