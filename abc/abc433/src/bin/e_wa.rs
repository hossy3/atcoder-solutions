use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

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
    let x1 = x0.iter().collect_vec();

    let mut y0 = BTreeMap::new();
    for (i, &y) in y.iter().enumerate() {
        if y0.contains_key(&y) {
            return None; // 同じ数字が複数あるのはおかしい
        }
        y0.insert(y, i);
    }
    let y1 = y0.iter().collect_vec();

    let mut rest = BTreeSet::new();
    for i in 1..=(n * m) {
        rest.insert(i);
    }

    // 縦横同じ数字が出ているところは確定
    for (i, &x) in x.iter().enumerate() {
        if let Some(&j) = y0.get(&x) {
            result[i][j] = x;
            rest.remove(&x);
        }
    }

    // 縦横片方だけ数字があるものは、残り物のうち一番大きそうなところに入れておく
    'outer: for (i, &x) in x.iter().enumerate() {
        if !rest.contains(&x) {
            continue;
        }
        let j_min = y1.partition_point(|&(&x1, _)| x1 < x);
        for j in j_min..m {
            let &j0 = y1[j].1;
            if result[i][j0] == usize::MAX {
                result[i][j0] = x;
                rest.remove(&x);
                continue 'outer;
            }
        }
        println!("{:?} {:?}", &result, &rest);
        return None;
    }

    'outer: for (i, &y) in y.iter().enumerate() {
        if !rest.contains(&y) {
            continue;
        }
        let j_min = x1.partition_point(|&(&y1, _)| y1 < y);
        for j in j_min..n {
            let &j0 = x1[j].1;
            if result[j0][i] == usize::MAX {
                result[j0][i] = y;
                rest.remove(&y);
                continue 'outer;
            }
        }
        println!("{:?} {:?}", &result, &rest);
        return None;
    }

    // 残ったものを適当に埋める
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        for j in 0..m {
            if result[i][j] == usize::MAX {
                let x = x[i].min(y[j]);
                heap.push((x, i, j));
            }
        }
    }

    while let Some((x, i, j)) = heap.pop() {
        let Some(y) = rest.pop_last() else {
            unreachable!()
        };
        if x < y {
            return None;
        }
        result[i][j] = y;
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
