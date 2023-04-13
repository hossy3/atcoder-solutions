// -*- coding:utf-8-unix -*-

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); n],
    }

    let mut map = HashMap::<usize, Vec<usize>>::new();
    for i in 0..n {
        let (a, b) = ab[i];
        map.entry(a - 1).or_insert(Vec::new()).push(i);
        map.entry(b - 1).or_insert(Vec::new()).push(i);
    }

    let mut diffs = vec![0i64; m + 1];
    let mut map0 = HashMap::<usize, usize>::new();
    let mut j = 0;
    if let Some(v) = map.get(&j) {
        for k in v {
            map0.insert(*k, 1);
        }
    }
    for i in 0..m {
        if i > 0 {
            if let Some(v) = map.get(&(i - 1)) {
                for k in v {
                    if let Some(x) = map0.get_mut(&k) {
                        if *x == 1 {
                            map0.remove(&k);
                        } else {
                            *x -= 1;
                        }
                    }
                }
            }
        }
        while j < m {
            if map0.len() == n {
                diffs[j - i] += 1;
                diffs[m - i] -= 1;
                break;
            }
            j += 1;
            if let Some(v) = map.get(&j) {
                for k in v {
                    if let Some(x) = map0.get_mut(&k) {
                        *x += 1;
                    } else {
                        map0.insert(*k, 1);
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for i in 0..(m - 1) {
        sum += diffs[i];
        print!("{} ", sum);
    }
    sum += diffs[m - 1];
    println!("{}", sum);
}
