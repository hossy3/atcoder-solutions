use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        c: [Usize1; n],
    }
    let mut counts = vec![0usize; n];
    for &x in &c {
        counts[x] += 1;
    }
    let max = *counts.iter().max().unwrap_or(&0);
    let result = n - max;
    println!("{result}");
}
