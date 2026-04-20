use ac_library::FenwickTree;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        g: [Usize1; m],
    }

    let mut fenwick = FenwickTree::new(n, 0isize);
    for &g in &g {
        fenwick.add(g, 1);
    }

    let mut result = 0;
    for &g in &g {
        fenwick.add(g, -1);
        result += fenwick.sum(0..g);
    }
    println!("{result}");
}
