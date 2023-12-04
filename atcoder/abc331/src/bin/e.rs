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

    let mut a0 = Vec::with_capacity(n);
    for (i, &x) in a.iter().enumerate() {
        a0.push((x, i));
    }
    a0.sort();
    a0.reverse();

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
    for &(x, i) in &a0 {
        if result >= x + b0[0].0 {
            break;
        }
        for &(y, j) in &b0 {
            if !set.contains(&(i, j)) {
                result = result.max(x + y);
                break;
            }
        }
    }
    println!("{result}");
}
