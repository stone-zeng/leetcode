// 451. Sort Characters By Frequency
// https://leetcode.com/problems/sort-characters-by-frequency/

struct Solution {}

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut count = (b'0'..=b'z').map(|c| (c, 0)).collect::<Vec<_>>();
        s.chars().for_each(|c| {
            let n = &mut count[c as usize - 0x30];
            *n = (n.0, n.1 + 1);
        });
        count.sort_unstable_by_key(|(_, n)| *n);
        count
            .iter()
            .filter(|(_, n)| *n > 0)
            .rev()
            .map(|&(c, n)| (c as char).to_string().repeat(n))
            .collect::<Vec<_>>()
            .join("")
    }
}

fn main() {
    ["tree", "cccaaa", "Aabb"]
        .iter()
        .for_each(|s| println!("{}", Solution::frequency_sort(s.to_string())));
}
