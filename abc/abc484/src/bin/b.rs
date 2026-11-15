use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        k: usize,
        a: [usize; k],
    }

    let set: BTreeSet<_> = a.iter().cloned().collect();
    if set.contains(&9) {
        let result = set.iter().next().unwrap();
        println!("{result}");
    } else {
        println!("-1");
    }
}
