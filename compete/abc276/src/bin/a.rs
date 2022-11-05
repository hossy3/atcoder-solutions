use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut result = -1;
    for i in 0..s.len() {
        if s[i] == 'a' {
            result = (i + 1) as i64;
        }
    }
    println!("{}", result);
}
