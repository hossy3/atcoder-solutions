// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
    }
    let mut map = BTreeMap::<i32, i32>::new();
    for _ in 0..n {
        input! {
            q: i32,
        }
        if q == 1 {
            input! {
                x: i32,
            }
            if map.contains_key(&x) {
                map.insert(x, map[&x] + 1);
            } else {
                map.insert(x, 1);
            }
        } else if q == 2 {
            input! {
                x: i32,
                c: i32,
            }
            if map.contains_key(&x) {
                let y = map[&x];
                if y <= c {
                    map.remove(&x);
                } else {
                    map.insert(x, y - c);
                }
            }
        } else if q == 3 {
            let min = map.iter().next().unwrap().0;
            let max = map.iter().last().unwrap().0;
            println!("{}", max - min);
        }
    }
}
