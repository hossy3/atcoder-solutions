use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let c = if s[0] != s[1] && s[1] == s[2] {
        s[1]
    } else {
        s[0]
    };
    let result = s.iter().position(|&c0| c0 != c).unwrap() + 1;
    println!("{result}");
}
