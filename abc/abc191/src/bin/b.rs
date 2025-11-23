use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    let result = a.iter().filter(|&&y| y != x).join(" ");
    println!("{result}");
}
