// -*- coding:utf-8-unix -*-

use proconio::{input};
use std::f64::consts::PI;

// 009 - Three Point Angle（★6）
// https://atcoder.jp/contests/typical90/tasks/typical90_i

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    };

    let mut score = 0.0;

    for i in 0..n {
        let mut degs = vec![0f64; n - 1];
        for j in 0..n {
            if i == j {
                continue;
            }
            let rad = (xy[j].1 - xy[i].1).atan2(xy[j].0 - xy[i].0);
            let deg = rad / PI * 180.0;
            let k = if j > i { j - 1 } else { j };
            degs[k] = deg;
        }
        degs.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut k = 0;
        for j in 0..(n - 1) {
            while k < n - 1 {
                let s = degs[k] - degs[j];
                let s2 = if s > 180.0 { 360.0 - s } else { s };
                score = f64::max(score, s2);
                if s > 180.0 {
                    break;
                }
                k += 1;
            }
        }
    }

    println!("{}", score);
}
