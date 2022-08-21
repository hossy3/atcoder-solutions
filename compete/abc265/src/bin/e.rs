use std::collections::{HashMap, HashSet};

use proconio::input;

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

    const MOD: usize = 998244353;
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
        for (&(x1, y1), &n) in &v {
            let x0 = x1 + a;
            let y0 = y1 + b;
            if !s.contains(&(x0, y0)) {
                if let Some(x) = vnext.get_mut(&(x0, y0)) {
                    *x = (*x + n) % MOD;
                } else {
                    vnext.insert((x0, y0), n);
                }
            }

            let x0 = x1 + c;
            let y0 = y1 + d;
            if !s.contains(&(x0, y0)) {
                if let Some(x) = vnext.get_mut(&(x0, y0)) {
                    *x = (*x + n) % MOD;
                } else {
                    vnext.insert((x0, y0), n);
                }
            }

            let x0 = x1 + e;
            let y0 = y1 + f;
            if !s.contains(&(x0, y0)) {
                if let Some(x) = vnext.get_mut(&(x0, y0)) {
                    *x = (*x + n) % MOD;
                } else {
                    vnext.insert((x0, y0), n);
                }
            }
        }
        v = vnext;
    }

    let mut score = 0;
    for (_, &n) in &v {
        score = (score + n) % MOD;
    }
    println!("{}", score);
}
