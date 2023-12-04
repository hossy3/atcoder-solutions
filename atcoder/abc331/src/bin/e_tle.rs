use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [usize; n],
        b: [usize; m],
        cd: [(Usize1, Usize1); l],
    }

    let mut b0 = Vec::with_capacity(m);
    for (i, &x) in b.iter().enumerate() {
        b0.push((x, i));
    }
    b0.sort();
    b0.reverse();

    let mut set = HashSet::new();
    for &(c, d) in &cd {
        set.insert((c, d));
    }

    let mut result = 0usize;
    for (i, &x) in a.iter().enumerate() {
        for &(y, j) in &b0 {
            if !set.contains(&(i, j)) {
                result = result.max(x + y);
                continue;
            }
        }
    }
    println!("{result}");
}
