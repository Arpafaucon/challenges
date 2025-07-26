import pytest
from . import longest_palindromic_substring

@pytest.mark.parametrize(("string", "sub"), [
    ("a", {"a"}),
    ("", {}),
    ("abc", set("abc")),
    ("abcb", {"bcb"}),
])
def test_palindromic_substring(string, sub):
    assert longest_palindromic_substring(string) in sub
