use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n]
    }
    let count = a
        .iter()
        .combinations(5)
        .filter(|v| v.iter().fold(1, |acc, &&x| (acc * x) % p) == q)
        .count();
    println!("{count}");
}
