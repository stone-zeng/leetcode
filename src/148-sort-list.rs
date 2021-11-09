// 148. Sort List
// https://leetcode.com/problems/sort-list/

/// Definition for singly-linked list.
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

struct Solution;

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut v = vec![];
        while let Some(node) = head {
            v.push(node.val);
            head = node.next;
        }
        v.sort_unstable();
        let mut res = None;
        while let Some(val) = v.pop() {
            res = Some(Box::new(ListNode { val, next: res }));
        }
        res
    }
}

fn main() {}
