// 1178. Number of Valid Words for Each Puzzle
// https://leetcode.com/problems/number-of-valid-words-for-each-puzzle/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut words_map = HashMap::new();
        words.iter().for_each(|word| {
            let c = words_map.entry(Self::word_to_bit_mask(word)).or_insert(0);
            *c += 1;
        });

        puzzles
            .iter()
            .map(|puzzle| {
                let first = 1 << (puzzle.as_bytes().first().unwrap() - b'a');
                let mask = Self::word_to_bit_mask(puzzle);
                words_map
                    .iter()
                    .filter(|(word, _)| Self::is_valid(**word, mask, first))
                    .map(|(_, num)| num)
                    .sum()
            })
            .collect()
    }

    fn word_to_bit_mask(word: &str) -> u32 {
        word.as_bytes()
            .iter()
            .map(|c| 1 << (c - b'a'))
            .fold(0, |res, i| res | i)
    }

    const fn is_valid(word: u32, puzzle: u32, first: u32) -> bool {
        puzzle & word == word && word & first == first
    }
}

fn main() {
    [
        (
            vec!["aaaa", "asas", "able", "ability", "actt", "actor", "access"],
            vec!["aboveyz", "abrodyz", "abslute", "absoryz", "actresz", "gaswxyz"],
            vec![1, 1, 3, 2, 4, 0],
        ),
        (
            vec!["apple", "pleas", "please"],
            vec!["aelwxyz", "aelpxyz", "aelpsxy", "saelpxy", "xaelpsy"],
            vec![0, 1, 3, 2, 0],
        ),
    ]
    .iter()
    .for_each(|(words, puzzles, res)| {
        let words = words.iter().map(|s| s.to_string()).collect();
        let puzzles = puzzles.iter().map(|s| s.to_string()).collect();
        assert_eq!(
            Solution::find_num_of_valid_words(words, puzzles),
            res.to_vec()
        )
    });
}
