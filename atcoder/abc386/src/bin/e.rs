use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let k0 = if k * 2 > n { n - k } else { k };

    let mut v = vec![HashSet::new(); k0 + 1];
    v[0].insert(0usize);
    for &x in &a {
        for i in (0..k0).rev() {
            let mut v0 = vec![];
            for &y in &v[i] {
                v0.push(x ^ y);
            }
            for y in v0 {
                v[i + 1].insert(y);
            }
        }
    }

    if k * 2 > n {
        let sum = a.iter().fold(0usize, |acc, x| acc ^ x);
        let Some(result) = v[k0].iter().map(|&x| x ^ sum).max() else {
            unreachable!()
        };
        println!("{result}");
    } else {
        let Some(&result) = v[k0].iter().max() else {
            unreachable!()
        };
        println!("{result}");
    }
}
