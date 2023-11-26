use std::collections::BTreeSet;

use proconio::input;

fn f(a: &[Vec<usize>]) -> usize {
    let n = a.len();
    let mut result = 0usize;
    let mut set = BTreeSet::new();
    for (i, a) in a.iter().enumerate().rev() {
        for &x in a {
            result += set.range(0..x).count();
        }

        let mut set0 = BTreeSet::new();
        for &x in a {
            set0.insert(x);
        }
        for &x in a {
            result += set0.range(0..=x).count() * (n - i - 1);
        }
        set.append(&mut set0);
    }
    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; m]; n],
    }
    let result = f(&a);
    println!("{}", result);
}
