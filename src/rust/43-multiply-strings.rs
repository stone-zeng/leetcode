// 43. Multiply Strings
// https://leetcode.com/problems/multiply-strings/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1_rev = Self::to_rev_digits(&num1);
        let num2_rev = Self::to_rev_digits(&num2);

        let mut products = HashMap::new();
        num2_rev.iter().filter(|j| j != &&0).for_each(|j| {
            products.entry(j).or_insert_with(|| {
                let mut temp = Vec::with_capacity(num1_rev.len() + 1);
                let mut carry = 0;
                num1_rev.iter().for_each(|i| {
                    let x = i * j + carry;
                    temp.push(x % 10);
                    carry = x / 10;
                });
                if carry > 0 {
                    temp.push(carry);
                }
                temp
            });
        });

        let mut res_rev = vec![0; num1.len() + num2.len()];
        num2_rev
            .iter()
            .enumerate()
            .filter(|(_, j)| j != &&0)
            .for_each(|(n, j)| {
                let product = &products[j];
                let mut carry = 0;
                product.iter().enumerate().for_each(|(i, s)| {
                    let item = &mut res_rev[n + i];
                    let x = *item + *s + carry;
                    *item = x % 10;
                    carry = x / 10;
                });
                if carry > 0 {
                    res_rev[n + product.len()] = carry;
                }
            });

        let res = res_rev
            .iter()
            .rev()
            .map(|i| i.to_string())
            .collect::<String>()
            .trim_start_matches('0')
            .to_string();

        if res.is_empty() {
            "0".to_string()
        } else {
            res
        }
    }

    fn to_rev_digits(num: &str) -> Vec<u8> {
        num.chars().rev().map(|c| c as u8 - b'0').collect()
    }
}

fn main() {
    [
        (0_u64, 0_u64),
        (2, 3),
        (123, 456),
        (15188, 921057),
        (9133, 0),
        (123456789, 987654321),
    ]
    .iter()
    .for_each(|(num1, num2)| {
        assert_eq!(
            Solution::multiply(num1.to_string(), num2.to_string()),
            (num1 * num2).to_string()
        )
    })
}
