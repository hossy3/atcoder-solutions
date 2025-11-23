use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: Chars,
        t: Chars,
    }

    let mut candidates = BTreeSet::new();
    for i in 0..(n - m + 1) {
        candidates.insert(i);
    }
    while let Some(i) = candidates.pop_first() {
        if s[i..(i + m)].iter().any(|&c| c != '#')
            && (0..m).all(|j| s[i + j] == '#' || s[i + j] == t[j])
        {
            s[i..(i + m)].fill('#');
            for j in (i.saturating_sub(m - 1))..i {
                candidates.insert(j);
            }
            for j in (i + m)..((i + m + m - 1).min(n - m + 1)) {
                candidates.insert(j);
            }
        }
    }
    let yes = s.iter().all(|&c| c == '#');
    println!("{}", if yes { "Yes" } else { "No" });
}
