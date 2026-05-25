use ac_library::FenwickTree;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        d: [usize; n],
        lrt: [(Usize1, Usize1, usize); q],
    }

    let mut events = vec![];
    for (i, &d) in d.iter().enumerate() {
        events.push((d, None, i));
    }
    for (i, &(l, r, t)) in lrt.iter().enumerate() {
        events.push((t, Some((l, r)), i));
    }
    events.sort_unstable();

    let mut results = vec![0; q];

    let mut fenwick = FenwickTree::new(n, 0usize);
    for (_, lr, i) in events {
        if let Some((l, r)) = lr {
            let x = fenwick.sum(l..=r);
            if x <= k {
                results[i] = x as isize;
            } else {
                results[i] = -1;
            }
        } else {
            fenwick.add(i, 1);
        }
    }

    for result in results {
        println!("{result}");
    }
}
