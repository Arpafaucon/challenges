from pathlib import Path
from pprint import pp
import typing as tp
import itertools

Grid = tp.List[tp.List[int]]

grids: tp.List[Grid] = []  # list of 5x5 grids

ijs = list(itertools.product(range(5), range(5)))


data_lines = Path("data/4.txt").read_text("utf8").splitlines()

bingo_seq = [int(n) for n in data_lines[0].split(",")]

num_grids = (len(data_lines) - 1) // 6



for i in range(num_grids):
    g = []
    grid_data = data_lines[2 + 6 * i : 7 + 6 * i]
    # pp(grid_data)
    for j, line in enumerate(grid_data):
        l = []
        for k in range(5):
            n = int(line[3 * k : 3 * k + 2])
            l.append(n)
        g.append(l)
    grids.append(g)

# print(bingo_seq)
# for g in grids:
#     pp(g)


def check_victory(g: Grid) -> bool:
    for i in range(5):
        if all(g[i][j] is None for j in range(5)) or all(
            g[j][i] is None for j in range(5)
        ):
            return True
    return False


def bingo(grids: tp.List[Grid], bingo_seq: tp.List[int]):
    active_boards = set(range(num_grids))
    for number in bingo_seq:
        current_run = active_boards.copy()
        for g_ix in current_run:
            g = grids[g_ix]
            for i,j in ijs:
                if g[i][j] == number:
                    g[i][j] = None
            if check_victory(g):
                s = score(g, number)
                print(f"board {g_ix} won, score = {s}")
                active_boards.remove(g_ix)


def score(g: Grid, n: int) -> int:
    return sum(
        filter(lambda a: a is not None, (g[i][j] for i,j in ijs))
    ) * n

n = bingo(grids, bingo_seq)
print(n)
