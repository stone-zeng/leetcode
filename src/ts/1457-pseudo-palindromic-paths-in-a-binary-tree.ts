// 1457. Pseudo-Palindromic Paths in a Binary Tree
// https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/

import { TreeNode } from './leetcode_util';

function pseudoPalindromicPaths(root: TreeNode | null): number {
    const count = [...Array(9)].map(() => 0);
    return countPaths(root, count);
}

function countPaths(root: TreeNode | null, count: number[]): number {
    if (!root) return 0;
    count[root.val - 1]++;
    if (!root.left && !root.right) return pseudoPalindromicPath(count) ? 1 : 0;
    return countPaths(root.left, [...count]) + countPaths(root.right, [...count]);
}

function pseudoPalindromicPath(count: number[]): boolean {
    const len = count.reduce((s, i) => s + i, 0);
    if (len % 2) {
        let nOdd = 0;
        for (const i of count) {
            if (i % 2) nOdd++;
            if (nOdd > 1) return false;
        }
        return true;
    }
    return !count.some((i) => i % 2);
}

console.log(
    [
        null,
        new TreeNode(2),
        new TreeNode(
            2,
            new TreeNode(3, new TreeNode(3), new TreeNode(1)),
            new TreeNode(1, null, new TreeNode(2))
        ),
        new TreeNode(
            2,
            new TreeNode(1, new TreeNode(1), new TreeNode(3, null, new TreeNode(1))),
            new TreeNode(1)
        ),
    ].map(pseudoPalindromicPaths)
);
