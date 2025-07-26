from . import integer_to_roman
import pytest

@pytest.mark.parametrize(("decimal", "roman"), [
    (1, "I"),
    (3, "III"),
    (34, "XXXIV"),
    (58, "LVIII"),
    (649, "DCXLIX"),
    (1994, "MCMXCIV"),
    (2999, "MMCMXCIX"),
])
def test_conversion(decimal: int, roman: str):
    assert integer_to_roman(decimal) == roman
