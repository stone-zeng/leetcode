// 876. Middle of the Linked List
// https://leetcode.com/problems/middle-of-the-linked-list/

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|h| {
            let mut i = &h;
            let mut j = &h;
            while let Some(i_next) = &i.next {
                i = i_next;
                j = j.next.as_ref().unwrap();
                if let Some(i_next) = &i.next {
                    i = i_next;
                }
            }
            j.clone()
        })
    }
}

fn main() {}
