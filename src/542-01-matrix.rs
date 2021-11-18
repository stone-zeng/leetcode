// 542. 01 Matrix
// https://leetcode.com/problems/01-matrix/

use std::collections::HashMap;

type DistanceMap = HashMap<(usize, usize), i32>;

struct Solution;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut map = Self::init(&mat);
        for d in 0.. {
            if map.len() == m * n {
                return Self::map_to_map(&map, m, n);
            }
            let mut temp = HashMap::new();
            #[rustfmt::skip]
            map.iter()
                .filter(|(_, c)| c == &&d)
                .for_each(|(&(i, j), _)| {
                    if i != 0     && !map.contains_key(&(i - 1, j)) { temp.insert((i - 1, j), d + 1); }
                    if i != m - 1 && !map.contains_key(&(i + 1, j)) { temp.insert((i + 1, j), d + 1); }
                    if j != 0     && !map.contains_key(&(i, j - 1)) { temp.insert((i, j - 1), d + 1); }
                    if j != n - 1 && !map.contains_key(&(i, j + 1)) { temp.insert((i, j + 1), d + 1); }
                });
            map.extend(temp.iter());
        }
        unreachable!()
    }

    fn init(mat: &[Vec<i32>]) -> DistanceMap {
        let mut map = HashMap::new();
        for (i, v) in mat.iter().enumerate() {
            for (j, c) in v.iter().enumerate() {
                if c == &0 {
                    map.insert((i, j), 0);
                }
            }
        }
        map
    }

    fn map_to_map(map: &DistanceMap, m: usize, n: usize) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; n]; m];
        map.iter()
            .filter(|(_, d)| d != &&0)
            .for_each(|(&(i, j), &d)| {
                res[i][j] = d;
            });
        res
    }
}

fn main() {
    for mat in [
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]],
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
        vec![vec![1, 1], vec![0, 1]],
        vec![
            vec![1, 1, 1, 1, 1, 0, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 1, 0, 1, 1, 0],
            vec![0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 1, 0, 1, 0, 1, 0, 1, 1, 1],
            vec![0, 1, 0, 1, 1, 1, 0, 0, 1, 1],
            vec![1, 1, 0, 1, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 1, 1, 1, 0, 0],
            vec![1, 1, 1, 0, 0, 1, 0, 1, 0, 1],
            vec![0, 1, 0, 0, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 0, 1, 1, 1, 0, 0, 0, 1],
        ],
    ] {
        println!("{:?}", Solution::update_matrix(mat));
    }
}
