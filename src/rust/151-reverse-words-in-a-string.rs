// 151. Reverse Words in a String
// https://leetcode.com/problems/reverse-words-in-a-string/

struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn main() {
    [
        ("the sky is blue", "blue is sky the"),
        ("  hello world  ", "world hello"),
        ("a good   example", "example good a"),
        ("  Bob    Loves  Alice   ", "Alice Loves Bob"),
        ("Alice does not even like bob", "bob like even not does Alice"),
    ]
    .iter()
    .for_each(|(s, res)| assert_eq!(Solution::reverse_words(s.to_string()), res.to_string()));
}
