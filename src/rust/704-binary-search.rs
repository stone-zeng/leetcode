// 704. Binary Search
// https://leetcode.com/problems/binary-search/

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut begin = 0;
        let mut end = nums.len();
        loop {
            if begin + 1 == end {
                return if nums[begin] == target {
                    begin as i32
                } else {
                    -1
                };
            }
            let mid = (begin + end) / 2;
            if target < nums[mid] {
                end = mid;
            } else {
                begin = mid;
            }
        }
    }
}

fn main() {
    [
        (vec![1], 1),
        (vec![1], 2),
        (vec![1, 2], 2),
        (vec![-1, 0, 3, 5, 9, 12], 9),
        (vec![-1, 0, 3, 5, 9, 12], 2),
    ]
    .iter()
    .for_each(|(nums, target)| {
        assert_eq!(
            Solution::search(nums.to_vec(), *target),
            match nums.binary_search(target) {
                Ok(n) => n as i32,
                _ => -1,
            }
        );
    })
}
