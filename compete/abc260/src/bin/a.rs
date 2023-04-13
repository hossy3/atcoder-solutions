// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

fn main() {
    input! {
        chars: Chars,
    }
    if chars[0] == chars[1] && chars[0] == chars[2] {
        println!("{}", -1);
    } else if chars[2] == chars[0] {
        println!("{}", chars[1]);
    } else if chars[0] == chars[1] {
        println!("{}", chars[2]);
    } else {
        println!("{}", chars[0]);
    }
}
