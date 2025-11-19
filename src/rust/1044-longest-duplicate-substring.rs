// 1044. Longest Duplicate Substring
// https://leetcode.com/problems/longest-duplicate-substring/

use std::cmp::Ordering::{Greater, Less};
use std::collections::HashSet;

type HashT = u128;
const HASH_BASE: HashT = 128;
const HASH_MODULUS: HashT = 2305843009213693951; // 2^61 - 1

struct Solution;

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let mut res = "";
        (0..s.len())
            .collect::<Vec<_>>()
            .binary_search_by(|&n| match Self::dup_substring(&s, n).map(|r| res = r) {
                Some(_) => Less,
                None => Greater,
            })
            .unwrap_or_default();
        res.to_string()
    }

    fn dup_substring(s: &str, n: usize) -> Option<&str> {
        match n {
            0 => None,
            _ => {
                let bytes = s.as_bytes();
                let pow = Self::pow(n);
                let mut hash = Self::hash(&bytes[..n]);
                let mut set = HashSet::new();
                set.insert(hash);
                for (i, (a, b)) in bytes.iter().zip(bytes[n..].iter()).enumerate() {
                    hash = Self::hash_next(hash, pow, *a, *b);
                    if !set.insert(hash) {
                        return Some(&s[(i + 1)..(i + n + 1)]);
                    }
                }
                None

                // A slower implementation, i.e. use the default hash.
                // let mut set = HashSet::new();
                // for (i, w) in s.as_bytes().windows(n).enumerate() {
                //     if !set.insert(w) {
                //         return Some(&s[i..(i + n)]);
                //     }
                // }
                // None
            }
        }
    }

    fn pow(n: usize) -> HashT {
        (1..n).fold(1, |res, _| res * HASH_BASE % HASH_MODULUS)
    }

    fn hash(s: &[u8]) -> HashT {
        s.iter()
            .fold(0, |res, &i| (res * HASH_BASE) % HASH_MODULUS + i as HashT)
            % HASH_MODULUS
    }

    const fn hash_next(hash: HashT, pow: HashT, a: u8, b: u8) -> HashT {
        let a = a as HashT;
        let b = b as HashT;
        ((hash + HASH_MODULUS - a * pow % HASH_MODULUS) * HASH_BASE + b) % HASH_MODULUS
    }
}

fn main() {
    [
        ("aa", "a"),
        ("zxcvdqkfawuytt", "t"),
        ("banana", "ana"),
        ("abcd", ""),
        ("ccbccbbbacaccbbaccabbccbbaccbc", "ccbbacc"),
    ]
    .iter()
    .for_each(|(s, res)| assert_eq!(Solution::longest_dup_substring(s.to_string()), *res));
}
