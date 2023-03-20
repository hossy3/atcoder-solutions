use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut s0 = BTreeSet::new();
    for i in 0..n {
        s0.insert(i + 1);
    }

    let mut s1 = BTreeSet::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                let x = *s0.iter().next().unwrap();
                s0.remove(&x);
                s1.insert(x);
            }
            2 => {
                input! {
                    x: usize,
                }
                s1.remove(&x);
            }
            _ => {
                let x = *s1.iter().next().unwrap();
                println!("{}", x);
            }
        }
    }
}
