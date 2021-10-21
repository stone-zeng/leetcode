// 33. Search in Rotated Sorted Array
// https://leetcode.com/problems/search-in-rotated-sorted-array/

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // `partition_point` is stable since Rust v1.52.0
        // let p = nums.partition_point(|&n| n >= nums[0]);
        let p = Self::max_pos(&nums) + 1;
        nums[..p]
            .binary_search(&target)
            .or_else(|_| nums[p..].binary_search(&target).map(|x| x + p))
            .map_or(-1, |x| x as i32)
    }

    fn max_pos(nums: &[i32]) -> usize {
        match nums.len() {
            0 => unreachable!(),
            1 => 0,
            2 => if nums[0] < nums[1] { 1 } else { 0 },
            len => {
                let i = len / 2;
                let a = nums[i - 1];
                let b = nums[i];
                let c = nums[i + 1];
                if a < b && b < c {
                    if a >= nums[0] {
                        i + Self::max_pos(&nums[i..])
                    } else {
                        Self::max_pos(&nums[..i])
                    }
                } else if a > b {
                    i - 1
                } else if b > c {
                    i
                } else {
                    unreachable!()
                }
            }
        }
    }
}

fn main() {
    [
        (vec![4, 5, 6, 7, 0, 1, 2], 0, 4),
        (vec![4, 5, 6, 7, 0, 1, 2], 4, 0),
        (vec![4, 5, 6, 7, 0, 1, 2], 7, 3),
        (vec![4, 5, 6, 7, 0, 1, 2], 3, -1),
        (vec![1], 0, -1),
    ]
    .iter()
    .for_each(|(nums, target, res)| assert_eq!(Solution::search(nums.clone(), *target), *res));
}
