use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        p: [u8; 26],
    }
    let result = p.iter().map(|x| ('a' as u8 + x - 1) as char).join("");
    println!("{}", result);
}
