use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let o = vec!['o'; n].iter().join("");
    println!("L{o}ng");
}
