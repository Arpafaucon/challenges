from pathlib import Path
import numpy as np

def read_depths() -> np.ndarray:
    data_txt = Path("data/1a.txt").read_text("utf8").splitlines()
    data_int = np.array([int(d) for d in data_txt])
    return data_int

depths = read_depths()


diff = np.diff(depths)

print(np.sum(diff > 0))

depth_sliding = depths[:-2] + depths[1:-1] + depths[2:]
print(np.sum(np.diff(depth_sliding)>0))