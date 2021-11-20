// 461. Hamming Distance
// https://leetcode.com/problems/hamming-distance/

struct Solution;

impl Solution {
    pub const fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}

fn main() {
    for (x, y, res) in [(1, 4, 2), (3, 1, 1)] {
        assert_eq!(Solution::hamming_distance(x, y), res)
    }
}
