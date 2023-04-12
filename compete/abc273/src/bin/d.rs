use std::collections::HashMap;

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        h: i64,
        w: i64,
        rs: i64,
        cs: i64,
        n: usize,
        rc: [(i64, i64); n],
        q: usize,
        dl: [(char, i64); q],
    }

    let empty_row = vec![0, w + 1];
    let empty_col = vec![0, h + 1];

    let mut rows = HashMap::new();
    let mut cols = HashMap::new();
    for &(r, c) in &rc {
        rows.entry(r).or_insert(empty_row.clone()).push(c);
        cols.entry(c).or_insert(empty_col.clone()).push(r);
    }
    for (_, v) in rows.iter_mut() {
        v.sort();
    }
    for (_, v) in cols.iter_mut() {
        v.sort();
    }

    let mut r = rs;
    let mut c = cs;
    for &(d, l) in &dl {
        match d {
            'U' => {
                let v = cols.get(&c).unwrap_or(&empty_col);
                r = (r - l).max(v[v.lower_bound(&r) - 1] + 1);
            }
            'D' => {
                let v = cols.get(&c).unwrap_or(&empty_col);
                r = (r + l).min(v[v.lower_bound(&r)] - 1);
            }
            'L' => {
                let v = rows.get(&r).unwrap_or(&empty_row);
                c = (c - l).max(v[v.lower_bound(&c) - 1] + 1);
            }
            'R' => {
                let v = rows.get(&r).unwrap_or(&empty_row);
                c = (c + l).min(v[v.lower_bound(&c)] - 1);
            }
            _ => {}
        }
        println!("{} {}", r, c);
    }
}
