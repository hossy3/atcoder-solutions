use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let result = if s[s.len() - 1] == 'r' { "er" } else { "ist" };
    println!("{}", result);
}
