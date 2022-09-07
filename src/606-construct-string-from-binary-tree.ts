// 606. Construct String from Binary Tree
// https://leetcode.com/problems/construct-string-from-binary-tree/

import { TreeNode } from './leetcode_util';

function tree2str(root: TreeNode | null): string {
    if (!root) return '';
    if (!root.right) return root.left ? `${root.val}(${tree2str(root.left)})` : `${root.val}`;
    return `${root.val}(${tree2str(root.left)})(${tree2str(root.right)})`;
}

console.log(
    [
        null,
        new TreeNode(1),
        new TreeNode(1, new TreeNode(2, new TreeNode(4)), new TreeNode(3)),
        new TreeNode(1, new TreeNode(2, null, new TreeNode(4)), new TreeNode(3)),
    ].map(tree2str)
);
