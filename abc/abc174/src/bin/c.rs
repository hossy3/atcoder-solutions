use std::collections::HashSet;

use proconio::input;

fn f(k: usize) -> i64 {
    let mut m = 7 % k;
    let mut len = 1;
    let mut set = HashSet::new();
    while !set.contains(&m) {
        set.insert(m);
        if m % k == 0 {
            return len;
        }
        m = (m * 10 + 7) % k;
        len += 1;
    }
    -1
}

fn main() {
    input! {
        k: usize,
    }
    let result = f(k);
    println!("{result}");
}
