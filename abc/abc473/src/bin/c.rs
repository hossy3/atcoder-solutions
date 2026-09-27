use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    }

    let mut counts = vec![0usize; k];
    for a in a {
        counts[a] += 1;
    }
    let max = *counts.iter().max().unwrap();
    let result = counts.iter().filter(|&&x| x + 1 >= max).count();
    println!("{result}");
}
