use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut x: Chars,
    }
    x.sort();
    if x[0] == '0' {
        for i in 1..(x.len()) {
            if x[i] != '0' {
                x.swap(0, i);
                break;
            }
        }
    }
    println!("{}", x.iter().join(""));
}
