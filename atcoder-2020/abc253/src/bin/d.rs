// -*- coding:utf-8-unix -*-

use proconio::input;

fn sum_k(n: i64, k: i64) -> i64 {
    let m = n / k;
    (1 + m) * m * k / 2
}

fn gcd(a: i64, b: i64) -> i64 {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    }
    let sum0 = (1 + n) * n / 2;
    let sum_a = sum_k(n, a);
    let sum_b = sum_k(n, b);
    let lcd = a * b / gcd(a, b);
    let sum_lcd = sum_k(n, lcd);
    let result = sum0 - sum_a - sum_b + sum_lcd;
    println!("{}", result);
}
