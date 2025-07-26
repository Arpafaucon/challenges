def longest_palindromic_substring(string: str) -> str:
    N = len(string)
    maxlen = [0 for _ in range(N)] # max palindromic ending at character k
    maxlen[0] = 1 # first char is always palindromic
    for i, char in enumerate(string, 1):
        # either we're finishing a "wrap" of a p
        symmetric_index = i -1 - maxlen[i-1]
        if symmetric_index >= 0 and string[symmetric_index] == char:
            # nice !
            maxlen[i] = maxlen[i-1] + 2
        else:
            # or we don't, in which case
            maxlen[i] = 1
