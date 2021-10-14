// 881. Boats to Save People
// https://leetcode.com/problems/boats-to-save-people/

struct Solution {}

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort_unstable();

        let mut i = 0;
        let mut j = people.len() - 1;
        let mut res = 0;

        loop {
            if i == j { return res + 1; }
            if i > j { return res; }
            if people[i] + people[j] <= limit {
                i += 1
            }
            j -= 1;
            res += 1;
        }
    }
}

fn main() {
    [
        (vec![1, 2], 3, 1),
        (vec![3, 2, 2, 1], 3, 3),
        (vec![3, 5, 3, 4], 5, 4),
        (vec![23, 35, 27, 13, 11, 6, 27, 35, 12, 20, 28, 22, 9, 3, 6, 37, 21, 5, 11, 29, 3, 36, 34, 29, 20, 6, 32, 15, 25, 4, 19, 13, 24, 19, 40, 3, 24, 17, 2, 11, 5, 11, 1, 23, 8, 9, 8, 21, 35, 21, 35, 16, 38, 19, 17, 8, 12, 23, 2, 26, 1, 28, 26, 10, 5, 10, 18, 38, 38, 5, 20, 4, 12, 10, 35, 11, 29, 7, 31, 14, 22, 36, 8, 1, 39, 22, 24, 27, 23, 14, 35, 2, 14, 19, 20, 28, 29, 40, 1, 32], 40, 51),
    ]
    .iter()
    .for_each(|(people, limit, res)| assert_eq!(Solution::num_rescue_boats(people.to_vec(), *limit), *res));
}
