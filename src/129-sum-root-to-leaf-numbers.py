'''129. Sum Root to Leaf Numbers
https://leetcode.com/problems/sum-root-to-leaf-numbers/
'''

from typing import Optional

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def sumNumbers(self, root: Optional[TreeNode]) -> int:
        if not root:
            return 0

        res = []
        temp = []
        stack = [[root, 0]]
        while stack:
            temp.append(root.val)
            if stack[-1][1] == 0:
                if root.left:
                    root = root.left
                    stack[-1][1] = 1
                    stack.append([root, 0])
                    continue
                if root.right:
                    root = root.right
                    stack[-1][1] = 2
                    stack.append([root, 0])
                    continue
                res.append(self.sum(temp))
                temp.pop()
                if not temp:
                    break
                temp.pop()
                stack.pop()
                root = stack[-1][0]
                continue
            if stack[-1][1] == 1:
                if root.right:
                    root = root.right
                    stack[-1][1] = 2
                    stack.append([root, 0])
                    continue
                temp.pop()
                if not temp:
                    break
                temp.pop()
                stack.pop()
                root = stack[-1][0]
                continue
            if stack[-1][1] == 2:
                stack.pop()
                temp.pop()
                if not temp:
                    break
                temp.pop()
                if stack:
                    root = stack[-1][0]

        return sum(res)

    @staticmethod
    def sum(nums: list) -> int:
        return sum(i * j for i, j in zip(reversed([10 ** i for i in range(len(nums))]), nums))

def _main():
    for root in [
        None,
        TreeNode(1),
        TreeNode(1, right=TreeNode(3)),
        TreeNode(1, left=TreeNode(2), right=TreeNode(3)),
        TreeNode(4, left=TreeNode(9, left=TreeNode(5), right=TreeNode(1)), right=TreeNode(0)),
        TreeNode(1, left=TreeNode(2), right=TreeNode(3, left=TreeNode(4))),
        TreeNode(1, left=TreeNode(2), right=TreeNode(3, left=TreeNode(4), right=TreeNode(7))),
        TreeNode(1, left=TreeNode(2, left=TreeNode(3, left=TreeNode(5)), right=TreeNode(4))),
    ]:
        print(Solution().sumNumbers(root))

if __name__ == '__main__':
    _main()
