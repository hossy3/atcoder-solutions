use std::collections::HashMap;

use proconio::input;

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

    let mut mp = HashMap::new();
    let mut r0 = 0;
    for l in 0..n {
        while r0 < n && cum[r0] - cum[l] < p {
            r0 += 1;
        }
        if cum[r0] - cum[l] == p {
            mp.insert(l, r0);
        }
    }

    let mut mq = HashMap::new();
    let mut r0 = 0;
    for l in 0..n {
        while r0 < n && cum[r0] - cum[l] < q {
            r0 += 1;
        }
        if cum[r0] - cum[l] == q {
            mq.insert(l, r0);
        }
    }

    let mut mr = HashMap::new();
    let mut r0 = 0;
    for l in 0..n {
        while r0 < n && cum[r0] - cum[l] < r {
            r0 += 1;
        }
        if cum[r0] - cum[l] == r {
            mr.insert(l, r0);
        }
    }

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
