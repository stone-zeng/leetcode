'''222. Count Complete Tree Nodes
https://leetcode.com/problems/count-complete-tree-nodes/
'''

from typing import Optional

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def countNodes(self, root: Optional[TreeNode]) -> int:
        if not root:
            return 0
        left = self.countNodes(root.left) if root.left else 0
        right = self.countNodes(root.right) if root.right else 0
        return left + right + 1
