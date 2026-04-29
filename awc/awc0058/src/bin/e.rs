use std::collections::BTreeSet;

use proconio::input;

// two-pointer technique (尺取り法)

fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
    }

    let mut result = 0;
    let mut r = 0;
    let mut set: BTreeSet<(isize, usize)> = BTreeSet::new();
    for l in 0..n {
        while r < a.len()
            && (set.len() == 0
                || set.last().unwrap().0.max(a[r]) - set.first().unwrap().0.min(a[r]) <= k)
        {
            set.insert((a[r], r));
            r += 1;
            result = result.max(set.len());
        }
        set.remove(&(a[l], l));
    }
    println!("{result}");
}
