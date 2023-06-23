use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        mut s: Chars,
        a: Usize1,
        b: Usize1,
    }
    s.swap(a, b);
    let result = s.iter().join("");
    println!("{}", result);
}
