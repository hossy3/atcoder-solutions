use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }
    let mut results = vec![0; n - 1];
    for i in 1..n {
        for j in (i + 1)..=n {
            let dist = (j - i)
                .min(x.abs_diff(i) + y.abs_diff(j) + 1)
                .min(x.abs_diff(j) + y.abs_diff(i) + 1);
            results[dist - 1] += 1;
        }
    }
    println!("{}", results.iter().join("\n"));
}
