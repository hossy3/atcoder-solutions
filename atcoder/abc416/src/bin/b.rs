use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut t = s.clone();
    let mut b = true;
    for (i, &c) in s.iter().enumerate() {
        match c {
            '#' => {
                b = true;
            }
            '.' => {
                if b {
                    t[i] = 'o';
                    b = false;
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{}", t.iter().join(""));
}
