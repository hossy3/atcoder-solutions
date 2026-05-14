use std::cmp::Reverse;

use ac_library::FenwickTree;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        adv: [(usize, Usize1, usize); n],
        lrt: [(Usize1, Usize1, Usize1); q],
    }

    let mut iadv = adv.into_iter().enumerate().collect::<Vec<_>>();
    iadv.sort_unstable_by_key(|&(_, (_, d, _))| Reverse(d)); // 降順

    let mut ilrt = lrt.into_iter().enumerate().collect::<Vec<_>>();
    ilrt.sort_unstable_by_key(|&(_, (_, _, t))| t); // 昇順

    let mut fenwick = FenwickTree::new(n, 0usize);
    let mut results = vec![0usize; q];
    for &(i, (l, r, t)) in &ilrt {
        while let Some(&(i, (a, d, v))) = iadv.last() {
            if d > t {
                break;
            }
            iadv.pop();
            fenwick.add(i, a * v);
        }
        results[i] = fenwick.sum(l..=r);
    }

    for result in results {
        println!("{result}");
    }
}
