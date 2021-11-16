// 1286. Iterator for Combination
// https://leetcode.com/problems/iterator-for-combination/

struct CombinationIterator {
    chars: Vec<u8>,
    combs: Vec<u16>,
    index: usize,
}

impl CombinationIterator {
    fn new(characters: String, combinationLength: i32) -> Self {
        let mut chars = characters.as_bytes().to_vec();
        chars.sort_unstable();

        let n = chars.len();
        let len = combinationLength as usize;

        let mut temp = vec![0];
        let mut combs = vec![];

        for i in 0..len {
            combs.clear();
            temp.iter().for_each(|&e| {
                let k = n - len + i;
                for j in (16 - u16::leading_zeros(e) as usize)..=k {
                    combs.push(e + (1 << j));
                }
            });
            temp = combs.clone();
        }

        Self {
            combs,
            chars,
            index: 0,
        }
    }

    fn next(&mut self) -> String {
        let comb = self.combs[self.index];
        let len = 16 - comb.leading_zeros();
        self.index += 1;
        let s = (0..len)
            .filter(|&i| (comb >> i) % 2 != 0)
            .map(|i| self.chars[i as usize])
            .collect();
        String::from_utf8(s).unwrap()
    }

    fn has_next(&self) -> bool {
        self.index < self.combs.len()
    }
}

fn main() {
    let mut iter = CombinationIterator::new("abcdefghijk".to_string(), 8);
    (0..10).for_each(|_| println!("{}", iter.next()));
    println!("{}", iter.has_next());
}
