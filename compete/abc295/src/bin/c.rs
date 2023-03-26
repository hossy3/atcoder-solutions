use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut s = HashSet::new();
    let mut count = 0usize;
    for x in a {
        if s.remove(&x) {
            count += 1;
        } else {
            s.insert(x);
        }
    }
    println!("{}", count);
}
