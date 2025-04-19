use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
        t: Chars,
    }

    if s == t {
        println!("0");
        return;
    }

    let mut set_s = HashSet::new();
    let mut set_t = HashSet::new();
    for &c in &s {
        set_s.insert(c);
    }
    for &c in &t {
        set_t.insert(c);
    }
    if set_s.len() == 26 && set_t.len() == 26 {
        println!("-1");
        return;
    }

    // 何から何に変えるか
    let mut map = HashMap::new();
    for (&s, &t) in std::iter::zip(s.iter(), t.iter()) {
        if let Some(&t0) = map.get(&s) {
            if t0 != t {
                println!("-1");
                return;
            }
        } else {
            map.insert(s, t);
        }
    }

    let mut result = map.iter().filter(|&(&k, &v)| k != v).count();
    if set_s == set_t {
        result += 1; // 別の文字を経由する必要あり
    }

    println!("{result}");
}
