use proconio::{input, marker::Chars};

fn f(c: char) -> usize {
    c as usize - '0' as usize
}

fn main() {
    input! {
        s: Chars,
    }
    let result = f(s[0]) * f(s[2]);
    println!("{result}");
}
