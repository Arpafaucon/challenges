from pathlib import Path


with Path("data/3.txt").open("r") as f:
    data = [l.strip() for l in f]

c = {}
for line in data:
    for i, char in enumerate(line):
        if char == "1":
            c[i] = c.get(i, 0) +1

num_chars = len(c)
gamma = []
epsilon = []
for i in range(num_chars):
    count = c[i]
    if count < len(data) // 2:
        d = 0
    else:
        d = 1
    gamma.append(str(d))
    epsilon.append(str(1-d))

gg = int("".join(gamma), 2)
ee = int("".join(epsilon), 2)
print(gamma, epsilon)
print(gg, ee)
print(gg*ee)

def filter(lines, keep_most:bool):
    num_chars = len(lines[0])
    for i in range(num_chars):
        # gather max value
        num_1 = sum(d[i] == "1" for d in lines)
        filter_val = str(1 if (num_1 < len(lines) / 2)^keep_most else 0)
        lines = [l for l in lines if l[i] == filter_val]
        if len(lines) == 1:
            return lines[0]
    raise ValueError

o2 = filter(data, True)
co2 = filter(data, False)

print(o2, co2)

print(int(o2, 2)*int(co2, 2))