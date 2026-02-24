use ac_library::FenwickTree;
use proconio::{input, marker::Usize1};

// 転倒数
fn inversion(a: &[usize]) -> usize {
    let n = a.len();
    let mut tree = FenwickTree::new(n, 0usize);
    let mut count = 0usize;
    for (_, &xr) in a.iter().enumerate() {
        tree.add(xr, 1);
        count += tree.sum((xr + 1)..);
    }

    count
}

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let result = inversion(&a);
    println!("{result}");
}
