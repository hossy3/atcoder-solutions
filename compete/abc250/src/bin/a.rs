// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h: i32,
        w: i32,
        r: i32,
        c: i32
    }
    let x = (if 1 < r { 1 } else { 0 })
        + (if 1 < c { 1 } else { 0 })
        + (if r < h { 1 } else { 0 })
        + (if c < w { 1 } else { 0 });
    println!("{}", x);
}
