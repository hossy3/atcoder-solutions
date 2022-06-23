// -*- coding:utf-8-unix -*-

use proconio::input;

// 001 - Yokan Party（★4）
// https://atcoder.jp/contests/typical90/tasks/typical90_a

fn divnum(x: i32, a: &[i32]) -> i32 {
    let mut num = 0;
    let mut y = 0;
    for z in a {
        y += z;
        if y >= x {
            num += 1;
            y = 0;
        }
    }
    num
}

#[test]
fn divnumtest() {
    assert_eq!(divnum(8, &[8, 5, 13, 8]), 3);
}

fn main() {
    input! {
        n: usize,
        l: i32,
        k: i32,
        a: [i32; n],
    }
    let mut v = vec![0; n + 1];
    v[0] = a[0];
    for i in 1..n {
        v[i] = a[i] - a[i - 1];
    }
    v[n] = l - a[n - 1];

    let mut x0 = 1;
    let mut x1 = (l + k - 1) / k;
    while x1 - x0 > 1 {
        let x = (x0 + x1) / 2;
        if divnum(x, &v) > k {
            x0 = x;
        } else {
            x1 = x;
        }
    }
    println!("{}", x0);
}
