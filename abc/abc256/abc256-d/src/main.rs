// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        lr: [(i32, i32); n],
    }
    let mut map = BTreeMap::<i32, i32>::new();
    for (l, r) in lr {
        if let Some(&r0) = map.get(&l) {
            if r > r0 {
                map.insert(l, r);
            }
        } else {
            map.insert(l, r);
        }
    }

    let first_key_value = map.iter().next().unwrap();
    let mut l0 = *first_key_value.0;
    let mut r0 = *first_key_value.1;
    for (l, r) in map {
        if r0 >= l {
            r0 = r0.max(r);
        } else {
            println!("{} {}", l0, r0);
            l0 = l;
            r0 = r;
        }
    }
    println!("{} {}", l0, r0);
}
