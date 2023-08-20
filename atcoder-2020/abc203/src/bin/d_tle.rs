use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[usize; n]; n],
    }

    let m = (k * k + 1) / 2;
    let mut result = std::usize::MAX;
    for i in 0..=(n - k) {
        let mut set0 = BTreeSet::new();
        let mut set1 = BTreeSet::new();
        for i in i..(i + k) {
            for j in 0..k {
                let x = (a[i][j], i, j);
                if set0.len() < m {
                    set0.insert(x);
                } else if let Some(&y) = set0.iter().last() {
                    if y < x {
                        set1.insert(x);
                    } else {
                        set0.insert(x);
                        set0.remove(&y);
                        set1.insert(y);
                    }
                }
            }
        }
        result = result.min(set0.iter().last().unwrap().0);

        for j in 0..(n - k) {
            for i in i..(i + k) {
                let x = (a[i][j], i, j);
                if set1.contains(&x) {
                    set1.remove(&x);
                } else {
                    set0.remove(&x);
                    if let Some(&y) = set1.iter().next() {
                        set1.remove(&y);
                        set0.insert(y);
                    }
                }

                let x = (a[i][j + k], i, j + k);
                if set0.len() < m {
                    set0.insert(x);
                } else if let Some(&y) = set0.iter().last() {
                    if y < x {
                        set1.insert(x);
                    } else {
                        set0.insert(x);
                        set0.remove(&y);
                        set1.insert(y);
                    }
                }
            }
            result = result.min(set0.iter().last().unwrap().0);
        }
    }

    println!("{}", result);
}
