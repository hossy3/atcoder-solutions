// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        mut a: [[i32; 2]; 2],
    }
    let x = a[r - 1][c - 1];
    println!("{}", x);
}
