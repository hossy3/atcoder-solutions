use std::collections::HashMap;

use proconio::input;

/// 半分全列挙: a から複数選んで和を s にする組み合わせ数の例
fn meet_in_the_middle(s: usize, a: &[usize]) -> usize {
    let n = a.len();
    assert!(n <= 40);

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

    result
}

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    let result = meet_in_the_middle(s, &a);
    println!("{result}");
}
