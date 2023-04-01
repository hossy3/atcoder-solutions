use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n],
    }
    let mut s = BTreeSet::new();
    for &i in &a {
        s.insert(i);
    }
    for &i in &a {
        let j = i - x;
        if s.contains(&j) {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}
