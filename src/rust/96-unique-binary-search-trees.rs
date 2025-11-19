// 96. Unique Binary Search Trees
// https://leetcode.com/problems/unique-binary-search-trees/

struct Solution;

impl Solution {
    /// It gives the Catalan number: C(n) = binomial(2n,n)/(n+1) = (2n)!/(n!(n+1)!)
    /// https://oeis.org/A000108
    pub fn num_trees(n: i32) -> i32 {
        const MAX_NUM: usize = 20;
        let mut data = vec![0; MAX_NUM];
        data[0] = 1;
        data[1] = 1;
        Self::num_trees_helper(n as usize, &mut data)
    }

    fn num_trees_helper(n: usize, data: &mut Vec<i32>) -> i32 {
        if data[n] > 0 {
            return data[n];
        }
        let mut res = (1..=n / 2)
            .map(|i| Self::num_trees_helper(i - 1, data) * Self::num_trees_helper(n - i, data) * 2)
            .sum();
        if n % 2 == 1 {
            let left = Self::num_trees_helper((n - 1) / 2, data);
            res += left * left;
        }
        data[n] = res;
        res
    }
}

fn main() {
    (1..20).for_each(|n| println!("{} => {}", n, Solution::num_trees(n)));
}
