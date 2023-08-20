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
            x: String,
        }

        if k == 1 {
            input! {
                y: usize,
            }
            map.insert(x, y);
        } else {
            if let Some(y) = map.get(&x) {
                println!("{}", y);
            }
        }
    }
}
