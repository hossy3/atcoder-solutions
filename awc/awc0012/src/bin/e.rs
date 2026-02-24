use ac_library::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [isize; n],
    }

    let mut segtree: Segtree<Max<_>> = vec![0isize; n].into();
    segtree.set(0, a[0]);
    for i in 1..n {
        let max = segtree.prod(i.saturating_sub(k)..i);
        segtree.set(i, max + a[i]);
    }
    let result = segtree.get(n - 1);
    println!("{result}");
}
