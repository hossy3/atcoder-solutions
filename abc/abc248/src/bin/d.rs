use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        q: usize,
        lrx: [(usize, usize, Usize1); q],
    }
    let mut v = vec![vec![]; n];
    for i in 0..n {
        v[a[i]].push(i + 1);
    }
    for &(l, r, x) in &lrx {
        let result = v[x].upper_bound(&r) - v[x].lower_bound(&l);
        println!("{}", result);
    }
}
