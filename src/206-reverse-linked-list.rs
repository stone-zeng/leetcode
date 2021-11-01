// 206. Reverse Linked List
// https://leetcode.com/problems/reverse-linked-list/

/// Definition for singly-linked list.
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(head) => {
                let mut v = vec![head.val];
                let mut head = head;
                while let Some(next) = head.next {
                    head = next;
                    v.push(head.val);
                }
                v.reverse();
                let mut res = None;
                while let Some(val) = v.pop() {
                    res = Some(Box::new(ListNode { val, next: res }))
                }
                res
            }
            None => None,
        }
    }
}

fn main() {}
