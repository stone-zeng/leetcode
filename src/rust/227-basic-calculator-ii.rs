// 227. Basic Calculator II
// https://leetcode.com/problems/basic-calculator-ii/

struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        Self::calculate_helper(&s) as i32
    }

    fn calculate_helper(s: &str) -> i64 {
        if let Some((i, c)) = Self::rfind(s, (b'+', b'-')) {
            let (left, right) = Self::next(s, i);
            match c {
                b'+' => left + right,
                b'-' => left - right,
                _ => unreachable!(),
            }
        } else if let Some((i, c)) = Self::rfind(s, (b'*', b'/')) {
            let (left, right) = Self::next(s, i);
            match c {
                b'*' => left * right,
                b'/' => left / right,
                _ => unreachable!(),
            }
        } else {
            s.trim().parse().unwrap()
        }
    }

    fn rfind(s: &str, op: (u8, u8)) -> Option<(usize, u8)> {
        s.bytes()
            .enumerate()
            .rfind(|&(_, c)| c == op.0 || c == op.1)
    }

    fn next(s: &str, i: usize) -> (i64, i64) {
        (
            Self::calculate_helper(&s[..i]),
            Self::calculate_helper(&s[i + 1..]),
        )
    }
}

fn main() {
    for (s, res) in [
        ("3+2*2", 7),
        (" 3/2 ", 1),
        (" 3+5 / 2 ", 5),
        ("8/2/2", 2),
        ("608+948/763/350-698/303+987+305", 1898),
    ] {
        assert_eq!(Solution::calculate(s.to_string()), res)
    }
}
