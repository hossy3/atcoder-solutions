use ac_library::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        w: [usize; n],
        c: [usize; m],
    }

    let mut segtree: Segtree<Max<_>> = c.clone().into();

    let mut result = 0usize;
    for &w in &w {
        if segtree.all_prod() >= w {
            result += 1;
            let i = segtree.max_right(0, |&x| x < w);
            segtree.set(i, 0);
        }
    }
    println!("{result}");
}
