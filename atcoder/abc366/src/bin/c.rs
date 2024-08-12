use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut map = HashMap::new();
    for _ in 0..q {
        input! {
            k: usize,
        }
        match k {
            1 => {
                input! {
                    x: usize,
                }
                *map.entry(x).or_insert(0usize) += 1;
            }
            2 => {
                input! {
                    x: usize,
                }
                let Some(n) = map.remove(&x) else { unreachable!() };
                if n > 1 {
                    map.insert(x, n - 1);
                }
            }
            3 => {
                let result = map.len();
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
