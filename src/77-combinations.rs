// 77. Combinations
// https://leetcode.com/problems/combinations/

struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        Self::combine_helper(1, n, k)
    }

    fn combine_helper(a: i32, b: i32, k: i32) -> Vec<Vec<i32>> {
        match k {
            1 => (a..=b).map(|i| vec![i]).collect(),
            _ => {
                let mut res = vec![];
                for i in a..(b - k + 2) {
                    for v in Self::combine_helper(i + 1, b, k - 1) {
                        let mut v = v;
                        v.insert(0, i);
                        res.push(v)
                    }
                }
                res
            }
        }
    }
}

fn main() {
    for (n, k) in [(4, 2), (1, 1), (5, 5), (10, 4)] {
        println!("{:?}", Solution::combine(n, k));
    }
}
