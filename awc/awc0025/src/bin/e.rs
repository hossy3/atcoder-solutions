use ac_library::{Additive, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut d: [usize; n],
        t: [Usize1; q],
    }

    let mut segtree: Segtree<Additive<_>> = vec![1usize; n].into();
    for t in t {
        let i = segtree.max_right(0, |&x| x <= t);
        if i < n {
            d[i] -= 1;
            if d[i] == 0 {
                segtree.set(i, 0);
            }
        }
        let result = segtree.all_prod();
        println!("{result}");
    }
}
