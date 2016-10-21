#!/usr/bin/env python3

import subprocess

PROGRAM = "./target/release/cache_algos"
CACHE_SIZE = 2
ALGOS = ["BELADY", "FIFO", "LRU", "LFU", "RR", "MRU", "SLRU"]
FILE = "/usr/bin/clang-3.8"
OUTPUT_NAME = "algos.gnuplot"

if __name__ == "__main__":
    with open(OUTPUT_NAME, "a") as f:
        f.write("# size\t")
        f.write("\t".join(ALGOS))
    for size in range(CACHE_SIZE, 100000):
        with open(OUTPUT_NAME, "a") as f:
            f.write("\n")
            f.write(str(size) + '\t')
        for algo in ALGOS:
            with subprocess.Popen([PROGRAM, "-f", FILE, "-S", str(size), algo], stdout=subprocess.PIPE) as cache_hit:
                with open(OUTPUT_NAME, "a") as f:
                    f.write(cache_hit.stdout.read().decode().rstrip("\n") + '\t')
