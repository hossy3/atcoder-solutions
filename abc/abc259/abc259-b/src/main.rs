// -*- coding:utf-8-unix -*-

use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64,
    }
    let rad = d * PI / 180.0;
    let a0 = a * rad.cos() - b * rad.sin();
    let b0 = a * rad.sin() + b * rad.cos();
    println!("{} {}", a0, b0);
}
