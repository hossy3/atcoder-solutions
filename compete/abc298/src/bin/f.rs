use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        rcx: [(usize, usize, usize); n],
    }

    let mut items = HashSet::new();
    let mut rows = HashMap::new(); // sum of rows
    let mut cols = HashMap::new(); // sum of columns
    for &(r, c, x) in &rcx {
        items.insert((r, c));
        *rows.entry(r).or_insert(0) += x;
        *cols.entry(c).or_insert(0) += x;
    }

    let mut result = 0;
    for &(r, c, x) in &rcx {
        result = result.max(rows.get(&r).unwrap() + cols.get(&c).unwrap() - x);
    }

    let mut cols0 = vec![];
    for col in &cols {
        cols0.push(col);
    }
    cols0.sort_by(|a, b| b.1.cmp(a.1));

    for row in &rows {
        for &col in &cols0 {
            if !items.contains(&(*row.0, *col.0)) {
                result = result.max(row.1 + col.1);
                break;
            } else if row.1 + col.1 < result {
                break;
            }
        }
    }

    println!("{}", result);
}
