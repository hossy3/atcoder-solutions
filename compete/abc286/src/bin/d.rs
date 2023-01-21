use std::collections::HashSet;

use proconio::input;

fn f(k: usize, s0: &HashSet<usize>, s1: &HashSet<usize>) -> bool {
    for &x in s0 {
        if s1.contains(&(k - x)) {
            return true;
        }
    }
    false
}

fn update(ab: &[(usize, usize)], s: &mut HashSet<usize>) {
    if ab.len() == 0 {
        return;
    }
    let h0 = s.clone();
    let (a, b) = ab[0];
    for &h0 in &h0 {
        for i in 1..=b {
            s.insert(h0 + a * i);
        }
    }
    update(&ab[1..], s);
}

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    }

    let mut s0 = HashSet::new();
    s0.insert(0);
    update(&ab[..(n / 2)], &mut s0);

    let mut s1 = HashSet::new();
    s1.insert(0);
    update(&ab[(n / 2)..], &mut s1);

    let yes = f(x, &s0, &s1);
    println!("{}", if yes { "Yes" } else { "No" });
}
