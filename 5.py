from __future__ import annotations
from pathlib import Path
from pprint import pp
import typing as tp
import itertools
import numpy as np


import dataclasses


def read_pair(t: str) -> tp.Tuple[int, int]:
    a, b = [int(i.strip()) for i in t.strip().split(",")]
    return a, b


@dataclasses.dataclass
class Line:
    x1: int
    y1: int
    x2: int
    y2: int

    @classmethod
    def from_str(cls, t: str) -> Line:
        a, b = t.split("->")
        x1, y1 = read_pair(a)
        x2, y2 = read_pair(b)
        return cls(x1, y1, x2, y2)

    def iterate(self):
        def minmax(a:int, b:int):
            c = min(a,b)
            d = max(a,b)
            return c,d
        def step(a:int,b:int):
            if a>b:
                return range(a, b-1, -1)
            elif a<b:
                return range(a, b+1)
            else:
                return itertools.repeat(a)
        return zip(step(self.x1, self.x2), step(self.y1, self.y2))


d = Path("data/5.txt").read_text()
lines = [Line.from_str(dd) for dd in d.splitlines()]
bound_x = max(itertools.chain((l.x1 for l in lines), (l.x2 for l in lines)))
bound_y = max(itertools.chain((l.y1 for l in lines), (l.y2 for l in lines)))

grid = np.zeros((bound_x + 1, bound_y + 1), dtype=int)

for l in lines:
    for coord in l.iterate():
        grid[coord] += 1

# def print_grid(g: np.ndarray):
#     print(g.transpose()[::-1, ::])

# print_grid(np.array([[00,1],[10,11]]))
print(grid.transpose())
print(sum((grid >= 2).flat))
