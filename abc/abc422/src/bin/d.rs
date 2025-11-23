use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u32,
        k: usize,
    }

    let m = 2usize.pow(n);
    let mut results = Vec::with_capacity(m);
    for i in 0..m {
        results.push(((i + 1) * k / m) - (i * k / m));
    }

    let x = if k % m == 0 { 0 } else { 1 };
    println!("{x}");
    println!("{}", results.iter().join(" "));
}
