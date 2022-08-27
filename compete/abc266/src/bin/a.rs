use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let i = (s.len() - 1) / 2;
    println!("{}", s[i]);
}
