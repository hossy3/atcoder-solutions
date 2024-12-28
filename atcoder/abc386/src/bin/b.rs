use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut i = 0usize;
    let mut result = 0usize;
    while i < s.len() {
        if i < s.len() - 1 && s[i] == '0' && s[i + 1] == '0' {
            i += 1;
        }
        i += 1;
        result += 1;
    }
    println!("{result}");
}
