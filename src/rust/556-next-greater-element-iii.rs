// 556. Next Greater Element III
// https://leetcode.com/problems/next-greater-element-iii/

struct Solution {}

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits = Self::to_digits(n);
        if let Some(pos) = digits.windows(2).rev().position(|w| w[0] < w[1]) {
            let pos = digits.len() - pos - 2;
            let i = digits[pos];
            let next = *digits[pos..].iter().rev().find(|&&x| x > i).unwrap();
            let mut temp = vec![];
            let mut has_next = false;
            digits[pos..].iter().for_each(|&x| {
                if x != next || has_next {
                    temp.push(x);
                } else {
                    has_next = true;
                }
            });
            temp.sort_unstable();
            temp.insert(0, next);
            temp.iter()
                .enumerate()
                .for_each(|(i, &x)| digits[pos + i] = x);
            Self::from_rev_digits(digits)
        } else {
            -1
        }
    }

    fn to_digits(n: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut n = n;
        while n >= 1 {
            res.push(n % 10);
            n /= 10;
        }
        res.reverse();
        res
    }

    fn from_rev_digits(digits: Vec<i32>) -> i32 {
        let mut digits = digits;
        let mut b = 1;
        let mut n = 0;
        loop {
            if digits.len() != 1 {
                n += b * digits.pop().unwrap();
                b *= 10;
            } else {
                let i = digits.pop().unwrap();
                if b == 1_000_000_000 && (i > 2 || i32::MAX - b * i < n) {
                    return -1;
                } else {
                    return n + b * i;
                }
            }
        }
    }
}

fn main() {
    [
        (12, 21),
        (21, -1),
        (153221, 211235),
        (1108045118, 1108045181),
        (1608676543, 1608734566),
        (985754132, 985754213),
        (2147483647, -1),
        (2147482987, -1),
    ]
    .iter()
    .for_each(|(n, res)| assert_eq!(Solution::next_greater_element(*n), *res));
}
