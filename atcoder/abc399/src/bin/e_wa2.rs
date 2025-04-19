use std::collections::{HashMap, HashSet};

use ac_library::Dsu;
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

    let mut result = 0usize;
    let mut dsu = Dsu::new(26);
    for (&s, &t) in &map {
        let s = s as usize - 'a' as usize;
        let t = t as usize - 'a' as usize;
        if s != t {
            if dsu.same(s, t) {
                result += 2;
            } else {
                dsu.merge(s, t);
                result += 1;
            }
        }
    }

    println!("{result}");
}
