// 130. Surrounded Regions
// https://leetcode.com/problems/surrounded-regions/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let x = board.len();
        let y = board[0].len();
        if x < 3 || y < 3 {
            return;
        }
        let mut flag = vec![vec![false; y]; x];
        for i in 1..x - 1 {
            for j in 1..y - 1 {
                let mut region = Region::new(board, &mut flag);
                region.check(i, j);
                region.flip();
            }
        }
    }
}

struct Region<'a> {
    board: &'a mut Vec<Vec<char>>,
    flag: &'a mut Vec<Vec<bool>>,
    coords: HashSet<(usize, usize)>,
    if_flip: bool,
}

impl<'a> Region<'a> {
    fn new(board: &'a mut Vec<Vec<char>>, flag: &'a mut Vec<Vec<bool>>) -> Self {
        Self {
            board,
            flag,
            coords: HashSet::new(),
            if_flip: true,
        }
    }

    fn check(&mut self, i: usize, j: usize) {
        if self.board[i][j] == 'O' && !self.flag[i][j] {
            let x = self.board.len();
            let y = self.board[0].len();
            self.coords.insert((i, j));
            self.flag[i][j] = true;
            if i == 0 || i == x - 1 || j == 0 || j == y - 1 { self.if_flip = false }
            if i != 0 { self.check(i - 1, j) }
            if i != x - 1 { self.check(i + 1, j) }
            if j != 0 { self.check(i, j - 1) }
            if j != y - 1 { self.check(i, j + 1) }
        }
    }

    fn flip(&mut self) {
        if self.if_flip {
            for &(i, j) in &self.coords {
                self.board[i][j] = 'X';
            }
        }
    }
}

fn main() {
    [
        vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ],
        vec![vec!['X']],
    ]
    .iter_mut()
    .for_each(|board| {
        Solution::solve(board);
        println!("{:?}", &board);
    });
}
