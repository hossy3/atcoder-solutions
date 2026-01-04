use std::collections::HashSet;

use proconio::input;

fn f(mut n: usize) -> bool {
    let mut set = HashSet::new();
    while set.insert(n) {
        let mut x = 0;
        while n > 0 {
            x += (n % 10).pow(2);
            n /= 10;
        }
        if x == 1 {
            return true;
        }
        n = x;
    }
    false
}

fn main() {
    input! {
        n: usize,
    }
    let yes = f(n);
    println!("{}", if yes { "Yes" } else { "No" });
}
