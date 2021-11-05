// 441. Arranging Coins
// https://leetcode.com/problems/arranging-coins/

struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let n = n as f64;
        (((8.0 * n + 1.0).sqrt() - 1.0) / 2.0).floor() as i32
    }
}

fn main() {
    [5, 8, 1804289383, 2146467959, i32::MAX]
        .iter()
        .for_each(|&n| println!("{}: {}", n, Solution::arrange_coins(n)));
}
