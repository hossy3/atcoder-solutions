// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }
    let mut red = 1_usize;
    let mut blue = 0_usize;
    for _ in 1..n {
        blue += red * x;
        red += blue;
        blue = blue * y;
    }
    println!("{}", blue);
}
