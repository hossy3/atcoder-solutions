use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        k: i64,
    }
    let mut count = -1; // ignore 0
    for j in 1..=10 {
        let iter = (0..=9)
            .combinations(j)
            .map(|v| v.iter().rev().fold(0usize, |acc, x| acc * 10 + x))
            .sorted();
        for x in iter {
            count += 1;
            if count == k {
                println!("{x}");
                return;
            }
        }
    }
}
