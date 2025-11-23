use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    s.sort();
    let result = if s[0] == s[1] { s[s.len() - 1] } else { s[0] };
    println!("{result}");
}
