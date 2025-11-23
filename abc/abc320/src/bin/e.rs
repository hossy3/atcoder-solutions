use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        tws: [(usize, usize, usize); m],
    }

    let mut set: BTreeSet<_> = (0..n).collect();
    let mut results = vec![0; n];

    let mut pairs = BTreeSet::<(usize, usize)>::new(); // time, id
    for (t, w, s) in tws {
        while let Some((t0, i)) = pairs.pop_first() {
            if t0 > t {
                pairs.insert((t0, i));
                break;
            }
            set.insert(i);
        }

        if let Some(i) = set.pop_first() {
            results[i] += w;
            pairs.insert((t + s, i));
        }
    }

    for x in results {
        println!("{x}");
    }
}
