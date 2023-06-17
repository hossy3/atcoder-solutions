use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; m]; n],
    }

    let mut result = 0usize;
    for i in 0..(n - 1) {
        let mut set = BTreeSet::new();
        for &x in &a[i] {
            set.insert(x);
        }
        for j in (i + 1)..n {
            let mut set = set.clone();
            for &x in &a[j] {
                set.insert(x);
            }
            result += a[i]
                .iter()
                .map(|&x| set.range(0..=x).count())
                .into_iter()
                .sum::<usize>();
        }
    }
    println!("{}", result);
}
