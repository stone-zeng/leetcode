// 977. Squares of a Sorted Array
// https://leetcode.com/problems/squares-of-a-sorted-array/

struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let p = nums.binary_search_by(|x| x.cmp(&0)).unwrap_or_else(|i| i);
        let mut i = nums[..p].iter().rev().peekable();
        let mut j = nums[p..].iter().peekable();
        let mut res = vec![];
        loop {
            match i.peek() {
                Some(&&x) => match j.peek() {
                    Some(&&y) if y < -x => {
                        res.push(y * y);
                        j.next();
                    }
                    _ => {
                        res.push(x * x);
                        i.next();
                    }
                },
                None => {
                    for &y in j {
                        res.push(y * y)
                    }
                    return res;
                }
            }
        }
    }
}

#[rustfmt::skip]
fn main() {
    [
        vec![-4, -1, 0, 3, 10],
        vec![-7, -3, 2, 3, 11],
        vec![-7, -3, -2, -1, 0],
        vec![-7, -3, -2, -1],
        vec![0, 1, 3, 5],
        vec![1, 3, 5],
        vec![0],
        vec![-98, -90, -88, -57, -56, -33, -28, -14, 10, 17, 22, 22, 67, 82, 91],
    ]
    .iter()
    .for_each(|nums| println!("{:?}", Solution::sorted_squares(nums.to_vec())))
}
