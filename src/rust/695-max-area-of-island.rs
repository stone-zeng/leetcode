// 695. Max Area of Island
// https://leetcode.com/problems/max-area-of-island/

struct Solution;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut data = grid;
        let mut grid = Grid::new(&mut data);
        let mut area = 0;
        for i in 0..grid.x {
            for j in 0..grid.y {
                if grid.data[i as usize][j as usize] == 1 {
                    area = area.max(grid.area(i, j));
                }
            }
        }
        area
    }
}

struct Grid<'a> {
    data: &'a mut Vec<Vec<i32>>,
    x: i32,
    y: i32,
}

impl<'a> Grid<'a> {
    fn new(data: &'a mut Vec<Vec<i32>>) -> Self {
        let (x, y) = (data.len() as i32, data[0].len() as i32);
        Self { data, x, y }
    }

    fn area(&mut self, i: i32, j: i32) -> i32 {
        if i < 0 || i >= self.x || j < 0 || j >= self.y || self.data[i as usize][j as usize] != 1 {
            0
        } else {
            self.data[i as usize][j as usize] = 2;
            1 + self.area(i - 1, j) + self.area(i + 1, j) + self.area(i, j - 1) + self.area(i, j + 1)
        }
    }
}

fn main() {
    [
        (
            vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            ],
            6,
        ),
        (vec![vec![0, 0, 0, 0, 0, 0, 0, 0]], 0),
    ]
    .iter()
    .for_each(|(grid, res)| assert_eq!(Solution::max_area_of_island(grid.to_vec()), *res))
}
