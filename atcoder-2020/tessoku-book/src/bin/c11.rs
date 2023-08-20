use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    const TOL: usize = 1_000_000;

    let mut v = vec![0; n];
    let sum_vote: usize = a.iter().sum();
    let mut sum = 0;
    let mut h = BinaryHeap::new();
    for i in 0..n {
        let x = a[i] * k / sum_vote;
        v[i] = x;
        sum += x;
        h.push((a[i] * TOL / (x + 1), i));
    }

    while sum < k {
        if let Some((_, i)) = h.pop() {
            v[i] += 1;
            sum += 1;
            h.push((a[i] * TOL / (v[i] + 1), i));
        } else {
            panic!()
        }
    }

    print!("{}", v[0]);
    for i in 1..n {
        print!(" {}", v[i]);
    }
    println!();
}
