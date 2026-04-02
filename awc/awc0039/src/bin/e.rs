use ac_library::FenwickTree;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        k: usize,
        sd: [(Usize1, usize); m],
        lrt: [(Usize1, Usize1, usize); q],
    }

    let lrti = lrt
        .iter()
        .enumerate()
        .map(|(i, &(l, r, t))| (l, r, t, i))
        .sorted_unstable_by_key(|&(_, _, t, _)| t)
        .rev()
        .collect::<Vec<_>>();
    let mut sd = sd
        .iter()
        .cloned()
        .sorted_by_key(|&(_, d)| d)
        .collect::<Vec<_>>();

    let mut fenwick = FenwickTree::new(n, 0usize);
    let mut results = vec![0usize; q];
    for (l, r, t, i) in lrti {
        while let Some(&(s, d)) = sd.last() {
            if d < t {
                break;
            }
            fenwick.add(s, 1);
            // eprintln!("add: {}", s);
            sd.pop();
        }
        // eprintln!("{}, {:?}, {}, {}, {}", i, &sd, l, r, fenwick.sum(l..=r));
        results[i] = fenwick.sum(l..=r);
    }
    for x in results {
        println!("{}", x.saturating_sub(k));
    }
}
