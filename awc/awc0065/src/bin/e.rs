use std::collections::BTreeSet;

use proconio::input;

// two-pointer technique (尺取り法)
// awc0058-e とほぼ同じ

fn main() {
    input! {
        n: usize,
        k: usize,
        d: isize,
        h: [isize; n],
    }

    let mut result = 0;
    let mut r = 0;
    let mut set: BTreeSet<(isize, usize)> = BTreeSet::new();
    for l in 0..n {
        while r < h.len()
            && (set.len() == 0
                || set.last().unwrap().0.max(h[r]) - set.first().unwrap().0.min(h[r]) <= d)
        {
            set.insert((h[r], r));
            r += 1;
            result = result.max(set.len());
        }
        set.remove(&(h[l], l));
    }

    if result >= k {
        println!("{result}");
    } else {
        println!("-1");
    }
}
