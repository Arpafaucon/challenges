from pathlib import Path

pos_lin = 0
pos_depth = 0
aim = 0

with Path("data/2.txt").open("r") as f:
    for line in f:
        direction,num = line.split(" ")
        num = int(num)
        # print(direction, num)
        if direction == "forward":
            pos_lin += num
            pos_depth += aim * num
        elif direction == "down":
            aim += num
        elif direction == "up":
            aim -= num

print(pos_lin*pos_depth)
