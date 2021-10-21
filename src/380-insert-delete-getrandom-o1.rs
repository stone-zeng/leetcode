// 380. Insert Delete GetRandom O(1)
// https://leetcode.com/problems/insert-delete-getrandom-o1/

use rand::{rngs::ThreadRng, seq::IteratorRandom, thread_rng};
use std::collections::HashSet;

struct RandomizedSet {
    data: HashSet<i32>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            data: HashSet::new(),
            rng: thread_rng(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        self.data.insert(val)
    }

    fn remove(&mut self, val: i32) -> bool {
        self.data.remove(&val)
    }

    fn get_random(&mut self) -> i32 {
        *self.data.iter().choose(&mut self.rng).unwrap()
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
fn main() {
    let mut obj = RandomizedSet::new();
    println!("{}", obj.insert(1));
    println!("{}", obj.remove(2));
    println!("{}", obj.insert(2));
    println!("{}", obj.get_random());
    println!("{}", obj.remove(1));
    println!("{}", obj.insert(2));
    println!("{}", obj.get_random());
}
