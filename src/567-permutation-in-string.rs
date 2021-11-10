// 567. Permutation in String
// https://leetcode.com/problems/permutation-in-string/

struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let len1 = s1.as_bytes().len();
        let len2 = s2.as_bytes().len();
        if len1 > len2 {
            false
        } else {
            let mask1 = Self::to_mask(s1.as_bytes());
            let mut mask2 = Self::to_mask(&s2.as_bytes()[..len1]);
            if mask1 == mask2 {
                true
            } else {
                s2.as_bytes().windows(len1 + 1).any(|w| {
                    Self::update_mask(&mut mask2, w[0], w[len1]);
                    mask1 == mask2
                })
            }
        }
    }

    fn to_mask(s: &[u8]) -> [usize; 26] {
        let mut mask = [0; 26];
        s.iter().for_each(|&i| mask[Self::to_index(i)] += 1);
        mask
    }

    fn update_mask(mask: &mut [usize; 26], i: u8, j: u8) {
        mask[Self::to_index(i)] -= 1;
        mask[Self::to_index(j)] += 1;
    }

    const fn to_index(i: u8) -> usize {
        (i - b'a') as usize
    }
}

fn main() {
    [
        ("a", "a", true),
        ("a", "b", false),
        ("ab", "a", false),
        ("ab", "bbbbba", true),
        ("ab", "eidbaooo", true),
        ("ab", "eidboaoo", false),
    ]
    .iter()
    .for_each(|&(s1, s2, res)| {
        assert_eq!(
            Solution::check_inclusion(s1.to_string(), s2.to_string()),
            res
        )
    })
}
