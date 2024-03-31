use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, d): (usize, u32),
        a: [i64; n],
    }
    let mut result = 1i64 << d;
    for i in 0..n {
        for v in a.iter().combinations(i + 1) {
            let x = v.iter().fold(0i64, |acc, &&x| acc | x);
            let num = 1i64 << (d - x.count_ones());
            result -= (-1i64).pow(i as u32) * num;
        }
    }
    println!("{result}");
}
