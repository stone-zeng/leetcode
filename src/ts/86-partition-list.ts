// 86. Partition List
// https://leetcode.com/problems/partition-list/

import { ListNode } from './leetcode_util';

function newListNode(array: number[]): ListNode | null {
    if (array.length === 0) return null;
    const head = new ListNode(array[0]);
    let temp: ListNode = head;
    array.slice(1).forEach((i) => {
        temp.next = new ListNode(i);
        temp = temp.next;
    });
    return head;
}

function printListNode(head: ListNode | null) {
    const array: number[] = [];
    while (head) {
        array.push(head.val);
        head = head.next;
    }
    console.log(array);
}

function partition(head: ListNode | null, x: number): ListNode | null {
    if (!head || !head.next) return head;

    const p = new ListNode(x - 1, head);
    let p1 = p;
    let p2 = p;

    while (p1.val < x && p1.next && p1.next.val < x) {
        p2 = p1.next;
        p1 = p1.next;
    }

    while (p2.next) {
        if (p2.val >= x && p2.next.val < x) {
            const temp = p1.next;
            p1.next = p2.next;
            p2.next = p2.next.next;
            p1.next.next = temp;
            p1 = p1.next;
        } else if (p2.next) {
            p2 = p2.next;
        }
    }

    return p.next;
}

(
    [
        [[], 0],
        [[1, 4, 3, 2, 5, 2], 3],
        [[2, 1, 4, 3, 2, 5, 2], 3],
        [[2, 1], 2],
        [[-53, 63, 91, -2, -88, -8, 6, -22, 8, -56, -76, 59, -93, 4, -69, 3, -27, -89, -54, 97], 0],
        [[18, 73, -4, 86, -82, -44, 15, -62, 57, 50, 73, -40, -78, 16, -3, 7, 99, -42, 14, -53], 0],
    ] as [number[], number][]
).forEach(([array, x]) => printListNode(partition(newListNode(array), x)));
