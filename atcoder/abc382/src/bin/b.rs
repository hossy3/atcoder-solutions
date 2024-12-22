use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut d: usize,
        mut s: Chars,
    }

    for i in (0..n).rev() {
        if s[i] == '@' {
            s[i] = '.';
            d -= 1;
            if d == 0 {
                break;
            }
        }
    }

    let result = s.iter().join("");
    println!("{result}");
}
