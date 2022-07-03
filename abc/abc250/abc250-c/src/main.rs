// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        xs: [usize; q],
    }
    let mut v = vec![0; n];
    let mut w = vec![0; n + 1];
    for i in 0..n {
        v[i] = i + 1;
        w[i + 1] = i;
    }
    for i in 0..q {
        let i0 = w[xs[i]];
        let i1 = if i0 + 1 == n { i0 - 1 } else { i0 + 1 };
        let i2 = v[i1];
        v.swap(i0, i1);
        w.swap(xs[i], i2);
    }
    for i in 0..(n - 1) {
        print!("{} ", v[i]);
    }
    println!("{}", v[n - 1]);
}
