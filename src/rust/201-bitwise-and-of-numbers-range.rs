// 201. Bitwise AND of Numbers Range
// https://leetcode.com/problems/bitwise-and-of-numbers-range/

struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        if left == right {
            return left;
        }
        if left == 0 || Self::log2(left) != Self::log2(right) {
            return 0;
        }
        let d = Self::log2(left ^ right);
        (left >> d) << d
    }

    fn log2(n: i32) -> i32 {
        let mut n = n;
        let mut res = 0;
        while n > 1 {
            n /= 2;
            res += 1;
        }
        res
    }
}

fn main() {
    [
        (5, 7, 4),
        (0, 0, 0),
        (1, 2147483647, 0),
        (7310, 7519, 7168),
        (96347321, 138997309, 0),
        (82063118, 120841998, 67108864),
    ]
    .iter()
    .for_each(|&(left, right, res)| assert_eq!(Solution::range_bitwise_and(left, right), res));
}
