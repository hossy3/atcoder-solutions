// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(i64, i64); n],
    }
    let mut min_time = 1i64 << 60;
    let mut sum: i64 = 0;
    for i in 0..n {
        if i >= x {
            break;
        }
        sum += ab[i].0 + ab[i].1;
        let time = sum + ab[i].1 * (x - i - 1) as i64;
        min_time = min_time.min(time);
    }
    println!("{}", min_time);
}
