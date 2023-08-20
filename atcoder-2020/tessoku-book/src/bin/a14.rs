use std::collections::HashSet;

use proconio::input;

// Split and List (半分全列挙)

fn f(k: i64, s0: &HashSet<i64>, s1: &HashSet<i64>) -> bool {
    for &x in s0 {
        if s1.contains(&(k - x)) {
            return true;
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n],
        d: [i64; n],
    }

    let mut s0 = HashSet::new();
    for &a in &a {
        for &b in &b {
            s0.insert(a + b);
        }
    }

    let mut s1 = HashSet::new();
    for &c in &c {
        for &d in &d {
            s1.insert(c + d);
        }
    }

    let yes = f(k, &s0, &s1);
    println!("{}", if yes { "Yes" } else { "No" });
}
