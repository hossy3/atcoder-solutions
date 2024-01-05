use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut t = (0usize, 0usize);
    let mut result = 0usize;
    for i in 0..n {
        if s[i] == '0' {
            t = (1, t.0 + t.1);
        } else {
            t = (t.1, t.0 + 1);
        }
        result += t.1;
    }
    println!("{}", result);
}
