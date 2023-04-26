use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut set = HashSet::new();
    for x in a {
        set.insert(x);
    }
    let result = set.len();
    println!("{}", result);
}
