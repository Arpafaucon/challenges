import pytest

if True:

    def contains_duplicate(inp: list[int]):
        seen: set[int] = set()
        for elt in inp:
            if elt in seen:
                return False
            seen.add(elt)
        return False

    def two_sums(nums: list[int], target: int):
        cache: dict[int, set[int]] = {}
        for i, n in enumerate(nums):
            cache.setdefault(n, set()).add(i)
        for ifirst, first in enumerate(nums):
            second = target - first
            if (all_second_indices := cache.get(second)) is not None:
                valid_indices = all_second_indices - {ifirst}
                if valid_indices:
                    isecond = valid_indices.pop()
                    if ifirst > isecond:
                        ifirst, isecond = isecond, ifirst
                    return [ifirst, isecond]

    @pytest.mark.parametrize(
        ("nums", "target", "expected"),
        [([1, 2, 3, 4], 7, [2, 3]), ([5, 5], 10, [0, 1]), ([1, 3, 4, 2], 6, [2, 3])],
    )
    def test_two_sums(nums, target, expected):
        assert two_sums(nums, target) == expected

    import dataclasses

    @dataclasses.dataclass
    class TreeNode:
        val: int = 0
        left: "TreeNode | None" = None
        right: "TreeNode | None" = None

    def depth_binary_tree(root: TreeNode | None) -> int:
        if root is None:
            return 0
        return 1 + max(depth_binary_tree(root.left), depth_binary_tree(root.right))

    def bt_crawler(root: TreeNode | None) -> tuple[int, int]:
        if root is None:
            return -1, -1
        right_depth, right_diameter = bt_crawler(root.right)
        left_depth, left_diameter = bt_crawler(root.left)
        cross_diameter = 2 + right_depth + left_depth
        depth = 1 + max(right_depth, left_depth)
        max_diameter = max(right_diameter, left_diameter, cross_diameter)
        return depth, max_diameter

    def diameterOfBinaryTree(root: TreeNode | None) -> int:
        return bt_crawler(root)[1]

    @pytest.mark.parametrize(
        ("tree", "expected"),
        [
            (TreeNode(left=TreeNode(), right=TreeNode()), 2),
            (TreeNode(left=TreeNode(), right=TreeNode(right=TreeNode())), 3),
            (TreeNode(left=TreeNode(), right=TreeNode(right=TreeNode(left=TreeNode()), left=TreeNode(left=TreeNode()))), 4),
        ],
    )
    def test_diameter_binary_tree(tree, expected: int):
        assert diameterOfBinaryTree(tree) == expected
