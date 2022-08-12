// 235. Lowest Common Ancestor of a Binary Search Tree
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/

import { TreeNode } from './leetcode_util';

function lowestCommonAncestor(
    root: TreeNode | null,
    p: TreeNode | null,
    q: TreeNode | null
): TreeNode | null {
    const left = findNode(root, p as TreeNode);
    const right = findNode(root, q as TreeNode);
    for (let i = 0; i < Math.min(left.length, right.length); i++) {
        const l = left[i];
        const r = right[i];
        if (l !== r) break;
        root = l === 0 ? (root as TreeNode).left : (root as TreeNode).right;
    }
    return root;
}

function findNode(root: TreeNode | null, p: TreeNode): number[] {
    let res: number[] = [];
    while (!root || root.val !== p.val) {
        if ((root as TreeNode).val > p.val) {
            res.push(0);
            root = (root as TreeNode).left;
        } else {
            res.push(1);
            root = (root as TreeNode).right;
        }
    }
    return res;
}

(() => {
    const root = new TreeNode(
        6,
        new TreeNode(2, new TreeNode(0), new TreeNode(4, new TreeNode(3), new TreeNode(5))),
        new TreeNode(8, new TreeNode(7), new TreeNode(9))
    );
    console.log(
        [
            [new TreeNode(2), new TreeNode(8)],
            [new TreeNode(2), new TreeNode(4)],
            [new TreeNode(3), new TreeNode(7)],
        ].map(([p, q]) => lowestCommonAncestor(root, p, q)?.val)
    );
})();
