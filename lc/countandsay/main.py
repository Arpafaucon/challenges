import itertools

def countandsay_next(d: str):
    # strategy
    # first pick first element as reference
    # then continue picking elements as long as they're equal to the rerf
    # if not, "flush": write down the count + reference, and take the current as next ref
    # don't forget to flush at the end
    out = []

    # take first element
    # invariant: string is never empty
    ref_current: str = d[0]
    ref_count: int = 1
    for char in d[1:]:
        if char == ref_current:
            ref_count+=1
            continue
        # different char
        # flush
        out.extend([str(ref_count), ref_current])
        # reset
        ref_current = char
        ref_count = 1
    # flush at the end
    out.extend([str(ref_count), ref_current])

    return "".join(out)

def countandsay(level: int):
    current_seq = "1"
    if level < 1:
        raise ValueError("Invalid level, should be >=1")
    for _ in range(1, level):
        current_seq = countandsay_next(current_seq)
    return current_seq


