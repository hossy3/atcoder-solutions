use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut v = vec![];
    for (i, &x) in p.iter().enumerate() {
        v.push((x, i, 1));
    }
    v.sort_by_key(|&(x, _, _)| Reverse(x));

    for i in 1..n {
        if v[i].0 == v[i - 1].0 {
            v[i].2 = v[i - 1].2;
        } else {
            v[i].2 = i + 1;
        }
    }

    v.sort_by_key(|&(_, i, _)| i);
    let results = v.iter().map(|&(_, _, j)| j).collect::<Vec<_>>();
    for &x in &results {
        println!("{x}");
    }
}
