// 46. Permutations
// https://leetcode.com/problems/permutations/

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        match nums.len() {
            1 => vec![nums],
            _ => {
                let mut res = vec![];
                for (i, &n) in nums.iter().enumerate() {
                    for v in Self::permute([&nums[..i], &nums[i + 1..]].concat()) {
                        let mut v = v;
                        v.push(n);
                        res.push(v);
                    }
                }
                res
            }
        }
    }
}

fn main() {
    for nums in [vec![1, 2, 3], vec![0, 1], vec![1]] {
        println!("{:?}", Solution::permute(nums));
    }
}
