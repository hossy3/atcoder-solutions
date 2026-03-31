use ac_library::{Max, Min, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let segtree_max: Segtree<Max<_>> = h.clone().into();
    let segtree_min: Segtree<Min<_>> = h.clone().into();

    let mut result = 0usize;
    for l in 0..n {
        let r = segtree_max.max_right(l, |&x| x == h[l] || x == usize::MIN);
        if r < n {
            let cost = segtree_min.prod(l..=r);
            result += cost;
        }
    }
    println!("{result}");
}
