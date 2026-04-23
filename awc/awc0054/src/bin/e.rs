use ac_library::FenwickTree;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Usize1; m],
    }

    let mut fenwick = FenwickTree::new(n, 0);
    for i in 0..n {
        fenwick.add(i, 1);
    }
    for &i in &s {
        println!("{}", fenwick.sum(..=i));
        fenwick.add(i, -1);
    }
}
