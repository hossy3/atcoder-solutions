// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u32,
    }
    println!("{:02}", n % 100);
}
