use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[Usize1; n]; m],
    }
    let mut set = HashSet::new();
    for a in a {
        for a in a.windows(2) {
            if a[0] < a[1] {
                set.insert((a[0], a[1]));
            } else {
                set.insert((a[1], a[0]));
            }
        }
    }
    let result = n * (n - 1) / 2 - set.len();
    println!("{}", result);
}
