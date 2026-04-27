use std::collections::HashSet;

use proconio::input;

/// 半分全列挙: a から複数選んで和を s にする組み合わせが存在するか
fn meet_in_the_middle(k: usize, ab: &[(usize, usize)]) -> bool {
    let n = ab.len();

    let mut s0 = HashSet::new();
    s0.insert(0usize);
    for &(a, b) in &ab[..(n / 2)] {
        let mut s = s0.clone();
        for s0 in s0 {
            s.insert(s0 + a);
            s.insert(s0 + b);
        }
        s0 = s;
    }

    let mut s1 = HashSet::new();
    s1.insert(0usize);
    for &(a, b) in &ab[(n / 2)..] {
        let mut s = s1.clone();
        for s1 in s1 {
            s.insert(s1 + a);
            s.insert(s1 + b);
        }
        s1 = s;
    }

    for s0 in s0 {
        if s0 <= k && s1.contains(&(k - s0)) {
            return true;
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }

    let yes = meet_in_the_middle(k, &ab);
    println!("{}", if yes { "Yes" } else { "No" });
}
