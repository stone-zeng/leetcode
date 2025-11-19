// 11. Container With Most Water
// https://leetcode.com/problems/container-with-most-water/

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut area = 0;
        while i < j {
            let (h_i, h_j) = (height[i], height[j]);
            area = area.max((j - i) as i32 * h_i.min(h_j));
            if h_i < h_j {
                while i < height.len() && h_i >= height[i] {
                    i += 1;
                }
            } else {
                while j > 0 && h_j >= height[j] {
                    j -= 1;
                }
            }
        }
        area
    }
}

fn main() {
    for (height, res) in [
        (vec![1, 8, 6, 2, 5, 4, 8, 3, 7], 49),
        (vec![1, 1], 1),
        (vec![4, 3, 2, 1, 4], 16),
        (vec![1, 2, 1], 2),
    ] {
        assert_eq!(Solution::max_area(height), res)
    }
}
