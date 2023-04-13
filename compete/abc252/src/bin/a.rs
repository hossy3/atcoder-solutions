// -*- coding:utf-8-unix -*-

use proconio::input;
use std::char;

fn main() {
    input! {
        n: u32,
    }
    println!("{}", char::from_u32(n).unwrap());
}
