use ac_library::{Min, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let segtree: Segtree<Min<_>> = a.into();
    let mut result = 0;
    for l in 0..n {
        for r in l..n {
            result = result.max((r - l + 1) * segtree.prod(l..=r));
        }
    }
    println!("{result}");
}
