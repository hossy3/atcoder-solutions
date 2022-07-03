// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32
    }
    for n0 in 0..n {
        for _ in 0..a {
            for n1 in 0..n {
                for _ in 0..b {
                    print!("{}", if (n0 + n1) % 2 == 0 { "." } else { "#" });
                }
            }
            println!();
        }
    }
}
