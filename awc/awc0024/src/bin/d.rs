use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        w: usize,
        k: usize,
        l: [Usize1; k],
    }

    let mut imos = vec![0isize; n + 1];
    for &i in &l {
        imos[i] += 1;
        imos[i + w] -= 1;
    }
    for i in 1..=n {
        imos[i] += imos[i - 1];
    }
    imos.pop();

    println!("{}", imos.iter().join(" "));
}
