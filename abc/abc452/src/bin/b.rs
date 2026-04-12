use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    println!("{}", vec!['#'; w].iter().join(""));
    for _ in 1..(h - 1) {
        println!("#{}#", vec!['.'; w - 2].iter().join(""));
    }
    println!("{}", vec!['#'; w].iter().join(""));
}
