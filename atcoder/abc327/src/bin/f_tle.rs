use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        w: usize,
        tx: [(usize, usize); n],
    }

    let t_max = *tx.iter().map(|(t, _)| t).max().unwrap_or(&1);
    let x_max = *tx.iter().map(|(_, x)| x).max().unwrap_or(&1);
    let mut queue = BTreeMap::new();
    for &(t, x) in &tx {
        let (t0, t1) = (t - 1, t + d - 1);
        let (x0, x1) = (x - 1, x + w - 1);
        *queue
            .entry(t0)
            .or_insert(BTreeMap::new())
            .entry(x0)
            .or_insert(0) += 1;
        if t1 < t_max {
            *queue
                .entry(t1)
                .or_insert(BTreeMap::new())
                .entry(x0)
                .or_insert(0) -= 1;
        }
        if x1 < x_max {
            *queue
                .entry(t0)
                .or_insert(BTreeMap::new())
                .entry(x1)
                .or_insert(0) -= 1;
        }
        if t1 < t_max && x1 < x_max {
            *queue
                .entry(t1)
                .or_insert(BTreeMap::new())
                .entry(x1)
                .or_insert(0) += 1;
        }
    }

    let mut result = 0;
    let mut v = vec![0i32; x_max];
    for (_, xs) in queue {
        let mut v0 = vec![0i32; x_max];
        for (x, y) in xs {
            v0[x] = y;
        }
        for i in 0..(x_max - 1) {
            v0[i + 1] += v0[i];
        }
        for i in 0..x_max {
            v[i] += v0[i];
            result = result.max(v[i]);
        }
    }
    println!("{result}");
}
