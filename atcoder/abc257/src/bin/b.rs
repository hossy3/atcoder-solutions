// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [usize; k],
        l: [usize; q],
    }

    for x in l {
        if x == k {
            if a[x - 1] < n {
                a[x - 1] += 1;
            }
        } else {
            if a[x - 1] + 1 < a[x] {
                a[x - 1] += 1;
            }
        }
    }

    for i in 0..(k - 1) {
        print!("{} ", a[i]);
    }
    println!("{}", a[k - 1]);
}
