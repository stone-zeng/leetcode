// 540. Single Element in a Sorted Array
// https://leetcode.com/problems/single-element-in-a-sorted-array/

struct Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        Self::single_non_duplicate_helper(&nums)
    }

    fn single_non_duplicate_helper(nums: &[i32]) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let m = (nums.len() - 1) / 2;
        let range = match (m % 2 == 0, nums[m] == nums[m - 1]) {
            (true, true) => &nums[..m - 1],
            (true, false) => &nums[m..],
            (false, true) => &nums[m + 1..],
            (false, false) => &nums[..m],
        };
        Self::single_non_duplicate_helper(range)
    }
}

fn main() {
    for (nums, res) in [
        (vec![1, 1, 2, 3, 3, 4, 4, 8, 8], 2),
        (vec![1, 1, 2, 2, 3, 4, 4, 8, 8], 3),
        (vec![3, 3, 7, 7, 10, 11, 11], 10),
        (vec![3, 3, 7, 10, 10, 11, 11], 7),
        (vec![3, 7, 7, 10, 10, 11, 11], 3),
        (vec![3, 3, 6], 6),
        (vec![3, 6, 6], 3),
        (vec![0], 0),
    ] {
        assert_eq!(Solution::single_non_duplicate(nums), res)
    }
}
