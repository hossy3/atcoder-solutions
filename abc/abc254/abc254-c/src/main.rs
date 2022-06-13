// -*- coding:utf-8-unix -*-

use proconio::input;

fn test(n: usize, k: usize, a: &[i32], s: &[i32]) -> bool {
    for i in 0..k {
        let mut aa = Vec::new();
        let mut ss = Vec::new();
        let mut j = i;
        while j < n {
            aa.push(a[j]);
            ss.push(s[j]);
            j += k;
        }
        aa.sort();

        for j in 0..(aa.len()) {
            if aa[j] != ss[j] {
                return false;
            }
        }
    }
    true
} 

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i32; n],
    }
    let mut s = a.clone();
    s.sort();

    let result = test(n, k, &a, &s);
    println!("{}", if result { "Yes" } else { "No" });
}
