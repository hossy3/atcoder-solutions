use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: isize,
        a: [isize; n],
    }

    let mut set: BTreeSet<(isize, usize)> = BTreeSet::new();
    let mut count = 0usize;
    let mut j = 0;
    for i in 0..n {
        while j < n {
            let x = a[j];
            if set.range((x - d + 1, 0)..(x + d, 0)).next().is_some() {
                break;
            }
            set.insert((x, j));
            j += 1;
        }
        count += j - i;
        let x = a[i];
        set.remove(&(x, i));
    }
    println!("{count}");
}
