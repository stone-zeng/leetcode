// 941. Valid Mountain Array
// https://leetcode.com/problems/valid-mountain-array/

struct Solution {}

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            false
        } else {
            match arr.windows(2).position(|u| u[0] >= u[1]) {
                Some(i) => {
                    if i == 0 || arr[i] == arr[i + 1] {
                        false
                    } else {
                        arr[i..].windows(2).all(|u| u[0] > u[1])
                    }
                }
                None => false,
            }
        }
    }
}

fn main() {
    [
        (vec![1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 3, 2, 1], true),
        (vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0], false),
        (vec![1, 2, 3, 4, 5, 6, 7, 7, 6, 5, 4, 3, 2, 1], false),
        (vec![2, 1], false),
        (vec![3, 5, 5], false),
        (vec![0, 3, 2, 1], true),
        (vec![1, 2, 3, 4, 5], false),
    ]
    .iter()
    .for_each(|(nums, res)| assert_eq!(Solution::valid_mountain_array(nums.to_vec()), *res));
}
