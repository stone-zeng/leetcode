// 98. Validate Binary Search Tree
// https://leetcode.com/problems/validate-binary-search-tree/

import { TreeNode } from './leetcode_util';

function isValidBST(root: TreeNode | null): boolean {
    if (!root) return true;
    if (!root.left)
        return !root.right ? true : root.val < min(root.right) && isValidBST(root.right);
    if (!root.right) return root.val > max(root.left) && isValidBST(root.left);

    return (
        root.val > max(root.left) &&
        root.val < min(root.right) &&
        isValidBST(root.left) &&
        isValidBST(root.right)
    );
}

function max(root: TreeNode): number {
    let res = root.val;
    while (root.right) {
        root = root.right;
        res = Math.max(res, root.val);
    }
    return res;
}

function min(root: TreeNode): number {
    let res = root.val;
    while (root.left) {
        root = root.left;
        res = Math.min(res, root.val);
    }
    return res;
}

console.log(
    [
        null,
        new TreeNode(42),
        new TreeNode(42, null, new TreeNode(3)),
        new TreeNode(26, new TreeNode(19, null, new TreeNode(27))),
        new TreeNode(2, new TreeNode(1), new TreeNode(3)),
        new TreeNode(5, new TreeNode(1), new TreeNode(4, new TreeNode(3), new TreeNode(6))),
    ].map(isValidBST)
);
