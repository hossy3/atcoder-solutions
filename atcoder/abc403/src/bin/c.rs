use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
    }

    let mut v = vec![HashSet::new(); n]; // [ユーザー][コンテスト] m は all

    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }
                v[x].insert(y);
            }
            2 => {
                input! {
                    x: Usize1,
                }
                v[x].insert(m);
            }
            3 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }
                let yes = v[x].contains(&m) || v[x].contains(&y);
                println!("{}", if yes { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
}
