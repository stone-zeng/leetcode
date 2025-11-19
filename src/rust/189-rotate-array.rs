// 189. Rotate Array
// https://leetcode.com/problems/rotate-array/

struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = nums.len() - k as usize % nums.len();
        *nums = nums[k..].iter().chain(nums[..k].iter()).copied().collect()
    }
}

#[rustfmt::skip]
fn main() {
    [
        (vec![1, 2, 3, 4, 5, 6, 7], 3),
        (vec![1, 2, 3, 4, 5, 6, 7], 100_000),
        (vec![-1, -100, 3, 99], 2),
        (vec![-1], 20),
    ]
    .iter_mut()
    .for_each(|(nums, k)| {
        Solution::rotate(nums, *k);
        println!("{:?}", nums);
    })
}
