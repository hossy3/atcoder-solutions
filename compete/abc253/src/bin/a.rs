// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    let yes = (a <= b && b <= c) || (a >= b && b >= c);
    println!("{}", if yes { "Yes" } else { "No" });
}
