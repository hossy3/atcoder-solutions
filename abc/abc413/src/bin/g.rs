use std::collections::HashMap;

use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        rc: [(Usize1, Usize1); k],
    }

    let mut map = HashMap::new();
    for (i, &(r, c)) in rc.iter().enumerate() {
        map.insert((r, c), i);
    }

    let mut dsu = Dsu::new(k);
    for (i, &(r, c)) in rc.iter().enumerate() {
        if r > 0 {
            if c > 0 {
                if let Some(&j) = map.get(&(r - 1, c - 1)) {
                    dsu.merge(i, j);
                }
            }
            if c < w - 1 {
                if let Some(&j) = map.get(&(r - 1, c + 1)) {
                    dsu.merge(i, j);
                }
            }
            if let Some(&j) = map.get(&(r - 1, c)) {
                dsu.merge(i, j);
            }
        }
        if r < h - 1 {
            if c > 0 {
                if let Some(&j) = map.get(&(r + 1, c - 1)) {
                    dsu.merge(i, j);
                }
            }
            if c < w - 1 {
                if let Some(&j) = map.get(&(r + 1, c + 1)) {
                    dsu.merge(i, j);
                }
            }
            if let Some(&j) = map.get(&(r + 1, c)) {
                dsu.merge(i, j);
            }
        }
        if c > 0 {
            if let Some(&j) = map.get(&(r, c - 1)) {
                dsu.merge(i, j);
            }
        }
        if c < w - 1 {
            if let Some(&j) = map.get(&(r, c + 1)) {
                dsu.merge(i, j);
            }
        }
    }

    for v in dsu.groups() {
        let mut min_r = usize::MAX;
        let mut max_r = 0;
        let mut min_c = usize::MAX;
        let mut max_c = 0;
        for &i in &v {
            let (r, c) = rc[i];
            min_r = min_r.min(r);
            max_r = max_r.max(r);
            min_c = min_c.min(c);
            max_c = max_c.max(c);
        }
        if (min_r == 0 && max_r == h - 1)
            || (min_c == 0 && max_c == w - 1)
            || (min_r == 0 && min_c == 0)
            || (max_r == h - 1 && max_c == w - 1)
        {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
