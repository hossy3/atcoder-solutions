use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }
    let result = if n == m {
        if s == t {
            0
        } else {
            3
        }
    } else if n > m {
        3
    } else {
        let prefix = (0..n).all(|i| s[i] == t[i]);
        let suffix = (0..n).all(|i| s[i] == t[m - n + i]);
        (if prefix { 0 } else { 2 }) + (if suffix { 0 } else { 1 })
    };
    println!("{result}");
}
