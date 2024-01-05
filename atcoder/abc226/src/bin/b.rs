use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut set = HashSet::new();
    for _ in 0..n {
        input! {
            a: [usize],
        }
        set.insert(a);
    }
    let result = set.len();
    println!("{}", result);
}
