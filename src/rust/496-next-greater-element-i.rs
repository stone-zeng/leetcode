// 496. Next Greater Element I
// https://leetcode.com/problems/next-greater-element-i/

struct Solution {}

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1
            .iter()
            .map(|i| {
                let pos = nums2.iter().position(|x| x == i).unwrap();
                *nums2[pos..].iter().find(|&x| x > i).unwrap_or(&-1)
            })
            .collect()
    }
}

fn main() {
    [
        (vec![4, 1, 2], vec![1, 3, 4, 2], vec![-1, 3, -1]),
        (vec![2, 4], vec![1, 2, 3, 4], vec![3, -1]),
    ]
    .iter()
    .for_each(|(nums1, nums2, res)| {
        let nums1 = nums1.to_vec();
        let nums2 = nums2.to_vec();
        let res = res.to_vec();
        assert_eq!(Solution::next_greater_element(nums1, nums2), res);
    });
}
