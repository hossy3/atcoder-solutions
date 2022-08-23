use std::collections::{HashMap, HashSet};

use proconio::input;

const MOD: usize = 998244353;

fn update(
    xy: &(i64, i64),
    n: usize,
    ab: &(i64, i64),
    s: &HashSet<(i64, i64)>,
    vnext: &mut HashMap<(i64, i64), usize>,
) {
    let x0 = xy.0 + ab.0;
    let y0 = xy.1 + ab.1;
    if !s.contains(&(x0, y0)) {
        if let Some(x) = vnext.get_mut(&(x0, y0)) {
            *x = (*x + n) % MOD;
        } else {
            vnext.insert((x0, y0), n);
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
        f: i64,
        xy: [(i64, i64); m],
    }

    if xy.len() == 0 {
        let mut score = 1;
        for _ in 0..n {
            score = (score * 3) % MOD;
        }
        println!("{}", score);
        return;
    }

    let mut s = HashSet::new();
    for (x, y) in xy {
        s.insert((x, y));
    }
    let mut v = HashMap::<(i64, i64), usize>::new();
    v.insert((0, 0), 1);
    for _ in 0..n {
        let mut vnext = HashMap::<(i64, i64), usize>::new();
        for (xy, &n) in &v {
            update(xy, n, &(a, b), &s, &mut vnext);
            update(xy, n, &(c, d), &s, &mut vnext);
            update(xy, n, &(e, f), &s, &mut vnext);
        }
        v = vnext;
    }

    let mut score = 0;
    for (_, &n) in &v {
        score = (score + n) % MOD;
    }
    println!("{}", score);
}
