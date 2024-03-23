use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        m: usize,
        b: [i64; m],
        l: usize,
        c: [i64; l],
        q: usize,
        x: [i64; q],
    }

    let mut ab = HashSet::new();
    for &a in &a {
        for &b in &b {
            ab.insert(a + b);
        }
    }

    for x in x {
        let yes = c.iter().any(|&y| ab.contains(&(x - y)));
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
