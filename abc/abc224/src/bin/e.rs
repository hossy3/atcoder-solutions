use std::collections::HashMap;

use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut rca: [(Usize1, Usize1, usize); n],
    }
    let mut rows = vec![vec![]; h];
    let mut cols = vec![vec![]; w];
    let mut map = HashMap::new();
    for (i, &(r, c, a)) in rca.iter().enumerate() {
        rows[r].push((c, a));
        cols[c].push((r, a));
        map.insert((r, c), i);
    }
    for r in 0..h {
        rows[r].sort_by_key(|x| x.1);
    }
    for c in 0..w {
        cols[c].sort_by_key(|x| x.1);
    }
    rca.sort_by_key(|x| x.2);

    let mut rows_cache = vec![(0, 0); h]; // a, step
    let mut cols_cache = vec![(0, 0); w];

    let mut v = vec![0; n];
    for &(r, c, a) in rca.iter().rev() {
        let &i = map.get(&(r, c)).unwrap();
        let mut result = 0;

        let j = rows[r].upper_bound_by_key(&a, |x| x.1);
        if j < rows[r].len() {
            let a = rows[r][j].1;
            if rows_cache[r].0 == a {
                result = rows_cache[r].1;
            } else {
                for j in j..(rows[r].len()) {
                    let (c0, a0) = rows[r][j];
                    if a0 == a {
                        let &i0 = map.get(&(r, c0)).unwrap();
                        result = result.max(v[i0] + 1);
                    } else {
                        break;
                    }
                }
                rows_cache[r] = (a, result);
            }
        }

        let j = cols[c].upper_bound_by_key(&a, |x| x.1);
        if j < cols[c].len() {
            let a = cols[c][j].1;
            if cols_cache[c].0 == a {
                result = cols_cache[c].1;
            } else {
                for j in j..(cols[c].len()) {
                    let (r0, a0) = cols[c][j];
                    if a0 == a {
                        let &i0 = map.get(&(r0, c)).unwrap();
                        result = result.max(v[i0] + 1);
                    } else {
                        break;
                    }
                }
                cols_cache[c] = (a, result);
            }
        }

        v[i] = result;
    }

    for &x in &v {
        println!("{}", x);
    }
}
