

def longest_substring(string: str):
  best_str = ""
  best_len = 0
  current = set()
  start = 0

  for ix, char in enumerate(string):
    print("trying to add", char)
    if char in current:
      # we need to purge characters from the substring until we get rid of char
      for start_ix in range(start, ix):
        startchar = string[start_ix]
        current.remove(startchar)
        print("purging", startchar, "->", current)
        if startchar == char:
          start = start_ix+1
          break
    current.add(char)
    print("adding", char, "->", current)
    if ix-start+1 > best_len:
      # new record
      best_len = ix-start+1
      best_str = string[start:ix+1]
      print("new record !", best_str)
  return best_str


import pytest

@pytest.mark.parametrize(("string", "expected"), [
    ("b", "b"),
    ("bbb", "b"),
    ("babc", "abc"),
    ("bdabcesthouet", "dabcesthou"),
])
def test_longest_substring(string: str, expected: str):
  assert longest_substring(string) == expected
