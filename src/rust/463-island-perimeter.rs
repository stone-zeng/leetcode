// 463. Island Perimeter
// https://leetcode.com/problems/island-perimeter/

struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let x = grid.len();
        let y = grid[0].len();
        match (x, y) {
            (1, 1) => 4,
            (1, _) => 2 * grid[0].iter().filter(|e| e == &&1).count() as i32 + 2,
            (_, 1) => 2 * grid.iter().filter(|v| v[0] == 1).count() as i32 + 2,
            (_, _) => {
                let mut g = grid;
                g.iter_mut().for_each(|v| {
                    v.insert(0, 0);
                    v.push(0);
                });
                g.insert(0, vec![0; y + 2]);
                g.push(vec![0; y + 2]);
                let mut perimeter = 0;
                for i in 1..=x {
                    for j in 1..=y {
                        if g[i][j] == 1 {
                            let neighbors = [g[i - 1][j], g[i + 1][j], g[i][j - 1], g[i][j + 1]];
                            perimeter += neighbors.iter().filter(|e| e == &&0).count();
                        }
                    }
                }
                perimeter as i32
            }
        }
    }
}

fn main() {
    [
        (vec![vec![0, 1, 0, 0], vec![1, 1, 1, 0], vec![0, 1, 0, 0], vec![1, 1, 0, 0]], 16),
        (vec![vec![1]], 4),
        (vec![vec![1, 0]], 4),
    ]
    .iter()
    .for_each(|(grid, res)| assert_eq!(Solution::island_perimeter(grid.to_vec()), *res));
}
