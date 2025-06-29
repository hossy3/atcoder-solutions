use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        xy: [(Usize1, Usize1); n],
        q: usize,
        query: [(u8, Usize1); q],
    }

    let mut x2ys = vec![HashSet::new(); h];
    let mut y2xs = vec![HashSet::new(); w];
    for &(x, y) in &xy {
        x2ys[x].insert(y);
        y2xs[y].insert(x);
    }

    for &(k, val) in &query {
        match k {
            1 => {
                let x = val;

                let result = x2ys[x].len();
                println!("{result}");

                for &y in &x2ys[x] {
                    y2xs[y].remove(&x);
                }
                x2ys[x].clear();
            }
            2 => {
                let y = val;

                let result = y2xs[y].len();
                println!("{result}");

                for &x in &y2xs[y] {
                    x2ys[x].remove(&y);
                }
                y2xs[y].clear();
            }
            _ => unreachable!(),
        }
    }
}
