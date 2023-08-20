use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut result = 0;
    let mut i = 0;
    while i < s.len() {
        let c = s[i];
        if c == '0' && i + 1 < s.len() && s[i + 1] == '0' {
            i += 1;
        }
        i += 1;
        result += 1;
    }
    println!("{}", result);
}
