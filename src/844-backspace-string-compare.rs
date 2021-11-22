// 844. Backspace String Compare
// https://leetcode.com/problems/backspace-string-compare/

struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::to_stack(&s) == Self::to_stack(&t)
    }

    fn to_stack(s: &str) -> Vec<u8> {
        let mut v = vec![];
        s.bytes().for_each(|c| match c {
            b'#' => { v.pop(); }
            _ => { v.push(c); }
        });
        v
    }
}

fn main() {
    for (s, t, res) in [
        ("ab#c", "ad#c", true),
        ("ab##", "c#d#", true),
        ("a##c", "#a#c", true),
        ("a#c", "b", false),
    ] {
        let (s, t) = (s.to_string(), t.to_string());
        assert_eq!(Solution::backspace_compare(s, t), res)
    }
}
