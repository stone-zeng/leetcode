// 557. Reverse Words in a String III
// https://leetcode.com/problems/reverse-words-in-a-string-iii/

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .map(|i| i.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn main() {
    [
        ("Let's take LeetCode contest", "s'teL ekat edoCteeL tsetnoc"),
        ("God Ding", "doG gniD"),
    ]
    .iter()
    .for_each(|&(s, res)| assert_eq!(Solution::reverse_words(s.to_string()), res));
}
