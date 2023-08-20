use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }

    let mut base = 0;
    let mut m = HashMap::new();
    for i in 0..n {
        m.insert(i, a[i]);
    } 

    for _ in 0..q {
        input! {
            k: usize,
        }
        match k {
            1 => {
                input! {
                    x: usize,
                }
                base = x;
                m.clear();
            }
            2 => {
                input! {
                    i: Usize1,
                    x: usize,
                }
                m.insert(i, m.get(&i).unwrap_or(&base) + x);
            }
            3 => {
                input! {
                    i: Usize1,
                }
                println!("{}", m.get(&i).unwrap_or(&base));
            }
            _ => {}
        }
    }
}
