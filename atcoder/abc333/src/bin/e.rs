use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        tx: [(u8, usize); n],
    }

    let mut results = vec![];
    let mut map = HashMap::new();

    for &(t, x) in tx.iter().rev() {
        match t {
            1 => {
                if let Some(&i) = map.get(&x) {
                    if i == 1 {
                        map.remove(&x);
                    } else {
                        map.insert(x, i - 1);
                    }
                    results.push(1);
                } else {
                    results.push(0);
                }
            }
            2 => {
                *map.entry(x).or_insert(0usize) += 1;
            }
            _ => unreachable!(),
        }
    }

    if map.len() > 0 {
        println!("-1");
        return;
    }

    let mut max_num = 0usize;
    let mut cur_num = 0usize;

    let mut v = results.clone();
    for &(t, _) in tx.iter() {
        match t {
            1 => {
                let b = v.pop().unwrap();
                if b == 1 {
                    cur_num += 1;
                    max_num = max_num.max(cur_num);
                }
            }
            2 => {
                cur_num -= 1;
            }
            _ => unreachable!(),
        }
    }

    println!("{max_num}");
    println!("{}", results.iter().rev().join(" "));
}
