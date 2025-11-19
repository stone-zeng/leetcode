// 429. N-ary Tree Level Order Traversal
// https://leetcode.com/problems/n-ary-tree-level-order-traversal/

import { Node } from './leetcode_util';

function levelOrder(root: Node | null): number[][] {
    if (!root) {
        return [];
    }

    const res = [[root.val]];
    let children = root.children;
    while (children.length !== 0) {
        const temp: number[] = [];
        const nextChildren: Node[] = [];
        children.forEach((node) => {
            temp.push(node.val);
            nextChildren.push(...node.children);
        });
        res.push(temp);
        children = nextChildren;
    }

    return res;
}

const t1 = null;
const t2 = (() => {
    const root = new Node(1);
    const c1 = new Node(3);
    c1.children = [new Node(5), new Node(6)];
    root.children = [c1, new Node(2), new Node(4)];
    return root;
})();

console.log([t1, t2].map(levelOrder));
