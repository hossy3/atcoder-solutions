use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        _m: usize,
    }

    let mut set = HashSet::new();
    for i in 0..n {
        input! {
            k: usize,
            c: [usize; k],
        }
        let set0: HashSet<_> = c.into_iter().collect();
        if i == 0 {
            set = set0;
        } else {
            set = set.intersection(&set0).cloned().collect();
        }
    }

    let result = set.len();
    println!("{result}");
}
