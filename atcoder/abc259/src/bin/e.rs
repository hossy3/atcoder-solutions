// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    input! {
        n: usize,
    }

    let mut v = Vec::<Vec<(i64, i64)>>::new();
    for _ in 0..n {
        input! {
            m: usize,
            mut pes: [(i64, i64); m],
        }
        let mut v0 = Vec::<(i64, i64)>::new();
        for j in 0..m {
            v0.push(pes[j]);
        }
        v.push(v0);
    }

    let mut map = BTreeMap::<i64, (i64, bool)>::new();
    for i in 0..n {
        for (p, e) in &v[i] {
            if !map.contains_key(&p) || map[&p].0 < *e {
                map.insert(*p, (*e, true)); // once
            } else if map[&p].0 == *e && map[&p].1 == true {
                map.insert(*p, (*e, false)); // twice or more
            }
        }
    }

    let mut set = BTreeSet::<Vec<i64>>::new();
    for i in 0..n {
        let mut v0 = Vec::<i64>::new();
        for (p, e) in &v[i] {
            if let Some((x, true)) = map.get(&p) {
                if x == e {
                    v0.push(*p);
                }
            }
        }
        set.insert(v0);
    }
    println!("{}", set.len());
}
