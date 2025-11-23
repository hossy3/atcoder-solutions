use std::collections::HashSet;

use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut set = HashSet::new();
    let m = n.sqrt();
    for i in 2..=m {
        let mut x = i * i;
        if set.contains(&x) {
            continue;
        }
        while x <= n {
            set.insert(x);
            x *= i;
        }
    }
    let result = n - set.len();
    println!("{}", result);
}
