// 10. Regular Expression Matching
// https://leetcode.com/problems/regular-expression-matching/

#[derive(Debug, Clone, Copy)]
enum Pattern {
    Char(char),
    RepeatedChar(char),
    Any,
    RepeatedAny,
}

impl Pattern {
    fn to_repeated(self) -> Self {
        match self {
            Self::Char(c) => Self::RepeatedChar(c),
            Self::Any => Self::RepeatedAny,
            _ => unreachable!(),
        }
    }
}

struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Self::is_match_helper(&s, &Self::to_pattern_vec(p))
    }

    fn is_match_helper(s: &str, patterns: &[Pattern]) -> bool {
        match patterns.len() {
            1 => Self::match_single(s, patterns[0]),
            _ => {
                let p = patterns[0];
                for i in 0..=s.len() {
                    if Self::match_single(&s[..i], p)
                        && Self::is_match_helper(&s[i..], &patterns[1..])
                    {
                        return true;
                    }
                }
                false
            }
        }
    }

    fn match_single(s: &str, p: Pattern) -> bool {
        match p {
            Pattern::Char(c) => s.len() == 1 && s.chars().all(|i| i == c),
            Pattern::RepeatedChar(c) => s.chars().all(|i| i == c),
            Pattern::Any => s.len() == 1,
            Pattern::RepeatedAny => true,
        }
    }

    fn to_pattern_vec(p: String) -> Vec<Pattern> {
        let mut res = Vec::<Pattern>::new();
        p.chars().for_each(|c| match c {
            '*' => {
                let last = res.last_mut().unwrap();
                *last = last.to_repeated();
            }
            '.' => res.push(Pattern::Any),
            _ => res.push(Pattern::Char(c)),
        });
        res
    }
}

fn main() {
    [
        ("aa", "a", false),
        ("aa", "a*", true),
        ("ab", ".*", true),
        ("aab", "c*a*b", true),
        ("mississippi", "mis*is*p*.", false),
        ("a", "aa", false),
    ]
    .iter()
    .for_each(|(s, p, res)| assert_eq!(Solution::is_match(s.to_string(), p.to_string()), *res));
}
