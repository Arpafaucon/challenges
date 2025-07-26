import pytest

if True:
    def minWindow(s: str, t: str) -> str:
        # remarks about implementation:
        # should be possible to use only one "diff_counter" that only counts what chars are missing
        # so its initialized at -TARGET_COUNTER
        # and we're happy if all the values are >0
        TARGET_COUNTER = {}
        current_counter = {}
        for letter in t:
            letter_count = TARGET_COUNTER.get(letter, 0)
            TARGET_COUNTER[letter] = letter_count+1
            current_counter[letter] = 0

        def current_matches_t():
            for letter, target_count in TARGET_COUNTER.items():
                if current_counter[letter] < target_count:
                    return False
            return True

        win_start = 0
        win_end = 0 # does not include end

        # None would do for the logic, but this has the benefit of deref to "" if no better match was found
        best_match = (0, 0)
        best_match_len = len(s) +1

        # objective: slide across the string s
        # taking characters only when we're not filling the TARGET_COUNTER
        while True:
            # first: check if we're above the counter mark
            print(f"Processing '{s[win_start:win_end]}'")
            if current_matches_t():
                print("  OK !")
                # check if best match
                if win_end - win_start  < best_match_len:
                    best_match = (win_start, win_end)
                    best_match_len = win_end - win_start
                    print(f"  best score {best_match_len} -> {best_match}")
                # remove our starting letter
                removed_letter = s[win_start]
                print(f"  removing {removed_letter}")
                if removed_letter in current_counter.keys():
                    current_counter[removed_letter] -= 1
                win_start += 1
            else:
                print("  NOK !")
                # we want to "eat" another letter
                if win_end >= len(s):
                    # not possible !
                    break
                added_letter = s[win_end]
                print(f"  adding {added_letter}")
                if added_letter in current_counter.keys():
                    current_counter[added_letter] += 1
                win_end += 1

        return s[best_match[0]:best_match[1]]

@pytest.mark.parametrize(("s", "t", "expected"), [
    ("xyz", "xyz", "xyz"),
    ("OUZODYXAZV",  "XYZ", "YXAZ"),
    ("x",  "toto", ""),
    ])
def test_min_window(s, t, expected):
    assert minWindow(s, t) == expected
