'''116. Populating Next Right Pointers in Each Node
https://leetcode.com/problems/populating-next-right-pointers-in-each-node/
'''

# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next

class Solution:
    def connect(self, root: 'Node') -> 'Node':
        if not root:
            return root
        if root.left and root.right:
            root.left = self.connect(root.left)
            root.right = self.connect(root.right)
            temp_left = root.left
            temp_right = root.right
            while temp_left:
                temp_left.next = temp_right
                temp_left = temp_left.right
                temp_right = temp_right.left
        return root
