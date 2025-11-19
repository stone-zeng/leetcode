// 260. Single Number III
// https://leetcode.com/problems/single-number-iii/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::new();
        nums.into_iter().for_each(|i| {
            if !set.insert(i) {
                set.remove(&i);
            }
        });
        set.into_iter().collect()
    }
}

fn main() {
    [
        vec![1, 2, 1, 3, 2, 5],
        vec![-1, 0],
        vec![0, 1],
        vec![52, 82, 50, 92, 62, 89, 93, 34, 46, 52, 82, 50, 92, 62, 89, 93, 34, 46, -16, -47],
    ]
    .iter()
    .for_each(|nums| println!("{:?}", Solution::single_number(nums.to_vec())))
}
