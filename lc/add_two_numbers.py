import dataclasses
import pytest


@dataclasses.dataclass
class ListNode:
    val: int  #
    next: "ListNode|None" = None

    @staticmethod
    def build(number: int):
        digits = []
        while number > 0:
            digit = number % 10
            digits.append(digit)
            number //= 10

        if not digits:
            return ListNode(val=0)
        first = ListNode(digits[0])
        current = first
        for digit in digits[1:]:
            next = ListNode(digit)
            current.next = next
            current = next

        return first

    def to_num(self):
        exponent = 1
        current = self
        num = 0
        while current is not None:
            num += current.val * exponent
            exponent *= 10
            current = current.next
        return num

    def to_list(self):
        current = self
        out = []
        while current is not None:
            out.append(current.val)
            current = current.next
        return out


def test_build():
    a = ListNode.build(143)
    assert a.val == 3
    assert a.next.val == 4
    assert a.next.next.val == 1
    assert a.next.next.next is None

    assert a.to_num() == 143
    assert a.to_list() == [3, 4, 1]


def add(a: ListNode, b: ListNode):
    lead = ListNode(0)  # for ease of use - won't be returned
    a_node = a
    b_node = b
    sum_node = lead
    carry = 0
    while a_node is not None or b_node is not None or carry > 0:
        a_val = a_node.val if a_node is not None else 0
        b_val = b_node.val if b_node is not None else 0

        sum_vals = a_val + b_val + carry
        digit = sum_vals % 10
        carry = sum_vals // 10

        next_node = ListNode(val=digit)
        sum_node.next = next_node
        sum_node = next_node
        print(lead.to_list())

        if a_node is not None:
            a_node = a_node.next
        if b_node is not None:
            b_node = b_node.next

    return lead.next


@pytest.mark.parametrize(
    ("a", "b", "expected"), [(1, 1, 2), (5, 5, 10), (999, 9999, 10998), (0, 0, 0)]
)
def test_sum(a, b, expected):
    node_a = ListNode.build(a)
    node_b = ListNode.build(b)
    node_sum = add(node_a, node_b)

    assert node_sum.to_num() == expected
