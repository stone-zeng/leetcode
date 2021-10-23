// 153. Find Minimum in Rotated Sorted Array
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/

struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        nums[Self::min_pos(&nums)]
    }

    fn min_pos(nums: &[i32]) -> usize {
        match nums.len() {
            0 => unreachable!(),
            1 => 0,
            #[rustfmt::skip]
            2 => if nums[0] < nums[1] { 0 } else { 1 },
            len => {
                let i = len / 2;
                let a = nums[i - 1];
                let b = nums[i];
                let c = nums[i + 1];
                if a < b && b < c {
                    if a <= *nums.last().unwrap() {
                        Self::min_pos(&nums[..i])
                    } else {
                        i + Self::min_pos(&nums[i..])
                    }
                } else if a > b {
                    i
                } else if b > c {
                    i + 1
                } else {
                    unreachable!()
                }
            }
        }
    }
}

fn main() {
    [
        (vec![3, 4, 5, 1, 2], 1),
        (vec![4, 5, 6, 7, 0, 1, 2], 0),
        (vec![11, 13, 15, 17], 11),
        (vec![2, 3, 4, 5, 1], 1),
    ]
    .iter()
    .for_each(|(nums, res)| assert_eq!(Solution::find_min(nums.to_vec()), *res));
}
