use ac_library::FenwickTree;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        l: [Usize1; n],
    }

    let a = l
        .iter()
        .enumerate()
        .sorted_by_key(|&(_, &l)| l)
        .collect::<Vec<_>>();

    let mut fenwick = FenwickTree::new(n, 0isize);
    for i in 0..n {
        fenwick.add(i, 1);
    }

    let mut results = vec![0isize; n];
    for (i, &l) in a {
        results[l] = fenwick.sum(0..=i);
        fenwick.add(i, -1);
    }
    for result in results {
        println!("{result}");
    }
}
