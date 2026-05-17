use std::collections::HashMap;

use proconio::input;

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn f(a: u128, b: u128) -> (u128, u128) {
    let g = gcd(a, b);
    (a / g, b / g)
}

/// 半分全列挙
fn meet_in_the_middle(pq: &[(u128, u128)]) -> usize {
    let n = pq.len();

    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            if pq[i].0 * pq[j].0 == pq[i].1 * pq[j].1 {
                return 2;
            }
        }
    }

    let mut m0 = HashMap::new();
    for &(p, q) in &pq[..(n / 2)] {
        let mut m = m0.clone();
        m.insert((p, q), 1);
        for (&(p0, q0), &c0) in &m0 {
            let k = f(p0 * p, q0 * q);
            if let Some(c1) = m.get_mut(&k) {
                *c1 = (*c1).min(c0 + 1);
            } else {
                m.insert(k, c0 + 1);
            }
        }
        m0 = m;
    }

    let mut m1 = HashMap::new();
    for &(p, q) in &pq[(n / 2)..] {
        let mut m = m1.clone();
        m.insert((p, q), 1);
        for (&(p0, q0), &c0) in &m1 {
            let k = f(p0 * p, q0 * q);
            if let Some(c1) = m.get_mut(&k) {
                *c1 = (*c1).min(c0 + 1);
            } else {
                m.insert(k, c0 + 1);
            }
        }
        m1 = m;
    }

    let mut result = usize::MAX;
    if let Some(&c0) = m0.get(&(1, 1)) {
        result = result.min(c0);
    }
    if let Some(&c1) = m1.get(&(1, 1)) {
        result = result.min(c1);
    }
    for (&(p0, q0), &c0) in &m0 {
        if let Some(&c1) = m1.get(&(q0, p0)) {
            result = result.min(c0 + c1);
        }
    }

    result
}

fn main() {
    input! {
        n: usize,
        pq: [(u128, u128); n],
    }
    let result = meet_in_the_middle(&pq);
    if result < usize::MAX {
        println!("{result}");
    } else {
        println!("-1");
    }
}
