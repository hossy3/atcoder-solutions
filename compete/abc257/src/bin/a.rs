// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    }
    let c = (b'A' + ((x - 1) / n) as u8) as char;
    println!("{}", c);
}
