// 374. Guess Number Higher or Lower
// https://leetcode.com/problems/guess-number-higher-or-lower/

use std::cmp::Ordering;

/// Forward declaration of guess API.
unsafe fn guess(num: i32) -> i32 {
    match 71.cmp(&num) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

struct Solution;

impl Solution {
    #[allow(non_snake_case)]
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut i = n;
        let mut n = n;
        loop {
            // It's equivalent to `n = (n + 1) / 2`, but will avoid overflow.
            n = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };
            match guess(i) {
                -1 => i -= n,
                1 => i += n,
                0 => return i,
                _ => unreachable!(),
            }
        }
    }
}

fn main() {
    [71, 100, 107, 2147483647]
        .iter()
        .for_each(|num| println!("{}", unsafe { Solution::guessNumber(*num) }));
}
