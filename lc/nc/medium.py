import pytest
import collections
import string

if True:
    def group_anagrams(strs: list[str]) -> list[list[str]]:
        def base_form(inp: str) -> tuple[int, ...]:
            counter = collections.Counter(inp)
            return tuple(counter.get(letter, 0) for letter in string.ascii_lowercase)

        anagram_groups: dict[tuple[int, ...], list[str]] = {}
        for current_item in strs:
            item_base_formr = base_form(current_item)
            anagram_groups.setdefault(item_base_formr, []).append(current_item)

        return list(anagram_groups.values())


@pytest.mark.parametrize(("strs", "expected"), [
    (["abc", "bac", "aaa"], [["abc", "bac"], ["aaa"]]),
    (["act","pots","tops","cat","stop","hat"], [["hat"],["act", "cat"],["stop", "pots", "tops"]])
])
def test_group_anagrams(strs, expected):
    grouped = group_anagrams(strs)
    grouped_set = {frozenset(subgroup) for subgroup in grouped}
    group_expected = {frozenset(subgroup) for subgroup in expected}
    assert group_expected == grouped_set
