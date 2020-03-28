// 118. Pascal's Triangle
// https://leetcode.com/problems/pascals-triangle/

struct Solution { }

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows == 0 {
            return vec![];
        }
        let mut result = vec!(vec![1]);
        for _ in 1..num_rows {
            result.push(Solution::generate_row(result.last().unwrap()));
        }
        result
    }

    fn generate_row(row: &Vec<i32>) -> Vec<i32> {
        [vec![1], (0..row.len() - 1).map(|n| row[n] + row[n + 1]).collect(), vec![1]].concat()
    }
}

fn main() {
    for i in &[0, 1, 2, 3, 4, 30] {
        println!("{:?}", Solution::generate(*i));
    }
}
