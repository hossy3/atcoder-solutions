use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut v = vec![HashSet::new(); k + 1];
    v[0].insert(0usize);
    for &x in &a {
        for i in (0..k).rev() {
            let mut v0 = vec![];
            for &y in &v[i] {
                v0.push(x ^ y);
            }
            for y in v0 {
                v[i + 1].insert(y);
            }
        }
    }

    let Some(&result) = v[k].iter().max() else {
        unreachable!()
    };
    println!("{result}");
}
