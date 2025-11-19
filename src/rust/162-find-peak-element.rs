// 162. Find Peak Element
// https://leetcode.com/problems/find-peak-element/

struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        nums.iter().enumerate().max_by_key(|(_, &i)| i).unwrap().0 as i32
    }
}

fn main() {
    for nums in [
        vec![1, 2, 3, 1],
        vec![1, 2, 1, 3, 5, 6, 4]
    ] {
        println!("{}", Solution::find_peak_element(nums))
    }
}
