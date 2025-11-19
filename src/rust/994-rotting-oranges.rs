// 994. Rotting Oranges
// https://leetcode.com/problems/rotting-oranges/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let grid = Self::pad_grid(&grid);

        let mut fresh_oranges = HashSet::new();
        let mut rotten_oranges = HashSet::new();

        #[rustfmt::skip]
        grid.iter().enumerate().for_each(|(i, v)| {
            v.iter().enumerate().for_each(|(j, c)| match c {
                1 => { fresh_oranges.insert((i, j)); }
                2 => { rotten_oranges.insert((i, j)); }
                _ => (),
            })
        });

        if fresh_oranges.is_empty() {
            return 0;
        }
        if rotten_oranges.is_empty() {
            return -1;
        }

        let mut time = 0;
        loop {
            if fresh_oranges.is_empty() {
                return time;
            }
            let mut temp = HashSet::new();
            #[rustfmt::skip]
            rotten_oranges.iter().for_each(|&(i, j)| {
                [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
                    .iter()
                    .filter(|&coord| fresh_oranges.remove(coord))
                    .for_each(|&coord| { temp.insert(coord); });
            });
            if rotten_oranges == temp {
                return -1;
            } else {
                rotten_oranges = temp
            }
            time += 1;
        }
    }

    fn pad_grid(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
        let mut g = grid.to_owned();
        g.iter_mut().for_each(|v| {
            v.insert(0, 0);
            v.push(0);
        });
        g.insert(0, vec![0; grid[0].len() + 2]);
        g.push(vec![0; grid[0].len() + 2]);
        g
    }
}

fn main() {
    [
        (vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]], 4),
        (vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]], -1),
        (vec![vec![0, 2]], 0),
        (vec![vec![0]], 0),
    ]
    .iter()
    .for_each(|(grid, res)| assert_eq!(Solution::oranges_rotting(grid.to_vec()), *res));
}
