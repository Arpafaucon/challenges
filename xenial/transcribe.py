import sys
from pathlib import Path

source = Path(sys.argv[1])
with source.open("r") as f:
    for i, line in enumerate(f, 600):
        line = line.strip("\n") # remove endline
        clean_line = line.strip()
        if not clean_line or clean_line[0] == "'":
            continue
        print(f"{i:<3d} {line}")