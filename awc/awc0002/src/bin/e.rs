use std::collections::HashMap;

use proconio::input;

/// 半分全列挙: a から複数選んで和を s にする組み合わせ数の例
fn meet_in_the_middle(s: usize, a: &[usize]) -> usize {
    let n = a.len();
    assert!(n <= 40);

    fn f(a: &[usize]) -> HashMap<usize, usize> {
        let mut m = HashMap::new();
        m.insert(0usize, 1usize);
        for &x in a {
            let mut m0 = m.clone();
            for (&k, &v) in &m {
                *m0.entry(k + x).or_insert(0) += v;
            }
            m = m0;
        }
        m
    }

    let m0 = f(&a[..(n / 2)]);
    let m1 = f(&a[(n / 2)..]);

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
