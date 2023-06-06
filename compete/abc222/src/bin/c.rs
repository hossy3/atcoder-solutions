use std::cmp::Reverse;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; 2 * n],
    }
    let mut v = vec![];
    for i in 0..(2 * n) {
        v.push((Reverse(0), i));
    }
    for i in 0..m {
        for j in 0..n {
            let j0 = j * 2;
            let j1 = j * 2 + 1;
            let c0 = a[v[j0].1][i];
            let c1 = a[v[j1].1][i];
            if (c0 == 'G' && c1 == 'C') || (c0 == 'C' && c1 == 'P') || (c0 == 'P' && c1 == 'G') {
                v[j0].0 = Reverse(v[j0].0 .0 + 1);
            } else if (c1 == 'G' && c0 == 'C')
                || (c1 == 'C' && c0 == 'P')
                || (c1 == 'P' && c0 == 'G')
            {
                v[j1].0 = Reverse(v[j1].0 .0 + 1);
            }
        }
        v.sort();
    }

    for (_, x) in v {
        println!("{}", x + 1);
    }
}
