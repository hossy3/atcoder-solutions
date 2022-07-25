// -*- coding:utf-8-unix -*-

use std::collections::HashMap;

use proconio::input;

fn prepare_ary(ws: &[i64], x: i64) -> Vec<(usize, usize)> {
    let n = ws.len();
    let sum_weight: i64 = ws.iter().sum();
    let mut ary = vec![(0usize, 0usize); n]; // (next start, count)
    let mut weight = (x / sum_weight) * sum_weight;
    let mut count = (x / sum_weight) as usize * n;
    let mut j = 0;
    for i in 0..n {
        if i > 0 {
            weight -= ws[i - 1];
            count -= 1;
        }
        while weight < x {
            weight += ws[j];
            j = (j + 1) % n;
            count += 1;
        }
        ary[i] = (j, count);
    }
    ary
}

fn main() {
    input! {
        n: usize,
        q: usize,
        x: i64,
        ws: [i64; n],
        ks: [usize; q],
    }

    let ary = prepare_ary(&ws, x);
    let mut v = vec![0];
    let mut l = 0usize; // loop start index
    let mut m = HashMap::<usize, usize>::new(); // start, index
    m.insert(0, 0);
    let mut i = 0;
    for j in 1..=(n + 1) {
        i = ary[i].0;
        if let Some(&j0) = m.get(&i) {
            l = j0;
            break;
        }
        m.insert(i, j);
        v.push(i);
    }

    for k in ks {
        let i = if k - 1 < l {
            k - 1
        } else {
            l + (k - 1 - l) % (v.len() - l)
        };
        println!("{}", ary[v[i]].1);
    }
}
