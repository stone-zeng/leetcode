// 733. Flood Fill
// https://leetcode.com/problems/flood-fill/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let (sr, sc) = (sr as usize, sc as usize);
        let (x_size, y_size) = (image.len(), image[0].len());
        let color = image[sr][sc];
        let mut img = image;
        let mut coords = HashSet::new();
        let mut next = HashSet::new();
        coords.insert((sr, sc));
        next.insert((sr, sc));

        let neighbors = |i, j| -> Vec<_> {
            let x_mask = [true, true, i != 0, i != x_size - 1];
            let y_mask = [j != 0, j != y_size - 1, true, true];
            [(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)]
                .iter()
                .zip(x_mask.iter().zip(y_mask.iter()))
                .filter(|&(_, (&a, &b))| a && b)
                .map(|(&c, _)| c)
                .collect()
        };

        while !next.is_empty() {
            let mut temp = HashSet::new();
            next.iter().for_each(|&(i, j)| {
                neighbors(i, j).iter().for_each(|&(x, y)| {
                    if img[x][y] == color && !coords.contains(&(x, y)) {
                        coords.insert((x, y));
                        temp.insert((x, y));
                    }
                })
            });
            next = temp;
        }
        coords.iter().for_each(|&(i, j)| img[i][j] = new_color);
        img
    }
}

fn main() {
    [
        (vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
        (vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 2),
        (vec![vec![0, 0, 0], vec![0, 1, 1]], 1, 1, 1),
    ]
    .iter()
    .for_each(|(nums, sr, sc, new_color)| {
        println!(
            "{:?}",
            Solution::flood_fill(nums.to_vec(), *sr, *sc, *new_color)
        )
    })
}
