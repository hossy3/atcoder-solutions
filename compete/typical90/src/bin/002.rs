// -*- coding:utf-8-unix -*-

use proconio::input;

// 002 - Encyclopedia of Parentheses（★3）
// https://atcoder.jp/contests/typical90/tasks/typical90_b

fn f(txt: &str, depth: i32, rest: i32) {
    if rest == 0 {
        println!("{}", &txt);
    } else {
        if depth < rest {
            f(&(txt.to_string() + "("), depth + 1, rest - 1);
        }
        if depth > 0 {
            f(&(txt.to_string() + ")"), depth - 1, rest - 1);
        }
    }
}

fn main() {
    input! {
        n: i32,
    }
    if n > 0 && n % 2 == 0 {
        f("", 0, n);
    }
}
