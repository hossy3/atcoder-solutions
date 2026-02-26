use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        v: u128,
        d: [u128; n - 1],
        t: [u128; n - 1],
    }

    let mut results = vec![];
    let mut d_sum = 0u128;
    for i in 0..n - 1 {
        d_sum += d[i];
        if d_sum < t[i] * v {
            results.push(i + 2);
        }
    }

    if results.is_empty() {
        println!("-1");
    } else {
        println!("{}", results.iter().join(" "));
    }
}
