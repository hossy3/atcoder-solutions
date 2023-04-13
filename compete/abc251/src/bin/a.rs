// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        txt: String,
    }
    let n = 6 / txt.len();
    for _ in 0..n {
        print!("{}", txt);
    }
    println!();
}
