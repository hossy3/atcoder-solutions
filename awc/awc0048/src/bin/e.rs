use std::collections::HashMap;

use proconio::input;

const N: usize = 1_000_000_007;

fn f(a: &[usize], m: usize) -> Vec<HashMap<usize, usize>> {
    let mut results = vec![HashMap::new(); a.len() + 1];
    results[0].insert(0, 1);
    for (i, &x) in a.iter().enumerate() {
        for j in (0..=i).rev() {
            let (v0, v1) = results.split_at_mut(j + 1);
            for (&k, &v) in &v0[j] {
                let k = (k + x) % m;
                *v1[0].entry(k).or_insert(0) += v;
            }
        }
    }
    results
}

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        a: [usize; n],
    }

    let v0 = f(&a[..(n / 2)], m);
    let v1 = f(&a[(n / 2)..], m);

    let mut result = 0;
    for i0 in 0..(v0.len().min(k + 1)) {
        for (&k0, &v0) in &v0[i0] {
            let i1 = k - i0;
            if i1 < v1.len() {
                let k1 = (m - k0) % m;
                if let Some(&v1) = v1[i1].get(&k1) {
                    result = (result + v0 * v1) % N;
                }
            }
        }
    }
    println!("{result}");
}
