// 784. Letter Case Permutation
// https://leetcode.com/problems/letter-case-permutation/

struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut res: Vec<Vec<u8>> = vec![vec![]];
        for ch in s.bytes() {
            let mut temp = vec![];
            let next_c = match ch.is_ascii_alphabetic() {
                true => vec![ch.to_ascii_lowercase(), ch.to_ascii_uppercase()],
                false => vec![ch],
            };
            for c in next_c {
                res.iter().for_each(|i| {
                    let mut i = i.clone();
                    i.push(c);
                    temp.push(i);
                });
            }
            res = temp;
        }
        res.iter()
            .map(|i| String::from_utf8(i.to_vec()).unwrap())
            .collect()
    }
}

fn main() {
    for s in ["a1b2", "3z4", "12345", "0"] {
        println!("{:?}", Solution::letter_case_permutation(s.to_string()));
    }
}
