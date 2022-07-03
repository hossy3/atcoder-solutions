// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    if n < 60 {
        println!("21:{:02}", n);
    } else {
        println!("22:{:02}", n - 60);
    };
}
