use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut set = HashSet::new();
    let m = (n as f64).sqrt() as usize;
    for i in 1..=m {
        let x = 2 * i * i;
        if x <= n {
            set.insert(x);
        }

        let x = 4 * i * i;
        if x <= n {
            set.insert(x);
        }
    }

    let result = set.len();
    println!("{result}");
}
