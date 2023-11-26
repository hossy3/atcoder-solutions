use ac_library::{Min, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        ix: [(Usize1, usize); q],
    }

    let mut segtree: Segtree<Min<i32>> = vec![0; n + 2].into();
    for &x in &a {
        if x < n {
            segtree.set(x, segtree.get(x) + 1);
        }
    }
    for &(i, x) in &ix {
        if a[i] < n {
            segtree.set(a[i], segtree.get(a[i]) - 1);
        }
        a[i] = x;
        if x < n {
            segtree.set(x, segtree.get(x) + 1);
        }
        let result = segtree.max_right(0, |&y| y > 0);
        println!("{result}");
    }
}
