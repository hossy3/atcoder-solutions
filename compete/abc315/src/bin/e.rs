use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }

    let mut v = vec![];
    for _ in 0..n {
        input! {
            c: usize,
            p: [Usize1; c],
        }
        v.push(p);
    }

    let mut results = vec![];
    let mut queue = vec![0];
    let mut set = HashSet::<usize>::new();
    while let Some(&x) = queue.last() {
        if set.contains(&x) {
            queue.pop();
            continue;
        }
        let mut added = false;
        for &x in &v[x] {
            if !set.contains(&x) {
                queue.push(x);
                added = true;
            }
        }
        if !added {
            queue.pop();
            set.insert(x);
            results.push(x);
        }
    }

    results.pop();
    let result = results.iter().map(|i| i + 1).join(" ");
    println!("{result}");
}
