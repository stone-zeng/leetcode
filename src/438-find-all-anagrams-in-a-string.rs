// 11. Container With Most Water
// https://leetcode.com/problems/container-with-most-water/

struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }

        let p_counter = Self::counter(p.as_bytes());
        let bytes = s.bytes().collect::<Vec<_>>();
        let mut s_counter = Self::counter(&bytes[..p.len()]);
        let mut res = vec![];

        bytes
            .iter()
            .enumerate()
            .zip(bytes.iter().skip(p.len()).chain([b'a'].iter()))
            .for_each(|((i, a), b)| {
                if p_counter == s_counter {
                    res.push(i as i32)
                }
                s_counter[(a - b'a') as usize] -= 1;
                s_counter[(b - b'a') as usize] += 1;
            });
        res
    }

    fn counter(s: &[u8]) -> [usize; 26] {
        let mut res = [0; 26];
        s.iter().for_each(|i| res[(i - b'a') as usize] += 1);
        res
    }
}

fn main() {
    for (s, p, res) in [
        ("cbaebabacd", "abc", vec![0, 6]),
        ("abab", "ab", vec![0, 1, 2]),
    ] {
        assert_eq!(Solution::find_anagrams(s.to_string(), p.to_string()), res)
    }
}
