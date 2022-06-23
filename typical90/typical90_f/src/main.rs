// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

// 006 - Smallest Subsequence（★5）
// https://atcoder.jp/contests/typical90/tasks/typical90_f

fn update_indexes(indexes: &mut Vec<Option<usize>>, s: &[char], i: usize) {
    for c in b'a'..=b'z' {
        if let Some(x) = indexes[c as usize] {
            if x <= i {
                indexes[c as usize] =
                    if let Some(y) = s[(i + 1)..].iter().position(|&x| x == c as char) {
                        Some(i + 1 + y)
                    } else {
                        None
                    };
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut indexes = vec![Some(0usize); 128];
    for c in b'a'..=b'z' {
        indexes[c as usize] = s.iter().position(|&x| x == c as char);
    }
    for i in 0..k {
        for c in b'a'..=b'z' {
            if let Some(x) = indexes[c as usize] {
                if n - x >= k - i {
                    print!("{}", c as char);
                    update_indexes(&mut indexes, &s, x);
                    break;
                }
            }
        }
    }
    println!();
}
