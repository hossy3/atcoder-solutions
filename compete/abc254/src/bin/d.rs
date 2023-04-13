// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut count = 0;
    for i in 1..=n {
        let mut x = i;
        let mut j = 2;
        while j * j <= x {
            if x % (j * j) == 0 {
                x /= j * j;
            } else if j == 2 {
                j += 1;
            } else {
                j += 2;
            }
        }
        count += ((n / x) as f64).sqrt() as i32;
    }
    println!("{}", count);
}
