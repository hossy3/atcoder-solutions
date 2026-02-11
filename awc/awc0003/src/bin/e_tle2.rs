use std::collections::HashSet;

use proconio::input;

fn f(w: &[usize], c: &mut [usize]) -> bool {
    if w.len() == 0 {
        return true;
    }

    let mut set = HashSet::new();
    let x = w[0];
    for i in 0..(c.len()) {
        if c[i] < x || !set.insert(c[i]) {
            continue;
        }
        c[i] -= x;
        if f(&w[1..], c) {
            return true;
        }
        c[i] += x;
    }

    false
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut w: [usize; n],
        mut c: [usize; m],
    }

    w.sort_unstable();
    w.reverse();

    let yes = f(&w, &mut c);
    println!("{}", if yes { "Yes" } else { "No" });
}
