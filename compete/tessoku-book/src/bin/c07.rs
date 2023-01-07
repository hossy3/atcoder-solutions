use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        c: [usize; n],
        q: usize,
        x: [usize; q],
    }

    let mut v = c.clone();
    v.sort();
    for i in 1..v.len() {
        v[i] += v[i - 1];
    }

    for x in &x {
        let i = v.upper_bound(x);
        println!("{}", i);
    }
}
