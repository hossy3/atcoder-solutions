use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    }

    let mut set = HashSet::new();
    set.insert(0usize);
    for &(a, b) in &ab {
        let v: Vec<usize> = set.into_iter().collect();
        set = HashSet::new();
        for x in v {
            set.insert(x + a);
            set.insert(x + b);
        }
    }
    let yes = set.contains(&x);
    println!("{}", if yes { "Yes" } else { "No" });
}
