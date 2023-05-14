use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [(usize, usize, usize); n],
    }
    let mut v = Vec::with_capacity(n);
    for &(p0, p1, p2) in &p {
        v.push(p0 + p1 + p2);
    }
    v.sort();
    for &(p0, p1, p2) in &p {
        let x = p0 + p1 + p2;
        let k0 = n - v.upper_bound(&(x + 300)) + 1;
        let yes = k0 <= k;
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
