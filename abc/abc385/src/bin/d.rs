use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        sx: i64,
        sy: i64,
        xy: [(i64, i64); n],
        dc: [(char, i64); m],
    }

    // 縦と横の map を作る
    let mut x2y = BTreeMap::new();
    for &(x, y) in &xy {
        x2y.entry(x).or_insert(BTreeSet::new()).insert(y);
    }

    let mut y2x = BTreeMap::new();
    for &(x, y) in &xy {
        y2x.entry(y).or_insert(BTreeSet::new()).insert(x);
    }

    let mut set = BTreeSet::new();
    let mut x = sx;
    let mut y = sy;

    for &(d, c) in &dc {
        match d {
            'U' => {
                let y0 = y + c;
                if let Some(ys) = x2y.get(&x) {
                    for &y in ys.range(y..=y0) {
                        set.insert((x, y));
                    }
                }
                y = y0;
            }
            'D' => {
                let y0 = y - c;
                if let Some(ys) = x2y.get(&x) {
                    for &y in ys.range(y0..=y) {
                        set.insert((x, y));
                    }
                }
                y = y0;
            }
            'L' => {
                let x0 = x - c;
                if let Some(xs) = y2x.get(&y) {
                    for &x in xs.range(x0..=x) {
                        set.insert((x, y));
                    }
                }
                x = x0;
            }
            'R' => {
                let x0 = x + c;
                if let Some(xs) = y2x.get(&y) {
                    for &x in xs.range(x..=x0) {
                        set.insert((x, y));
                    }
                }
                x = x0;
            }
            _ => unreachable!(),
        };
    }

    let result = set.len();
    println!("{x} {y} {result}");
}
