import pytest
from .main import countandsay, countandsay_next


@pytest.mark.parametrize(("d", "next"), (("1", "11"), ("11", "21")))
def test_cas_next(d, next):
    assert countandsay_next(d) == next


@pytest.mark.parametrize(
    ("level", "expected"),
    (
        (1, "1"),
        (2, "11"),
        (3, "21"),
        (4, "1211"),
        (5, "111221"),
    ),
)
def test_cas(level, expected):
    assert countandsay(level) == expected
