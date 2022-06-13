// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        x: [i64; q],
    }
    a.sort();
    
    let mut aa = vec![0i64; n + 1];
    aa[0] = 0;
    for i in 0..n {
        aa[i + 1] = aa[i] + a[i];
    }

    for y in x {
        let j = match a.binary_search(&y) {
            Ok(j) => j,
            Err(j) => j,
        };
        let sum = (y * (j as i64) - aa[j]) + ((aa[n] - aa[j]) - y * ((n - j) as i64));
        println!("{}", sum);
    }
}
