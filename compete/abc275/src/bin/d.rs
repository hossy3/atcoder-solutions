use std::collections::HashMap;

use proconio::input;

fn f(n: usize, m: &mut HashMap<usize, usize>) -> usize {
    if !m.contains_key(&n) {
        let x = f(n / 2, m) + f(n / 3, m);
        m.insert(n, x);
    }
    m[&n]
}

fn main() {
    input! {
        n: usize,
    }
    let mut m = HashMap::new();
    m.insert(0, 1);
    let result = f(n, &mut m);
    println!("{}", result);
}
