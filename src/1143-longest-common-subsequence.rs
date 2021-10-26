// 1143. Longest Common Subsequence
// https://leetcode.com/problems/longest-common-subsequence/

struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        if text1.is_empty() || text2.is_empty() {
            return 0;
        }
        let mut res = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        text1.chars().enumerate().for_each(|(i, ci)| {
            text2.chars().enumerate().for_each(|(j, cj)| {
                let temp = if ci == cj {
                    res[i][j] + 1
                } else {
                    i32::max(res[i][j + 1], res[i + 1][j])
                };
                res[i + 1][j + 1] = temp;
            })
        });
        *res.last().unwrap().last().unwrap()
    }
}

fn main() {
    [
        ("abcde", "ace", 3),
        ("abc", "abc", 3),
        ("abc", "def", 0),
        ("sejsrtcaprkbllymqysx", "bbwwvqarnjnhakzqyaun", 5),
    ]
    .iter()
    .for_each(|(text1, text2, res)| {
        assert_eq!(
            Solution::longest_common_subsequence(text1.to_string(), text2.to_string()),
            *res
        )
    })
}
