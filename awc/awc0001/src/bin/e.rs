use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [isize; n],
    }
    let mut set = BTreeSet::new();
    for i in 0..k {
        set.insert((h[i], i));
    }
    let mut result = set.last().unwrap().0 - set.first().unwrap().0;
    for i in k..n {
        set.remove(&(h[i - k], i - k));
        set.insert((h[i], i));
        result = result.max(set.last().unwrap().0 - set.first().unwrap().0);
    }
    println!("{result}");
}
