// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        _: i32,
    }
    println!("297");
    print!("1");
    for i in 2..=99 {
        print!(" {}", i);
    }
    for i in 1..=99 {
        print!(" {}", i * 100);
    }
    for i in 1..=99 {
        print!(" {}", i * 10000);
    }
}
