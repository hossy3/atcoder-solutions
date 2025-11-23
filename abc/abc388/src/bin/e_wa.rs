use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut set = BTreeSet::new();
    for (i, &x) in a.iter().enumerate() {
        set.insert((x, i));
    }

    let mut result = 0usize;
    while let Some((x, _)) = set.pop_first() {
        let Some(&(x0, i0)) = set.range((x * 2, 0)..).next() else {
            break;
        };
        result += 1;
        set.remove(&(x0, i0));
    }
    println!("{result}");
}
