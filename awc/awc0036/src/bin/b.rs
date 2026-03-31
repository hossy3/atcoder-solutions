use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
        tx: [(u8, usize); q],
    }

    let mut set = HashSet::new();
    for (t, x) in tx {
        if t == 1 {
            set.insert(x);
        } else {
            let yes = set.contains(&x);
            println!("{}", if yes { "Yes" } else { "No" });
        }
    }
}
