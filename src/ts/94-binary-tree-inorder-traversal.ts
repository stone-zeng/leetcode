// 94. Binary Tree Inorder Traversal
// https://leetcode.com/problems/binary-tree-inorder-traversal/

import { TreeNode } from './leetcode_util';

function inorderTraversal(root: TreeNode | null): number[] {
    return !root ? [] : inorderTraversal(root.left).concat(root.val, inorderTraversal(root.right));
}

console.log(
    [
        null,
        new TreeNode(42),
        new TreeNode(42, null, new TreeNode(3)),
        new TreeNode(26, new TreeNode(19, null, new TreeNode(27))),
        new TreeNode(2, new TreeNode(1), new TreeNode(3)),
        new TreeNode(5, new TreeNode(1), new TreeNode(4, new TreeNode(3), new TreeNode(6))),
    ].map(inorderTraversal)
);
