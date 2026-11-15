use ac_library::{Max, Min, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [isize; n],
    }

    let mut segtree_min: Segtree<Min<_>> = Segtree::new(n);
    let mut segtree_max: Segtree<Max<_>> = Segtree::new(n);

    for (i, &x) in a.iter().enumerate() {
        if x == -1 {
            segtree_min.set(i, isize::MAX);
            segtree_max.set(i, isize::MIN);
        } else {
            segtree_min.set(i, x);
            segtree_max.set(i, x);
        }
    }

    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    i: Usize1,
                    x: isize,
                }
                if x == -1 {
                    segtree_min.set(i, isize::MAX);
                    segtree_max.set(i, isize::MIN);
                } else {
                    segtree_min.set(i, x);
                    segtree_max.set(i, x);
                }
            }
            2 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                }
                let min = segtree_min.prod(l..=r);
                let max = segtree_max.prod(l..=r);
                let yes = min >= max;
                println!("{}", if yes { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
}
