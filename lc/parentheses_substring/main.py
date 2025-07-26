def is_valid(chain: str):
    level = 0
    for char in chain:
        if char == "(":
            level += 1
        else:
            level -= 1
            if level < 0:
                return False
    return level == 0


def longest_ss_brute(input: str) -> str:
    length = len(input)
    best = (0, 0)  # end not included
    best_score = 0
    for start in range(0, length - 1):
        for end in range(start + 1, length + 1):
            if is_valid(input[start:end]):
                new_score = end - start - 1
                if new_score > best_score:
                    best_score = new_score
                    best = (start, end)
    start, end = best
    return input[start:end]

def longest_ss_correction(input: str) -> str:
    opened_tracker: list[int]= [-1]
    best_range = (0, 0) # end not included
    best_score = 0
    for i, char in enumerate(input):
        if char == '(':
            opened_tracker.append(i)
        else:
            opened_tracker.pop()
            if opened_tracker:
                next_matching = opened_tracker[-1]
                # we're closing a valid sequence
                closed_len = i - next_matching + 1+ 1
                if closed_len > best_score:
                    best_range = (next_matching+1, i+1) # end not included
                    best_score = closed_len
            else:
                opened_tracker.append(i)

    return input[slice(*best_range)]
