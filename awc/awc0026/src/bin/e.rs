use ac_library::{Monoid, Segtree};
use proconio::input;

pub struct MinMax;
impl Monoid for MinMax {
    type S = (isize, isize);
    fn identity() -> Self::S {
        (isize::MAX, isize::MIN)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0.min(b.0), a.1.max(b.1))
    }
}

fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
    }

    let segtree: Segtree<MinMax> = a.iter().map(|&x| (x, x)).collect();
    let mut count = 0usize;
    for l in 0..n {
        count += segtree.max_right(l, |&(min, max)| max == isize::MIN || max - min <= k)
            - segtree.max_right(l, |&(min, max)| max == isize::MIN || max - min < k);
    }
    println!("{count}");
}
