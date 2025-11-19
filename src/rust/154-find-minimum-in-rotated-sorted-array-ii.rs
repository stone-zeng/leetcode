// 154. Find Minimum in Rotated Sorted Array II
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/

struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        *nums.iter().min().unwrap()
    }
}

fn main() {
    [
        (vec![1, 3, 5], 1),
        (vec![2, 2, 2, 0, 1], 0),
        (vec![3, 3, 3, 3, 3, 3, 3, 3, 1, 3], 1),
        (vec![3, 3, 3, 3, 3, 1, 3, 3, 3, 3], 1),
        (vec![3, 3, 1, 3, 3, 3, 3, 3, 3, 3], 1),
    ]
    .iter()
    .for_each(|(nums, res)| assert_eq!(Solution::find_min(nums.to_vec()), *res));
}
