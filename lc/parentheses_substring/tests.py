from .main import longest_ss_brute , longest_ss_correction
import pytest

TEST_SETS =  (("", ""), ("()", "()"),( "(())", "(())"), (")", ""), ("(", ""), ("()()", "()()"), (")()())", "()()"))

@pytest.mark.parametrize("fun", (longest_ss_brute, longest_ss_correction))
@pytest.mark.parametrize(("input", "substring"),
                         TEST_SETS
)
def test_longest_ss(fun, input, substring):
    assert fun(input) == substring
