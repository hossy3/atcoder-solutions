use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }

    let mut rest: i64 = ab.iter().map(|&(a, _)| a).sum();

    let mut a = ab.iter().map(|(a, b)| a * 2 + b).collect_vec();
    a.sort();
    a.reverse();

    for (i, &a) in a.iter().enumerate() {
        if rest < 0 {
            println!("{i}");
            return;
        }
        rest -= a;
    }
    println!("{n}");
}
