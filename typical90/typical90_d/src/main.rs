// -*- coding:utf-8-unix -*-

use proconio::input;

// 004 - Cross Sum（★2）
// https://atcoder.jp/contests/typical90/tasks/typical90_d

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h],
    }

    let mut sum_c = vec![0; w];
    let mut sum_r = vec![0; h];
    for r in 0..h {
        for c in 0..w {
            sum_c[c] += a[r][c];
            sum_r[r] += a[r][c];
        }
    }

    for r in 0..h {
        for c in 0..w {
            if c > 0 {
                print!(" ");
            }
            print!("{}", sum_c[c] + sum_r[r] - a[r][c]);
        }
        println!();
    }
}
