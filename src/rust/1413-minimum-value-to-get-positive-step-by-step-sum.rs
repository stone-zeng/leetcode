// 1413. Minimum Value to Get Positive Step by Step Sum
// https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/

struct Solution;

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 1), |(sum, res), i| (sum + i, res.max(1 - sum - i)))
            .1
    }
}

fn main() {
    [
        (vec![-3, 2, -3, 4, 2], 5),
        (vec![1, 2], 1),
        (vec![1, -2, -3], 5),
    ]
    .iter()
    .for_each(|(nums, res)| assert_eq!(Solution::min_start_value(nums.to_vec()), *res))
}
