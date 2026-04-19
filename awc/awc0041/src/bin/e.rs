use ac_library::{Max, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        h: [usize; n],
        x: [Usize1; q],
    }

    let segtree: Segtree<Max<_>> = h.clone().into();
    let mut v = vec![0usize; n + 1];
    for x in 0..n {
        let l = segtree.min_left(x, |&h0| h0 <= h[x]);
        let r = segtree.max_right(x, |&h0| h0 <= h[x]);
        let x0 = r - l;
        // eprintln!("{x}, {l}, {r}, {x0}");
        v[x0] += 1;
    }

    let mut cum = vec![0usize; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + v[i + 1];
    }

    for x in x {
        let result = cum[n] - cum[x];
        println!("{result}");
    }
}
