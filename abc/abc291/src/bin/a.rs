use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    for (i, &c) in s.iter().enumerate() {
        if c >= 'A' && c <= 'Z' {
            println!("{}", i + 1);
            break;
        }
    }
}
