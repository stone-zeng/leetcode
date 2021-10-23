// 32. Longest Valid Parentheses
// https://leetcode.com/problems/longest-valid-parentheses/

struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut i = 0;
        let mut len = 0;
        let mut stack = Vec::with_capacity(s.len());
        stack.push((0, 0));
        s.chars().for_each(|c| {
            match c {
                '(' => {
                    i += 1;
                    stack.push((i, -1));
                }
                ')' => {
                    i -= 1;
                    stack.pop();
                    if stack.is_empty() {
                        stack.push((i, -1));
                    }
                }
                _ => unreachable!(),
            }
            stack.iter_mut().for_each(|(_, n)| *n += 1);
            println!("{:?}", stack);
            len = len.max(stack.last().unwrap().1);
        });
        len
    }
}

fn main() {
    [
        ("(()", 2),
        (")()())", 4),
        ("", 0),
        (")(())()((((()()(()())()()()(())(()(()))(", 28),
    ]
    .iter()
    .for_each(|&(s, res)| assert_eq!(Solution::longest_valid_parentheses(s.to_string()), res));
}
