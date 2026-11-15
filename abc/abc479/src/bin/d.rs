use ac_library::{Min, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        l: [usize; n - 1],
        r: [usize; n - 1],
        st: [(Usize1, Usize1); q],
    }

    let seg_l: Segtree<Min<_>> = l.iter().cloned().collect();
    let seg_r: Segtree<Min<_>> = r.iter().cloned().collect();

    for &(s, t) in &st {
        let result = if s == 0 {
            r[s]
        } else if s == n - 1 {
            l[s - 1]
        } else if s < t {
            r[s].min(l[s - 1] + seg_r.prod(..s))
        } else {
            l[s - 1].min(r[s] + seg_l.prod(s..))
        };
        println!("{result}");
    }
}
