// 3. Longest Substring Without Repeating Characters
// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let bytes = s.as_bytes();
        let mut set = HashSet::new();
        let mut len = Self::update_set(&mut set, bytes);
        let mut i = 1;
        while i + len < bytes.len() {
            set.remove(&bytes[i - 1]);
            let begin = i + set.len();
            len = len.max(Self::update_set(&mut set, &bytes[begin..]));
            i += 1;
        }
        len as i32
    }

    fn update_set(set: &mut HashSet<u8>, bytes: &[u8]) -> usize {
        bytes.iter().any(|c| !set.insert(*c));
        set.len()
    }
}

fn main() {
    [
        ("abcabcbb", 3),
        ("bbbbb", 1),
        ("abc", 3),
        ("pwwkew", 3),
        ("pwwkewpwwkewpwwkewpwwkewpwwkewpwwkewpwwkewpwwkewpwwkewpwwkewpwwkewpwwkewpwwkewpwwkewpwwkewpwwkew", 4),
        ("", 0),
        ("b", 1),
        ("bb", 1),
        ("ab", 2),
        ("abb", 2),
        ("https://leetcode.com/problems/longest-substring-without-repeating-characters/", 10),
    ]
    .iter()
    .for_each(|&(s, res)| assert_eq!(Solution::length_of_longest_substring(s.to_string()), res))
}
