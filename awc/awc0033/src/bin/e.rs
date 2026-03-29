use ac_library::{Min, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut segtree: Segtree<Min<_>> = vec![0usize; n].into();
    segtree.set(0, a[0]);
    for i in 1..n {
        let x = segtree.prod(i.saturating_sub(k)..i) + a[i];
        segtree.set(i, x);
    }

    let result = segtree.get(n - 1);
    println!("{result}");
}
