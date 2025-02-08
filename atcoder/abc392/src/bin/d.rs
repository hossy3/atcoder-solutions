use std::collections::HashMap;

use proconio::input;

fn f(a0: &[usize], a1: &[usize]) -> f64 {
    let mut h0 = HashMap::new();
    for &x0 in a0 {
        *h0.entry(x0).or_insert(0usize) += 1;
    }

    let mut h1 = HashMap::new();
    for &x1 in a1 {
        *h1.entry(x1).or_insert(0usize) += 1;
    }

    let n_all = (a0.len() * a1.len()) as f64;

    let mut result = 0.0_f64;
    for (&x, &n0) in &h0 {
        let Some(&n1) = h1.get(&x) else {
            continue;
        };
        let p = n0 as f64 * n1 as f64 / n_all;
        result += p;
    }

    result
}

fn main() {
    input! {
        n: usize,
    }

    let mut m = vec![];
    for _ in 0..n {
        input! {
            k: usize,
            a: [usize; k],
        }
        m.push(a);
    }

    let mut result = 0.0_f64;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let p = f(&m[i], &m[j]);
            result = result.max(p);
        }
    }
    println!("{result}");
}
