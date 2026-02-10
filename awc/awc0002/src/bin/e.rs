use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    // 半分全列挙
    let mut m0 = HashMap::new();
    m0.insert(0usize, 1usize);
    for &x in &a[..(n / 2)] {
        let mut m = m0.clone();
        for (&k, &v) in &m0 {
            *m.entry(k + x).or_insert(0) += v;
        }
        m0 = m;
    }

    let mut m1 = HashMap::new();
    m1.insert(0usize, 1usize);
    for &x in &a[(n / 2)..] {
        let mut m = m1.clone();
        for (&k, &v) in &m1 {
            *m.entry(k + x).or_insert(0) += v;
        }
        m1 = m;
    }

    let mut result = 0usize;
    for (&k0, &v0) in &m0 {
        if k0 <= s {
            if let Some(&v1) = m1.get(&(s - k0)) {
                result += v0 * v1;
            }
        }
    }

    println!("{result}");
}
