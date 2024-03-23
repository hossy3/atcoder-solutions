use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        (h, w, m): (usize, usize, usize),
        tax: [(u8, usize, usize); m],
    }
    let mut map = BTreeMap::new(); // 確定した個数 (0は最後に処理)
    let mut rows = BTreeSet::new(); // 確定済みの行
    let mut cols = BTreeSet::new(); // 確定済みの列

    for &(t, a, x) in tax.iter().rev() {
        match t {
            1 => {
                if rows.insert(a) {
                    let y = w - cols.len();
                    if y > 0 {
                        *map.entry(x).or_insert(0) += y;
                    }
                }
            }
            2 => {
                if cols.insert(a) {
                    let y = h - rows.len();
                    if y > 0 {
                        *map.entry(x).or_insert(0) += y;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    let y = h * w - map.iter().map(|(_, &v)| v).sum::<usize>();
    if y > 0 {
        *map.entry(0).or_insert(0) += y;
    }

    println!("{}", map.len());
    for (k, v) in map {
        println!("{k} {v}");
    }
}
