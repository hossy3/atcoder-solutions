use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    let c = s[s.len() - 1];
    println!("{}", c);
}
