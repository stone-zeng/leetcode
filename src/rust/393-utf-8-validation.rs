// 393. UTF-8 Validation
// https://leetcode.com/problems/utf-8-validation/

struct Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut iter = data.iter();
        while let Some(&b0) = iter.next() {
            if b0 >> 7 == 0 {
                continue;
            } else if b0 >> 5 == 0b110 {
                match iter.next() {
                    Some(&b1) if b1 >> 6 == 0b10 => continue,
                    _ => return false,
                }
            } else if b0 >> 4 == 0b1110 {
                match iter.next() {
                    Some(&b1) if b1 >> 6 == 0b10 => match iter.next() {
                        Some(&b2) if b2 >> 6 == 0b10 => continue,
                        _ => return false,
                    },
                    _ => return false,
                }
            } else if b0 >> 3 == 0b11110 {
                match iter.next() {
                    Some(&b1) if b1 >> 6 == 0b10 => match iter.next() {
                        Some(&b2) if b2 >> 6 == 0b10 => match iter.next() {
                            Some(&b3) if b3 >> 6 == 0b10 => continue,
                            _ => return false,
                        },
                        _ => return false,
                    },
                    _ => return false,
                }
            }
            return false;
        }
        true
    }
}

fn main() {
    for (data, res) in [
        (vec![197, 130, 1], true),
        (vec![235, 140, 4], false),
        (vec![255], false),
    ] {
        assert_eq!(Solution::valid_utf8(data), res)
    }
}
