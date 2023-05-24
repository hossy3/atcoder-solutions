use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        rca: [(Usize1, Usize1, usize); n],
    }
    let mut rows = vec![vec![]; h];
    let mut cols = vec![vec![]; w];
    for &(r, c, a) in &rca {
        rows[r].push((c, a));
        cols[c].push((r, a));
    }
    for r in 0..h {
        rows[r].sort_by_key(|x| x.1);
    }
    for c in 0..w {
        cols[c].sort_by_key(|x| x.1);
    }

    for i in 0..n {
        let (r, c, a) = rca[i];
        let mut result = 0;
        let mut set = HashSet::new();
        let mut queue = BinaryHeap::new();
        queue.push((Reverse(a), 0, (r, c, a)));
        while let Some((_, step, (r, c, a))) = queue.pop() {
            if !set.insert((r, c, a)) {
                continue;
            }
            result = result.max(step);

            let i = rows[r].upper_bound_by_key(&a, |x| x.1);
            if i < rows[r].len() {
                let a = rows[r][i].1;
                for j in i..(rows[r].len()) {
                    let (c0, a0) = rows[r][j];
                    if a0 == a {
                        let rca = (r, c0, a0);
                        if !set.contains(&rca) {
                            queue.push((Reverse(a0), step + 1, rca));
                        }
                    } else {
                        break;
                    }
                }
            }

            let i = cols[c].upper_bound_by_key(&a, |x| x.1);
            if i < cols[c].len() {
                let a = cols[c][i].1;
                for j in i..(cols[c].len()) {
                    let (r0, a0) = cols[c][j];
                    if a0 == a {
                        let rca = (r0, c, a0);
                        if !set.contains(&rca) {
                            queue.push((Reverse(a0), step + 1, rca));
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        println!("{}", result);
    }
}
