use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut set0 = BTreeSet::new();
    let mut set1 = BTreeSet::new();
    for (i, &x) in a.iter().enumerate() {
        if i < n / 2 {
            set0.insert((x, i));
        } else {
            set1.insert((x, i));
        }
    }

    let mut result = 0usize;
    while let Some((x, _)) = set0.pop_first() {
        let Some(&(x0, i0)) = set1.range((x * 2, 0)..).next() else {
            continue;
        };
        result += 1;
        set1.remove(&(x0, i0));
    }
    println!("{result}");
}
