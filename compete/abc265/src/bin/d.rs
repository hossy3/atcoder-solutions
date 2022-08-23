use std::collections::HashMap;

use proconio::input;

fn f(cum: &[i64], p: i64) -> HashMap<usize, usize> {
    let n = cum.len() - 1;
    let mut m = HashMap::new();
    let mut r = 1;
    for l in 0..n {
        while r < n && cum[r] - cum[l] < p {
            r += 1;
        }
        if cum[r] - cum[l] == p {
            m.insert(l, r);
        }
    }
    m
}

fn main() {
    input! {
        n: usize,
        p: i64,
        q: i64,
        r: i64,
        a: [i64; n],
    }
    let mut cum = vec![0i64; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }

    let mp = f(&cum, p);
    let mq = f(&cum, q);
    let mr = f(&cum, r);

    for (_, pr) in mp {
        if let Some(&qr) = mq.get(&pr) {
            if mr.contains_key(&qr) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
