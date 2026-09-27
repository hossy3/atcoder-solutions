use ac_library::z_algorithm;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        q: usize,
        s: String,
        t: String,
        lr: [(Usize1, Usize1); q],
    }

    let str = t.clone() + &s;
    let lcp = z_algorithm(str.as_str());

    let mut v = vec![false; s.len()];
    for i in 0..s.len() {
        if lcp[i + t.len()] >= t.len() {
            v[i] = true;
        }
    }

    let mut sum = vec![0usize; s.len() + 1];
    for i in 0..s.len() {
        sum[i + 1] = sum[i] + if v[i] { 1 } else { 0 };
    }

    // eprintln!("{:?}", &sum);

    let mut results = vec![];
    for (l, r) in lr {
        // eprintln!("{}, {}", r - l + 1, t.len());

        if r - l + 1 < t.len() {
            results.push(false);
            continue;
        }
        let r0 = sum[r - t.len() + 2];
        let l0 = sum[l];
        let yes = r0 > l0;
        results.push(yes);
    }

    for result in results {
        println!("{}", if result { "Yes" } else { "No" });
    }
}
