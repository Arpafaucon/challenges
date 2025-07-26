import dataclasses
import typing as tp
import enum

@dataclasses.dataclass
class TreeNode:
    val: int = 0
    left: "TreeNode | None" = None
    right: "TreeNode | None" = None

class Control(enum.Enum):
    NOCHILD = enum.auto()
    BUILD = enum.auto()

def serialize(t: TreeNode, out: list[tp.Literal[BUILD], ]):

