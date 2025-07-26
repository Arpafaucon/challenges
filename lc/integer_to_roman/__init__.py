TABLE = {
    0: ("I", "V", "X"),
    1: ("X", "L", "C"),
    2: ("C", "D", "M"),
    3: ("M", None, None)
}

def convert_digit(digit: int, symbols: tuple[str, str, str]):
    unit, five, ten = symbols
    if digit == 9:
        return [unit, ten]
    if digit == 4:
        return [unit, five]
    stack = []
    if digit >= 5:
        stack.append(five)
        digit -= 5
    stack.extend(digit * unit)
    return stack


def integer_to_roman(num: int) -> str:
    assert 1<=num <=3999
    letters = []
    for exponent, symbols in reversed(TABLE.items()):
        digit = (num // 10**exponent) % 10
        letters.extend(convert_digit(digit, symbols))
    return "".join(letters)


