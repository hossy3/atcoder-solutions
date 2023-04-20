use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut set = HashSet::new();
    for &x in &b {
        set.insert(x);
    }
    let x = a.iter().zip(b.iter()).filter(|(&a, &b)| a == b).count();
    let y = a.iter().filter(|&x| set.contains(&x)).count() - x;
    println!("{}", x);
    println!("{}", y);
}
