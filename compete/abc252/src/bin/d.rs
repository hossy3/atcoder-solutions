// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut b = vec![0usize; 200_001];
    let mut c = vec![0usize; 200_001];

    for x in a {
        b[x] += 1;
        c[b[x]] += 1;
        if b[x] > 1 {
            c[b[x] - 1] -= 1;
        }
    }

    let mut count = n * (n - 1) * (n - 2) / 6;
    for i in 2..=n {
        count -= c[i] * (i * (i - 1) * (n - i)) / 2;
    }
    for i in 3..=n {
        count -= c[i] * (i * (i - 1) * (i - 2)) / 6;
    }

    println!("{}", count);
}
