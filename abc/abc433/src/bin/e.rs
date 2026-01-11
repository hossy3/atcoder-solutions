use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;
use proconio::input;

fn f(x: &[usize], y: &[usize]) -> Option<Vec<Vec<usize>>> {
    let n = x.len();
    let m = y.len();
    let mut result = vec![vec![usize::MAX; m]; n];

    let mut x0 = BTreeMap::new();
    for (i, &x) in x.iter().enumerate() {
        if x0.contains_key(&x) {
            return None; // 同じ数字が複数あるのはおかしい
        }
        x0.insert(x, i);
    }

    let mut y0 = BTreeMap::new();
    for (i, &y) in y.iter().enumerate() {
        if y0.contains_key(&y) {
            return None; // 同じ数字が複数あるのはおかしい
        }
        y0.insert(y, i);
    }

    let mut set = BTreeSet::new();
    for (i, &x) in x.iter().enumerate() {
        for (j, &y) in y.iter().enumerate() {
            set.insert((x.min(y), (i, j)));
        }
    }

    // 縦横同じ数字が出ているところは確定
    for (i, &x) in x.iter().enumerate() {
        if let Some(&j) = y0.get(&x) {
            result[i][j] = x;
            set.remove(&(x, (i, j)));
        }
    }

    for x in (1..=(n * m)).rev() {
        if x0.contains_key(&x) && y0.contains_key(&x) {
            continue; // 縦横同じ数字が出ているところは確定
        }

        // 縦横片方だけ数字がある場合
        if x0.contains_key(&x) || y0.contains_key(&x) {
            let Some(&(_, (i, j))) = set.range((x, (0, 0))..(x + 1, (0, 0))).next() else {
                return None;
            };
            result[i][j] = x;
            set.remove(&(x, (i, j)));
            continue;
        }

        // 残ったものを適当に埋める
        let Some((y, (i, j))) = set.pop_last() else {
            unreachable!()
        };
        if x > y {
            return None; // 大きなマスが空いていない
        }
        result[i][j] = x;
    }

    Some(result)
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            x: [usize; n],
            y: [usize; m],
        }
        if let Some(result) = f(&x, &y) {
            println!("Yes");
            for result in result {
                println!("{}", result.iter().join(" "));
            }
        } else {
            println!("No");
        }
    }
}
