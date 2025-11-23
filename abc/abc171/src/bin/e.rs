use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let sum = a.iter().fold(0, |acc, x| acc ^ x);
    let result = a.iter().map(|x| sum ^ x).join(" ");
    println!("{result}");
}
