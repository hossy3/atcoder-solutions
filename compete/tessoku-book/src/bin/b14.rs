use std::collections::HashSet;

use proconio::input;

fn f(k: usize, s0: &HashSet<usize>, s1: &HashSet<usize>) -> bool {
    for &x in s0 {
        if k >= x && s1.contains(&(k - x)) {
            return true;
        }
    }
    false
}

fn update(a: &[usize], s: &mut HashSet<usize>) {
    if a.len() == 0 {
        return;
    }
    let h0 = s.clone();
    for &h0 in &h0 {
        s.insert(h0 + a[0]);
    }
    update(&a[1..], s);
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut s0 = HashSet::new();
    s0.insert(0);
    update(&a[..(n / 2)], &mut s0);

    let mut s1 = HashSet::new();
    s1.insert(0);
    update(&a[(n / 2)..], &mut s1);

    let yes = f(k, &s0, &s1);
    println!("{}", if yes { "Yes" } else { "No" });
}
