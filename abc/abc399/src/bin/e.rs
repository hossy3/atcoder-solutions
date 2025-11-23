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
        let s = s as usize - 'a' as usize;
        let t = t as usize - 'a' as usize;
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
    let mut candidates = vec![];

    for (&s, &t) in &map {
        if s != t {
            if dsu.same(s, t) {
                candidates.push(s);
            } else {
                dsu.merge(s, t);
            }
            result += 1;
        }
    }

    // ループしている かつ ループ外側から飛ぶものがない場合は数を増やす
    for &s in &candidates {
        let mut v = vec![s];
        loop {
            let Some(&s) = v.last() else { unreachable!() };
            let Some(&t) = map.get(&s) else {
                unreachable!()
            };
            if v[0] == t {
                break;
            }
            v.push(t);
        }
        let count = (0..26).filter(|&i| dsu.same(s, i)).count();
        if v.len() == count {
            result += 1;
        }
    }

    println!("{result}");
}
